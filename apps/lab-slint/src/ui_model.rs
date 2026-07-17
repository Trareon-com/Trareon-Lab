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

/// One row in the evidence file browser (Day 19).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceFileRow {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub deleted: bool,
}

/// Artifact hit row (Day 29).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactHitRow {
    pub kind: String,
    pub summary: String,
    pub provenance_ref: String,
}

/// Finding row (Day 37/39).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FindingRow {
    pub claim: String,
    pub bookmark_uuid: String,
}

/// UI-facing case/evidence/coverage snapshot.
#[derive(Debug, Clone, PartialEq)]
pub struct UiSnapshot {
    pub case_title: String,
    pub case_state: CaseState,
    pub evidence_count: i32,
    pub coverage_count: i32,
    pub bookmark_count: i32,
    pub open_case_focused: bool,
    pub active_screen: NavScreen,
    pub evidence_files: Vec<EvidenceFileRow>,
    pub selected_file_index: Option<usize>,
    pub search_query: String,
    pub search_results: Vec<String>,
    pub artifact_hits: Vec<ArtifactHitRow>,
    pub timeline_labels: Vec<String>,
    pub findings: Vec<FindingRow>,
    pub report_state: String,
    pub last_shortcut: String,
    pub provenance_open: Option<String>,
    pub transfer_status: String,
    pub second_method_count: i32,
    pub blind_pt_status: String,
    pub about_disclosure: String,
    pub dark_mode: bool,
    pub locale: String,
    pub progress_ratio: f64,
    pub progress_stage: String,
    pub progress_message: String,
    pub progress_visible: bool,
    pub progress_cancelled: bool,
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
            evidence_files: Vec::new(),
            selected_file_index: None,
            search_query: String::new(),
            search_results: Vec::new(),
            artifact_hits: Vec::new(),
            timeline_labels: Vec::new(),
            findings: Vec::new(),
            report_state: "draft".into(),
            last_shortcut: String::new(),
            provenance_open: None,
            transfer_status: String::new(),
            second_method_count: 0,
            blind_pt_status: "none".into(),
            about_disclosure: "SBOM: release-evidence/sbom/; licenses: docs/DEPENDENCY-AND-LICENSE-POLICY.md; notes: docs/user/RELEASE-NOTES-1.0.0.md".into(),
            dark_mode: true,
            locale: "en".into(),
            progress_ratio: 0.0,
            progress_stage: String::new(),
            progress_message: String::new(),
            progress_visible: false,
            progress_cancelled: false,
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

    /// Replace evidence file listing (Day 19).
    pub fn set_evidence_files(&mut self, files: Vec<EvidenceFileRow>) {
        self.evidence_files = files;
        self.selected_file_index = None;
        self.active_screen = NavScreen::Evidence;
        self.open_case_focused = false;
    }

    pub fn select_file(&mut self, index: usize) -> bool {
        if index < self.evidence_files.len() {
            self.selected_file_index = Some(index);
            true
        } else {
            false
        }
    }

    /// Day 20 keyboard stubs.
    pub fn handle_shortcut(&mut self, key: &str) {
        self.last_shortcut = key.to_string();
        match key {
            "/" => {
                self.navigate_to(NavScreen::Search);
                self.open_case_focused = false;
            }
            "b" if self.selected_file_index.is_some() => {
                self.bookmark_count += 1;
                self.navigate_to(NavScreen::Bookmarks);
            }
            "Enter" if self.selected_file_index.is_some() => {
                self.open_case_focused = false;
            }
            "Escape" => {
                self.selected_file_index = None;
                self.provenance_open = None;
            }
            _ => {}
        }
    }

    pub fn set_artifact_hits(&mut self, hits: Vec<ArtifactHitRow>) {
        self.artifact_hits = hits;
        self.active_screen = NavScreen::Search;
    }

    pub fn open_provenance(&mut self, provenance_ref: impl Into<String>) {
        self.provenance_open = Some(provenance_ref.into());
    }

    pub fn set_timeline(&mut self, labels: Vec<String>) {
        self.timeline_labels = labels;
        self.active_screen = NavScreen::Timeline;
    }

    pub fn set_findings(&mut self, findings: Vec<FindingRow>) {
        self.findings = findings;
    }

    pub fn set_report_state(&mut self, state: impl Into<String>) {
        self.report_state = state.into();
        self.active_screen = NavScreen::Report;
    }

    pub fn set_search(&mut self, query: impl Into<String>, results: Vec<String>) {
        self.search_query = query.into();
        self.search_results = results;
        self.active_screen = NavScreen::Search;
    }

    pub fn set_transfer_status(&mut self, status: impl Into<String>) {
        self.transfer_status = status.into();
    }

    /// FR-VAL-009 UI hook: record that a second-method comparison is open.
    pub fn record_second_method_stub(&mut self) {
        self.second_method_count += 1;
        self.active_screen = NavScreen::Report;
    }

    /// FR-VAL-010 UI hook: participant blind-PT import/export status string.
    pub fn set_blind_pt_status(&mut self, status: impl Into<String>) {
        self.blind_pt_status = status.into();
        self.active_screen = NavScreen::CaseHome;
    }

    pub fn set_dark_mode(&mut self, dark: bool) {
        self.dark_mode = dark;
    }

    pub fn set_locale(&mut self, locale: impl Into<String>) {
        let loc = locale.into();
        self.locale = if loc == "en" || loc == "id" {
            loc
        } else {
            "en".into()
        };
    }

    /// Update import/analysis progress bar state.
    pub fn set_progress(
        &mut self,
        stage: impl Into<String>,
        done: u64,
        total: Option<u64>,
        message: impl Into<String>,
    ) {
        self.progress_visible = true;
        self.progress_stage = stage.into();
        self.progress_message = message.into();
        self.progress_ratio = match total {
            Some(t) if t > 0 => (done as f64 / t as f64).clamp(0.0, 1.0),
            _ => 0.0,
        };
    }

    pub fn clear_progress(&mut self) {
        self.progress_visible = false;
        self.progress_ratio = 0.0;
        self.progress_stage.clear();
        self.progress_message.clear();
        self.progress_cancelled = false;
    }

    pub fn request_cancel_progress(&mut self) {
        self.progress_cancelled = true;
        self.progress_message = "cancelling…".into();
    }
}
