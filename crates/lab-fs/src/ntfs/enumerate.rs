//! NTFS file enumeration over live volumes.

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

struct PendingEntry {
    record_number: u64,
    sequence: u16,
    in_use: bool,
    is_directory: bool,
    size: u64,
    filename: FileNameAttr,
    si: Option<(u64, u64)>,
}

/// Enumerate files from $MFT (FILE_NAME Win32 preferred; includes deleted).
pub fn enumerate_ntfs(
    image: &mut dyn ImageReader,
    progress: &mut dyn ProgressSink,
    _opts: NtfsEnumerateOptions,
) -> LabResult<Vec<FsEntry>> {
    let volume = NtfsVolume::open(image)?;
    let mut mft = MftIterator::open(image, volume)?;
    let mut pending: Vec<PendingEntry> = Vec::new();

    mft.for_each_record(progress, |rec| {
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
            pending.push(PendingEntry {
                record_number: rec.record_number,
                sequence: rec.sequence,
                in_use: rec.flags.in_use,
                is_directory: rec.flags.is_directory,
                size: data_size.max(fnattr.real_size),
                filename: fnattr.clone(),
                si,
            });
        }
    })?;

    let mut entries = Vec::new();
    for item in &pending {
        if item.record_number < 5 {
            continue; // skip $MFT..$AttrDef style
        }
        let path = build_path(item.record_number, item.filename.parent_record, &pending);
        entries.push(FsEntry {
            record_number: item.record_number,
            parent_record: item.filename.parent_record,
            kind: if item.is_directory {
                FsEntryKind::Directory
            } else {
                FsEntryKind::File
            },
            deleted: !item.in_use,
            size: item.size,
            name: item.filename.name.clone(),
            path,
            sequence: item.sequence,
            si_created: item.si.map(|s| s.0),
            si_modified: item.si.map(|s| s.1),
            fn_created: Some(item.filename.timestamps.created),
            fn_modified: Some(item.filename.timestamps.modified),
        });
    }
    Ok(entries)
}

fn build_path(id: u64, parent: u64, pending: &[PendingEntry]) -> String {
    let mut parts = Vec::new();
    let mut cur_parent = parent;
    let name_of = |rid: u64| {
        pending
            .iter()
            .find(|p| p.record_number == rid)
            .map(|p| p.filename.name.clone())
    };
    if let Some(n) = name_of(id) {
        parts.push(n);
    }
    let mut guard = 0;
    while cur_parent >= 5 && guard < 64 {
        if let Some(n) = name_of(cur_parent) {
            parts.push(n);
        }
        let next = pending
            .iter()
            .find(|p| p.record_number == cur_parent)
            .map(|p| p.filename.parent_record);
        match next {
            Some(p) if p != cur_parent => cur_parent = p,
            _ => break,
        }
        guard += 1;
    }
    parts.reverse();
    format!("\\{}", parts.join("\\"))
}
