//! Workbench chrome: inspector, palette, log, nav collapse.

use lab_slint::UiSnapshot;

#[test]
fn workbench_chrome_defaults_and_toggles() {
    let mut snap = UiSnapshot::default();
    assert!(!snap.inspector_open);
    assert!(!snap.nav_collapsed); // labeled rail default (ASCII)
    assert!(!snap.log_open);
    assert!(!snap.palette_open);

    snap.handle_shortcut("palette");
    assert!(snap.palette_open);
    snap.push_log("hashed evidence.bin");
    assert_eq!(snap.log_lines.len(), 1);
    assert_eq!(snap.last_action, "hashed evidence.bin");

    snap.handle_shortcut("inspector");
    assert!(snap.inspector_open);
    snap.handle_shortcut("log");
    assert!(snap.log_open);
    snap.handle_shortcut("nav");
    assert!(snap.nav_collapsed); // collapsed to icon rail
}
