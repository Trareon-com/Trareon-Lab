//! DFIR workbench UX: palette, exceptions, timeline import, Runs.

use lab_core::IntegrityState;
use lab_slint::{EvidenceFileRow, NavScreen, UiSnapshot};

#[test]
fn dark_mode_default_for_workbench() {
    let snap = UiSnapshot::default();
    assert!(snap.dark_mode);
    assert!(snap.inspector_open);
    assert!(!snap.palette_open);
}

#[test]
fn palette_commands_and_toggles() {
    let mut snap = UiSnapshot::default();
    snap.handle_shortcut("/");
    assert!(snap.palette_open);
    snap.activate_palette_command("Toggle Nav");
    assert!(snap.nav_collapsed);
    assert!(!snap.palette_open);
    snap.filter_palette("run");
    assert!(snap
        .palette_commands
        .iter()
        .any(|c| c.to_ascii_lowercase().contains("run")));
}

#[test]
fn exceptions_and_runs_placeholder() {
    let mut snap = UiSnapshot::default();
    snap.set_exceptions(vec!["failed · e01 · timeout".into()]);
    assert_eq!(snap.exception_lines.len(), 1);
    snap.set_run_compare(vec!["tool: a → b".into()]);
    assert_eq!(snap.active_screen, NavScreen::Runs);
    snap.navigate_to(NavScreen::Graph);
    assert!(snap.placeholder_body.contains("No correlation") || !snap.placeholder_title.is_empty());
}

#[test]
fn evidence_designation_in_selection() {
    let mut snap = UiSnapshot::default();
    snap.set_evidence_files(vec![EvidenceFileRow {
        path: "/e.dd".into(),
        name: "e.dd".into(),
        size: 10,
        deleted: false,
        integrity: IntegrityState::VerifiedMatch,
        designation: "forensic_copy".into(),
    }]);
    assert!(snap.select_file(0));
    assert_eq!(
        snap.selected_file().map(|f| f.designation.as_str()),
        Some("forensic_copy")
    );
}

#[test]
fn timeline_csv_import_adapter() {
    let mut snap = UiSnapshot::default();
    snap.import_timeline_csv_lines(vec![
        "2020-01-01,MFT,foo".into(),
        "2020-01-02,EVTX,bar".into(),
    ]);
    assert_eq!(snap.active_screen, NavScreen::Timeline);
    assert_eq!(snap.timeline_labels.len(), 2);
}

#[test]
fn search_coverage_label() {
    let mut snap = UiSnapshot::default();
    snap.set_search_coverage("partial · page limit");
    snap.set_search("malware", vec!["hit1".into()]);
    assert!(snap.search_coverage_label.contains("partial"));
}

#[test]
fn cheatsheet_shortcut() {
    let mut snap = UiSnapshot::default();
    snap.handle_shortcut("?");
    assert!(snap.cheatsheet_open);
    snap.handle_shortcut("Escape");
    assert!(!snap.cheatsheet_open);
}
