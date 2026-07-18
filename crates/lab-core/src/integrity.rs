//! Typed evidence integrity states (Lab Core Perfect).

/// Integrity / verification state for an evidence object or package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntegrityState {
    NotRun,
    VerifiedMatch,
    VerifiedMismatch,
    ComputedUnanchored,
    SignatureInvalid,
    IncompleteInput,
}

impl IntegrityState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotRun => "NotRun",
            Self::VerifiedMatch => "VerifiedMatch",
            Self::VerifiedMismatch => "VerifiedMismatch",
            Self::ComputedUnanchored => "ComputedUnanchored",
            Self::SignatureInvalid => "SignatureInvalid",
            Self::IncompleteInput => "IncompleteInput",
        }
    }

    pub fn is_critical_block(self) -> bool {
        matches!(
            self,
            Self::VerifiedMismatch | Self::SignatureInvalid | Self::IncompleteInput
        )
    }
}

impl std::fmt::Display for IntegrityState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Processing coverage rollup for a case or run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoverageStatus {
    NotRun,
    Running,
    Complete,
    CompletedWithLimitations,
    Failed,
}

impl CoverageStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotRun => "NotRun",
            Self::Running => "Running",
            Self::Complete => "Complete",
            Self::CompletedWithLimitations => "CompletedWithLimitations",
            Self::Failed => "Failed",
        }
    }
}

/// Epistemic claim type — automated results must start as Indication.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClaimType {
    Fact,
    Observation,
    Indication,
    Hypothesis,
    Conclusion,
}

impl ClaimType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fact => "Fact",
            Self::Observation => "Observation",
            Self::Indication => "Indication",
            Self::Hypothesis => "Hypothesis",
            Self::Conclusion => "Conclusion",
        }
    }

    /// Default for parser/YARA/hash automations.
    pub const AUTOMATED_DEFAULT: Self = Self::Indication;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mismatch_blocks_finalize() {
        assert!(IntegrityState::VerifiedMismatch.is_critical_block());
        assert!(!IntegrityState::VerifiedMatch.is_critical_block());
    }

    #[test]
    fn automated_default_is_indication() {
        assert_eq!(ClaimType::AUTOMATED_DEFAULT, ClaimType::Indication);
    }
}
