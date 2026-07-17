//! Import a raw disk image into case ledgers (Day 13).

use std::path::Path;

use lab_case::{CaseDb, EvidenceObject, ProvenanceEvent};
use lab_core::LabResult;

use crate::raw::{detect_image_kind, RawImage};

/// Result of importing a raw image into the case registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportImageResult {
    pub evidence_uuid: String,
    pub sha256_hex: String,
    pub byte_length: u64,
    pub display_name: String,
}

/// Import raw/dd path: hash, append evidence + provenance (fail closed).
pub fn import_raw_image(
    db: &CaseDb,
    case_uuid: &str,
    path: &Path,
    evidence_uuid: &str,
    provenance_uuid: &str,
    now_utc: &str,
) -> LabResult<ImportImageResult> {
    let _kind = detect_image_kind(path)?;
    let mut image = RawImage::open_raw(path)?;
    let sha256_hex = image.sha256_hex()?;
    let display_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("image.raw")
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
    db.append_provenance_event(&ProvenanceEvent {
        event_uuid: provenance_uuid.to_string(),
        case_uuid: case_uuid.to_string(),
        created_at_utc: now_utc.to_string(),
        evidence_uuid: evidence_uuid.to_string(),
        activity: "import_raw_image".to_string(),
        detail_json: format!(
            "{{\"path\":{path_json},\"sha256\":\"{sha256_hex}\",\"bytes\":{}}}",
            image.byte_length()
        ),
    })?;

    Ok(ImportImageResult {
        evidence_uuid: evidence_uuid.to_string(),
        sha256_hex,
        byte_length: image.byte_length(),
        display_name,
    })
}
