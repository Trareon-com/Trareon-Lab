use std::fs;

use lab_core::NullProgress;
use lab_fs::{
    detect_apfs, enumerate_apfs, parse_apfs_container, write_minimal_apfs_image, FsEntryKind,
};
use lab_storage::RawImage;

#[test]
fn apfs_golden_enumerates_volume_and_catalog() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("apfs.img");
    write_minimal_apfs_image(&path).unwrap();

    let mut image = RawImage::open_raw(&path).unwrap();
    assert!(detect_apfs(&mut image).unwrap());
    let container = parse_apfs_container(&mut image).unwrap();
    assert_eq!(container.block_size, 4096);
    assert_eq!(container.volume_count, 1);
    assert_eq!(container.uuid, *b"TRAREON-APFS-NX!");

    let volumes = enumerate_apfs(&mut image, &mut NullProgress).unwrap();
    assert_eq!(volumes.len(), 1);
    assert_eq!(volumes[0].name, "Golden");
    assert_eq!(volumes[0].uuid, *b"TRAREON-APFS-VOL");
    assert_eq!(volumes[0].entries.len(), 2);
    assert_eq!(volumes[0].entries[0].path, "/Documents");
    assert_eq!(volumes[0].entries[0].kind, FsEntryKind::Directory);
    assert_eq!(volumes[0].entries[1].path, "/Documents/hello.txt");
    assert_eq!(volumes[0].entries[1].kind, FsEntryKind::File);
    assert_eq!(volumes[0].entries[1].size, 4);
}

#[test]
fn apfs_truncated_input_returns_error() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("truncated.img");
    fs::write(&path, [0_u8; 35]).unwrap();
    let mut image = RawImage::open_raw(&path).unwrap();
    assert!(parse_apfs_container(&mut image).is_err());
}

#[test]
fn apfs_bad_magic_returns_error() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("bad-magic.img");
    fs::write(&path, [0_u8; 4096]).unwrap();
    let mut image = RawImage::open_raw(&path).unwrap();
    assert!(parse_apfs_container(&mut image).is_err());
}
