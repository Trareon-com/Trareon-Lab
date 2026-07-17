//! Typed foundation errors with stable machine-readable codes.
//! Trust/lifecycle failures must not be stringly-typed at call sites.

use std::fmt;

/// Authoritative error taxonomy for Trareon Lab foundation APIs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LabError {
    /// `.fsnap` package rejected by preflight or import contract.
    FsNapRejected { reason: String },
    /// Case lifecycle transition denied.
    CaseStateInvalid { from: String, to: String },
    /// Exclusive case lock cannot be acquired or is held by another process.
    CaseLockConflict { detail: String },
    /// Integrity or signature verification failed.
    IntegrityFailed { detail: String },
    /// Schema validation failed for a foundation record.
    SchemaInvalid { schema: String, detail: String },
    /// Internal invariant broken; treat as fail-closed.
    Internal { detail: String },
}

impl LabError {
    /// Stable error code for UI, audit, and export surfaces.
    pub fn code(&self) -> &'static str {
        match self {
            Self::FsNapRejected { .. } => "FSNAP_REJECTED",
            Self::CaseStateInvalid { .. } => "CASE_STATE_INVALID",
            Self::CaseLockConflict { .. } => "CASE_LOCK_CONFLICT",
            Self::IntegrityFailed { .. } => "INTEGRITY_FAILED",
            Self::SchemaInvalid { .. } => "SCHEMA_INVALID",
            Self::Internal { .. } => "INTERNAL",
        }
    }
}

impl fmt::Display for LabError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FsNapRejected { reason } => {
                write!(f, "{}: {reason}", self.code())
            }
            Self::CaseStateInvalid { from, to } => {
                write!(f, "{}: {from} -> {to}", self.code())
            }
            Self::CaseLockConflict { detail }
            | Self::IntegrityFailed { detail }
            | Self::Internal { detail } => {
                write!(f, "{}: {detail}", self.code())
            }
            Self::SchemaInvalid { schema, detail } => {
                write!(f, "{}: {schema}: {detail}", self.code())
            }
        }
    }
}

impl std::error::Error for LabError {}
