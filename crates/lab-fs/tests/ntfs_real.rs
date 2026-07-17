//! Real NTFS volume enumeration tests.

use lab_core::NullProgress;
use lab_fs::{
    enumerate_ntfs, parse_usn_journal, write_minimal_ntfs_image, FsEntryKind, NtfsEnumerateOptions,
    NtfsVolume,
};
use lab_storage::{ImageReader, RawImage};
use std::sync::atomic::{AtomicU32, Ordering};

#[test]
fn enumerate_minimal_ntfs_fixture() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("mini.ntfs.img");
    write_minimal_ntfs_image(&path).unwrap();

    let mut img = RawImage::open_raw(&path).unwrap();
    let vol = NtfsVolume::open(&mut img).unwrap();
    assert_eq!(vol.boot.bytes_per_sector, 512);
    assert_eq!(vol.boot.mft_record_size, 1024);

    let ticks = AtomicU32::new(0);
    let mut progress = lab_core::FnProgress(|_| {
        ticks.fetch_add(1, Ordering::SeqCst);
    });
    let entries = enumerate_ntfs(&mut img, &mut progress, NtfsEnumerateOptions::default()).unwrap();

    let secret = entries
        .iter()
        .find(|e| e.name == "secret.txt")
        .expect("secret.txt");
    assert!(!secret.deleted);
    assert_eq!(secret.kind, FsEntryKind::File);
    assert!(secret.path.contains("secret.txt"));

    let gone = entries
        .iter()
        .find(|e| e.name == "gone.txt")
        .expect("gone.txt");
    assert!(gone.deleted);

    assert!(ticks.load(Ordering::SeqCst) >= 1);
}

#[test]
fn parse_usn_sample() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("mini.ntfs.img");
    write_minimal_ntfs_image(&path).unwrap();
    let mut img = RawImage::open_raw(&path).unwrap();
    let len = img.byte_length();
    let mut buf = vec![0u8; 4096];
    img.read_at(len - 4096, &mut buf).unwrap();
    let recs = parse_usn_journal(&buf).unwrap();
    assert!(!recs.is_empty());
    assert_eq!(recs[0].file_name, "secret.txt");
}
