use lab_slint::UiSnapshot;

#[test]
fn progress_bar_model_updates() {
    let mut snap = UiSnapshot::default();
    assert!(!snap.progress_visible);
    snap.set_progress("import", 50, Some(100), "hashing");
    assert!(snap.progress_visible);
    assert!((snap.progress_ratio - 0.5).abs() < f64::EPSILON);
    assert_eq!(snap.progress_stage, "import");
    snap.request_cancel_progress();
    assert!(snap.progress_cancelled);
    snap.clear_progress();
    assert!(!snap.progress_visible);
}
