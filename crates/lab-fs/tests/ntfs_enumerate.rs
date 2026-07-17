//! Day 16: enumerate synthetic NTFS corpus.

use lab_fs::{enumerate_ntfs_synthetic, write_ntfs_synthetic_corpus, FsEntryKind, NtfsSynthEntry};
use std::path::PathBuf;

fn demo_entries() -> Vec<NtfsSynthEntry> {
    vec![
        NtfsSynthEntry {
            record_number: 5,
            parent_record: 5,
            kind: FsEntryKind::Directory,
            deleted: false,
            size: 0,
            name: ".".into(),
        },
        NtfsSynthEntry {
            record_number: 100,
            parent_record: 5,
            kind: FsEntryKind::Directory,
            deleted: false,
            size: 0,
            name: "Windows".into(),
        },
        NtfsSynthEntry {
            record_number: 101,
            parent_record: 100,
            kind: FsEntryKind::Directory,
            deleted: false,
            size: 0,
            name: "System32".into(),
        },
        NtfsSynthEntry {
            record_number: 200,
            parent_record: 101,
            kind: FsEntryKind::File,
            deleted: false,
            size: 4096,
            name: "notepad.exe".into(),
        },
        NtfsSynthEntry {
            record_number: 201,
            parent_record: 101,
            kind: FsEntryKind::File,
            deleted: true,
            size: 128,
            name: "gone.tmp".into(),
        },
        NtfsSynthEntry {
            record_number: 150,
            parent_record: 5,
            kind: FsEntryKind::File,
            deleted: false,
            size: 16,
            name: "readme.txt".into(),
        },
    ]
}

fn corpus_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../validation/synthetic/ntfs/ntfs-synth-demo.trfsnt")
}

#[test]
fn enumerate_demo_ntfs_synthetic_corpus() {
    let path = corpus_path();
    write_ntfs_synthetic_corpus(&path, &demo_entries()).expect("write corpus");

    let entries = enumerate_ntfs_synthetic(&path).expect("enumerate");
    assert_eq!(entries.len(), 6);

    let notepad = entries
        .iter()
        .find(|e| e.name == "notepad.exe")
        .expect("notepad");
    assert_eq!(notepad.path, "\\Windows\\System32\\notepad.exe");
    assert_eq!(notepad.kind, FsEntryKind::File);
    assert!(!notepad.deleted);
    assert_eq!(notepad.size, 4096);

    let gone = entries.iter().find(|e| e.name == "gone.tmp").expect("gone");
    assert!(gone.deleted);
    assert_eq!(gone.path, "\\Windows\\System32\\gone.tmp");

    let root = entries.iter().find(|e| e.record_number == 5).expect("root");
    assert_eq!(root.path, "\\");
    assert_eq!(root.kind, FsEntryKind::Directory);
}

#[test]
fn bad_magic_fails_closed() {
    let dir = tempfile::tempdir().expect("dir");
    let path = dir.path().join("bad.trfsnt");
    std::fs::write(&path, b"NOTMAGIC").expect("write");
    let err = enumerate_ntfs_synthetic(&path).expect_err("bad magic");
    assert!(format!("{err:?}").contains("magic"));
}

#[test]
fn missing_parent_fails_closed() {
    let dir = tempfile::tempdir().expect("dir");
    let path = dir.path().join("orphan.trfsnt");
    write_ntfs_synthetic_corpus(
        &path,
        &[NtfsSynthEntry {
            record_number: 9,
            parent_record: 999,
            kind: FsEntryKind::File,
            deleted: false,
            size: 1,
            name: "orphan.bin".into(),
        }],
    )
    .expect("write");
    let err = enumerate_ntfs_synthetic(&path).expect_err("orphan");
    assert!(format!("{err:?}").contains("parent"));
}
