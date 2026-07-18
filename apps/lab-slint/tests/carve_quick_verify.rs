//! Quick Verify / carve wiring (headless).

use lab_slint::{LabSession, NavScreen, UiSnapshot};
use std::io::Write;

#[test]
fn carve_common_media_on_demo_fixture() {
    let demo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../testdata/demo/demo-disk.dd");
    assert!(demo.is_file(), "missing {}", demo.display());
    let hits = LabSession::carve_common_media(&demo).expect("carve");
    assert!(
        hits.iter()
            .any(|h| h.starts_with("jpg") || h.starts_with("pdf")),
        "expected jpg/pdf hits, got {hits:?}"
    );
}

#[test]
fn quick_verify_nav_and_meta_model() {
    let mut snap = UiSnapshot::default();
    snap.navigate_to(NavScreen::QuickVerify);
    assert_eq!(snap.active_screen, NavScreen::QuickVerify);
    snap.carve_hit_lines = vec!["jpg · 0-10 · abcd · High".into()];
    snap.quick_verify_meta_lines = vec!["path · /tmp/x".into()];
    snap.update_placeholder_for_screen();
    assert!(snap.placeholder_body.contains("1 carve"));
}

#[test]
fn palette_lists_quick_verify_and_demo() {
    let cmds = UiSnapshot::default_palette_commands();
    assert!(cmds.iter().any(|c| c == "Quick Verify"));
    assert!(cmds.iter().any(|c| c == "Load Demo Case"));
    assert!(cmds.iter().any(|c| c == "Run Carving"));
}

#[test]
fn carve_tmp_raw_roundtrip() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("tiny.dd");
    let mut blob = vec![0u8; 512];
    let jpeg = [0xFFu8, 0xD8, 0xFF, 0xE0, 1, 2, 3, 4, 0xFF, 0xD9];
    blob[10..10 + jpeg.len()].copy_from_slice(&jpeg);
    std::fs::File::create(&path)
        .unwrap()
        .write_all(&blob)
        .unwrap();
    let hits = LabSession::carve_common_media(&path).unwrap();
    assert!(hits.iter().any(|h| h.starts_with("jpg")));
}
