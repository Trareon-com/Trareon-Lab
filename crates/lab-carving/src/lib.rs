//! Signature-based file carving.

use lab_core::{LabError, LabResult, ProgressEvent, ProgressSink};
use lab_storage::ImageReader;
use sha2::{Digest, Sha256};

/// Default RAM ceiling for full-image carve (ponytail: streaming window later).
pub const DEFAULT_MAX_CARVE_BYTES: u64 = 64 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CarveConfidence {
    High,
    Medium,
    Low,
    Fragment,
}

#[derive(Debug, Clone)]
pub struct FileSignature {
    pub ext: &'static str,
    pub mime: &'static str,
    pub header: &'static [u8],
    /// Bytes to skip before matching `header` (e.g. MP4 `ftyp` at offset 4).
    pub header_offset: usize,
    pub footer: Option<&'static [u8]>,
    pub min_size: usize,
    pub max_size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CarvedFile {
    pub signature_name: String,
    pub offset_start: u64,
    pub offset_end: u64,
    pub sha256: String,
    pub confidence: CarveConfidence,
}

pub struct Carver {
    pub signatures: Vec<FileSignature>,
    pub max_image_bytes: u64,
}

impl Default for Carver {
    fn default() -> Self {
        Self {
            signatures: default_signatures(),
            max_image_bytes: DEFAULT_MAX_CARVE_BYTES,
        }
    }
}

impl Carver {
    /// Media/docs set without noisy PE (`MZ`) hits — preferred for Quick Verify.
    pub fn common_media() -> Self {
        Self {
            signatures: default_signatures()
                .into_iter()
                .filter(|s| s.ext != "exe")
                .collect(),
            max_image_bytes: DEFAULT_MAX_CARVE_BYTES,
        }
    }

    pub fn carve(
        &self,
        image: &mut dyn ImageReader,
        progress: &mut dyn ProgressSink,
    ) -> LabResult<Vec<CarvedFile>> {
        let total = image.byte_length();
        if total > self.max_image_bytes {
            return Err(LabError::ScopeDenied {
                detail: format!(
                    "carve refused: image {total} bytes exceeds {} byte RAM ceiling",
                    self.max_image_bytes
                ),
            });
        }
        let mut data = vec![0u8; total as usize];
        let mut done = 0u64;
        while done < total {
            if progress.is_cancelled() {
                break;
            }
            let n = image.read_at(done, &mut data[done as usize..])?;
            if n == 0 {
                break;
            }
            done += n as u64;
            progress.report(ProgressEvent::new(
                "carve.scan",
                done,
                Some(total),
                format!("read {done}/{total}"),
            ));
        }
        let data = &data[..done as usize];
        let mut found = Vec::new();
        for sig in &self.signatures {
            let mut search = 0usize;
            while let Some(rel) = find_signature_at(&data[search..], sig) {
                let start = search + rel;
                let body_start = start + sig.header_offset;
                let (end, confidence) = match sig.footer {
                    Some(footer) => {
                        let search_to = (body_start + sig.max_size).min(data.len());
                        if body_start >= search_to {
                            search = start + 1;
                            continue;
                        }
                        if let Some(frel) = find_subslice(&data[body_start..search_to], footer) {
                            let end = body_start + frel + footer.len();
                            if end - start >= sig.min_size {
                                (end, CarveConfidence::High)
                            } else {
                                search = start + 1;
                                continue;
                            }
                        } else {
                            let end =
                                (start + sig.max_size.min(data.len() - start)).min(data.len());
                            (end, CarveConfidence::Medium)
                        }
                    }
                    None => {
                        let end = (start + sig.max_size).min(data.len());
                        (end, CarveConfidence::Medium)
                    }
                };
                let slice = &data[start..end];
                let mut hasher = Sha256::new();
                hasher.update(slice);
                found.push(CarvedFile {
                    signature_name: sig.ext.to_string(),
                    offset_start: start as u64,
                    offset_end: end as u64,
                    sha256: hex::encode(hasher.finalize()),
                    confidence,
                });
                search = end.max(start + 1);
            }
        }
        Ok(found)
    }
}

fn find_signature_at(hay: &[u8], sig: &FileSignature) -> Option<usize> {
    let needle = sig.header;
    if needle.is_empty() {
        return None;
    }
    let skip = sig.header_offset;
    if hay.len() < skip + needle.len() {
        return None;
    }
    if skip == 0 {
        return find_subslice(hay, needle);
    }
    // Match header at absolute offset `i + skip` within hay window starting at i.
    hay.windows(skip + needle.len())
        .position(|w| &w[skip..] == needle)
}

fn find_subslice(hay: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || hay.len() < needle.len() {
        return None;
    }
    hay.windows(needle.len()).position(|w| w == needle)
}

/// Default signature table. OOXML (docx/xlsx/pptx) shares ZIP magic — do not duplicate.
pub fn default_signatures() -> Vec<FileSignature> {
    vec![
        FileSignature {
            ext: "jpg",
            mime: "image/jpeg",
            header: &[0xFF, 0xD8, 0xFF],
            header_offset: 0,
            footer: Some(&[0xFF, 0xD9]),
            min_size: 4,
            max_size: 5_000_000,
        },
        FileSignature {
            ext: "png",
            mime: "image/png",
            header: &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A],
            header_offset: 0,
            footer: Some(&[0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82]),
            min_size: 16,
            max_size: 10_000_000,
        },
        FileSignature {
            ext: "gif",
            mime: "image/gif",
            header: b"GIF89a",
            header_offset: 0,
            footer: Some(&[0x00, 0x3B]),
            min_size: 14,
            max_size: 8_000_000,
        },
        FileSignature {
            ext: "bmp",
            mime: "image/bmp",
            header: b"BM",
            header_offset: 0,
            footer: None,
            min_size: 14,
            max_size: 20_000_000,
        },
        FileSignature {
            ext: "tiff",
            mime: "image/tiff",
            header: &[0x49, 0x49, 0x2A, 0x00],
            header_offset: 0,
            footer: None,
            min_size: 8,
            max_size: 40_000_000,
        },
        FileSignature {
            ext: "pdf",
            mime: "application/pdf",
            header: b"%PDF",
            header_offset: 0,
            footer: Some(b"%%EOF"),
            min_size: 8,
            max_size: 20_000_000,
        },
        // OLE Compound File — covers legacy doc/xls/ppt (one magic).
        FileSignature {
            ext: "ole",
            mime: "application/x-ole-storage",
            header: &[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1],
            header_offset: 0,
            footer: None,
            min_size: 512,
            max_size: 50_000_000,
        },
        FileSignature {
            ext: "zip",
            mime: "application/zip",
            header: &[0x50, 0x4B, 0x03, 0x04],
            header_offset: 0,
            footer: Some(&[0x50, 0x4B, 0x05, 0x06]),
            min_size: 30,
            max_size: 50_000_000,
        },
        FileSignature {
            ext: "rar",
            mime: "application/vnd.rar",
            header: b"Rar!\x1a\x07",
            header_offset: 0,
            footer: None,
            min_size: 20,
            max_size: 50_000_000,
        },
        FileSignature {
            ext: "7z",
            mime: "application/x-7z-compressed",
            header: &[0x37, 0x7A, 0xBC, 0xAF, 0x27, 0x1C],
            header_offset: 0,
            footer: None,
            min_size: 32,
            max_size: 50_000_000,
        },
        FileSignature {
            ext: "mp3",
            mime: "audio/mpeg",
            header: b"ID3",
            header_offset: 0,
            footer: None,
            min_size: 128,
            max_size: 30_000_000,
        },
        FileSignature {
            ext: "wav",
            mime: "audio/wav",
            header: b"WAVE",
            header_offset: 8,
            footer: None,
            min_size: 44,
            max_size: 50_000_000,
        },
        FileSignature {
            ext: "avi",
            mime: "video/x-msvideo",
            header: b"AVI ",
            header_offset: 8,
            footer: None,
            min_size: 64,
            max_size: 100_000_000,
        },
        FileSignature {
            ext: "mp4",
            mime: "video/mp4",
            header: b"ftyp",
            header_offset: 4,
            footer: None,
            min_size: 32,
            max_size: 100_000_000,
        },
        // Noisy — excluded from Carver::common_media().
        FileSignature {
            ext: "exe",
            mime: "application/vnd.microsoft.portable-executable",
            header: b"MZ",
            header_offset: 0,
            footer: None,
            min_size: 64,
            max_size: 20_000_000,
        },
    ]
}
