//! Day 11: open synthetic raw/dd and compute SHA-256.

use lab_storage::{detect_image_kind, ImageKind, RawImage};
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn open_synthetic_raw_and_hash() {
    let mut file = NamedTempFile::new().expect("tmp");
    let payload = b"TRA REON-RAW-FIXTURE-DAY11";
    file.write_all(payload).expect("write");
    file.flush().expect("flush");

    let kind = detect_image_kind(file.path()).expect("detect");
    assert_eq!(kind, ImageKind::RawDd);

    let mut image = RawImage::open_raw(file.path()).expect("open");
    assert_eq!(image.byte_length(), payload.len() as u64);
    assert_eq!(image.kind(), ImageKind::RawDd);

    let digest = image.sha256_hex().expect("hash");
    assert_eq!(digest.len(), 64);
    // Second pass must be identical (deterministic).
    assert_eq!(image.sha256_hex().expect("hash2"), digest);
}

#[test]
fn empty_raw_fails_closed() {
    let file = NamedTempFile::new().expect("tmp");
    let err = RawImage::open_raw(file.path()).expect_err("empty");
    let msg = format!("{err:?}");
    assert!(msg.contains("empty"), "{msg}");
}
