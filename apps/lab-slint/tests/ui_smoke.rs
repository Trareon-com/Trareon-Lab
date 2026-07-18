//! F10 + Day 8: UI smoke — navigation model for examination screens.

use lab_slint::{NavScreen, UiSnapshot};

#[test]
fn ui_smoke_opens_case_shows_coverage_and_focus() {
    let mut snap = UiSnapshot::default();
    assert_eq!(snap.case_title, "(no case)");
    assert_eq!(snap.coverage_count, 0);
    assert!(snap.open_case_focused);
    assert_eq!(snap.active_screen, NavScreen::CaseHome);

    snap.open_case("CASE-SMOKE", 3, 5);
    assert_eq!(snap.case_title, "CASE-SMOKE");
    assert_eq!(snap.evidence_count, 3);
    assert_eq!(snap.coverage_count, 5);
    assert!(snap.open_case_focused);

    snap.open_case_focused = false;
    snap.focus_open_case();
    assert!(
        snap.open_case_focused,
        "primary Open Case action must take focus"
    );
}

#[test]
fn navigation_covers_examination_screens() {
    let mut snap = UiSnapshot::default();
    snap.open_case("CASE-NAV", 1, 1);
    snap.set_bookmark_count(2);

    let expected = NavScreen::all();
    assert_eq!(expected.len(), 16);

    for &screen in expected {
        snap.navigate_to(screen);
        assert_eq!(snap.active_screen, screen);
        assert_eq!(snap.active_screen.label(), screen.label());
        if screen == NavScreen::CaseHome {
            assert!(snap.open_case_focused);
        } else {
            assert!(!snap.open_case_focused);
        }
    }

    assert_eq!(snap.bookmark_count, 2);
}

#[test]
fn import_evidence_stub_disabled_without_demo_seed() {
    let mut snap = UiSnapshot::default();
    snap.open_case("CASE-IMPORT", 0, 0);
    snap.import_evidence_stub();
    assert_eq!(snap.evidence_count, 0);
    assert_eq!(snap.active_screen, NavScreen::CaseHome);

    snap.demo_seed = true;
    snap.import_evidence_stub();
    assert_eq!(snap.evidence_count, 1);
    assert_eq!(snap.active_screen, NavScreen::Evidence);
}

#[test]
fn intake_timeline_graph_and_ai_defaults_are_honest() {
    let mut snap = UiSnapshot::default();
    assert_eq!(snap.intake_status, "pending");
    assert!(!snap.intake_accepted);
    assert!(!snap.ai_enabled);
    assert!(snap.live_preflight_message.contains("Acquire"));

    snap.set_timeline_tz("Asia/Jakarta");
    assert_eq!(snap.timeline_tz, "Asia/Jakarta");
    snap.add_graph_edge("evidence:1", "hex:0x200");
    assert_eq!(snap.graph_edges, ["evidence:1 → hex:0x200"]);

    snap.accept_intake();
    assert_eq!(snap.intake_status, "accepted");
    assert!(snap.intake_accepted);
    assert!(!snap
        .report_blockers
        .iter()
        .any(|blocker| blocker.contains("intake_acceptance")));
    snap.reject_intake();
    assert_eq!(snap.intake_status, "rejected");
    assert!(!snap.intake_accepted);
    assert!(snap
        .report_blockers
        .iter()
        .any(|blocker| blocker.contains("intake_acceptance")));
}
