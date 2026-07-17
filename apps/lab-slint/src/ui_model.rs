//! Presentation model for the Foundation Slint shell (testable without a display).

use lab_case::CaseState;

/// UI-facing case/evidence/coverage snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiSnapshot {
    pub case_title: String,
    pub case_state: CaseState,
    pub evidence_count: i32,
    pub coverage_count: i32,
    pub open_case_focused: bool,
}

impl Default for UiSnapshot {
    fn default() -> Self {
        Self {
            case_title: "(no case)".into(),
            case_state: CaseState::Created,
            evidence_count: 0,
            coverage_count: 0,
            open_case_focused: true,
        }
    }
}

impl UiSnapshot {
    pub fn open_case(&mut self, title: impl Into<String>, evidence: i32, coverage: i32) {
        self.case_title = title.into();
        self.case_state = CaseState::Open;
        self.evidence_count = evidence;
        self.coverage_count = coverage;
        self.open_case_focused = true;
    }

    pub fn focus_open_case(&mut self) {
        self.open_case_focused = true;
    }
}
