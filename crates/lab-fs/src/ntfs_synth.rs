//! Synthetic NTFS corpus reader (Day 16).
//!
//! This is intentionally a controlled synthetic corpus format that models NTFS
//! enumeration semantics (MFT-like record numbers, parent links, directory/file
//! flags, deleted markers). It is **not** a claim of full on-disk NTFS boot/MFT
//! parsing; real volume parsing remains a follow-on once a redistributable
//! raw NTFS image corpus is governed under `docs/CORPUS-GOVERNANCE.md`.

use std::fs;
use std::io::{Read, Write};
use std::path::Path;

use lab_core::{LabError, LabResult};

/// Magic for `ntfs-synth-v1` corpus files.
pub const NTFS_SYNTH_MAGIC: &[u8; 8] = b"TRFSNT01";

/// Kind of filesystem entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsEntryKind {
    Directory,
    File,
}

/// One enumerated entry from a synthetic NTFS corpus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynthFsEntry {
    pub record_number: u64,
    pub parent_record: u64,
    pub kind: FsEntryKind,
    pub deleted: bool,
    pub size: u64,
    pub name: String,
    /// NTFS-style path using `\` separators from the synthetic root.
    pub path: String,
}

/// Input row used when writing a synthetic corpus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NtfsSynthEntry {
    pub record_number: u64,
    pub parent_record: u64,
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

fn read_u16(buf: &[u8], off: &mut usize) -> LabResult<u16> {
    if *off + 2 > buf.len() {
        return Err(fail("ntfs-synth truncated reading u16"));
    }
    let v = u16::from_le_bytes([buf[*off], buf[*off + 1]]);
    *off += 2;
    Ok(v)
}

fn read_u32(buf: &[u8], off: &mut usize) -> LabResult<u32> {
    if *off + 4 > buf.len() {
        return Err(fail("ntfs-synth truncated reading u32"));
    }
    let v = u32::from_le_bytes(buf[*off..*off + 4].try_into().unwrap());
    *off += 4;
    Ok(v)
}

fn read_u64(buf: &[u8], off: &mut usize) -> LabResult<u64> {
    if *off + 8 > buf.len() {
        return Err(fail("ntfs-synth truncated reading u64"));
    }
    let v = u64::from_le_bytes(buf[*off..*off + 8].try_into().unwrap());
    *off += 8;
    Ok(v)
}

/// Write a deterministic synthetic NTFS corpus to `path`.
pub fn write_ntfs_synthetic_corpus(path: &Path, entries: &[NtfsSynthEntry]) -> LabResult<()> {
    let mut out = Vec::new();
    out.extend_from_slice(NTFS_SYNTH_MAGIC);
    out.extend_from_slice(&1_u32.to_le_bytes()); // version
    out.extend_from_slice(&(entries.len() as u32).to_le_bytes());
    for e in entries {
        out.extend_from_slice(&e.record_number.to_le_bytes());
        out.extend_from_slice(&e.parent_record.to_le_bytes());
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
            return Err(fail("ntfs-synth name too long"));
        }
        out.extend_from_slice(&(name_bytes.len() as u16).to_le_bytes());
        out.extend_from_slice(name_bytes);
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| fail(format!("create corpus dir: {e}")))?;
    }
    let mut file = fs::File::create(path).map_err(|e| fail(format!("create corpus: {e}")))?;
    file.write_all(&out)
        .map_err(|e| fail(format!("write corpus: {e}")))?;
    Ok(())
}

/// Enumerate entries from a synthetic NTFS corpus file.
///
/// Builds NTFS-style paths (`\Windows\System32\...`) from parent links.
/// Fail-closed on bad magic, truncation, unknown version, or cycles.
pub fn enumerate_ntfs_synthetic(path: &Path) -> LabResult<Vec<SynthFsEntry>> {
    let mut file = fs::File::open(path).map_err(|e| fail(format!("open corpus: {e}")))?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)
        .map_err(|e| fail(format!("read corpus: {e}")))?;
    if buf.len() < 8 {
        return Err(fail("ntfs-synth corpus too small"));
    }
    if &buf[0..8] != NTFS_SYNTH_MAGIC {
        return Err(fail("ntfs-synth bad magic"));
    }
    if buf.len() < 16 {
        return Err(fail("ntfs-synth corpus too small"));
    }
    let mut off = 8;
    let version = read_u32(&buf, &mut off)?;
    if version != 1 {
        return Err(fail(format!("ntfs-synth unsupported version {version}")));
    }
    let count = read_u32(&buf, &mut off)? as usize;
    let mut raw = Vec::with_capacity(count);
    for _ in 0..count {
        let record_number = read_u64(&buf, &mut off)?;
        let parent_record = read_u64(&buf, &mut off)?;
        let flags = read_u32(&buf, &mut off)?;
        let size = read_u64(&buf, &mut off)?;
        let name_len = read_u16(&buf, &mut off)? as usize;
        if off + name_len > buf.len() {
            return Err(fail("ntfs-synth truncated name"));
        }
        let name = std::str::from_utf8(&buf[off..off + name_len])
            .map_err(|e| fail(format!("ntfs-synth name utf8: {e}")))?
            .to_string();
        off += name_len;
        let kind = if flags & 0x1 != 0 {
            FsEntryKind::Directory
        } else if flags & 0x2 != 0 {
            FsEntryKind::File
        } else {
            return Err(fail(format!(
                "ntfs-synth record {record_number} missing kind flags"
            )));
        };
        raw.push(NtfsSynthEntry {
            record_number,
            parent_record,
            kind,
            deleted: flags & 0x4 != 0,
            size,
            name,
        });
    }
    if off != buf.len() {
        return Err(fail("ntfs-synth trailing bytes"));
    }

    let by_id: std::collections::HashMap<u64, &NtfsSynthEntry> =
        raw.iter().map(|e| (e.record_number, e)).collect();
    if by_id.len() != raw.len() {
        return Err(fail("ntfs-synth duplicate record_number"));
    }

    let mut out = Vec::with_capacity(raw.len());
    for e in &raw {
        let path = build_path(e, &by_id)?;
        out.push(SynthFsEntry {
            record_number: e.record_number,
            parent_record: e.parent_record,
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

fn build_path(
    entry: &NtfsSynthEntry,
    by_id: &std::collections::HashMap<u64, &NtfsSynthEntry>,
) -> LabResult<String> {
    // NTFS root directory is conventionally MFT record 5.
    const ROOT: u64 = 5;
    if entry.record_number == ROOT {
        return Ok(String::from("\\"));
    }
    let mut parts = Vec::new();
    let mut cur = entry;
    let mut guard = 0_u32;
    loop {
        if cur.record_number == ROOT {
            break;
        }
        parts.push(cur.name.as_str());
        if cur.parent_record == ROOT {
            break;
        }
        cur = by_id.get(&cur.parent_record).copied().ok_or_else(|| {
            fail(format!(
                "ntfs-synth missing parent {} for {}",
                cur.parent_record, cur.record_number
            ))
        })?;
        guard += 1;
        if guard > 10_000 {
            return Err(fail("ntfs-synth parent cycle detected"));
        }
    }
    parts.reverse();
    Ok(format!("\\{}", parts.join("\\")))
}
