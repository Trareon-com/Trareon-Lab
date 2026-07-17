//! Synthetic filesystem file content store (Day 18).
//!
//! Companion to enumeration corpora: keyed by record/cluster id so callers can
//! read file bytes and ingest them into `lab-store` CAS without claiming a full
//! on-disk filesystem volume reader.

use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

use lab_core::{LabError, LabResult};
use lab_store::CasStore;

/// Magic for `fs-content-synth-v1`.
pub const FS_CONTENT_SYNTH_MAGIC: &[u8; 8] = b"TRFSCT01";

fn fail(detail: impl Into<String>) -> LabError {
    LabError::Internal {
        detail: detail.into(),
    }
}

/// One content payload keyed by synthetic record/cluster id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynthFileContent {
    pub record_number: u64,
    pub bytes: Vec<u8>,
}

/// Write a deterministic synthetic content store.
pub fn write_fs_content_synthetic(path: &Path, files: &[SynthFileContent]) -> LabResult<()> {
    let mut out = Vec::new();
    out.extend_from_slice(FS_CONTENT_SYNTH_MAGIC);
    out.extend_from_slice(&1_u32.to_le_bytes());
    out.extend_from_slice(&(files.len() as u32).to_le_bytes());
    for f in files {
        out.extend_from_slice(&f.record_number.to_le_bytes());
        let len = f.bytes.len();
        if len > u32::MAX as usize {
            return Err(fail("fs-content payload too large"));
        }
        out.extend_from_slice(&(len as u32).to_le_bytes());
        out.extend_from_slice(&f.bytes);
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| fail(format!("create content dir: {e}")))?;
    }
    fs::File::create(path)
        .and_then(|mut file| file.write_all(&out))
        .map_err(|e| fail(format!("write content store: {e}")))?;
    Ok(())
}

fn read_u32(buf: &[u8], off: &mut usize) -> LabResult<u32> {
    if *off + 4 > buf.len() {
        return Err(fail("fs-content truncated u32"));
    }
    let v = u32::from_le_bytes(buf[*off..*off + 4].try_into().unwrap());
    *off += 4;
    Ok(v)
}

fn read_u64(buf: &[u8], off: &mut usize) -> LabResult<u64> {
    if *off + 8 > buf.len() {
        return Err(fail("fs-content truncated u64"));
    }
    let v = u64::from_le_bytes(buf[*off..*off + 8].try_into().unwrap());
    *off += 8;
    Ok(v)
}

/// Load all payloads from a synthetic content store.
pub fn load_fs_content_synthetic(path: &Path) -> LabResult<HashMap<u64, Vec<u8>>> {
    let mut buf = Vec::new();
    fs::File::open(path)
        .and_then(|mut f| f.read_to_end(&mut buf))
        .map_err(|e| fail(format!("read content store: {e}")))?;
    if buf.len() < 8 {
        return Err(fail("fs-content too small"));
    }
    if &buf[0..8] != FS_CONTENT_SYNTH_MAGIC {
        return Err(fail("fs-content bad magic"));
    }
    if buf.len() < 16 {
        return Err(fail("fs-content too small"));
    }
    let mut off = 8;
    let version = read_u32(&buf, &mut off)?;
    if version != 1 {
        return Err(fail(format!("fs-content unsupported version {version}")));
    }
    let count = read_u32(&buf, &mut off)? as usize;
    let mut map = HashMap::with_capacity(count);
    for _ in 0..count {
        let record_number = read_u64(&buf, &mut off)?;
        let len = read_u32(&buf, &mut off)? as usize;
        if off + len > buf.len() {
            return Err(fail(format!(
                "fs-content truncated payload for record {record_number}"
            )));
        }
        let bytes = buf[off..off + len].to_vec();
        off += len;
        if map.insert(record_number, bytes).is_some() {
            return Err(fail(format!(
                "fs-content duplicate record_number {record_number}"
            )));
        }
    }
    if off != buf.len() {
        return Err(fail("fs-content trailing bytes"));
    }
    Ok(map)
}

/// Read one file's bytes from a synthetic content store (fail closed if missing).
pub fn read_synth_file_bytes(path: &Path, record_number: u64) -> LabResult<Vec<u8>> {
    let map = load_fs_content_synthetic(path)?;
    map.get(&record_number)
        .cloned()
        .ok_or_else(|| fail(format!("fs-content missing record {record_number}")))
}

/// Result of ingesting a synthetic file into CAS.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CasIngestResult {
    pub record_number: u64,
    pub digest_hex: String,
    pub byte_length: u64,
}

/// Read synthetic file bytes and store them in the case CAS; returned digest must
/// round-trip to identical bytes via `CasStore::get`.
pub fn ingest_synth_file_to_cas(
    content_store: &Path,
    record_number: u64,
    cas: &CasStore,
) -> LabResult<CasIngestResult> {
    let bytes = read_synth_file_bytes(content_store, record_number)?;
    let digest_hex = cas.put(&bytes)?;
    let roundtrip = cas.get(&digest_hex)?;
    if roundtrip.as_slice() != bytes.as_slice() {
        return Err(LabError::IntegrityFailed {
            detail: format!(
                "CAS roundtrip mismatch for record {record_number} digest {digest_hex}"
            ),
        });
    }
    Ok(CasIngestResult {
        record_number,
        digest_hex,
        byte_length: bytes.len() as u64,
    })
}
