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
    QuickVerify,
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
            Self::QuickVerify => "QuickVerify",
        }
    }

    pub fn from_label(label: &str) -> Self {
        match label {
            "Home" | "Case" | "CaseHome" => Self::CaseHome,
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
            "QuickVerify" | "Quick Verify" => Self::QuickVerify,
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
            Self::QuickVerify,
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
    /// Coverage / parser failures for Case Home Exceptions inbox.
    pub exception_lines: Vec<String>,
    /// RunManifest compare lines for Runs screen.
    pub run_compare_lines: Vec<String>,
    /// Honest search coverage banner (e.g. "partial · first page").
    pub search_coverage_label: String,
    /// Palette command ids currently shown.
    pub palette_commands: Vec<String>,
    /// YARA / hash-set hit summaries (Validated engine → UI).
    pub yara_hit_lines: Vec<String>,
    pub hashset_hit_lines: Vec<String>,
    /// Transfer / export honesty labels.
    pub export_status: String,
    pub transfer_trust_label: String,
    /// Placeholder body for nav destinations without a dedicated screen.
    pub placeholder_title: String,
    pub placeholder_body: String,
    /// Prefs path last used (persisted when available).
    pub prefs_last_case: String,
    /// Keyboard cheat-sheet overlay.
    pub cheatsheet_open: bool,
    /// Signature carve hit lines (Quick Verify / Run carving).
    pub carve_hit_lines: Vec<String>,
    /// Quick Verify file metadata lines (path, size, sha256, mtime).
    pub quick_verify_meta_lines: Vec<String>,
    /// Ephemeral Quick Verify case directory (for cleanup).
    pub ephemeral_case_dir: Option<String>,
    /// Host-file timeline honesty (mtime only).
    pub quick_verify_timeline_lines: Vec<String>,
    /// Inspector content-viewer tab: "properties" | "hex".
    pub inspector_tab: String,
    /// Ambient status-bar last action line.
    pub last_action: String,
    /// Window width &lt; 1100 — compact chrome (set from host resize).
    pub layout_compact: bool,
    /// When compact, inspector draws as overlay instead of side panel.
    pub inspector_overlay: bool,
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
            capabilities: Self::default_capabilities(),
            graph_edges: Vec::new(),
            ai_enabled: false,
            live_preflight_message: "Live intake requires Trareon Acquire — Open Acquire".into(),
            nav_collapsed: true,
            inspector_open: false,
            log_open: false,
            palette_open: false,
            log_lines: Vec::new(),
            exception_lines: Vec::new(),
            run_compare_lines: Vec::new(),
            search_coverage_label: String::new(),
            palette_commands: Self::default_palette_commands(),
            yara_hit_lines: Vec::new(),
            hashset_hit_lines: Vec::new(),
            export_status: String::new(),
            transfer_trust_label: String::new(),
            placeholder_title: String::new(),
            placeholder_body: String::new(),
            prefs_last_case: String::new(),
            cheatsheet_open: false,
            carve_hit_lines: Vec::new(),
            quick_verify_meta_lines: Vec::new(),
            ephemeral_case_dir: None,
            quick_verify_timeline_lines: Vec::new(),
            inspector_tab: "properties".into(),
            last_action: String::new(),
            layout_compact: false,
            inspector_overlay: false,
        }
    }
}

impl UiSnapshot {
    pub fn default_palette_commands() -> Vec<String> {
        vec![
            "Open Case".into(),
            "Import Evidence".into(),
            "Quick Verify".into(),
            "Load Demo Case".into(),
            "Run Carving".into(),
            "Go Case".into(),
            "Go Evidence".into(),
            "Go Search".into(),
            "Go Timeline".into(),
            "Go Bookmarks".into(),
            "Go Report".into(),
            "Go Runs".into(),
            "Go Transfer".into(),
            "Go Artifacts".into(),
            "Go Graph".into(),
            "Go Capabilities".into(),
            "Go About".into(),
            "Focus Hex".into(),
            "Toggle Inspector".into(),
            "Toggle Log".into(),
            "Toggle Nav".into(),
            "Toggle Cheatsheet".into(),
        ]
    }

    /// Honest capability matrix rows (display-only; guide-backed Validated claims).
    pub fn default_capabilities() -> Vec<(String, String, String)> {
        vec![
            (
                "case".into(),
                "Validated".into(),
                "Case lifecycle + lock".into(),
            ),
            (
                "examination".into(),
                "Validated".into(),
                "Evidence / search / timeline shell".into(),
            ),
            (
                "transfer".into(),
                "Validated".into(),
                "Ed25519 signed package".into(),
            ),
            (
                "export".into(),
                "Validated".into(),
                "HTML / PDF/A / CASE-UCO drafts".into(),
            ),
            (
                "hex".into(),
                "Indication".into(),
                "Byte preview via Inspector".into(),
            ),
            (
                "graph".into(),
                "Indication".into(),
                "Edge list only — no force layout".into(),
            ),
        ]
    }

    pub fn selected_file(&self) -> Option<&EvidenceFileRow> {
        self.selected_file_index
            .and_then(|i| self.evidence_files.get(i))
    }

    pub fn set_exceptions(&mut self, lines: Vec<String>) {
        self.exception_lines = lines;
    }

    pub fn set_run_compare(&mut self, lines: Vec<String>) {
        self.run_compare_lines = lines;
        self.active_screen = NavScreen::Runs;
    }

    pub fn set_search_coverage(&mut self, label: impl Into<String>) {
        self.search_coverage_label = label.into();
    }

    pub fn activate_palette_command(&mut self, cmd: &str) {
        self.palette_open = false;
        self.push_log(format!("palette · {cmd}"));
        match cmd {
            "Open Case" => {
                self.open_case_focused = true;
                self.navigate_to(NavScreen::CaseHome);
            }
            "Import Evidence" => self.navigate_to(NavScreen::Evidence),
            "Quick Verify" => self.navigate_to(NavScreen::QuickVerify),
            "Load Demo Case" => self.navigate_to(NavScreen::CaseHome),
            "Run Carving" => self.navigate_to(NavScreen::Evidence),
            "Go Case" => self.navigate_to(NavScreen::CaseHome),
            "Go Evidence" => self.navigate_to(NavScreen::Evidence),
            "Go Search" => self.navigate_to(NavScreen::Search),
            "Go Timeline" => self.navigate_to(NavScreen::Timeline),
            "Go Bookmarks" => self.navigate_to(NavScreen::Bookmarks),
            "Go Report" => self.navigate_to(NavScreen::Report),
            "Go Runs" => self.navigate_to(NavScreen::Runs),
            "Go Transfer" => self.navigate_to(NavScreen::Transfer),
            "Go Artifacts" => self.navigate_to(NavScreen::Artifacts),
            "Go Graph" => self.navigate_to(NavScreen::Graph),
            "Go Capabilities" => self.navigate_to(NavScreen::Capabilities),
            "Go About" => self.navigate_to(NavScreen::About),
            "Focus Hex" => {
                self.inspector_open = true;
                self.inspector_tab = "hex".into();
                self.last_action = "inspector · hex".into();
            }
            "Toggle Inspector" => self.inspector_open = !self.inspector_open,
            "Toggle Log" => self.log_open = !self.log_open,
            "Toggle Nav" => self.nav_collapsed = !self.nav_collapsed,
            "Toggle Cheatsheet" => self.cheatsheet_open = !self.cheatsheet_open,
            _ => {}
        }
    }

    pub fn filter_palette(&mut self, filter: &str) {
        let f = filter.trim().to_ascii_lowercase();
        if f.is_empty() {
            self.palette_commands = Self::default_palette_commands();
            return;
        }
        self.palette_commands = Self::default_palette_commands()
            .into_iter()
            .filter(|c| c.to_ascii_lowercase().contains(&f))
            .collect();
    }

    pub fn update_placeholder_for_screen(&mut self) {
        let (title, body) = match self.active_screen {
            NavScreen::Hex => (
                "Hex".into(),
                "Hex lives in the Inspector when a file is selected. Open Evidence, select a row."
                    .into(),
            ),
            NavScreen::Artifacts => (
                "Artifacts".into(),
                "Artifact parsers run via Search / intake. No fabricated hits.".into(),
            ),
            NavScreen::Graph => {
                if self.graph_edges.is_empty() {
                    (
                        "Graph".into(),
                        "No correlation edges yet. Edges appear when session records them.".into(),
                    )
                } else {
                    ("Graph".into(), self.graph_edges.join("\n"))
                }
            }
            NavScreen::Hypotheses => (
                "Hypotheses".into(),
                "Hypothesis register is not in this build.".into(),
            ),
            NavScreen::SecondMethod => (
                "Second method".into(),
                "Record second-method validation from Runs / validation hooks.".into(),
            ),
            NavScreen::Pt => (
                "Blind PT".into(),
                format!("Status: {}", self.blind_pt_status),
            ),
            NavScreen::Capabilities => (
                "Capabilities".into(),
                if self.capabilities.is_empty() {
                    "See docs/validation dossiers for Validated subsets.".into()
                } else {
                    self.capabilities
                        .iter()
                        .map(|(id, status, note)| format!("{id} · {status} · {note}"))
                        .collect::<Vec<_>>()
                        .join("\n")
                },
            ),
            NavScreen::About => ("About".into(), self.about_disclosure.clone()),
            NavScreen::QuickVerify => (
                "Quick Verify".into(),
                if self.carve_hit_lines.is_empty() {
                    "Pick a file — ephemeral scan (hash + signature carve). Not a custody case."
                        .into()
                } else {
                    format!("{} carve hit(s)", self.carve_hit_lines.len())
                },
            ),
            NavScreen::Transfer => (
                "Transfer".into(),
                if self.transfer_status.is_empty() {
                    "Export a signed package from an open case (Ed25519). Tamper → Invalid.".into()
                } else {
                    format!(
                        "{}\nTrust: {}",
                        self.transfer_status, self.transfer_trust_label
                    )
                },
            ),
            NavScreen::Runs => (
                "Runs".into(),
                if self.run_compare_lines.is_empty() {
                    "No run manifests compared yet.".into()
                } else {
                    self.run_compare_lines.join("\n")
                },
            ),
            _ => (String::new(), String::new()),
        };
        self.placeholder_title = title;
        self.placeholder_body = body;
    }

    /// Import Plaso/Hayabusa-style CSV lines into timeline (honest adapter; no sidecar spawn).
    pub fn import_timeline_csv_lines(&mut self, lines: Vec<String>) {
        self.timeline_labels = lines;
        self.active_screen = NavScreen::Timeline;
        self.push_log(format!(
            "timeline import · {} event label(s)",
            self.timeline_labels.len()
        ));
    }

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
        self.update_placeholder_for_screen();
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
        let line = line.into();
        self.last_action = line.clone();
        self.log_lines.push(line);
    }

    pub fn step_hex_offset(&mut self, delta: i64) {
        let next = if delta < 0 {
            self.hex_offset.saturating_sub((-delta) as u64)
        } else {
            self.hex_offset.saturating_add(delta as u64)
        };
        self.hex_offset = next;
        self.last_action = format!("hex · offset {next:#x}");
    }

    /// Responsive breakpoints (desktop Slint — not CSS). Preferred 1280; compact &lt; 1100.
    pub fn apply_layout_width(&mut self, width_px: i32) {
        let compact = width_px < 1100;
        self.layout_compact = compact;
        self.inspector_overlay = compact;
        if compact {
            self.nav_collapsed = true;
        }
    }

    pub fn nav_expanded(&self) -> bool {
        !self.nav_collapsed && !self.layout_compact
    }

    pub fn inspector_as_side(&self) -> bool {
        self.inspector_open && !self.inspector_overlay
    }

    pub fn inspector_as_overlay(&self) -> bool {
        self.inspector_open && self.inspector_overlay
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
                } else if self.cheatsheet_open {
                    self.cheatsheet_open = false;
                } else if self.log_open {
                    self.log_open = false;
                } else if self.inspector_open {
                    self.inspector_open = false;
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
            "i" => self.inspector_open = !self.inspector_open,
            "?" => self.cheatsheet_open = !self.cheatsheet_open,
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
