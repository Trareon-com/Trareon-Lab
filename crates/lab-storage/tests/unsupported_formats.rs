//! Day 14+: format detection — E01 recognized; Ex01 still limited.

use lab_storage::{detect_image_kind, ImageKind};
use std::path::Path;

#[test]
fn e01_extension_detected() {
    let kind = detect_image_kind(Path::new("disk.E01")).expect("e01");
    assert_eq!(kind, ImageKind::E01);
}

#[test]
fn unknown_extension_fails_closed() {
    let err = detect_image_kind(Path::new("mystery.xyz")).expect_err("xyz");
    let msg = format!("{err:?}");
    assert!(msg.contains("unsupported"), "{msg}");
}
