//! APFS volume and single-leaf catalog enumeration.

use std::collections::HashMap;

use lab_core::{LabResult, ProgressEvent, ProgressSink};
use lab_storage::ImageReader;

use crate::ntfs_synth::FsEntryKind;

use super::superblock::parse_apfs_container;
use super::{fail, read_object, verify_object, ApfsContainerSuperblock, APFS_VOLUME_MAGIC};

const OBJECT_TYPE_BTREE: u32 = 2;
const OBJECT_TYPE_OMAP: u32 = 11;
const OBJECT_TYPE_APFS: u32 = 13;
const APFS_TYPE_INODE: u64 = 3;
const APFS_TYPE_DIR_REC: u64 = 9;
const APFS_OBJ_ID_MASK: u64 = 0x0fff_ffff_ffff_ffff;
const APFS_ROOT_DIR_INO: u64 = 2;
const BTNODE_ROOT: u16 = 0x0001;
const BTNODE_LEAF: u16 = 0x0002;
const BTNODE_FIXED_KV_SIZE: u16 = 0x0004;
const BTREE_INFO_SIZE: usize = 40;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApfsFileEntry {
    pub object_id: u64,
    pub parent_id: u64,
    pub name: String,
    pub path: String,
    pub kind: FsEntryKind,
    pub size: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApfsVolume {
    pub object_id: u64,
    pub index: u32,
    pub uuid: [u8; 16],
    pub name: String,
    pub root_tree_oid: u64,
    pub entries: Vec<ApfsFileEntry>,
}

/// Enumerate APFS volumes and their minimal file catalog.
///
/// This corpus-validated subset resolves object maps but accepts only
/// single-node leaf object-map and catalog B-trees.
pub fn enumerate_apfs(
    image: &mut dyn ImageReader,
    progress: &mut dyn ProgressSink,
) -> LabResult<Vec<ApfsVolume>> {
    let container = parse_apfs_container(image)?;
    progress.report(ProgressEvent::new(
        "apfs.container",
        1,
        Some(1),
        "parsed NXSB",
    ));

    let mut volumes = Vec::with_capacity(container.volume_oids.len());
    for (position, &volume_oid) in container.volume_oids.iter().enumerate() {
        let volume_paddr = resolve_object(
            image,
            &container,
            container.object_map_oid,
            volume_oid,
            container.transaction_id,
            "container object map",
        )?;
        let block = read_object(
            image,
            container.block_size,
            container.block_count,
            volume_paddr,
            OBJECT_TYPE_APFS,
            "volume superblock",
        )?;
        let volume = parse_volume(image, &container, volume_oid, &block)?;
        progress.report(ProgressEvent::new(
            "apfs.volume",
            (position + 1) as u64,
            Some(container.volume_oids.len() as u64),
            volume.name.clone(),
        ));
        volumes.push(volume);
    }
    Ok(volumes)
}

fn parse_volume(
    image: &mut dyn ImageReader,
    container: &ApfsContainerSuperblock,
    volume_oid: u64,
    block: &[u8],
) -> LabResult<ApfsVolume> {
    if block.get(32..36) != Some(APFS_VOLUME_MAGIC.as_slice()) {
        return Err(fail("APFS volume bad APSB magic"));
    }
    let index = le_u32(block, 0x24, "volume index")?;
    let object_map_oid = le_u64(block, 0x80, "volume object map")?;
    let root_tree_oid = le_u64(block, 0x88, "volume root tree")?;
    let transaction_id = le_u64(block, 0x10, "volume transaction identifier")?;
    if object_map_oid == 0 || root_tree_oid == 0 {
        return Err(fail("APFS volume is missing an object map or root tree"));
    }

    let mut uuid = [0_u8; 16];
    uuid.copy_from_slice(
        block
            .get(0xf0..0x100)
            .ok_or_else(|| fail("APFS volume missing UUID"))?,
    );
    let name_bytes = block
        .get(0x2c0..0x3c0)
        .ok_or_else(|| fail("APFS volume missing name"))?;
    let name_end = name_bytes
        .iter()
        .position(|&byte| byte == 0)
        .ok_or_else(|| fail("APFS volume name is not terminated"))?;
    let name = std::str::from_utf8(&name_bytes[..name_end])
        .map_err(|_| fail("APFS volume name is not UTF-8"))?
        .to_owned();

    let root_paddr = resolve_object(
        image,
        container,
        object_map_oid,
        root_tree_oid,
        transaction_id,
        "volume object map",
    )?;
    let catalog = read_object(
        image,
        container.block_size,
        container.block_count,
        root_paddr,
        OBJECT_TYPE_BTREE,
        "catalog B-tree",
    )?;
    let entries = parse_catalog(&catalog)?;

    Ok(ApfsVolume {
        object_id: volume_oid,
        index,
        uuid,
        name,
        root_tree_oid,
        entries,
    })
}

fn resolve_object(
    image: &mut dyn ImageReader,
    container: &ApfsContainerSuperblock,
    omap_paddr: u64,
    target_oid: u64,
    max_xid: u64,
    label: &str,
) -> LabResult<u64> {
    let omap = read_object(
        image,
        container.block_size,
        container.block_count,
        omap_paddr,
        OBJECT_TYPE_OMAP,
        label,
    )?;
    let tree_paddr = le_u64(&omap, 0x30, "object-map tree")?;
    let tree = read_object(
        image,
        container.block_size,
        container.block_count,
        tree_paddr,
        OBJECT_TYPE_BTREE,
        "object-map B-tree",
    )?;

    let mut selected = None;
    for record in btree_leaf_records(&tree, "object-map B-tree")? {
        if record.key.len() != 16 || record.value.len() != 16 {
            return Err(fail("APFS object-map record has invalid size"));
        }
        let oid = le_u64(record.key, 0, "object-map key OID")?;
        let xid = le_u64(record.key, 8, "object-map key XID")?;
        let flags = le_u32(record.value, 0, "object-map value flags")?;
        let size = le_u32(record.value, 4, "object-map value size")?;
        let paddr = le_u64(record.value, 8, "object-map value address")?;
        if oid == target_oid
            && xid <= max_xid
            && flags & 1 == 0
            && size == container.block_size
            && selected.is_none_or(|(selected_xid, _)| xid > selected_xid)
        {
            selected = Some((xid, paddr));
        }
    }
    let (_, paddr) =
        selected.ok_or_else(|| fail(format!("APFS {label} does not map OID {target_oid}")))?;
    if paddr >= container.block_count {
        return Err(fail(format!("APFS {label} maps outside the container")));
    }
    Ok(paddr)
}

#[derive(Clone, Copy)]
struct BtreeRecord<'a> {
    key: &'a [u8],
    value: &'a [u8],
}

fn btree_leaf_records<'a>(block: &'a [u8], label: &str) -> LabResult<Vec<BtreeRecord<'a>>> {
    verify_object(block, OBJECT_TYPE_BTREE, label)?;
    if block.len() < 56 + BTREE_INFO_SIZE {
        return Err(fail(format!("APFS {label} is truncated")));
    }
    let flags = le_u16(block, 0x20, "B-tree node flags")?;
    let level = le_u16(block, 0x22, "B-tree node level")?;
    if flags & (BTNODE_ROOT | BTNODE_LEAF) != BTNODE_ROOT | BTNODE_LEAF || level != 0 {
        return Err(fail(format!(
            "APFS {label} multi-node trees are outside the validated subset"
        )));
    }

    let count = le_u32(block, 0x24, "B-tree key count")? as usize;
    let table_offset = le_u16(block, 0x28, "B-tree table offset")? as usize;
    let table_length = le_u16(block, 0x2a, "B-tree table length")? as usize;
    let fixed = flags & BTNODE_FIXED_KV_SIZE != 0;
    let toc_entry_size = if fixed { 4 } else { 8 };
    let required_table = count
        .checked_mul(toc_entry_size)
        .ok_or_else(|| fail(format!("APFS {label} table size overflow")))?;
    if table_length < required_table {
        return Err(fail(format!("APFS {label} table is too short")));
    }

    let table_start = 56_usize
        .checked_add(table_offset)
        .ok_or_else(|| fail(format!("APFS {label} table offset overflow")))?;
    let key_base = table_start
        .checked_add(table_length)
        .ok_or_else(|| fail(format!("APFS {label} key offset overflow")))?;
    let value_end = block.len() - BTREE_INFO_SIZE;
    if key_base > value_end || table_start + required_table > key_base {
        return Err(fail(format!("APFS {label} table is out of bounds")));
    }

    let info_start = value_end;
    let fixed_key_length = le_u32(block, info_start + 8, "B-tree fixed key size")? as usize;
    let fixed_value_length = le_u32(block, info_start + 12, "B-tree fixed value size")? as usize;
    let mut records = Vec::with_capacity(count);
    for index in 0..count {
        let toc = table_start + index * toc_entry_size;
        let key_offset = le_u16(block, toc, "B-tree key offset")? as usize;
        let (key_length, value_offset_at, value_length) = if fixed {
            (fixed_key_length, toc + 2, fixed_value_length)
        } else {
            (
                le_u16(block, toc + 2, "B-tree key length")? as usize,
                toc + 4,
                le_u16(block, toc + 6, "B-tree value length")? as usize,
            )
        };
        let value_offset = le_u16(block, value_offset_at, "B-tree value offset")? as usize;
        if value_offset == u16::MAX as usize {
            return Err(fail(format!(
                "APFS {label} contains an unsupported ghost value"
            )));
        }

        let key_start = key_base
            .checked_add(key_offset)
            .ok_or_else(|| fail(format!("APFS {label} key offset overflow")))?;
        let key_end = key_start
            .checked_add(key_length)
            .ok_or_else(|| fail(format!("APFS {label} key length overflow")))?;
        let value_start = value_end
            .checked_sub(value_offset)
            .ok_or_else(|| fail(format!("APFS {label} value offset underflow")))?;
        let value_slice_end = value_start
            .checked_add(value_length)
            .ok_or_else(|| fail(format!("APFS {label} value length overflow")))?;
        if key_end > value_end || value_slice_end > value_end || key_end > value_start {
            return Err(fail(format!("APFS {label} key/value is out of bounds")));
        }
        records.push(BtreeRecord {
            key: &block[key_start..key_end],
            value: &block[value_start..value_slice_end],
        });
    }
    Ok(records)
}

#[derive(Clone, Copy)]
struct Inode {
    kind: FsEntryKind,
    size: u64,
}

struct DirectoryRecord {
    object_id: u64,
    parent_id: u64,
    name: String,
    kind: FsEntryKind,
}

fn parse_catalog(block: &[u8]) -> LabResult<Vec<ApfsFileEntry>> {
    let mut inodes = HashMap::new();
    let mut directory_records = Vec::new();
    for record in btree_leaf_records(block, "catalog B-tree")? {
        if record.key.len() < 8 {
            return Err(fail("APFS catalog key is truncated"));
        }
        let key_header = le_u64(record.key, 0, "catalog key header")?;
        let object_id = key_header & APFS_OBJ_ID_MASK;
        match key_header >> 60 {
            APFS_TYPE_INODE => {
                if record.value.len() < 0x5c {
                    return Err(fail("APFS inode value is truncated"));
                }
                let mode = le_u16(record.value, 0x50, "inode mode")?;
                let kind = if mode & 0xf000 == 0x4000 {
                    FsEntryKind::Directory
                } else {
                    FsEntryKind::File
                };
                let size = le_u64(record.value, 0x54, "inode uncompressed size")?;
                if inodes.insert(object_id, Inode { kind, size }).is_some() {
                    return Err(fail("APFS catalog contains duplicate inode records"));
                }
            }
            APFS_TYPE_DIR_REC => {
                if record.key.len() < 11 || record.value.len() < 18 {
                    return Err(fail("APFS directory record is truncated"));
                }
                let name_length = le_u16(record.key, 8, "directory name length")? as usize;
                if name_length == 0 || 10 + name_length > record.key.len() {
                    return Err(fail("APFS directory name length is invalid"));
                }
                let name_bytes = &record.key[10..10 + name_length];
                if name_bytes.last() != Some(&0) {
                    return Err(fail("APFS directory name is not terminated"));
                }
                let name = std::str::from_utf8(&name_bytes[..name_length - 1])
                    .map_err(|_| fail("APFS directory name is not UTF-8"))?
                    .to_owned();
                if name.is_empty() || name == "." || name == ".." || name.contains('/') {
                    return Err(fail("APFS directory name is invalid"));
                }
                let child_id = le_u64(record.value, 0, "directory child identifier")?;
                let kind = if le_u16(record.value, 16, "directory flags")? & 0xf == 4 {
                    FsEntryKind::Directory
                } else {
                    FsEntryKind::File
                };
                directory_records.push(DirectoryRecord {
                    object_id: child_id,
                    parent_id: object_id,
                    name,
                    kind,
                });
            }
            _ => {}
        }
    }

    let mut links = HashMap::new();
    for record in &directory_records {
        if links
            .insert(record.object_id, (record.parent_id, record.name.clone()))
            .is_some()
        {
            return Err(fail(
                "APFS hard links are outside the validated catalog subset",
            ));
        }
    }

    let mut entries = Vec::with_capacity(directory_records.len());
    for record in directory_records {
        let inode = inodes
            .get(&record.object_id)
            .ok_or_else(|| fail("APFS directory record has no inode"))?;
        if inode.kind != record.kind {
            return Err(fail("APFS directory and inode types disagree"));
        }
        let path = build_path(record.parent_id, &record.name, &links)?;
        entries.push(ApfsFileEntry {
            object_id: record.object_id,
            parent_id: record.parent_id,
            name: record.name,
            path,
            kind: inode.kind,
            size: inode.size,
        });
    }
    entries.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(entries)
}

fn build_path(
    mut parent_id: u64,
    name: &str,
    links: &HashMap<u64, (u64, String)>,
) -> LabResult<String> {
    let mut parts = vec![name];
    let mut depth = 0_usize;
    while parent_id != APFS_ROOT_DIR_INO {
        let (next_parent, parent_name) = links
            .get(&parent_id)
            .ok_or_else(|| fail("APFS catalog entry has a missing parent"))?;
        parts.push(parent_name.as_str());
        parent_id = *next_parent;
        depth += 1;
        if depth > links.len() {
            return Err(fail("APFS catalog parent cycle"));
        }
    }
    parts.reverse();
    Ok(format!("/{}", parts.join("/")))
}

fn le_u16(buf: &[u8], offset: usize, label: &str) -> LabResult<u16> {
    let bytes = buf
        .get(offset..offset + 2)
        .ok_or_else(|| fail(format!("APFS missing {label}")))?;
    Ok(u16::from_le_bytes(
        bytes
            .try_into()
            .map_err(|_| fail(format!("APFS invalid {label}")))?,
    ))
}

fn le_u32(buf: &[u8], offset: usize, label: &str) -> LabResult<u32> {
    let bytes = buf
        .get(offset..offset + 4)
        .ok_or_else(|| fail(format!("APFS missing {label}")))?;
    Ok(u32::from_le_bytes(
        bytes
            .try_into()
            .map_err(|_| fail(format!("APFS invalid {label}")))?,
    ))
}

fn le_u64(buf: &[u8], offset: usize, label: &str) -> LabResult<u64> {
    let bytes = buf
        .get(offset..offset + 8)
        .ok_or_else(|| fail(format!("APFS missing {label}")))?;
    Ok(u64::from_le_bytes(
        bytes
            .try_into()
            .map_err(|_| fail(format!("APFS invalid {label}")))?,
    ))
}
