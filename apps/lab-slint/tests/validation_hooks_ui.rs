//! FR-VAL-009/010 UI hooks.

use lab_slint::UiSnapshot;

#[test]
fn second_method_and_blind_pt_ui_hooks() {
    let mut snap = UiSnapshot::default();
    snap.record_second_method_stub();
    assert_eq!(snap.second_method_count, 1);
    snap.set_blind_pt_status("imported_embargoed");
    assert_eq!(snap.blind_pt_status, "imported_embargoed");
}
