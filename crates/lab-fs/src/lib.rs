//! Filesystem enumeration for Trareon Lab (Day 16+).

mod ntfs_synth;

pub use ntfs_synth::{
    enumerate_ntfs_synthetic, write_ntfs_synthetic_corpus, FsEntryKind, NtfsSynthEntry,
    SynthFsEntry, NTFS_SYNTH_MAGIC,
};
