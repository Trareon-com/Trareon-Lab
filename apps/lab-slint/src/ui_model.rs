//! Presentation model for the Lab shell (testable without a display).

use lab_case::CaseState;
use lab_core::{
    report_blockers, report_finalizable, ClaimType, CoverageStatus, IntegrityState, ScopeBounds,
};

/// Primary examination navigation destinations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavScreen {
    CaseHome,
    Evidence,
    Hex,
    Artifacts,
    Search,
    Timeline,
    Bookmarks,
    Graph,
    Hypotheses,
    Runs,
    SecondMethod,
    Pt,
    Report,
    Transfer,
    Capabilities,
    About,
}

impl NavScreen {
    pub fn label(self) -> &'static str {
        match self {
            Self::CaseHome => "Case",
            Self::Evidence => "Evidence",
            Self::Hex => "Hex",
            Self::Artifacts => "Artifacts",
            Self::Search => "Search",
            Self::Timeline => "Timeline",
            Self::Bookmarks => "Bookmarks",
            Self::Graph => "Graph",
            Self::Hypotheses => "Hypotheses",
            Self::Runs => "Runs",
            Self::SecondMethod => "SecondMethod",
            Self::Pt => "PT",
            Self::Report => "Report",
            Self::Transfer => "Transfer",
            Self::Capabilities => "Capabilities",
            Self::About => "About",
        }
    }

    pub fn from_label(label: &str) -> Self {
        match label {
            "Evidence" => Self::Evidence,
            "Hex" => Self::Hex,
            "Artifacts" => Self::Artifacts,
            "Search" => Self::Search,
            "Timeline" => Self::Timeline,
            "Bookmarks" => Self::Bookmarks,
            "Graph" => Self::Graph,
            "Hypotheses" => Self::Hypotheses,
            "Runs" => Self::Runs,
            "SecondMethod" => Self::SecondMethod,
            "PT" => Self::Pt,
            "Report" => Self::Report,
            "Transfer" => Self::Transfer,
            "Capabilities" => Self::Capabilities,
            "About" => Self::About,
            _ => Self::CaseHome,
        }
    }

    pub fn all() -> &'static [Self] {
        &[
            Self::CaseHome,
            Self::Evidence,
            Self::Hex,
            Self::Artifacts,
            Self::Search,
            Self::Timeline,
            Self::Bookmarks,
            Self::Graph,
            Self::Hypotheses,
            Self::Runs,
            Self::SecondMethod,
            Self::Pt,
            Self::Report,
            Self::Transfer,
            Self::Capabilities,
            Self::About,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceFileRow {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub deleted: bool,
    pub integrity: IntegrityState,
    pub designation: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactHitRow {
    pub kind: String,
    pub summary: String,
    pub provenance_ref: String,
    pub claim_type: ClaimType,
    pub uncertainty: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FindingRow {
    pub claim: String,
    pub bookmark_uuid: String,
    pub claim_type: ClaimType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecentCaseRow {
    pub id: String,
    pub title: String,
    pub state_label: String,
    pub evidence_count: i32,
    pub modified_label: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
    pub label: String,
    pub depth: i32,
    pub kind: String,
    pub file_index: i32,
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
    pub timeline_tz: String,
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
    /// Fabricated demo data — must stay false for Perfect/sellable.
    pub demo_seed: bool,
    pub coverage_status: CoverageStatus,
    pub coverage_processed: i32,
    pub coverage_skipped: i32,
    pub coverage_failed: i32,
    pub coverage_unsupported: i32,
    pub scope: ScopeBounds,
    pub report_blockers: Vec<String>,
    pub report_finalizable: bool,
    pub require_sod: bool,
    pub recent_cases: Vec<RecentCaseRow>,
    pub tree_nodes: Vec<TreeNode>,
    pub hex_offset: u64,
    pub hex_lines: Vec<String>,
    pub hex_status: String,
    pub list_total: i32,
    pub list_offset: i32,
    pub list_page_size: i32,
    pub disclosure_line: String,
    pub reduce_motion: bool,
    pub partial_review_enabled: bool,
    pub intake_status: String,
    pub intake_accepted: bool,
    pub hashset_pin: Option<String>,
    pub capabilities: Vec<(String, String, String)>,
    pub graph_edges: Vec<String>,
    pub ai_enabled: bool,
    pub live_preflight_message: String,
    /// Workbench chrome (from institutional UI; model-backed toggles).
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
            timeline_tz: "UTC".into(),
            findings: Vec::new(),
            report_state: "draft".into(),
            last_shortcut: String::new(),
            provenance_open: None,
            transfer_status: String::new(),
            second_method_count: 0,
            blind_pt_status: "none".into(),
            about_disclosure: "SBOM: release-evidence/sbom/; UNSIGNED — Lab use only; NOT court-ready / NOT ISO-certified".into(),
            dark_mode: false,
            locale: "en".into(),
            progress_ratio: 0.0,
            progress_stage: String::new(),
            progress_message: String::new(),
            progress_visible: false,
            progress_cancelled: false,
            demo_seed: false,
            coverage_status: CoverageStatus::NotRun,
            coverage_processed: 0,
            coverage_skipped: 0,
            coverage_failed: 0,
            coverage_unsupported: 0,
            scope: ScopeBounds::default(),
            report_blockers: Vec::new(),
            report_finalizable: false,
            require_sod: true,
            recent_cases: Vec::new(),
            tree_nodes: Vec::new(),
            hex_offset: 0,
            hex_lines: Vec::new(),
            hex_status: "No file selected".into(),
            list_total: 0,
            list_offset: 0,
            list_page_size: 200,
            disclosure_line: "UNSIGNED — Lab use only · NOT court-ready · NOT ISO-certified".into(),
            reduce_motion: false,
            partial_review_enabled: false,
            intake_status: "pending".into(),
            intake_accepted: false,
            hashset_pin: None,
            capabilities: Vec::new(),
            graph_edges: Vec::new(),
            ai_enabled: false,
            live_preflight_message: "Live intake requires Trareon Acquire — Open Acquire".into(),
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
        self.recompute_report_gate();
        self.rebuild_tree();
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

    /// Stub import — only when `demo_seed` is explicitly enabled.
    pub fn import_evidence_stub(&mut self) {
        if !self.demo_seed {
            return;
        }
        self.evidence_count += 1;
        self.active_screen = NavScreen::Evidence;
        self.open_case_focused = false;
    }

    pub fn set_evidence_files(&mut self, files: Vec<EvidenceFileRow>) {
        self.list_total = files.len() as i32;
        self.evidence_count = self.list_total;
        self.evidence_files = files;
        self.selected_file_index = None;
        self.active_screen = NavScreen::Evidence;
        self.open_case_focused = false;
        self.rebuild_tree();
        self.recompute_report_gate();
    }

    pub fn select_file(&mut self, index: usize) -> bool {
        if index < self.evidence_files.len() {
            self.selected_file_index = Some(index);
            true
        } else {
            false
        }
    }

    pub fn push_log(&mut self, line: impl Into<String>) {
        self.log_lines.push(line.into());
    }

    pub fn handle_shortcut(&mut self, key: &str) {
        self.last_shortcut = key.to_string();
        match key {
            "/" => {
                self.palette_open = true;
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
                if self.palette_open {
                    self.palette_open = false;
                } else {
                    self.selected_file_index = None;
                    self.provenance_open = None;
                }
            }
            "1" => self.navigate_to(NavScreen::CaseHome),
            "2" => self.navigate_to(NavScreen::Evidence),
            "3" => self.navigate_to(NavScreen::Search),
            "4" => self.navigate_to(NavScreen::Timeline),
            "5" => self.navigate_to(NavScreen::Bookmarks),
            "6" => self.navigate_to(NavScreen::Report),
            "palette" => self.palette_open = !self.palette_open,
            "inspector" => self.inspector_open = !self.inspector_open,
            "log" => self.log_open = !self.log_open,
            "nav" => self.nav_collapsed = !self.nav_collapsed,
            _ => {}
        }
    }

    pub fn set_artifact_hits(&mut self, hits: Vec<ArtifactHitRow>) {
        // Enforce Indication default for anything unmarked.
        self.artifact_hits = hits
            .into_iter()
            .map(|mut h| {
                if h.claim_type != ClaimType::Indication
                    && h.claim_type != ClaimType::Observation
                    && h.claim_type != ClaimType::Fact
                {
                    h.claim_type = ClaimType::AUTOMATED_DEFAULT;
                }
                h
            })
            .collect();
        self.active_screen = NavScreen::Search;
    }

    pub fn open_provenance(&mut self, provenance_ref: impl Into<String>) {
        self.provenance_open = Some(provenance_ref.into());
    }

    pub fn set_timeline(&mut self, labels: Vec<String>) {
        self.timeline_labels = labels;
        self.active_screen = NavScreen::Timeline;
    }

    pub fn set_timeline_tz(&mut self, timezone: impl Into<String>) {
        self.timeline_tz = timezone.into();
    }

    pub fn accept_intake(&mut self) {
        self.intake_status = "accepted".into();
        self.intake_accepted = true;
        self.recompute_report_gate();
    }

    pub fn reject_intake(&mut self) {
        self.intake_status = "rejected".into();
        self.intake_accepted = false;
        self.recompute_report_gate();
    }

    pub fn add_graph_edge(&mut self, evidence_ref: &str, hex_ref: &str) {
        self.graph_edges.push(format!("{evidence_ref} → {hex_ref}"));
    }

    pub fn set_findings(&mut self, findings: Vec<FindingRow>) {
        self.findings = findings;
        self.recompute_report_gate();
    }

    pub fn set_report_state(&mut self, state: impl Into<String>) {
        self.report_state = state.into();
        self.active_screen = NavScreen::Report;
    }

    /// Honest search: empty results unless caller supplies real hits (no invented matches).
    pub fn set_search(&mut self, query: impl Into<String>, results: Vec<String>) {
        self.search_query = query.into();
        self.search_results = results;
        self.active_screen = NavScreen::Search;
    }

    pub fn set_transfer_status(&mut self, status: impl Into<String>) {
        self.transfer_status = status.into();
    }

    pub fn record_second_method_stub(&mut self) {
        self.second_method_count += 1;
        self.active_screen = NavScreen::SecondMethod;
    }

    pub fn set_blind_pt_status(&mut self, status: impl Into<String>) {
        self.blind_pt_status = status.into();
        self.active_screen = NavScreen::Pt;
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
        if self.progress_ratio < 1.0 {
            self.coverage_status = CoverageStatus::Running;
        }
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

    pub fn recompute_report_gate(&mut self) {
        let integrity: Vec<IntegrityState> =
            self.evidence_files.iter().map(|f| f.integrity).collect();
        let missing = if self.report_state == "draft" && !self.intake_accepted {
            vec!["intake_acceptance".into()]
        } else {
            vec![]
        };
        self.report_blockers = report_blockers(&integrity, self.coverage_status, &missing);
        self.report_finalizable = report_finalizable(&integrity, self.coverage_status, &missing);
    }

    pub fn rebuild_tree(&mut self) {
        let mut nodes = vec![TreeNode {
            label: self.case_title.clone(),
            depth: 0,
            kind: "case".into(),
            file_index: -1,
        }];
        if !self.evidence_files.is_empty() {
            nodes.push(TreeNode {
                label: "Evidence".into(),
                depth: 1,
                kind: "package".into(),
                file_index: -1,
            });
        }
        for (i, f) in self.evidence_files.iter().enumerate() {
            nodes.push(TreeNode {
                label: f.name.clone(),
                depth: 2,
                kind: "file".into(),
                file_index: i as i32,
            });
        }
        self.tree_nodes = nodes;
    }

    pub fn page_slice_indices(&self) -> (usize, usize) {
        let start = self.list_offset.max(0) as usize;
        let end = (start + self.list_page_size.max(1) as usize).min(self.evidence_files.len());
        (start, end)
    }
}
