//! Progress reporting during raw image hash.

use lab_core::{FnProgress, ProgressEvent};
use lab_storage::{ImageReader, RawImage};
use std::io::Write;

#[test]
fn hash_emits_progress_events() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("sample.raw");
    {
        let mut f = std::fs::File::create(&path).unwrap();
        f.write_all(&[0u8; 200_000]).unwrap();
    }
    let mut img = RawImage::open_raw(&path).unwrap();
    let mut ticks = 0u32;
    let mut sink = FnProgress(|_ev: ProgressEvent| {
        ticks += 1;
    });
    let digest = img.sha256_hex_with_progress(&mut sink).unwrap().unwrap();
    assert_eq!(digest.len(), 64);
    assert!(ticks >= 2, "expected multiple progress ticks, got {ticks}");
    assert_eq!(img.byte_length(), 200_000);
    let mut buf = [0u8; 16];
    assert_eq!(img.read_at(0, &mut buf).unwrap(), 16);
}
