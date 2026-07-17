//! Content-addressed derived object store under a case directory.

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use lab_core::{LabError, LabResult};
use sha2::{Digest, Sha256};

/// SHA-256 content-addressed store (`objects/ab/cdef...`).
pub struct CasStore {
    root: PathBuf,
}

impl CasStore {
    /// Open or create the CAS root (`<case>/derived/cas`).
    pub fn open(case_dir: &Path) -> LabResult<Self> {
        let root = case_dir.join("derived").join("cas");
        fs::create_dir_all(&root).map_err(|e| LabError::Internal {
            detail: format!("create cas root: {e}"),
        })?;
        Ok(Self { root })
    }

    /// Store bytes; returns lowercase hex SHA-256. Existing objects are never overwritten.
    pub fn put(&self, bytes: &[u8]) -> LabResult<String> {
        let digest = hex::encode(Sha256::digest(bytes));
        let path = self.object_path(&digest)?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| LabError::Internal {
                detail: format!("create cas shard: {e}"),
            })?;
        }
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|e| {
                if path.exists() {
                    LabError::IntegrityFailed {
                        detail: format!("cas object already exists: {digest}"),
                    }
                } else {
                    LabError::Internal {
                        detail: format!("create cas object: {e}"),
                    }
                }
            })?;
        file.write_all(bytes).map_err(|e| LabError::Internal {
            detail: format!("write cas object: {e}"),
        })?;
        file.sync_all().map_err(|e| LabError::Internal {
            detail: format!("sync cas object: {e}"),
        })?;
        Ok(digest)
    }

    /// Read bytes for a digest; fails closed if missing or unreadable.
    pub fn get(&self, digest: &str) -> LabResult<Vec<u8>> {
        let path = self.object_path(digest)?;
        fs::read(&path).map_err(|e| LabError::IntegrityFailed {
            detail: format!("cas get {digest}: {e}"),
        })
    }

    fn object_path(&self, digest: &str) -> LabResult<PathBuf> {
        if digest.len() != 64 || !digest.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(LabError::IntegrityFailed {
                detail: format!("invalid sha256 digest: {digest}"),
            });
        }
        let shard = &digest[..2];
        Ok(self.root.join(shard).join(digest))
    }
}
