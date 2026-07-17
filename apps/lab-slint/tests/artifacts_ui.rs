//! Days 29/35/39 UI coverage.

use lab_slint::{ArtifactHitRow, FindingRow, NavScreen, UiSnapshot};

#[test]
fn artifacts_timeline_findings_report_transfer() {
    let mut snap = UiSnapshot::default();
    snap.set_artifact_hits(vec![ArtifactHitRow {
        kind: "windows.prefetch".into(),
        summary: "NOTEPAD".into(),
        provenance_ref: "prefetch:NOTEPAD".into(),
    }]);
    snap.open_provenance("prefetch:NOTEPAD");
    assert_eq!(snap.provenance_open.as_deref(), Some("prefetch:NOTEPAD"));
    snap.set_timeline(vec!["2026-01-01 NOTEPAD".into()]);
    assert_eq!(snap.active_screen, NavScreen::Timeline);
    snap.set_findings(vec![FindingRow {
        claim: "ran".into(),
        bookmark_uuid: "b1".into(),
    }]);
    snap.set_report_state("draft");
    assert_eq!(snap.report_state, "draft");
    snap.set_transfer_status("export_ready");
    assert_eq!(snap.transfer_status, "export_ready");
    snap.set_search("notepad", vec!["NOTEPAD".into()]);
    assert_eq!(snap.search_results.len(), 1);
}
