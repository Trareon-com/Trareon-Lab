//! Hand-crafted APFS golden image for the validated parser subset.

use std::fs::File;
use std::io::Write;
use std::path::Path;

use lab_core::LabResult;

use super::{fail, object_checksum, APFS_NX_MAGIC, APFS_VOLUME_MAGIC};

const BLOCK_SIZE: usize = 4096;
const BLOCK_COUNT: usize = 7;
const XID: u64 = 1;

pub fn write_minimal_apfs_image(path: &Path) -> LabResult<()> {
    let mut image = vec![0_u8; BLOCK_SIZE * BLOCK_COUNT];

    // Block 0: nx_superblock_t. NXSB follows the 32-byte object header.
    let nx = block_mut(&mut image, 0);
    write_object_header(nx, 1, XID, 0x8000_0001, 0);
    nx[0x20..0x24].copy_from_slice(APFS_NX_MAGIC);
    put_u32(nx, 0x24, BLOCK_SIZE as u32);
    put_u64(nx, 0x28, BLOCK_COUNT as u64);
    nx[0x48..0x58].copy_from_slice(b"TRAREON-APFS-NX!");
    put_u64(nx, 0x58, 201);
    put_u64(nx, 0x60, XID + 1);
    put_u64(nx, 0xa0, 1); // Physical container object map.
    put_u32(nx, 0xb4, 100);
    put_u64(nx, 0xb8, 100); // Virtual volume-superblock OID.

    // Blocks 1-2: container object map and its one-node physical B-tree.
    write_omap(block_mut(&mut image, 1), 1, 2);
    write_leaf(
        block_mut(&mut image, 2),
        2,
        11,
        vec![omap_record(100, 3)],
        Some((16, 16)),
    );

    // Block 3: apfs_superblock_t.
    let volume = block_mut(&mut image, 3);
    write_object_header(volume, 100, XID, 13, 0);
    volume[0x20..0x24].copy_from_slice(APFS_VOLUME_MAGIC);
    put_u32(volume, 0x24, 0);
    put_u64(volume, 0x80, 4); // Physical volume object map.
    put_u64(volume, 0x88, 200); // Virtual catalog-root OID.
    put_u64(volume, 0xb8, 1);
    put_u64(volume, 0xc0, 2);
    volume[0xf0..0x100].copy_from_slice(b"TRAREON-APFS-VOL");
    let volume_name = b"Golden\0";
    volume[0x2c0..0x2c0 + volume_name.len()].copy_from_slice(volume_name);

    // Blocks 4-5: volume object map and its one-node physical B-tree.
    write_omap(block_mut(&mut image, 4), 4, 5);
    write_leaf(
        block_mut(&mut image, 5),
        5,
        11,
        vec![omap_record(200, 6)],
        Some((16, 16)),
    );

    // Block 6: real APFS file-system catalog record layouts in one leaf.
    let records = vec![
        inode_record(2, 1, 0x41ed, 0),
        drec_record(2, 10, "Documents", 4),
        inode_record(10, 2, 0x41ed, 0),
        drec_record(10, 11, "hello.txt", 8),
        inode_record(11, 10, 0x81a4, 4),
    ];
    write_leaf(block_mut(&mut image, 6), 200, 14, records, None);

    for number in 0..BLOCK_COUNT {
        let block = block_mut(&mut image, number);
        let checksum = object_checksum(&block[8..]);
        put_u64(block, 0, checksum);
    }

    File::create(path)
        .and_then(|mut file| file.write_all(&image))
        .map_err(|error| fail(format!("APFS fixture: {error}")))?;
    Ok(())
}

fn write_omap(block: &mut [u8], oid: u64, tree_oid: u64) {
    write_object_header(block, oid, XID, 0x4000_000b, 0);
    put_u64(block, 0x30, tree_oid);
}

fn omap_record(oid: u64, paddr: u64) -> (Vec<u8>, Vec<u8>) {
    let mut key = vec![0_u8; 16];
    put_u64(&mut key, 0, oid);
    put_u64(&mut key, 8, XID);
    let mut value = vec![0_u8; 16];
    put_u32(&mut value, 4, BLOCK_SIZE as u32);
    put_u64(&mut value, 8, paddr);
    (key, value)
}

fn inode_record(oid: u64, parent_id: u64, mode: u16, size: u64) -> (Vec<u8>, Vec<u8>) {
    let mut key = vec![0_u8; 8];
    put_u64(&mut key, 0, oid | (3_u64 << 60));
    let mut value = vec![0_u8; 0x5c];
    put_u64(&mut value, 0, parent_id);
    put_u64(&mut value, 8, oid);
    if size != 0 {
        put_u64(&mut value, 0x30, 0x0004_0000);
    }
    put_u16(&mut value, 0x50, mode);
    put_u64(&mut value, 0x54, size);
    (key, value)
}

fn drec_record(parent_id: u64, oid: u64, name: &str, file_type: u16) -> (Vec<u8>, Vec<u8>) {
    let name_length = name.len() + 1;
    let mut key = vec![0_u8; 10 + name_length];
    put_u64(&mut key, 0, parent_id | (9_u64 << 60));
    put_u16(&mut key, 8, name_length as u16);
    key[10..10 + name.len()].copy_from_slice(name.as_bytes());
    let mut value = vec![0_u8; 18];
    put_u64(&mut value, 0, oid);
    put_u16(&mut value, 16, file_type);
    (key, value)
}

fn write_leaf(
    block: &mut [u8],
    oid: u64,
    subtype: u32,
    records: Vec<(Vec<u8>, Vec<u8>)>,
    fixed_sizes: Option<(u32, u32)>,
) {
    write_object_header(block, oid, XID, 0x4000_0002, subtype);
    let fixed = fixed_sizes.is_some();
    put_u16(
        block,
        0x20,
        0x0001 | 0x0002 | if fixed { 0x0004 } else { 0 },
    );
    put_u32(block, 0x24, records.len() as u32);

    let toc_entry_size = if fixed { 4 } else { 8 };
    let table_length = records.len() * toc_entry_size;
    put_u16(block, 0x28, 0);
    put_u16(block, 0x2a, table_length as u16);
    let key_base = 56 + table_length;
    let value_end = BLOCK_SIZE - 40;
    let mut key_offset = 0_usize;
    let mut value_offset = 0_usize;
    let mut longest_key = 0_usize;
    let mut longest_value = 0_usize;

    for (index, (key, value)) in records.iter().enumerate() {
        let toc = 56 + index * toc_entry_size;
        value_offset += value.len();
        put_u16(block, toc, key_offset as u16);
        if fixed {
            put_u16(block, toc + 2, value_offset as u16);
        } else {
            put_u16(block, toc + 2, key.len() as u16);
            put_u16(block, toc + 4, value_offset as u16);
            put_u16(block, toc + 6, value.len() as u16);
        }
        block[key_base + key_offset..key_base + key_offset + key.len()].copy_from_slice(key);
        block[value_end - value_offset..value_end - value_offset + value.len()]
            .copy_from_slice(value);
        key_offset += key.len();
        longest_key = longest_key.max(key.len());
        longest_value = longest_value.max(value.len());
    }
    assert!(key_base + key_offset <= value_end - value_offset);

    // btree_info_t footer. Nonaligned K/V storage keeps the fixture compact.
    put_u32(block, value_end, 0x0000_0040);
    put_u32(block, value_end + 4, BLOCK_SIZE as u32);
    if let Some((key_size, value_size)) = fixed_sizes {
        put_u32(block, value_end + 8, key_size);
        put_u32(block, value_end + 12, value_size);
    }
    put_u32(block, value_end + 16, longest_key as u32);
    put_u32(block, value_end + 20, longest_value as u32);
    put_u64(block, value_end + 24, records.len() as u64);
    put_u64(block, value_end + 32, 1);
}

fn write_object_header(block: &mut [u8], oid: u64, xid: u64, object_type: u32, subtype: u32) {
    put_u64(block, 8, oid);
    put_u64(block, 16, xid);
    put_u32(block, 24, object_type);
    put_u32(block, 28, subtype);
}

fn block_mut(image: &mut [u8], number: usize) -> &mut [u8] {
    &mut image[number * BLOCK_SIZE..(number + 1) * BLOCK_SIZE]
}

fn put_u16(buf: &mut [u8], offset: usize, value: u16) {
    buf[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
}

fn put_u32(buf: &mut [u8], offset: usize, value: u32) {
    buf[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn put_u64(buf: &mut [u8], offset: usize, value: u64) {
    buf[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}
