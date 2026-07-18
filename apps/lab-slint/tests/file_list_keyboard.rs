//! Day 19–20 UI: file list + keyboard stubs.

use lab_core::IntegrityState;
use lab_slint::{EvidenceFileRow, NavScreen, UiSnapshot};

fn row(path: &str, name: &str, size: u64, deleted: bool) -> EvidenceFileRow {
    EvidenceFileRow {
        path: path.into(),
        name: name.into(),
        size,
        deleted,
        integrity: IntegrityState::NotRun,
        designation: "logical_export".into(),
    }
}

#[test]
fn evidence_file_list_and_selection() {
    let mut snap = UiSnapshot::default();
    snap.open_case("CASE", 1, 0);
    snap.set_evidence_files(vec![
        row(r"\Windows\notepad.exe", "notepad.exe", 4096, false),
        row(r"\gone.tmp", "gone.tmp", 10, true),
    ]);
    assert_eq!(snap.active_screen, NavScreen::Evidence);
    assert_eq!(snap.evidence_files.len(), 2);
    assert!(snap.select_file(1));
    assert_eq!(snap.selected_file_index, Some(1));
    assert!(!snap.select_file(9));
}

#[test]
fn keyboard_shortcuts_bookmark_and_search() {
    let mut snap = UiSnapshot::default();
    snap.open_case("CASE", 1, 0);
    snap.set_evidence_files(vec![row(r"\a.txt", "a.txt", 1, false)]);
    snap.select_file(0);
    snap.handle_shortcut("b");
    assert_eq!(snap.bookmark_count, 1);
    assert_eq!(snap.active_screen, NavScreen::Bookmarks);
    snap.handle_shortcut("/");
    assert!(snap.palette_open);
    snap.handle_shortcut("Escape");
    assert!(!snap.palette_open);
    assert_eq!(snap.selected_file_index, Some(0));
    snap.handle_shortcut("Escape");
    assert!(snap.selected_file_index.is_none());
    snap.handle_shortcut("2");
    assert_eq!(snap.active_screen, NavScreen::Evidence);
    assert!(!snap.nav_collapsed); // labeled rail default (ASCII)
    snap.handle_shortcut("nav");
    assert!(snap.nav_collapsed); // collapse to icon rail
}

#[test]
fn demo_seed_off_blocks_stub_import() {
    let mut snap = UiSnapshot::default();
    assert!(!snap.demo_seed);
    snap.import_evidence_stub();
    assert_eq!(snap.evidence_count, 0);
}

#[test]
fn mismatch_blocks_finalize() {
    let mut snap = UiSnapshot::default();
    snap.open_case("CASE", 1, 0);
    snap.intake_accepted = true;
    snap.coverage_status = lab_core::CoverageStatus::Complete;
    snap.set_evidence_files(vec![EvidenceFileRow {
        path: "/e".into(),
        name: "e".into(),
        size: 1,
        deleted: false,
        integrity: IntegrityState::VerifiedMismatch,
        designation: "forensic_copy".into(),
    }]);
    assert!(!snap.report_finalizable);
    assert!(!snap.report_blockers.is_empty());
}
