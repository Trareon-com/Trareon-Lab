//! Authoritative forensic core for Trareon Lab (Foundation).

pub mod error;
pub mod export;
pub mod integrity;
pub mod progress;
pub mod report_gate;
pub mod result;
pub mod run_manifest;
pub mod schema_validate;
pub mod scope;

pub use error::LabError;
pub use export::{
    export_case_html, export_case_pdfa, export_case_skeleton, export_case_uco, ExportArtifact,
    ExportMode,
};
pub use integrity::{ClaimType, CoverageStatus, IntegrityState};
pub use progress::{FnProgress, NullProgress, ProgressEvent, ProgressSink, SharedProgress};
pub use report_gate::{report_blockers, report_finalizable};
pub use result::LabResult;
pub use run_manifest::{compare_runs, RunCompareLine, RunManifest, CASE_UCO_PROFILE_VERSION};
pub use scope::{ScopeAction, ScopeBounds, ScopeGuard, ScopeOverride};

/// Workspace package version as a semver string (`MAJOR.MINOR.PATCH`).
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_non_empty() {
        assert!(!version().is_empty());
    }
}
