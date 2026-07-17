//! Hostile-input preflight for `.fsnap` packages (directory layout for Foundation).

use std::collections::HashSet;
use std::fs;
use std::path::{Component, Path};

use lab_core::{LabError, LabResult};
use serde::Deserialize;

/// Decompression / size budgets for fail-closed preflight.
#[derive(Debug, Clone)]
pub struct PreflightLimits {
    pub max_member_bytes: u64,
    pub max_total_bytes: u64,
}

impl Default for PreflightLimits {
    fn default() -> Self {
        Self {
            max_member_bytes: 64 * 1024 * 1024,
            max_total_bytes: 512 * 1024 * 1024,
        }
    }
}

#[derive(Debug, Deserialize)]
struct Manifest {
    package_version: String,
    members: Vec<String>,
}

/// Result of a successful preflight (ready for import).
#[derive(Debug, Clone)]
pub struct PreflightOk {
    pub package_version: String,
    pub members: Vec<String>,
    pub total_bytes: u64,
}

fn reject(reason: impl Into<String>) -> LabError {
    LabError::FsNapRejected {
        reason: reason.into(),
    }
}

fn is_safe_member_path(name: &str) -> bool {
    if name.is_empty() || name.starts_with('/') || name.contains('\0') {
        return false;
    }
    let path = Path::new(name);
    for c in path.components() {
        match c {
            Component::Normal(_) => {}
            Component::CurDir => {}
            _ => return false, // ParentDir, RootDir, Prefix, etc.
        }
    }
    !name.contains("..")
}

/// Verify package layout without mutating the source package.
pub fn preflight_package(package_dir: &Path, limits: PreflightLimits) -> LabResult<PreflightOk> {
    let manifest_path = package_dir.join("manifest.json");
    let raw = fs::read_to_string(&manifest_path).map_err(|_| reject("missing_or_unreadable_manifest"))?;
    let manifest: Manifest =
        serde_json::from_str(&raw).map_err(|_| reject("manifest_parse_failed"))?;

    if !(manifest.package_version == "1.0" || manifest.package_version == "1.1") {
        return Err(reject(format!(
            "unsupported_package_version:{}",
            manifest.package_version
        )));
    }

    let mut seen = HashSet::new();
    let mut total_bytes = 0u64;

    for name in &manifest.members {
        if !is_safe_member_path(name) {
            return Err(reject(format!("path_traversal:{name}")));
        }
        if !seen.insert(name.clone()) {
            return Err(reject(format!("duplicate_member:{name}")));
        }
        let member_path = package_dir.join(name);
        // Ensure resolved path stays under package_dir.
        let canonical_pkg = fs::canonicalize(package_dir).map_err(|_| reject("canonicalize_package"))?;
        if member_path.exists() {
            let canonical_member = fs::canonicalize(&member_path)
                .map_err(|_| reject(format!("canonicalize_member:{name}")))?;
            if !canonical_member.starts_with(&canonical_pkg) {
                return Err(reject(format!("path_escape:{name}")));
            }
        } else {
            return Err(reject(format!("missing_member:{name}")));
        }
        let meta = fs::metadata(&member_path).map_err(|_| reject(format!("stat_member:{name}")))?;
        if meta.file_type().is_symlink() {
            return Err(reject(format!("symlink_rejected:{name}")));
        }
        let len = meta.len();
        if len > limits.max_member_bytes {
            return Err(reject(format!("member_bomb:{name}:{len}")));
        }
        total_bytes = total_bytes.saturating_add(len);
        if total_bytes > limits.max_total_bytes {
            return Err(reject(format!("total_bomb:{total_bytes}")));
        }
    }

    Ok(PreflightOk {
        package_version: manifest.package_version,
        members: manifest.members,
        total_bytes,
    })
}
