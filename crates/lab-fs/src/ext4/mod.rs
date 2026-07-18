//! Real EXT4 volume parser (superblock, inodes, extents, dirs).

mod enumerate;
mod fixture;
mod superblock;

pub use enumerate::{enumerate_ext4, Ext4FsEntry};
pub use fixture::write_minimal_ext4_image;
pub use superblock::{Ext4Superblock, parse_superblock};
