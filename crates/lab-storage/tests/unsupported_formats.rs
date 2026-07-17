//! Day 14: unsupported / E01 formats fail closed; capability remains Limited.

use lab_storage::detect_image_kind;
use std::path::Path;

#[test]
fn e01_extension_fails_closed_as_limited() {
    let err = detect_image_kind(Path::new("disk.E01")).expect_err("e01");
    let msg = format!("{err:?}");
    assert!(msg.contains("E01") || msg.contains("Limited"), "{msg}");
}

#[test]
fn unknown_extension_fails_closed() {
    let err = detect_image_kind(Path::new("mystery.xyz")).expect_err("xyz");
    let msg = format!("{err:?}");
    assert!(msg.contains("unsupported"), "{msg}");
}
