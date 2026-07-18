//! Raw / dd disk image reader (Day 11+).

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use lab_core::{LabError, LabResult, NullProgress, ProgressEvent, ProgressSink};
use sha2::{Digest, Sha256};

use crate::image::{ImageReader, IntegrityReport};

/// Supported image container kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageKind {
    RawDd,
    E01,
}

impl ImageKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RawDd => "raw_dd",
            Self::E01 => "e01",
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
        self.sha256_hex_with_progress(&mut NullProgress)?
            .ok_or_else(|| LabError::Internal {
                detail: "hash cancelled".into(),
            })
    }

    /// Hash with progress + cooperative cancel.
    pub fn sha256_hex_with_progress(
        &mut self,
        progress: &mut dyn ProgressSink,
    ) -> LabResult<Option<String>> {
        self.file
            .seek(SeekFrom::Start(0))
            .map_err(|e| LabError::Internal {
                detail: format!("seek raw image: {e}"),
            })?;
        let mut hasher = Sha256::new();
        let mut buf = [0_u8; 64 * 1024];
        let mut done = 0_u64;
        let total = self.byte_length;
        while done < total {
            if progress.is_cancelled() {
                return Ok(None);
            }
            let want = std::cmp::min((total - done) as usize, buf.len());
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
            done += n as u64;
            progress.report(ProgressEvent::new(
                "storage.hash",
                done,
                Some(total),
                format!("hashing {}", self.path.display()),
            ));
        }
        Ok(Some(hex::encode(hasher.finalize())))
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

impl ImageReader for RawImage {
    fn kind(&self) -> ImageKind {
        self.kind
    }

    fn byte_length(&self) -> u64 {
        self.byte_length
    }

    fn read_at(&mut self, offset: u64, buf: &mut [u8]) -> LabResult<usize> {
        if offset >= self.byte_length {
            return Ok(0);
        }
        let max = std::cmp::min(buf.len() as u64, self.byte_length - offset) as usize;
        self.file
            .seek(SeekFrom::Start(offset))
            .map_err(|e| LabError::Internal {
                detail: format!("seek raw image: {e}"),
            })?;
        self.file
            .read(&mut buf[..max])
            .map_err(|e| LabError::Internal {
                detail: format!("read_at raw image: {e}"),
            })
    }

    fn verify_integrity(&mut self, progress: &mut dyn ProgressSink) -> LabResult<IntegrityReport> {
        // Raw images have no per-chunk CRC; a full read/hash proves readability.
        let _ = self.sha256_hex_with_progress(progress)?;
        Ok(IntegrityReport {
            ok: true,
            crc_errors: 0,
            chunks_checked: 1,
            message: "raw image readable".into(),
        })
    }
}

/// Detect supported image kind from path extension.
pub fn detect_image_kind(path: &Path) -> LabResult<ImageKind> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    match ext.as_str() {
        "dd" | "raw" | "img" | "bin" | "" => Ok(ImageKind::RawDd),
        // E01 dispatch succeeds so callers can open via ImageReader.
        "e01" => Ok(ImageKind::E01),
        "ex01" => Err(lab_core::LabError::Internal {
            detail: "Ex01 not implemented yet".into(),
        }),
        other => Err(lab_core::LabError::Internal {
            detail: format!("unsupported image format extension: {other}"),
        }),
    }
}
