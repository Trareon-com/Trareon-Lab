//! Signature-based file carving.

use lab_core::{LabResult, ProgressEvent, ProgressSink};
use lab_storage::ImageReader;
use sha2::{Digest, Sha256};

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
}

impl Default for Carver {
    fn default() -> Self {
        Self {
            signatures: default_signatures(),
        }
    }
}

impl Carver {
    pub fn carve(
        &self,
        image: &mut dyn ImageReader,
        progress: &mut dyn ProgressSink,
    ) -> LabResult<Vec<CarvedFile>> {
        let total = image.byte_length();
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
            while let Some(rel) = find_subslice(&data[search..], sig.header) {
                let start = search + rel;
                let (end, confidence) = match sig.footer {
                    Some(footer) => {
                        let search_to = (start + sig.max_size).min(data.len());
                        if let Some(frel) = find_subslice(&data[start..search_to], footer) {
                            let end = start + frel + footer.len();
                            if end - start >= sig.min_size {
                                (end, CarveConfidence::High)
                            } else {
                                search = start + 1;
                                continue;
                            }
                        } else {
                            let end = (start + sig.max_size.min(data.len() - start)).min(data.len());
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

fn find_subslice(hay: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || hay.len() < needle.len() {
        return None;
    }
    hay.windows(needle.len()).position(|w| w == needle)
}

pub fn default_signatures() -> Vec<FileSignature> {
    vec![
        FileSignature {
            ext: "jpg",
            mime: "image/jpeg",
            header: &[0xFF, 0xD8, 0xFF],
            footer: Some(&[0xFF, 0xD9]),
            min_size: 4,
            max_size: 5_000_000,
        },
        FileSignature {
            ext: "png",
            mime: "image/png",
            header: &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A],
            footer: Some(&[0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82]),
            min_size: 16,
            max_size: 10_000_000,
        },
        FileSignature {
            ext: "pdf",
            mime: "application/pdf",
            header: b"%PDF",
            footer: Some(b"%%EOF"),
            min_size: 8,
            max_size: 20_000_000,
        },
        FileSignature {
            ext: "zip",
            mime: "application/zip",
            header: &[0x50, 0x4B, 0x03, 0x04],
            footer: Some(&[0x50, 0x4B, 0x05, 0x06]),
            min_size: 30,
            max_size: 50_000_000,
        },
    ]
}
