//! Day 17: FAT32 + exFAT synthetic enumeration.

use lab_fs::{
    enumerate_exfat_synthetic, enumerate_fat32_synthetic, write_exfat_synthetic_corpus,
    write_fat32_synthetic_corpus, FatSynthEntry, FsEntryKind,
};
use std::path::PathBuf;

fn sample() -> Vec<FatSynthEntry> {
    vec![
        FatSynthEntry {
            cluster_id: 0,
            parent_cluster: 0,
            kind: FsEntryKind::Directory,
            deleted: false,
            size: 0,
            name: ".".into(),
        },
        FatSynthEntry {
            cluster_id: 2,
            parent_cluster: 0,
            kind: FsEntryKind::Directory,
            deleted: false,
            size: 0,
            name: "docs".into(),
        },
        FatSynthEntry {
            cluster_id: 3,
            parent_cluster: 2,
            kind: FsEntryKind::File,
            deleted: false,
            size: 42,
            name: "note.txt".into(),
        },
        FatSynthEntry {
            cluster_id: 4,
            parent_cluster: 0,
            kind: FsEntryKind::File,
            deleted: true,
            size: 7,
            name: "old.bak".into(),
        },
    ]
}

#[test]
fn enumerate_fat32_and_exfat_demo_corpora() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../validation/synthetic");
    let fat32_path = root.join("fat32/fat32-synth-demo.trfsfa");
    let exfat_path = root.join("exfat/exfat-synth-demo.trfsex");

    write_fat32_synthetic_corpus(&fat32_path, &sample()).expect("write fat32");
    write_exfat_synthetic_corpus(&exfat_path, &sample()).expect("write exfat");

    let fat32 = enumerate_fat32_synthetic(&fat32_path).expect("enum fat32");
    let exfat = enumerate_exfat_synthetic(&exfat_path).expect("enum exfat");
    assert_eq!(fat32.len(), 4);
    assert_eq!(exfat.len(), 4);

    let note = fat32.iter().find(|e| e.name == "note.txt").expect("note");
    assert_eq!(note.path, "/docs/note.txt");
    assert_eq!(note.size, 42);

    let old = exfat.iter().find(|e| e.name == "old.bak").expect("old");
    assert!(old.deleted);
    assert_eq!(old.path, "/old.bak");
}

#[test]
fn fat_magic_mismatch_fails_closed() {
    let dir = tempfile::tempdir().expect("dir");
    let path = dir.path().join("x.trfsfa");
    write_exfat_synthetic_corpus(&path, &sample()).expect("write exfat bytes");
    let err = enumerate_fat32_synthetic(&path).expect_err("wrong magic");
    assert!(format!("{err:?}").contains("magic"));
}
