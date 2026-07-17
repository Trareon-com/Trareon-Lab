//! Filesystem enumeration for Trareon Lab (Day 16+).

mod fat_synth;
mod ntfs_synth;

pub use fat_synth::{
    enumerate_exfat_synthetic, enumerate_fat32_synthetic, write_exfat_synthetic_corpus,
    write_fat32_synthetic_corpus, FatSynthEntry, EXFAT_SYNTH_MAGIC, FAT32_SYNTH_MAGIC,
};
pub use ntfs_synth::{
    enumerate_ntfs_synthetic, write_ntfs_synthetic_corpus, FsEntryKind, NtfsSynthEntry,
    SynthFsEntry, NTFS_SYNTH_MAGIC,
};
