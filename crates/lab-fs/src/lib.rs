//! Filesystem enumeration for Trareon Lab (Day 16+).

mod content;
pub mod ext4;
pub mod fat;
mod fat_synth;
pub mod ntfs;
mod ntfs_synth;
mod unix_synth;

pub use content::{
    ingest_synth_file_to_cas, load_fs_content_synthetic, read_synth_file_bytes,
    write_fs_content_synthetic, CasIngestResult, SynthFileContent, FS_CONTENT_SYNTH_MAGIC,
};
pub use ext4::{enumerate_ext4, write_minimal_ext4_image, Ext4FsEntry};
pub use fat::{
    enumerate_exfat, enumerate_fat32, write_minimal_exfat_image, write_minimal_fat32_image,
    FatFsEntry,
};
pub use fat_synth::{
    enumerate_exfat_synthetic, enumerate_fat32_synthetic, write_exfat_synthetic_corpus,
    write_fat32_synthetic_corpus, FatSynthEntry, EXFAT_SYNTH_MAGIC, FAT32_SYNTH_MAGIC,
};
pub use ntfs::{
    enumerate_ntfs, parse_usn_journal, write_minimal_ntfs_image, Attribute, FileNameAttr, FsEntry,
    MftIterator, MftRecord, MftRecordFlags, NtfsBootRecord, NtfsEnumerateOptions, NtfsVolume,
    StdInfo, Timestamps, UsnRecord,
};
pub use ntfs_synth::{
    enumerate_ntfs_synthetic, write_ntfs_synthetic_corpus, FsEntryKind, NtfsSynthEntry,
    SynthFsEntry, NTFS_SYNTH_MAGIC,
};
pub use unix_synth::{
    enumerate_apfs_synthetic, enumerate_ext4_synthetic, recover_deleted_partial,
    write_apfs_synthetic_corpus, write_ext4_synthetic_corpus, DeletedRecoveryHit, UnixFsSynthEntry,
    APFS_SYNTH_MAGIC, EXT4_SYNTH_MAGIC,
};
