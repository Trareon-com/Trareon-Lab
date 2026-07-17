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
#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// Workbench chrome (desktop UX redesign).
    pub nav_collapsed: bool,
    pub inspector_open: bool,
    pub log_open: bool,
    pub palette_open: bool,
    pub log_lines: Vec<String>,
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
            about_disclosure: "Trareon Lab v1 · offline case DB · raw/dd import + hash/provenance · path/name/hash search · bookmarks · unsigned builds — see docs/SELLING-UNSIGNED.md · NOT court-ready / NOT ISO-accredited".into(),
            dark_mode: true,
            locale: "en".into(),
            nav_collapsed: false,
            inspector_open: true,
            log_open: false,
            palette_open: false,
            log_lines: Vec::new(),
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

    /// Workbench keyboard: palette, chrome toggles, screen jump, bookmark.
    pub fn handle_shortcut(&mut self, key: &str) {
        self.last_shortcut = key.to_string();
        match key {
            "/" | "palette" => {
                self.palette_open = true;
            }
            "Escape" => {
                if self.palette_open {
                    self.palette_open = false;
                } else {
                    self.selected_file_index = None;
                    self.provenance_open = None;
                }
            }
            "b" if self.selected_file_index.is_some() => {
                self.bookmark_count += 1;
                self.navigate_to(NavScreen::Bookmarks);
            }
            "Enter" if self.selected_file_index.is_some() => {
                self.open_case_focused = false;
            }
            "1" => self.navigate_to(NavScreen::CaseHome),
            "2" => self.navigate_to(NavScreen::Evidence),
            "3" => self.navigate_to(NavScreen::Search),
            "4" => self.navigate_to(NavScreen::Timeline),
            "5" => self.navigate_to(NavScreen::Bookmarks),
            "6" => self.navigate_to(NavScreen::Report),
            "nav" => self.nav_collapsed = !self.nav_collapsed,
            "inspector" => self.inspector_open = !self.inspector_open,
            "log" => self.log_open = !self.log_open,
            _ => {}
        }
    }

    pub fn push_log(&mut self, line: impl Into<String>) {
        self.log_lines.insert(0, line.into());
        if self.log_lines.len() > 40 {
            self.log_lines.truncate(40);
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
}
