//! Day 19–20 UI: file list + keyboard stubs.

use lab_slint::{EvidenceFileRow, NavScreen, UiSnapshot};

#[test]
fn evidence_file_list_and_selection() {
    let mut snap = UiSnapshot::default();
    snap.open_case("CASE", 1, 0);
    snap.set_evidence_files(vec![
        EvidenceFileRow {
            path: r"\Windows\notepad.exe".into(),
            name: "notepad.exe".into(),
            size: 4096,
            deleted: false,
        },
        EvidenceFileRow {
            path: r"\gone.tmp".into(),
            name: "gone.tmp".into(),
            size: 10,
            deleted: true,
        },
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
    snap.set_evidence_files(vec![EvidenceFileRow {
        path: r"\a.txt".into(),
        name: "a.txt".into(),
        size: 1,
        deleted: false,
    }]);
    snap.select_file(0);
    snap.handle_shortcut("b");
    assert_eq!(snap.bookmark_count, 1);
    assert_eq!(snap.active_screen, NavScreen::Bookmarks);
    snap.handle_shortcut("/");
    assert_eq!(snap.active_screen, NavScreen::Search);
    snap.handle_shortcut("Escape");
    assert!(snap.selected_file_index.is_none());
}
