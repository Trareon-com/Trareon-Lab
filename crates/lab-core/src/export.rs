//! Deterministic export / report skeleton.

use crate::error::LabError;
use crate::result::LabResult;
use sha2::{Digest, Sha256};

/// Export mode — drafts must be labeled non-final.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportMode {
    Draft,
    Sealed,
}

/// Skeleton export artifact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportArtifact {
    pub mode: ExportMode,
    pub body: String,
    pub digest_sha256: String,
    pub labeled_non_final: bool,
}

/// Build a deterministic JSON/HTML skeleton export for a sealed or draft case.
pub fn export_case_skeleton(
    case_uuid: &str,
    title: &str,
    evidence_count: u32,
    coverage_count: u32,
    mode: ExportMode,
) -> LabResult<ExportArtifact> {
    if case_uuid.is_empty() || title.is_empty() {
        return Err(LabError::Internal {
            detail: "export requires case_uuid and title".into(),
        });
    }
    let labeled_non_final = matches!(mode, ExportMode::Draft);
    let status = match mode {
        ExportMode::Draft => "draft_non_final",
        ExportMode::Sealed => "sealed",
    };
    // Canonical key order for determinism.
    let body = format!(
        "{{\n  \"schema_version\": \"export-skeleton-1\",\n  \"case_uuid\": \"{case_uuid}\",\n  \"title\": \"{title}\",\n  \"evidence_count\": {evidence_count},\n  \"coverage_count\": {coverage_count},\n  \"status\": \"{status}\",\n  \"labeled_non_final\": {labeled_non_final}\n}}\n"
    );
    let digest_sha256 = hex::encode(Sha256::digest(body.as_bytes()));
    Ok(ExportArtifact {
        mode,
        body,
        digest_sha256,
        labeled_non_final,
    })
}
