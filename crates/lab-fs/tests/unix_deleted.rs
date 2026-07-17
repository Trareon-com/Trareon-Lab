//! Days 21–23: ext4/APFS enum + deleted recovery.

use lab_fs::{
    enumerate_apfs_synthetic, enumerate_ext4_synthetic, recover_deleted_partial,
    write_apfs_synthetic_corpus, write_ext4_synthetic_corpus, FsEntryKind, UnixFsSynthEntry,
};
use tempfile::tempdir;

fn sample() -> Vec<UnixFsSynthEntry> {
    vec![
        UnixFsSynthEntry {
            inode: 2,
            parent_inode: 2,
            kind: FsEntryKind::Directory,
            deleted: false,
            size: 0,
            name: ".".into(),
        },
        UnixFsSynthEntry {
            inode: 10,
            parent_inode: 2,
            kind: FsEntryKind::Directory,
            deleted: false,
            size: 0,
            name: "etc".into(),
        },
        UnixFsSynthEntry {
            inode: 11,
            parent_inode: 10,
            kind: FsEntryKind::File,
            deleted: false,
            size: 100,
            name: "hosts".into(),
        },
        UnixFsSynthEntry {
            inode: 12,
            parent_inode: 2,
            kind: FsEntryKind::File,
            deleted: true,
            size: 20,
            name: "secret.bak".into(),
        },
    ]
}

#[test]
fn ext4_and_apfs_enumerate() {
    let dir = tempdir().unwrap();
    let ext4 = dir.path().join("e.trfsxt");
    let apfs = dir.path().join("a.trfsap");
    write_ext4_synthetic_corpus(&ext4, &sample()).unwrap();
    write_apfs_synthetic_corpus(&apfs, &sample()).unwrap();
    let e = enumerate_ext4_synthetic(&ext4).unwrap();
    let a = enumerate_apfs_synthetic(&apfs).unwrap();
    assert_eq!(e.len(), 4);
    assert_eq!(
        a.iter().find(|x| x.name == "hosts").unwrap().path,
        "/etc/hosts"
    );
}

#[test]
fn deleted_recovery_partial_ambiguous() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("e.trfsxt");
    write_ext4_synthetic_corpus(&path, &sample()).unwrap();
    let entries = enumerate_ext4_synthetic(&path).unwrap();
    let hits = recover_deleted_partial(&entries);
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].name, "secret.bak");
    assert_eq!(hits[0].confidence, "partial");
    assert_eq!(hits[0].label, "ambiguous_deleted_name_retained");
}
