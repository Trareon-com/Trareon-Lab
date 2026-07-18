//! Import a disk image into case ledgers (Day 13+).

use std::path::Path;

use lab_case::{CaseDb, EvidenceObject, ProvenanceEvent};
use lab_core::{LabResult, NullProgress};

use crate::e01::E01Image;
use crate::image::ImageReader;
use crate::raw::{detect_image_kind, ImageKind, RawImage};

/// Result of importing an image into the case registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportImageResult {
    pub evidence_uuid: String,
    pub sha256_hex: String,
    pub byte_length: u64,
    pub display_name: String,
}

/// Open any supported image as a boxed [`ImageReader`].
pub fn open_image(path: &Path) -> LabResult<Box<dyn ImageReader>> {
    match detect_image_kind(path)? {
        ImageKind::RawDd => Ok(Box::new(RawImage::open_raw(path)?)),
        ImageKind::E01 => Ok(Box::new(E01Image::open(path)?)),
    }
}

/// Import raw/dd or E01 path: hash, append evidence + provenance (fail closed).
pub fn import_raw_image(
    db: &CaseDb,
    case_uuid: &str,
    path: &Path,
    evidence_uuid: &str,
    provenance_uuid: &str,
    now_utc: &str,
) -> LabResult<ImportImageResult> {
    let kind = detect_image_kind(path)?;
    let (sha256_hex, byte_length, extra_json) = match kind {
        ImageKind::RawDd => {
            let mut image = RawImage::open_raw(path)?;
            let sha = image.sha256_hex()?;
            let len = image.byte_length();
            (sha, len, String::new())
        }
        ImageKind::E01 => {
            let mut image = E01Image::open(path)?;
            let meta = image.metadata().clone();
            let len = image.byte_length();
            // Hash virtual media via ImageReader
            let sha = hash_image_reader(&mut image)?;
            let case_json =
                serde_json::to_string(&meta.case_number).unwrap_or_else(|_| "\"\"".into());
            let exam_json = serde_json::to_string(&meta.examiner).unwrap_or_else(|_| "\"\"".into());
            let hashes =
                serde_json::to_string(&meta.hash_algorithms).unwrap_or_else(|_| "[]".into());
            let extra = format!(
                ",\"e01\":{{\"case_number\":{case_json},\"examiner\":{exam_json},\"hash_algorithms\":{hashes},\"crc_errors\":{}}}",
                image.crc_errors()
            );
            (sha, len, extra)
        }
    };

    let display_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("image")
        .to_string();

    db.append_evidence_object(&EvidenceObject {
        evidence_uuid: evidence_uuid.to_string(),
        case_uuid: case_uuid.to_string(),
        created_at_utc: now_utc.to_string(),
        display_name: display_name.clone(),
        evidence_class: "disk_image".to_string(),
        validation_state: "ComputedUnanchored".to_string(),
    })?;

    let path_json =
        serde_json::to_string(&path.display().to_string()).unwrap_or_else(|_| "\"\"".into());
    let activity = match kind {
        ImageKind::RawDd => "import_raw_image",
        ImageKind::E01 => "import_e01_image",
    };
    db.append_provenance_event(&ProvenanceEvent {
        event_uuid: provenance_uuid.to_string(),
        case_uuid: case_uuid.to_string(),
        created_at_utc: now_utc.to_string(),
        evidence_uuid: evidence_uuid.to_string(),
        activity: activity.to_string(),
        detail_json: format!(
            "{{\"path\":{path_json},\"sha256\":\"{sha256_hex}\",\"bytes\":{byte_length},\"kind\":\"{}\"{extra_json}}}",
            kind.as_str()
        ),
    })?;

    Ok(ImportImageResult {
        evidence_uuid: evidence_uuid.to_string(),
        sha256_hex,
        byte_length,
        display_name,
    })
}

fn hash_image_reader(image: &mut dyn ImageReader) -> LabResult<String> {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 64 * 1024];
    let mut off = 0_u64;
    let total = image.byte_length();
    let mut progress = NullProgress;
    while off < total {
        let want = std::cmp::min(buf.len() as u64, total - off) as usize;
        let n = image.read_at(off, &mut buf[..want])?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
        off += n as u64;
        let _ = &mut progress;
    }
    Ok(hex::encode(hasher.finalize()))
}
