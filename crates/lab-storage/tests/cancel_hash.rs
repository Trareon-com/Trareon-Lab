//! Day 12: cancellable hash must not return a partial digest as success.

use lab_storage::RawImage;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tempfile::NamedTempFile;

#[test]
fn cancel_hash_returns_none_without_false_digest() {
    let mut file = NamedTempFile::new().expect("tmp");
    // Large enough to require multiple 64KiB chunks.
    let chunk = vec![0xA5_u8; 64 * 1024];
    for _ in 0..8 {
        file.write_all(&chunk).expect("write");
    }
    file.flush().expect("flush");

    let cancel = Arc::new(AtomicBool::new(false));
    let cancel_flag = Arc::clone(&cancel);
    let mut image = RawImage::open_raw(file.path()).expect("open");

    let mut chunks_seen = 0_u32;
    let result = image
        .sha256_hex_cancellable(|| {
            chunks_seen += 1;
            if chunks_seen >= 2 {
                cancel_flag.store(true, Ordering::SeqCst);
                true
            } else {
                false
            }
        })
        .expect("cancellable hash");

    assert!(result.is_none(), "cancel must not yield a digest");
    assert!(cancel.load(Ordering::SeqCst));
}

#[test]
fn complete_cancellable_hash_returns_digest() {
    let mut file = NamedTempFile::new().expect("tmp");
    file.write_all(b"complete-hash").expect("write");
    file.flush().expect("flush");
    let mut image = RawImage::open_raw(file.path()).expect("open");
    let digest = image
        .sha256_hex_cancellable(|| false)
        .expect("ok")
        .expect("digest");
    assert_eq!(digest, image.sha256_hex().expect("full"));
}
