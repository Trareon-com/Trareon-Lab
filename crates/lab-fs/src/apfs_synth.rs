//! Experimental synthetic APFS corpus support.
//!
//! This namespace is deliberately separate from the real `NXSB` parser in
//! [`crate::apfs`]. Its on-disk marker is the Trareon-only `TRFSAPFS`.

pub use crate::unix_synth::{
    enumerate_apfs_synthetic, write_apfs_synthetic_corpus, UnixFsSynthEntry, APFS_SYNTH_MAGIC,
};
