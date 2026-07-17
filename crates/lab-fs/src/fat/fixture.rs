//! Minimal FAT32 / exFAT fixtures.

use std::fs::File;
use std::io::Write;
use std::path::Path;

use lab_core::{LabError, LabResult};

pub fn write_minimal_fat32_image(path: &Path) -> LabResult<()> {
    // Layout: 1 reserved + 1 FAT sector + data starting cluster 2
    let bps = 512u16;
    let spc = 1u8;
    let reserved = 1u16;
    let fats = 1u8;
    let spf = 1u32;
    let root = 2u32;
    let mut img = vec![0u8; 64 * 512];

    img[0] = 0xEB;
    img[1] = 0x58;
    img[2] = 0x90;
    img[3..11].copy_from_slice(b"MSDOS5.0");
    img[0x0B..0x0D].copy_from_slice(&bps.to_le_bytes());
    img[0x0D] = spc;
    img[0x0E..0x10].copy_from_slice(&reserved.to_le_bytes());
    img[0x10] = fats;
    let total_sectors = (img.len() as u32) / 512;
    img[0x20..0x24].copy_from_slice(&total_sectors.to_le_bytes());
    img[0x24..0x28].copy_from_slice(&spf.to_le_bytes());
    img[0x2C..0x30].copy_from_slice(&root.to_le_bytes());
    img[0x52..0x57].copy_from_slice(b"FAT32");
    img[0x1FE] = 0x55;
    img[0x1FF] = 0xAA;

    // FAT at sector 1: cluster 2 = EOC
    let fat_off = reserved as usize * 512;
    img[fat_off..fat_off + 4].copy_from_slice(&0x0FFF_FFF8u32.to_le_bytes()); // media
    img[fat_off + 4..fat_off + 8].copy_from_slice(&0x0FFF_FFFFu32.to_le_bytes());
    img[fat_off + 8..fat_off + 12].copy_from_slice(&0x0FFF_FFFFu32.to_le_bytes()); // cluster 2 EOC

    // Root dir at cluster 2
    let data_off = fat_off + spf as usize * 512;
    let dir_off = data_off; // cluster 2
    // HELLO.TXT short entry
    let ent = &mut img[dir_off..dir_off + 32];
    ent[0..11].copy_from_slice(b"HELLO   TXT");
    ent[0x0B] = 0x20;
    ent[0x1A..0x1C].copy_from_slice(&3u16.to_le_bytes()); // first cluster low
    ent[0x1C..0x20].copy_from_slice(&5u32.to_le_bytes());
    // cluster 3 FAT EOC + content
    img[fat_off + 12..fat_off + 16].copy_from_slice(&0x0FFF_FFFFu32.to_le_bytes());
    let file_off = data_off + 512; // cluster 3
    img[file_off..file_off + 5].copy_from_slice(b"hello");

    // Deleted entry
    let dent = &mut img[dir_off + 32..dir_off + 64];
    dent[0..11].copy_from_slice(b"\xE5OLD    TXT");
    dent[0x0B] = 0x20;
    dent[0x1C..0x20].copy_from_slice(&3u32.to_le_bytes());

    File::create(path)
        .and_then(|mut f| f.write_all(&img))
        .map_err(|e| LabError::Internal {
            detail: format!("fat32 fixture: {e}"),
        })?;
    Ok(())
}

pub fn write_minimal_exfat_image(path: &Path) -> LabResult<()> {
    let mut img = vec![0u8; 128 * 512];
    img[0] = 0xEB;
    img[1] = 0x76;
    img[2] = 0x90;
    img[3..11].copy_from_slice(b"EXFAT   ");
    img[0x6C] = 9; // 512 byte sectors
    img[0x6D] = 0; // 1 sector/cluster
    img[0x50..0x54].copy_from_slice(&1u32.to_le_bytes()); // fat offset
    img[0x54..0x58].copy_from_slice(&1u32.to_le_bytes()); // fat length
    img[0x58..0x5C].copy_from_slice(&2u32.to_le_bytes()); // heap offset
    img[0x5C..0x60].copy_from_slice(&100u32.to_le_bytes());
    img[0x60..0x64].copy_from_slice(&2u32.to_le_bytes()); // root cluster
    img[0x1FE] = 0x55;
    img[0x1FF] = 0xAA;

    // Root at cluster 2 → heap_off + (2-2)*512
    let root = 2 * 512;
    // File entry 0x85 + stream 0xC0 + name 0xC1
    img[root] = 0x85;
    img[root + 1] = 2; // secondary count
    img[root + 32] = 0xC0;
    img[root + 32 + 8..root + 32 + 16].copy_from_slice(&4u64.to_le_bytes());
    img[root + 32 + 20..root + 32 + 24].copy_from_slice(&3u32.to_le_bytes());
    img[root + 64] = 0xC1;
    let name = "hi".encode_utf16().flat_map(|u| u.to_le_bytes()).collect::<Vec<_>>();
    img[root + 66..root + 66 + name.len()].copy_from_slice(&name);

    File::create(path)
        .and_then(|mut f| f.write_all(&img))
        .map_err(|e| LabError::Internal {
            detail: format!("exfat fixture: {e}"),
        })?;
    Ok(())
}
