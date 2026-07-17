//! Synthetic FAT32 / exFAT corpus readers (Day 17).

use std::fs;
use std::io::{Read, Write};
use std::path::Path;

use lab_core::{LabError, LabResult};

use crate::ntfs_synth::{FsEntryKind, SynthFsEntry};

/// Magic for `fat32-synth-v1`.
pub const FAT32_SYNTH_MAGIC: &[u8; 8] = b"TRFSFA32";
/// Magic for `exfat-synth-v1`.
pub const EXFAT_SYNTH_MAGIC: &[u8; 8] = b"TRFSEXF1";

/// Input row for FAT-family synthetic corpora.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FatSynthEntry {
    pub cluster_id: u32,
    pub parent_cluster: u32,
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

fn write_corpus(path: &Path, magic: &[u8; 8], entries: &[FatSynthEntry]) -> LabResult<()> {
    let mut out = Vec::new();
    out.extend_from_slice(magic);
    out.extend_from_slice(&1_u32.to_le_bytes());
    out.extend_from_slice(&(entries.len() as u32).to_le_bytes());
    for e in entries {
        out.extend_from_slice(&e.cluster_id.to_le_bytes());
        out.extend_from_slice(&e.parent_cluster.to_le_bytes());
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
        let name_bytes = e.name.as_bytes();
        if name_bytes.len() > u16::MAX as usize {
            return Err(fail("fat-synth name too long"));
        }
        out.extend_from_slice(&(name_bytes.len() as u16).to_le_bytes());
        out.extend_from_slice(name_bytes);
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| fail(format!("create corpus dir: {e}")))?;
    }
    fs::File::create(path)
        .and_then(|mut f| f.write_all(&out))
        .map_err(|e| fail(format!("write fat corpus: {e}")))?;
    Ok(())
}

fn read_u16(buf: &[u8], off: &mut usize) -> LabResult<u16> {
    if *off + 2 > buf.len() {
        return Err(fail("fat-synth truncated u16"));
    }
    let v = u16::from_le_bytes([buf[*off], buf[*off + 1]]);
    *off += 2;
    Ok(v)
}

fn read_u32(buf: &[u8], off: &mut usize) -> LabResult<u32> {
    if *off + 4 > buf.len() {
        return Err(fail("fat-synth truncated u32"));
    }
    let v = u32::from_le_bytes(buf[*off..*off + 4].try_into().unwrap());
    *off += 4;
    Ok(v)
}

fn read_u64(buf: &[u8], off: &mut usize) -> LabResult<u64> {
    if *off + 8 > buf.len() {
        return Err(fail("fat-synth truncated u64"));
    }
    let v = u64::from_le_bytes(buf[*off..*off + 8].try_into().unwrap());
    *off += 8;
    Ok(v)
}

fn enumerate_corpus(
    path: &Path,
    expect_magic: &[u8; 8],
    label: &str,
) -> LabResult<Vec<SynthFsEntry>> {
    let mut buf = Vec::new();
    fs::File::open(path)
        .and_then(|mut f| f.read_to_end(&mut buf))
        .map_err(|e| fail(format!("read {label} corpus: {e}")))?;
    if buf.len() < 8 {
        return Err(fail(format!("{label} corpus too small")));
    }
    if &buf[0..8] != expect_magic {
        return Err(fail(format!("{label} bad magic")));
    }
    if buf.len() < 16 {
        return Err(fail(format!("{label} corpus too small")));
    }
    let mut off = 8;
    let version = read_u32(&buf, &mut off)?;
    if version != 1 {
        return Err(fail(format!("{label} unsupported version {version}")));
    }
    let count = read_u32(&buf, &mut off)? as usize;
    let mut raw = Vec::with_capacity(count);
    for _ in 0..count {
        let cluster_id = read_u32(&buf, &mut off)?;
        let parent_cluster = read_u32(&buf, &mut off)?;
        let flags = read_u32(&buf, &mut off)?;
        let size = read_u64(&buf, &mut off)?;
        let name_len = read_u16(&buf, &mut off)? as usize;
        if off + name_len > buf.len() {
            return Err(fail(format!("{label} truncated name")));
        }
        let name = std::str::from_utf8(&buf[off..off + name_len])
            .map_err(|e| fail(format!("{label} name utf8: {e}")))?
            .to_string();
        off += name_len;
        let kind = if flags & 0x1 != 0 {
            FsEntryKind::Directory
        } else if flags & 0x2 != 0 {
            FsEntryKind::File
        } else {
            return Err(fail(format!("{label} cluster {cluster_id} missing kind")));
        };
        raw.push(FatSynthEntry {
            cluster_id,
            parent_cluster,
            kind,
            deleted: flags & 0x4 != 0,
            size,
            name,
        });
    }
    if off != buf.len() {
        return Err(fail(format!("{label} trailing bytes")));
    }

    let by_id: std::collections::HashMap<u32, &FatSynthEntry> =
        raw.iter().map(|e| (e.cluster_id, e)).collect();
    if by_id.len() != raw.len() {
        return Err(fail(format!("{label} duplicate cluster_id")));
    }

    // FAT root is cluster 0 in this synthetic model.
    const ROOT: u32 = 0;
    let mut out = Vec::with_capacity(raw.len());
    for e in &raw {
        let path = if e.cluster_id == ROOT {
            "/".to_string()
        } else {
            let mut parts = Vec::new();
            let mut cur = e;
            let mut guard = 0_u32;
            loop {
                if cur.cluster_id == ROOT {
                    break;
                }
                parts.push(cur.name.as_str());
                if cur.parent_cluster == ROOT {
                    break;
                }
                cur = by_id.get(&cur.parent_cluster).copied().ok_or_else(|| {
                    fail(format!(
                        "{label} missing parent {} for {}",
                        cur.parent_cluster, cur.cluster_id
                    ))
                })?;
                guard += 1;
                if guard > 10_000 {
                    return Err(fail(format!("{label} parent cycle")));
                }
            }
            parts.reverse();
            format!("/{}", parts.join("/"))
        };
        out.push(SynthFsEntry {
            record_number: e.cluster_id as u64,
            parent_record: e.parent_cluster as u64,
            kind: e.kind,
            deleted: e.deleted,
            size: e.size,
            name: e.name.clone(),
            path,
        });
    }
    out.sort_by(|a, b| {
        a.path
            .cmp(&b.path)
            .then(a.record_number.cmp(&b.record_number))
    });
    Ok(out)
}

pub fn write_fat32_synthetic_corpus(path: &Path, entries: &[FatSynthEntry]) -> LabResult<()> {
    write_corpus(path, FAT32_SYNTH_MAGIC, entries)
}

pub fn write_exfat_synthetic_corpus(path: &Path, entries: &[FatSynthEntry]) -> LabResult<()> {
    write_corpus(path, EXFAT_SYNTH_MAGIC, entries)
}

pub fn enumerate_fat32_synthetic(path: &Path) -> LabResult<Vec<SynthFsEntry>> {
    enumerate_corpus(path, FAT32_SYNTH_MAGIC, "fat32-synth")
}

pub fn enumerate_exfat_synthetic(path: &Path) -> LabResult<Vec<SynthFsEntry>> {
    enumerate_corpus(path, EXFAT_SYNTH_MAGIC, "exfat-synth")
}
