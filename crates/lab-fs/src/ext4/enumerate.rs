//! EXT4 directory enumeration via inode table + dir entries.

use lab_core::{LabResult, ProgressEvent, ProgressSink};
use lab_storage::ImageReader;

use crate::ext4::superblock::parse_superblock;
use crate::ntfs_synth::FsEntryKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ext4FsEntry {
    pub inode: u32,
    pub name: String,
    pub path: String,
    pub kind: FsEntryKind,
    pub deleted: bool,
    pub size: u64,
}

pub fn enumerate_ext4(
    image: &mut dyn ImageReader,
    progress: &mut dyn ProgressSink,
) -> LabResult<Vec<Ext4FsEntry>> {
    let sb = parse_superblock(image)?;
    progress.report(ProgressEvent::new("ext4.super", 1, Some(1), "parsed"));

    // Group 0 descriptor at block first_data_block+1 (or block 1 if block_size>1024)
    let gdt_block = if sb.block_size == 1024 {
        sb.first_data_block + 1
    } else {
        1
    };
    let mut gdt = [0u8; 32];
    image.read_at(gdt_block as u64 * sb.block_size as u64, &mut gdt)?;
    let inode_table_block = u32::from_le_bytes(gdt[8..12].try_into().unwrap());

    // Root inode is #2
    let root = read_inode(image, &sb, inode_table_block, 2)?;
    let mut out = Vec::new();
    walk_dir(image, &sb, inode_table_block, &root, "/", &mut out, progress, 0)?;
    Ok(out)
}

struct Inode {
    mode: u16,
    size: u64,
    block: [u8; 60],
}

fn read_inode(
    image: &mut dyn ImageReader,
    sb: &crate::ext4::superblock::Ext4Superblock,
    inode_table_block: u32,
    ino: u32,
) -> LabResult<Inode> {
    let index = ino - 1;
    let off = inode_table_block as u64 * sb.block_size as u64
        + index as u64 * sb.inode_size as u64;
    let mut buf = vec![0u8; sb.inode_size as usize];
    image.read_at(off, &mut buf)?;
    let mode = u16::from_le_bytes(buf[0..2].try_into().unwrap());
    let size_lo = u32::from_le_bytes(buf[4..8].try_into().unwrap()) as u64;
    let size_hi = u32::from_le_bytes(buf[0x6C..0x70].try_into().unwrap()) as u64;
    let mut block = [0u8; 60];
    block.copy_from_slice(&buf[0x28..0x64]);
    Ok(Inode {
        mode,
        size: size_lo | (size_hi << 32),
        block,
    })
}

#[allow(clippy::too_many_arguments)]
fn walk_dir(
    image: &mut dyn ImageReader,
    sb: &crate::ext4::superblock::Ext4Superblock,
    inode_table_block: u32,
    inode: &Inode,
    prefix: &str,
    out: &mut Vec<Ext4FsEntry>,
    progress: &mut dyn ProgressSink,
    depth: u32,
) -> LabResult<()> {
    if depth > 16 {
        return Ok(());
    }
    progress.report(ProgressEvent::new("ext4.dir", depth as u64, None, prefix));
    let blocks = dir_blocks(inode, sb);
    for b in blocks {
        let mut buf = vec![0u8; sb.block_size as usize];
        image.read_at(b as u64 * sb.block_size as u64, &mut buf)?;
        let mut off = 0usize;
        while off + 8 <= buf.len() {
            let inode_nr = u32::from_le_bytes(buf[off..off + 4].try_into().unwrap());
            let rec_len = u16::from_le_bytes(buf[off + 4..off + 6].try_into().unwrap()) as usize;
            if rec_len < 8 {
                break;
            }
            let name_len = buf[off + 6] as usize;
            let file_type = buf[off + 7];
            if inode_nr != 0 && name_len > 0 && off + 8 + name_len <= buf.len() {
                let name = String::from_utf8_lossy(&buf[off + 8..off + 8 + name_len]).into_owned();
                if name != "." && name != ".." {
                    let kind = if file_type == 2 || (inode.mode & 0xF000 == 0x4000 && file_type == 0)
                    {
                        FsEntryKind::Directory
                    } else {
                        FsEntryKind::File
                    };
                    // Prefer file_type for dirs
                    let kind = if file_type == 2 {
                        FsEntryKind::Directory
                    } else if file_type == 1 || file_type == 0 {
                        FsEntryKind::File
                    } else {
                        kind
                    };
                    let path = if prefix.ends_with('/') {
                        format!("{prefix}{name}")
                    } else {
                        format!("{prefix}/{name}")
                    };
                    let child = read_inode(image, sb, inode_table_block, inode_nr).ok();
                    out.push(Ext4FsEntry {
                        inode: inode_nr,
                        name: name.clone(),
                        path: path.clone(),
                        kind,
                        deleted: false,
                        size: child.as_ref().map(|c| c.size).unwrap_or(0),
                    });
                    if file_type == 2 {
                        if let Some(c) = child {
                            walk_dir(
                                image,
                                sb,
                                inode_table_block,
                                &c,
                                &path,
                                out,
                                progress,
                                depth + 1,
                            )?;
                        }
                    }
                }
            }
            off += rec_len;
        }
    }
    Ok(())
}

fn dir_blocks(inode: &Inode, sb: &crate::ext4::superblock::Ext4Superblock) -> Vec<u32> {
    // Extent tree magic at i_block[0..2]
    let magic = u16::from_le_bytes(inode.block[0..2].try_into().unwrap());
    if magic == 0xF30A {
        let entries = inode.block[2];
        let mut out = Vec::new();
        for i in 0..entries as usize {
            let base = 12 + i * 12;
            if base + 12 > inode.block.len() {
                break;
            }
            let len = u16::from_le_bytes(inode.block[base + 4..base + 6].try_into().unwrap());
            let start = u32::from_le_bytes(inode.block[base + 8..base + 12].try_into().unwrap());
            for b in 0..len as u32 {
                out.push(start + b);
            }
        }
        if !out.is_empty() {
            return out;
        }
    }
    // Classic direct blocks
    let mut out = Vec::new();
    for i in 0..12 {
        let b = u32::from_le_bytes(inode.block[i * 4..i * 4 + 4].try_into().unwrap());
        if b != 0 {
            out.push(b);
        }
    }
    let _ = sb;
    out
}
