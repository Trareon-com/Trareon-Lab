//! Scope policy enforcement (FR-CASE-004) — not display filters alone.

use crate::{LabError, LabResult};
use std::collections::BTreeSet;

/// Bounds that must be enforced on process / preview / search / export / report.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ScopeBounds {
    pub case_uuid: String,
    /// Path prefixes allowed (empty = unrestricted paths).
    pub path_prefixes: Vec<String>,
    /// Artifact kinds allowed (empty = unrestricted).
    pub artifact_kinds: BTreeSet<String>,
    pub date_start_utc: Option<String>,
    pub date_end_utc: Option<String>,
    pub enforced: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeAction {
    Process,
    Preview,
    Search,
    Export,
    Report,
}

/// Central guard — all enforcement paths should call this.
#[derive(Debug, Clone)]
pub struct ScopeGuard {
    bounds: ScopeBounds,
}

impl ScopeGuard {
    pub fn new(bounds: ScopeBounds) -> Self {
        Self { bounds }
    }

    pub fn bounds(&self) -> &ScopeBounds {
        &self.bounds
    }

    pub fn is_enforced(&self) -> bool {
        self.bounds.enforced
    }

    /// Check a path against enforced prefixes.
    pub fn allow_path(&self, path: &str, action: ScopeAction) -> LabResult<()> {
        let _ = action;
        if !self.bounds.enforced || self.bounds.path_prefixes.is_empty() {
            return Ok(());
        }
        if self
            .bounds
            .path_prefixes
            .iter()
            .any(|p| path.starts_with(p) || path.contains(p))
        {
            return Ok(());
        }
        Err(LabError::ScopeDenied {
            detail: format!("path '{path}' outside enforced prefixes for {action:?}"),
        })
    }

    pub fn allow_artifact_kind(&self, kind: &str, action: ScopeAction) -> LabResult<()> {
        let _ = action;
        if !self.bounds.enforced || self.bounds.artifact_kinds.is_empty() {
            return Ok(());
        }
        if self.bounds.artifact_kinds.contains(kind) {
            return Ok(());
        }
        Err(LabError::ScopeDenied {
            detail: format!("artifact kind '{kind}' not in scope"),
        })
    }
}

/// Audited override request (persist via case audit ledger in callers).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopeOverride {
    pub reason: String,
    pub approver_note: String,
    pub expires_at_utc: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enforced_path_deny() {
        let g = ScopeGuard::new(ScopeBounds {
            case_uuid: "c".into(),
            path_prefixes: vec!["/Users/".into()],
            artifact_kinds: BTreeSet::new(),
            date_start_utc: None,
            date_end_utc: None,
            enforced: true,
        });
        assert!(g.allow_path("/Users/a/x", ScopeAction::Search).is_ok());
        assert!(matches!(
            g.allow_path("/Windows/x", ScopeAction::Search),
            Err(LabError::ScopeDenied { .. })
        ));
        assert!(matches!(
            g.allow_path("/Windows/x", ScopeAction::Export),
            Err(LabError::ScopeDenied { .. })
        ));
    }

    #[test]
    fn unenforced_allows_all() {
        let g = ScopeGuard::new(ScopeBounds {
            enforced: false,
            path_prefixes: vec!["/Users/".into()],
            ..Default::default()
        });
        assert!(g.allow_path("/anywhere", ScopeAction::Process).is_ok());
    }
}
