//! Case lifecycle states and legal transitions (ADR-004).

use lab_core::{LabError, LabResult};

/// Exact case states from `docs/contracts/CASE-LIFECYCLE.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaseState {
    Created,
    Open,
    ReadOnly,
    RecoveryRequired,
    Closed,
    Archived,
}

impl CaseState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Created => "CREATED",
            Self::Open => "OPEN",
            Self::ReadOnly => "READ_ONLY",
            Self::RecoveryRequired => "RECOVERY_REQUIRED",
            Self::Closed => "CLOSED",
            Self::Archived => "ARCHIVED",
        }
    }

    pub fn parse(s: &str) -> LabResult<Self> {
        match s {
            "CREATED" => Ok(Self::Created),
            "OPEN" => Ok(Self::Open),
            "READ_ONLY" => Ok(Self::ReadOnly),
            "RECOVERY_REQUIRED" => Ok(Self::RecoveryRequired),
            "CLOSED" => Ok(Self::Closed),
            "ARCHIVED" => Ok(Self::Archived),
            other => Err(LabError::CaseStateInvalid {
                from: other.to_string(),
                to: "PARSE".into(),
            }),
        }
    }

    /// Whether `to` is a legal transition from `self`.
    pub fn can_transition_to(self, to: Self) -> bool {
        matches!(
            (self, to),
            (Self::Created, Self::Open)
                | (Self::Open, Self::ReadOnly)
                | (Self::Open, Self::RecoveryRequired)
                | (Self::Open, Self::Closed)
                | (Self::ReadOnly, Self::Open)
                | (Self::ReadOnly, Self::Closed)
                | (Self::RecoveryRequired, Self::Open)
                | (Self::RecoveryRequired, Self::Closed)
                | (Self::Closed, Self::Archived)
                | (Self::Closed, Self::ReadOnly)
                | (Self::Archived, Self::ReadOnly)
        )
    }

    pub fn transition(self, to: Self) -> LabResult<Self> {
        if self.can_transition_to(to) {
            Ok(to)
        } else {
            Err(LabError::CaseStateInvalid {
                from: self.as_str().to_string(),
                to: to.as_str().to_string(),
            })
        }
    }
}
