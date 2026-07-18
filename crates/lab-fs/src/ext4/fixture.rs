//! Minimal EXT4 fixture (hand-crafted).

use std::fs::File;
use std::io::Write;
use std::path::Path;

use lab_core::{LabError, LabResult};

pub fn write_minimal_ext4_image(path: &Path) -> LabResult<()> {
    // 1 MiB image, 1024-byte blocks
    let block = 1024usize;
    let mut img = vec![0u8; 256 * block];

    // Superblock at offset 1024
    let sb = &mut img[1024..2048];
    sb[0..4].copy_from_slice(&32u32.to_le_bytes()); // inodes
    sb[4..8].copy_from_slice(&200u32.to_le_bytes()); // blocks
    sb[0x14..0x18].copy_from_slice(&1u32.to_le_bytes()); // first_data_block
    sb[0x18..0x1C].copy_from_slice(&0u32.to_le_bytes()); // log_block_size → 1024
    sb[0x20..0x24].copy_from_slice(&200u32.to_le_bytes()); // blocks_per_group
    sb[0x28..0x2C].copy_from_slice(&32u32.to_le_bytes()); // inodes_per_group
    sb[0x38..0x3A].copy_from_slice(&0xEF53u16.to_le_bytes());
    sb[0x58..0x5A].copy_from_slice(&128u16.to_le_bytes()); // inode_size
    sb[0x60..0x64].copy_from_slice(&0x40u32.to_le_bytes()); // INCOMPAT_EXTENTS-ish optional

    // GDT at block 2 (first_data_block+1 = 2)
    let gdt = &mut img[2 * block..2 * block + 32];
    gdt[8..12].copy_from_slice(&5u32.to_le_bytes()); // inode table block

    // Inode table at block 5; inode 2 = root at index 1
    let inode_size = 128usize;
    let root_off = 5 * block + inode_size;
    let root = &mut img[root_off..root_off + inode_size];
    root[0..2].copy_from_slice(&0x41EDu16.to_le_bytes()); // dir mode
    root[4..8].copy_from_slice(&(block as u32).to_le_bytes());
    // direct block 0 → block 10
    root[0x28..0x2C].copy_from_slice(&10u32.to_le_bytes());

    // Inode 11 = file
    let file_ino = 11u32;
    let file_off = 5 * block + (file_ino as usize - 1) * inode_size;
    let fine = &mut img[file_off..file_off + inode_size];
    fine[0..2].copy_from_slice(&0x81A4u16.to_le_bytes());
    fine[4..8].copy_from_slice(&4u32.to_le_bytes());
    fine[0x28..0x2C].copy_from_slice(&11u32.to_le_bytes());
    img[11 * block..11 * block + 4].copy_from_slice(b"data");

    // Directory block 10
    let dir = &mut img[10 * block..11 * block];
    // .
    write_dirent(dir, 0, 2, ".", 2);
    // ..
    write_dirent(dir, 12, 2, "..", 2);
    // hello.txt
    write_dirent(dir, 24, file_ino, "hello.txt", 1);

    File::create(path)
        .and_then(|mut f| f.write_all(&img))
        .map_err(|e| LabError::Internal {
            detail: format!("ext4 fixture: {e}"),
        })?;
    Ok(())
}

fn write_dirent(buf: &mut [u8], off: usize, inode: u32, name: &str, ftype: u8) {
    let name_b = name.as_bytes();
    let rec = (8 + name_b.len()).div_ceil(4) * 4;
    buf[off..off + 4].copy_from_slice(&inode.to_le_bytes());
    buf[off + 4..off + 6].copy_from_slice(&(rec as u16).to_le_bytes());
    buf[off + 6] = name_b.len() as u8;
    buf[off + 7] = ftype;
    buf[off + 8..off + 8 + name_b.len()].copy_from_slice(name_b);
}
