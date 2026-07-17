//! FAT32 / exFAT boot sector (BPB) parsing.

use lab_core::{LabError, LabResult};
use lab_storage::ImageReader;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fat32Boot {
    pub bytes_per_sector: u16,
    pub sectors_per_cluster: u8,
    pub reserved_sectors: u16,
    pub num_fats: u8,
    pub sectors_per_fat: u32,
    pub root_cluster: u32,
    pub total_sectors: u32,
}

impl Fat32Boot {
    pub fn cluster_size(&self) -> u64 {
        self.bytes_per_sector as u64 * self.sectors_per_cluster as u64
    }

    pub fn fat_offset(&self) -> u64 {
        self.reserved_sectors as u64 * self.bytes_per_sector as u64
    }

    pub fn data_offset(&self) -> u64 {
        self.fat_offset()
            + self.num_fats as u64 * self.sectors_per_fat as u64 * self.bytes_per_sector as u64
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExfatBoot {
    pub bytes_per_sector_shift: u8,
    pub sectors_per_cluster_shift: u8,
    pub fat_offset_sectors: u32,
    pub fat_length_sectors: u32,
    pub cluster_heap_offset: u32,
    pub cluster_count: u32,
    pub root_cluster: u32,
}

impl ExfatBoot {
    pub fn bytes_per_sector(&self) -> u64 {
        1u64 << self.bytes_per_sector_shift
    }

    pub fn cluster_size(&self) -> u64 {
        self.bytes_per_sector() << self.sectors_per_cluster_shift
    }
}

pub fn parse_fat32_boot(image: &mut dyn ImageReader) -> LabResult<Fat32Boot> {
    let mut s = [0u8; 512];
    image.read_at(0, &mut s)?;
    if &s[0x52..0x57] != b"FAT32" {
        return Err(LabError::Internal {
            detail: "not a FAT32 boot sector".into(),
        });
    }
    Ok(Fat32Boot {
        bytes_per_sector: u16::from_le_bytes(s[0x0B..0x0D].try_into().unwrap()),
        sectors_per_cluster: s[0x0D],
        reserved_sectors: u16::from_le_bytes(s[0x0E..0x10].try_into().unwrap()),
        num_fats: s[0x10],
        sectors_per_fat: u32::from_le_bytes(s[0x24..0x28].try_into().unwrap()),
        root_cluster: u32::from_le_bytes(s[0x2C..0x30].try_into().unwrap()),
        total_sectors: {
            let t16 = u16::from_le_bytes(s[0x13..0x15].try_into().unwrap());
            if t16 != 0 {
                t16 as u32
            } else {
                u32::from_le_bytes(s[0x20..0x24].try_into().unwrap())
            }
        },
    })
}

pub fn parse_exfat_boot(image: &mut dyn ImageReader) -> LabResult<ExfatBoot> {
    let mut s = [0u8; 512];
    image.read_at(0, &mut s)?;
    if &s[3..11] != b"EXFAT   " {
        return Err(LabError::Internal {
            detail: "not an exFAT boot sector".into(),
        });
    }
    Ok(ExfatBoot {
        bytes_per_sector_shift: s[0x6C],
        sectors_per_cluster_shift: s[0x6D],
        fat_offset_sectors: u32::from_le_bytes(s[0x50..0x54].try_into().unwrap()),
        fat_length_sectors: u32::from_le_bytes(s[0x54..0x58].try_into().unwrap()),
        cluster_heap_offset: u32::from_le_bytes(s[0x58..0x5C].try_into().unwrap()),
        cluster_count: u32::from_le_bytes(s[0x5C..0x60].try_into().unwrap()),
        root_cluster: u32::from_le_bytes(s[0x60..0x64].try_into().unwrap()),
    })
}
