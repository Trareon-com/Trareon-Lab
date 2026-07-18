use std::time::{Duration, Instant};

use lab_storage::E01Image;

#[test]
fn oversized_section_fails_closed_within_budget() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("oversized.E01");
    let mut bytes = Vec::from(&b"EVF\x09\x0d\x0a\xff\x00"[..]);
    let mut header = [0u8; 76];
    header[..6].copy_from_slice(b"header");
    header[24..32].copy_from_slice(&u64::MAX.to_le_bytes());
    bytes.extend_from_slice(&header);
    std::fs::write(&path, bytes).unwrap();

    let started = Instant::now();
    let error = E01Image::open(&path).expect_err("hostile E01 must fail");
    assert!(started.elapsed() < Duration::from_secs(1));
    assert!(format!("{error:?}").contains("exceeds file length"));
}

#[test]
fn truncated_section_fails_closed() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("truncated.E01");
    std::fs::write(&path, b"EVF\x09\x0d\x0a\xff\x00header").unwrap();
    let error = E01Image::open(&path).expect_err("truncated E01 must fail");
    assert!(format!("{error:?}").contains("truncated section header"));
}
