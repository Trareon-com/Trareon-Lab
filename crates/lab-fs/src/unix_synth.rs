//! Synthetic ext4 / APFS corpora (Days 21–22) and deleted recovery (Day 23).

use std::fs;
use std::io::{Read, Write};
use std::path::Path;

use lab_core::{LabError, LabResult};

use crate::ntfs_synth::{FsEntryKind, SynthFsEntry};

pub const EXT4_SYNTH_MAGIC: &[u8; 8] = b"TRFSEXT4";
pub const APFS_SYNTH_MAGIC: &[u8; 8] = b"TRFSAPFS";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnixFsSynthEntry {
    pub inode: u64,
    pub parent_inode: u64,
    pub kind: FsEntryKind,
    pub deleted: bool,
    pub size: u64,
    pub name: String,
}

fn fail(detail: impl Into<String>) -> LabError {
    LabError::Internal {
        detail: detail.into(),
    }
}

fn write_unix_corpus(path: &Path, magic: &[u8; 8], entries: &[UnixFsSynthEntry]) -> LabResult<()> {
    let mut out = Vec::new();
    out.extend_from_slice(magic);
    out.extend_from_slice(&1_u32.to_le_bytes());
    out.extend_from_slice(&(entries.len() as u32).to_le_bytes());
    for e in entries {
        out.extend_from_slice(&e.inode.to_le_bytes());
        out.extend_from_slice(&e.parent_inode.to_le_bytes());
        let mut flags = 0_u32;
        match e.kind {
            FsEntryKind::Directory => flags |= 0x1,
            FsEntryKind::File => flags |= 0x2,
        }
        if e.deleted {
            flags |= 0x4;
        }
        out.extend_from_slice(&flags.to_le_bytes());
        out.extend_from_slice(&e.size.to_le_bytes());
        let nb = e.name.as_bytes();
        out.extend_from_slice(&(nb.len() as u16).to_le_bytes());
        out.extend_from_slice(nb);
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| fail(format!("mkdir: {e}")))?;
    }
    fs::File::create(path)
        .and_then(|mut f| f.write_all(&out))
        .map_err(|e| fail(format!("write: {e}")))?;
    Ok(())
}

fn enumerate_unix(path: &Path, magic: &[u8; 8], label: &str) -> LabResult<Vec<SynthFsEntry>> {
    let mut buf = Vec::new();
    fs::File::open(path)
        .and_then(|mut f| f.read_to_end(&mut buf))
        .map_err(|e| fail(format!("read {label}: {e}")))?;
    if buf.len() < 8 || &buf[0..8] != magic {
        return Err(fail(format!("{label} bad magic")));
    }
    let mut off = 8usize;
    let _ver = u32::from_le_bytes(buf[off..off + 4].try_into().unwrap());
    off += 4;
    let count = u32::from_le_bytes(buf[off..off + 4].try_into().unwrap()) as usize;
    off += 4;
    let mut raw = Vec::new();
    for _ in 0..count {
        let inode = u64::from_le_bytes(buf[off..off + 8].try_into().unwrap());
        off += 8;
        let parent = u64::from_le_bytes(buf[off..off + 8].try_into().unwrap());
        off += 8;
        let flags = u32::from_le_bytes(buf[off..off + 4].try_into().unwrap());
        off += 4;
        let size = u64::from_le_bytes(buf[off..off + 8].try_into().unwrap());
        off += 8;
        let nlen = u16::from_le_bytes(buf[off..off + 2].try_into().unwrap()) as usize;
        off += 2;
        let name = String::from_utf8(buf[off..off + nlen].to_vec())
            .map_err(|e| fail(format!("{label} utf8: {e}")))?;
        off += nlen;
        let kind = if flags & 1 != 0 {
            FsEntryKind::Directory
        } else {
            FsEntryKind::File
        };
        raw.push(UnixFsSynthEntry {
            inode,
            parent_inode: parent,
            kind,
            deleted: flags & 4 != 0,
            size,
            name,
        });
    }
    let by: std::collections::HashMap<u64, &UnixFsSynthEntry> =
        raw.iter().map(|e| (e.inode, e)).collect();
    const ROOT: u64 = 2;
    let mut out = Vec::new();
    for e in &raw {
        let path = if e.inode == ROOT {
            "/".into()
        } else {
            let mut parts = Vec::new();
            let mut cur = e;
            let mut guard = 0;
            loop {
                if cur.inode == ROOT {
                    break;
                }
                parts.push(cur.name.as_str());
                if cur.parent_inode == ROOT {
                    break;
                }
                cur = by
                    .get(&cur.parent_inode)
                    .copied()
                    .ok_or_else(|| fail(format!("{label} missing parent")))?;
                guard += 1;
                if guard > 10_000 {
                    return Err(fail(format!("{label} cycle")));
                }
            }
            parts.reverse();
            format!("/{}", parts.join("/"))
        };
        out.push(SynthFsEntry {
            record_number: e.inode,
            parent_record: e.parent_inode,
            kind: e.kind,
            deleted: e.deleted,
            size: e.size,
            name: e.name.clone(),
            path,
        });
    }
    out.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(out)
}

pub fn write_ext4_synthetic_corpus(path: &Path, entries: &[UnixFsSynthEntry]) -> LabResult<()> {
    write_unix_corpus(path, EXT4_SYNTH_MAGIC, entries)
}

pub fn write_apfs_synthetic_corpus(path: &Path, entries: &[UnixFsSynthEntry]) -> LabResult<()> {
    write_unix_corpus(path, APFS_SYNTH_MAGIC, entries)
}

pub fn enumerate_ext4_synthetic(path: &Path) -> LabResult<Vec<SynthFsEntry>> {
    enumerate_unix(path, EXT4_SYNTH_MAGIC, "ext4-synth")
}

pub fn enumerate_apfs_synthetic(path: &Path) -> LabResult<Vec<SynthFsEntry>> {
    enumerate_unix(path, APFS_SYNTH_MAGIC, "apfs-synth")
}

/// Day 23: R1 deleted recovery — surface deleted-marked entries as partial/ambiguous.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeletedRecoveryHit {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub confidence: &'static str,
    pub label: &'static str,
}

pub fn recover_deleted_partial(entries: &[SynthFsEntry]) -> Vec<DeletedRecoveryHit> {
    entries
        .iter()
        .filter(|e| e.deleted && e.kind == FsEntryKind::File)
        .map(|e| DeletedRecoveryHit {
            path: e.path.clone(),
            name: e.name.clone(),
            size: e.size,
            confidence: "partial",
            label: "ambiguous_deleted_name_retained",
        })
        .collect()
}
