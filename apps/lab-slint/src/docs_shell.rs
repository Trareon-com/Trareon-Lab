//! Offline documentation shell index for Foundation.

use std::fs;
use std::path::{Path, PathBuf};

use lab_core::{LabError, LabResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuideStatus {
    Present,
    Missing,
}

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
        "Disk images",
        "Filesystems",
        "Offline collaboration",
        "Reporting",
    ] {
        if !body.contains(topic) {
            return Err(LabError::Internal {
                detail: format!("docs index missing topic: {topic}"),
            });
        }
    }
    Ok(body)
}

/// Report whether a capability has a matching installed offline guide.
pub fn guide_status(capability: &str) -> GuideStatus {
    let guide = match capability.to_ascii_lowercase().as_str() {
        "case" | "case lifecycle" => "case-lifecycle.md",
        "evidence" | "hex" | "artifacts" | "search" | "timeline" | "examination" => {
            "examination.md"
        }
        "raw" | "e01" | "disk images" => "disk-images.md",
        "apfs" | "filesystems" => "filesystems.md",
        "transfer" | "offline collaboration" => "offline-collaboration.md",
        "export" | "pdf/a" | "case/uco" | "reporting" => "reporting.md",
        _ => return GuideStatus::Missing,
    };
    if docs_root().join(guide).is_file() {
        GuideStatus::Present
    } else {
        GuideStatus::Missing
    }
}

/// Prevent a capability claim from becoming Validated without an offline guide.
pub fn validate_capability_status(capability: &str, status: &str) -> LabResult<()> {
    if status == "Validated" && guide_status(capability) == GuideStatus::Missing {
        return Err(LabError::Internal {
            detail: format!("Validated capability '{capability}' is missing an offline guide"),
        });
    }
    Ok(())
}
