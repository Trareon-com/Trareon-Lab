//! Real FAT32 / exFAT volume parsers.

mod bpb;
mod enumerate;
mod fixture;

pub use bpb::{ExfatBoot, Fat32Boot};
pub use enumerate::{enumerate_exfat, enumerate_fat32, FatFsEntry};
pub use fixture::{write_minimal_exfat_image, write_minimal_fat32_image};
