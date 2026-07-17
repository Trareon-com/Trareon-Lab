//! Day 15: hostile / truncated raw fixtures fail closed.

use lab_storage::RawImage;
use std::fs;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn missing_path_fails_closed() {
    let dir = tempdir().expect("dir");
    let path = dir.path().join("missing.dd");
    let err = RawImage::open_raw(&path).expect_err("missing");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("stat") || msg.contains("open") || msg.contains("missing"),
        "{msg}"
    );
}

#[test]
fn truncated_zero_byte_fails_closed() {
    let dir = tempdir().expect("dir");
    let path = dir.path().join("empty.raw");
    fs::File::create(&path).expect("create");
    let err = RawImage::open_raw(&path).expect_err("empty");
    assert!(format!("{err:?}").contains("empty"));
}

#[test]
fn truncated_after_open_detected_during_hash() {
    let dir = tempdir().expect("dir");
    let path = dir.path().join("live.raw");
    {
        let mut f = fs::File::create(&path).expect("create");
        f.write_all(&[1, 2, 3, 4]).expect("write");
    }
    let mut image = RawImage::open_raw(&path).expect("open");
    // Truncate underneath the open handle size snapshot — hash must fail closed
    // if the file shrinks below measured length.
    fs::write(&path, b"x").expect("truncate");
    let err = image.sha256_hex().expect_err("truncated hash");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("EOF") || msg.contains("read") || msg.contains("unexpected"),
        "{msg}"
    );
}
