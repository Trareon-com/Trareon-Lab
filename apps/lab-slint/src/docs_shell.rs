//! Offline documentation shell index for Foundation.

use std::fs;
use std::path::{Path, PathBuf};

use lab_core::{LabError, LabResult};

/// Resolve installed/offline docs root (repo `docs/user` during development).
pub fn docs_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/user")
}

/// Load the offline docs index and ensure Foundation topics are listed.
pub fn load_docs_index() -> LabResult<String> {
    let path = docs_root().join("INDEX.md");
    let body = fs::read_to_string(&path).map_err(|e| LabError::Internal {
        detail: format!("open docs index {}: {e}", path.display()),
    })?;
    for topic in [
        "Case lifecycle",
        "Evidence object",
        ".fsnap",
        "Coverage",
        "Export",
        "Local AI",
        "Examination shell",
    ] {
        if !body.contains(topic) {
            return Err(LabError::Internal {
                detail: format!("docs index missing topic: {topic}"),
            });
        }
    }
    Ok(body)
}
