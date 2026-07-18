//! EXT4 superblock.

use lab_core::{LabError, LabResult};
use lab_storage::ImageReader;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ext4Superblock {
    pub inodes_count: u32,
    pub blocks_count: u32,
    pub block_size: u32,
    pub blocks_per_group: u32,
    pub inodes_per_group: u32,
    pub inode_size: u16,
    pub first_data_block: u32,
    pub feature_incompat: u32,
}

pub fn parse_superblock(image: &mut dyn ImageReader) -> LabResult<Ext4Superblock> {
    let mut sb = [0u8; 1024];
    image.read_at(1024, &mut sb)?;
    let magic = u16::from_le_bytes(sb[0x38..0x3A].try_into().unwrap());
    if magic != 0xEF53 {
        return Err(LabError::Internal {
            detail: format!("bad ext magic {magic:#x}"),
        });
    }
    let log_block_size = u32::from_le_bytes(sb[0x18..0x1C].try_into().unwrap());
    Ok(Ext4Superblock {
        inodes_count: u32::from_le_bytes(sb[0x00..0x04].try_into().unwrap()),
        blocks_count: u32::from_le_bytes(sb[0x04..0x08].try_into().unwrap()),
        block_size: 1024u32 << log_block_size,
        blocks_per_group: u32::from_le_bytes(sb[0x20..0x24].try_into().unwrap()),
        inodes_per_group: u32::from_le_bytes(sb[0x28..0x2C].try_into().unwrap()),
        inode_size: u16::from_le_bytes(sb[0x58..0x5A].try_into().unwrap()),
        first_data_block: u32::from_le_bytes(sb[0x14..0x18].try_into().unwrap()),
        feature_incompat: u32::from_le_bytes(sb[0x60..0x64].try_into().unwrap()),
    })
}
