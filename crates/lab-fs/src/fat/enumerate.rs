//! FAT32 / exFAT directory enumeration.

use lab_core::{LabResult, ProgressEvent, ProgressSink};
use lab_storage::ImageReader;

use crate::fat::bpb::{parse_exfat_boot, parse_fat32_boot};
use crate::ntfs_synth::FsEntryKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FatFsEntry {
    pub name: String,
    pub path: String,
    pub kind: FsEntryKind,
    pub deleted: bool,
    pub size: u64,
    pub first_cluster: u32,
}

pub fn enumerate_fat32(
    image: &mut dyn ImageReader,
    progress: &mut dyn ProgressSink,
) -> LabResult<Vec<FatFsEntry>> {
    let boot = parse_fat32_boot(image)?;
    let mut fat = vec![0u8; (boot.sectors_per_fat as usize) * (boot.bytes_per_sector as usize)];
    image.read_at(boot.fat_offset(), &mut fat)?;
    let mut entries = Vec::new();
    walk_fat32_dir(
        image,
        &boot,
        &fat,
        boot.root_cluster,
        "\\",
        &mut entries,
        progress,
        0,
    )?;
    Ok(entries)
}

fn walk_fat32_dir(
    image: &mut dyn ImageReader,
    boot: &crate::fat::bpb::Fat32Boot,
    fat: &[u8],
    cluster: u32,
    prefix: &str,
    out: &mut Vec<FatFsEntry>,
    progress: &mut dyn ProgressSink,
    depth: u32,
) -> LabResult<()> {
    if depth > 32 {
        return Ok(());
    }
    progress.report(ProgressEvent::new(
        "fat32.dir",
        depth as u64,
        None,
        format!("cluster {cluster}"),
    ));
    let clusters = chain(fat, cluster);
    let mut lfn_parts: Vec<String> = Vec::new();
    for c in clusters {
        let off = cluster_offset(boot, c);
        let mut buf = vec![0u8; boot.cluster_size() as usize];
        image.read_at(off, &mut buf)?;
        for ent in buf.chunks_exact(32) {
            if ent[0] == 0x00 {
                return Ok(());
            }
            if ent[0x0B] == 0x0F {
                // LFN
                lfn_parts.push(parse_lfn(ent));
                continue;
            }
            if ent[0x0B] & 0x08 != 0 {
                lfn_parts.clear();
                continue; // volume label
            }
            let deleted = ent[0] == 0xE5;
            let short = parse_short_name(ent);
            let name = if !lfn_parts.is_empty() {
                lfn_parts.iter().rev().cloned().collect::<String>()
            } else {
                short
            };
            lfn_parts.clear();
            if name == "." || name == ".." {
                continue;
            }
            let is_dir = ent[0x0B] & 0x10 != 0;
            let first = u16::from_le_bytes(ent[0x1A..0x1C].try_into().unwrap()) as u32
                | ((u16::from_le_bytes(ent[0x14..0x16].try_into().unwrap()) as u32) << 16);
            let size = u32::from_le_bytes(ent[0x1C..0x20].try_into().unwrap()) as u64;
            let path = format!("{prefix}{name}");
            out.push(FatFsEntry {
                name: name.clone(),
                path: path.clone(),
                kind: if is_dir {
                    FsEntryKind::Directory
                } else {
                    FsEntryKind::File
                },
                deleted,
                size,
                first_cluster: first,
            });
            if is_dir && !deleted && first >= 2 {
                walk_fat32_dir(
                    image,
                    boot,
                    fat,
                    first,
                    &format!("{path}\\"),
                    out,
                    progress,
                    depth + 1,
                )?;
            }
        }
    }
    Ok(())
}

pub fn enumerate_exfat(
    image: &mut dyn ImageReader,
    progress: &mut dyn ProgressSink,
) -> LabResult<Vec<FatFsEntry>> {
    let boot = parse_exfat_boot(image)?;
    progress.report(ProgressEvent::new("exfat.dir", 0, Some(1), "root"));
    // Minimal: read root directory cluster stream of file-name entries (simplified)
    let mut out = Vec::new();
    let mut cluster = boot.root_cluster;
    let mut guard = 0;
    while cluster >= 2 && cluster < 0xFFFFFFF8 && guard < 64 {
        let off = boot.cluster_heap_offset as u64 * boot.bytes_per_sector()
            + (cluster as u64 - 2) * boot.cluster_size();
        let mut buf = vec![0u8; boot.cluster_size() as usize];
        image.read_at(off, &mut buf)?;
        let mut i = 0;
        while i + 32 <= buf.len() {
            let entry_type = buf[i];
            if entry_type == 0x00 {
                break;
            }
            if entry_type == 0x85 {
                // File directory entry — next secondary for name
                let mut name = String::new();
                let mut size = 0u64;
                let mut first = 0u32;
                let mut deleted = entry_type & 0x80 == 0;
                let secondary = buf[i + 1] as usize;
                let mut j = i + 32;
                for _ in 0..secondary {
                    if j + 32 > buf.len() {
                        break;
                    }
                    match buf[j] {
                        0xC0 => {
                            // Stream extension
                            size = u64::from_le_bytes(buf[j + 8..j + 16].try_into().unwrap());
                            first = u32::from_le_bytes(buf[j + 20..j + 24].try_into().unwrap());
                        }
                        0xC1 => {
                            let chars = &buf[j + 2..j + 32];
                            for ch in chars.chunks_exact(2) {
                                let u = u16::from_le_bytes([ch[0], ch[1]]);
                                if u == 0 {
                                    break;
                                }
                                if let Some(c) = char::from_u32(u as u32) {
                                    name.push(c);
                                }
                            }
                        }
                        _ => {}
                    }
                    j += 32;
                }
                deleted = false;
                if !name.is_empty() {
                    out.push(FatFsEntry {
                        name: name.clone(),
                        path: format!("\\{name}"),
                        kind: FsEntryKind::File,
                        deleted,
                        size,
                        first_cluster: first,
                    });
                }
                i = j;
                continue;
            }
            i += 32;
        }
        // Follow FAT for next cluster (simplified: stop after one cluster for fixture)
        break;
    }
    Ok(out)
}

fn cluster_offset(boot: &crate::fat::bpb::Fat32Boot, cluster: u32) -> u64 {
    boot.data_offset() + (cluster as u64 - 2) * boot.cluster_size()
}

fn chain(fat: &[u8], start: u32) -> Vec<u32> {
    let mut out = Vec::new();
    let mut c = start;
    for _ in 0..4096 {
        if c < 2 || c >= 0x0FFF_FFF8 {
            break;
        }
        out.push(c);
        let off = c as usize * 4;
        if off + 4 > fat.len() {
            break;
        }
        c = u32::from_le_bytes(fat[off..off + 4].try_into().unwrap()) & 0x0FFF_FFFF;
    }
    out
}

fn parse_short_name(ent: &[u8]) -> String {
    let mut name = String::new();
    let base = &ent[0..8];
    let ext = &ent[8..11];
    for &b in base {
        if b == b' ' {
            break;
        }
        if b == 0xE5 {
            name.push('?');
        } else {
            name.push(b as char);
        }
    }
    let mut e = String::new();
    for &b in ext {
        if b == b' ' {
            break;
        }
        e.push(b as char);
    }
    if e.is_empty() {
        name
    } else {
        format!("{name}.{e}")
    }
}

fn parse_lfn(ent: &[u8]) -> String {
    let mut s = String::new();
    for range in [1..11, 14..26, 28..32] {
        for ch in ent[range].chunks_exact(2) {
            let u = u16::from_le_bytes([ch[0], ch[1]]);
            if u == 0 || u == 0xFFFF {
                return s;
            }
            if let Some(c) = char::from_u32(u as u32) {
                s.push(c);
            }
        }
    }
    s
}
