//! Raw / dd disk image reader (Day 11+).

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use lab_core::{LabError, LabResult};
use sha2::{Digest, Sha256};

/// Supported image container kinds for R1 storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageKind {
    RawDd,
}

impl ImageKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RawDd => "raw_dd",
        }
    }
}

/// Opened raw image handle with measured size.
#[derive(Debug)]
pub struct RawImage {
    path: PathBuf,
    kind: ImageKind,
    byte_length: u64,
    file: File,
}

impl RawImage {
    /// Open a raw/dd image and measure byte length (fail closed on missing/empty).
    pub fn open_raw(path: &Path) -> LabResult<Self> {
        let meta = std::fs::metadata(path).map_err(|e| LabError::Internal {
            detail: format!("stat raw image {}: {e}", path.display()),
        })?;
        if !meta.is_file() {
            return Err(LabError::Internal {
                detail: format!("raw image is not a file: {}", path.display()),
            });
        }
        let byte_length = meta.len();
        if byte_length == 0 {
            return Err(LabError::Internal {
                detail: format!("raw image is empty: {}", path.display()),
            });
        }
        let file = File::open(path).map_err(|e| LabError::Internal {
            detail: format!("open raw image {}: {e}", path.display()),
        })?;
        Ok(Self {
            path: path.to_path_buf(),
            kind: ImageKind::RawDd,
            byte_length,
            file,
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn kind(&self) -> ImageKind {
        self.kind
    }

    pub fn byte_length(&self) -> u64 {
        self.byte_length
    }

    /// Compute SHA-256 over the full image (streaming).
    pub fn sha256_hex(&mut self) -> LabResult<String> {
        self.file
            .seek(SeekFrom::Start(0))
            .map_err(|e| LabError::Internal {
                detail: format!("seek raw image: {e}"),
            })?;
        let mut hasher = Sha256::new();
        let mut buf = [0_u8; 64 * 1024];
        let mut remaining = self.byte_length;
        while remaining > 0 {
            let want = std::cmp::min(remaining as usize, buf.len());
            let n = self
                .file
                .read(&mut buf[..want])
                .map_err(|e| LabError::Internal {
                    detail: format!("read raw image: {e}"),
                })?;
            if n == 0 {
                return Err(LabError::Internal {
                    detail: "unexpected EOF while hashing raw image".into(),
                });
            }
            hasher.update(&buf[..n]);
            remaining -= n as u64;
        }
        Ok(hex::encode(hasher.finalize()))
    }

    /// Hash with cooperative cancel check between chunks (Day 12).
    pub fn sha256_hex_cancellable<F>(&mut self, mut should_cancel: F) -> LabResult<Option<String>>
    where
        F: FnMut() -> bool,
    {
        self.file
            .seek(SeekFrom::Start(0))
            .map_err(|e| LabError::Internal {
                detail: format!("seek raw image: {e}"),
            })?;
        let mut hasher = Sha256::new();
        let mut buf = [0_u8; 64 * 1024];
        let mut remaining = self.byte_length;
        while remaining > 0 {
            if should_cancel() {
                return Ok(None);
            }
            let want = std::cmp::min(remaining as usize, buf.len());
            let n = self
                .file
                .read(&mut buf[..want])
                .map_err(|e| LabError::Internal {
                    detail: format!("read raw image: {e}"),
                })?;
            if n == 0 {
                return Err(LabError::Internal {
                    detail: "unexpected EOF while hashing raw image".into(),
                });
            }
            hasher.update(&buf[..n]);
            remaining -= n as u64;
        }
        Ok(Some(hex::encode(hasher.finalize())))
    }
}

/// Detect supported image kind from path extension / magic (R1: raw/dd only).
pub fn detect_image_kind(path: &Path) -> LabResult<ImageKind> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    match ext.as_str() {
        "dd" | "raw" | "img" | "" => Ok(ImageKind::RawDd),
        "e01" | "ex01" => Err(lab_core::LabError::Internal {
            detail: "E01/Ex01 not implemented yet; Limited until Day 14".into(),
        }),
        other => Err(lab_core::LabError::Internal {
            detail: format!("unsupported image format extension: {other}"),
        }),
    }
}
