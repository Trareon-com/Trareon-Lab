//! Presentation model for the Foundation Slint shell (testable without a display).

use lab_case::CaseState;

/// Primary examination navigation destinations (Day 8).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavScreen {
    CaseHome,
    Evidence,
    Search,
    Timeline,
    Bookmarks,
    Report,
}

impl NavScreen {
    pub fn label(self) -> &'static str {
        match self {
            Self::CaseHome => "Case",
            Self::Evidence => "Evidence",
            Self::Search => "Search",
            Self::Timeline => "Timeline",
            Self::Bookmarks => "Bookmarks",
            Self::Report => "Report",
        }
    }

    pub fn all() -> [NavScreen; 6] {
        [
            Self::CaseHome,
            Self::Evidence,
            Self::Search,
            Self::Timeline,
            Self::Bookmarks,
            Self::Report,
        ]
    }
}

/// UI-facing case/evidence/coverage snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiSnapshot {
    pub case_title: String,
    pub case_state: CaseState,
    pub evidence_count: i32,
    pub coverage_count: i32,
    pub bookmark_count: i32,
    pub open_case_focused: bool,
    pub active_screen: NavScreen,
}

impl Default for UiSnapshot {
    fn default() -> Self {
        Self {
            case_title: "(no case)".into(),
            case_state: CaseState::Created,
            evidence_count: 0,
            coverage_count: 0,
            bookmark_count: 0,
            open_case_focused: true,
            active_screen: NavScreen::CaseHome,
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
        self.active_screen = NavScreen::CaseHome;
    }

    pub fn focus_open_case(&mut self) {
        self.open_case_focused = true;
    }

    pub fn navigate_to(&mut self, screen: NavScreen) {
        self.active_screen = screen;
        self.open_case_focused = screen == NavScreen::CaseHome;
    }

    pub fn set_bookmark_count(&mut self, count: i32) {
        self.bookmark_count = count;
    }

    /// Demo/stub import: increments evidence count and jumps to Evidence screen.
    pub fn import_evidence_stub(&mut self) {
        self.evidence_count += 1;
        self.active_screen = NavScreen::Evidence;
        self.open_case_focused = false;
    }
}
