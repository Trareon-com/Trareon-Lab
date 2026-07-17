//! F10: UI smoke — open case, coverage counts, keyboard focus on primary action.

use lab_slint::UiSnapshot;

#[test]
fn ui_smoke_opens_case_shows_coverage_and_focus() {
    let mut snap = UiSnapshot::default();
    assert_eq!(snap.case_title, "(no case)");
    assert_eq!(snap.coverage_count, 0);
    assert!(snap.open_case_focused);

    snap.open_case("CASE-SMOKE", 3, 5);
    assert_eq!(snap.case_title, "CASE-SMOKE");
    assert_eq!(snap.evidence_count, 3);
    assert_eq!(snap.coverage_count, 5);
    assert!(snap.open_case_focused);

    snap.open_case_focused = false;
    snap.focus_open_case();
    assert!(snap.open_case_focused, "primary Open Case action must take focus");
}
