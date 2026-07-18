//! Report finalize fail-closed helpers.

use crate::{CoverageStatus, IntegrityState};

/// Reasons Final report must not be released.
pub fn report_blockers(
    integrity: &[IntegrityState],
    coverage: CoverageStatus,
    required_sections_missing: &[String],
) -> Vec<String> {
    let mut blockers = Vec::new();
    for s in integrity {
        if s.is_critical_block() {
            blockers.push(format!("integrity:{}", s.as_str()));
        }
    }
    match coverage {
        CoverageStatus::Running | CoverageStatus::NotRun | CoverageStatus::Failed => {
            blockers.push(format!("coverage:{}", coverage.as_str()));
        }
        CoverageStatus::CompletedWithLimitations => {
            // Limitations do not auto-block; examiner must acknowledge via checklist.
        }
        CoverageStatus::Complete => {}
    }
    for s in required_sections_missing {
        blockers.push(format!("section_missing:{s}"));
    }
    blockers
}

pub fn report_finalizable(
    integrity: &[IntegrityState],
    coverage: CoverageStatus,
    required_sections_missing: &[String],
) -> bool {
    report_blockers(integrity, coverage, required_sections_missing).is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mismatch_blocks() {
        assert!(!report_finalizable(
            &[IntegrityState::VerifiedMismatch],
            CoverageStatus::Complete,
            &[]
        ));
    }

    #[test]
    fn clean_allows() {
        assert!(report_finalizable(
            &[IntegrityState::VerifiedMatch],
            CoverageStatus::Complete,
            &[]
        ));
    }
}
