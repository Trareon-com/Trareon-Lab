//! NTFS file enumeration over live volumes.

use std::collections::HashMap;

use lab_core::{LabResult, ProgressSink};
use lab_storage::ImageReader;

use crate::ntfs::attribute::{Attribute, FileNameAttr};
use crate::ntfs::boot::NtfsVolume;
use crate::ntfs::mft::MftIterator;
use crate::ntfs_synth::FsEntryKind;

/// One enumerated NTFS entry (compatible shape with index pipeline).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsEntry {
    pub record_number: u64,
    pub parent_record: u64,
    pub kind: FsEntryKind,
    pub deleted: bool,
    pub size: u64,
    pub name: String,
    pub path: String,
    pub sequence: u16,
    pub si_created: Option<u64>,
    pub si_modified: Option<u64>,
    pub fn_created: Option<u64>,
    pub fn_modified: Option<u64>,
}

#[derive(Debug, Clone, Default)]
pub struct NtfsEnumerateOptions {
    pub include_system: bool,
}

/// Enumerate files from $MFT (FILE_NAME Win32 preferred; includes deleted).
pub fn enumerate_ntfs(
    image: &mut dyn ImageReader,
    progress: &mut dyn ProgressSink,
    _opts: NtfsEnumerateOptions,
) -> LabResult<Vec<FsEntry>> {
    let volume = NtfsVolume::open(image)?;
    let mut mft = MftIterator::open(image, volume)?;
    let mut pending: Vec<(u64, u16, bool, bool, u64, FileNameAttr, Option<(u64, u64)>)> =
        Vec::new();

    mft.for_each_record(progress, |rec| {
        if rec.record_number < 5 && rec.record_number != 5 {
            // keep root (5) and user files; skip most meta except we still parse all for paths
        }
        let mut si = None;
        let mut names: Vec<FileNameAttr> = Vec::new();
        let mut data_size = 0u64;
        for a in &rec.attributes {
            match a {
                Attribute::StandardInformation(s) => {
                    si = Some((s.timestamps.created, s.timestamps.modified));
                }
                Attribute::FileName(fnattr) => names.push(fnattr.clone()),
                Attribute::Data(d) if d.name.is_empty() => data_size = d.data_size,
                _ => {}
            }
        }
        // Prefer Win32 namespace (1); fall back to any
        let chosen = names
            .iter()
            .find(|n| n.name_space == 1 || n.name_space == 3)
            .or_else(|| names.first());
        if let Some(fnattr) = chosen {
            pending.push((
                rec.record_number,
                rec.sequence,
                rec.flags.in_use,
                rec.flags.is_directory,
                data_size.max(fnattr.real_size),
                fnattr.clone(),
                si,
            ));
        }
    })?;

    let by_id: HashMap<u64, &str> = pending
        .iter()
        .map(|(id, _, _, _, _, fnattr, _)| (*id, fnattr.name.as_str()))
        .collect();

    let mut entries = Vec::new();
    for (id, seq, in_use, is_dir, size, fnattr, si) in &pending {
        if *id < 5 {
            continue; // skip $MFT..$AttrDef style; keep root separately
        }
        let path = build_path(*id, fnattr.parent_record, &pending);
        entries.push(FsEntry {
            record_number: *id,
            parent_record: fnattr.parent_record,
            kind: if *is_dir {
                FsEntryKind::Directory
            } else {
                FsEntryKind::File
            },
            deleted: !*in_use,
            size: *size,
            name: fnattr.name.clone(),
            path,
            sequence: *seq,
            si_created: si.map(|s| s.0),
            si_modified: si.map(|s| s.1),
            fn_created: Some(fnattr.timestamps.created),
            fn_modified: Some(fnattr.timestamps.modified),
        });
        let _ = &by_id;
    }
    Ok(entries)
}

fn build_path(
    id: u64,
    parent: u64,
    pending: &[(u64, u16, bool, bool, u64, FileNameAttr, Option<(u64, u64)>)],
) -> String {
    let mut parts = Vec::new();
    let mut cur_parent = parent;
    let mut cur_id = id;
    let name_of = |rid: u64| {
        pending
            .iter()
            .find(|(i, _, _, _, _, _, _)| *i == rid)
            .map(|(_, _, _, _, _, f, _)| f.name.clone())
    };
    if let Some(n) = name_of(cur_id) {
        parts.push(n);
    }
    let mut guard = 0;
    while cur_parent >= 5 && guard < 64 {
        if let Some(n) = name_of(cur_parent) {
            parts.push(n);
        }
        let next = pending
            .iter()
            .find(|(i, _, _, _, _, _, _)| *i == cur_parent)
            .map(|(_, _, _, _, _, f, _)| f.parent_record);
        match next {
            Some(p) if p != cur_parent => {
                cur_id = cur_parent;
                cur_parent = p;
            }
            _ => break,
        }
        guard += 1;
    }
    parts.reverse();
    format!("\\{}", parts.join("\\"))
}
