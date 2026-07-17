//! Unified disk-image reader trait.

use lab_core::{LabResult, ProgressSink};

use crate::raw::ImageKind;

/// Integrity summary after verifying an image container.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct IntegrityReport {
    pub ok: bool,
    pub crc_errors: u64,
    pub chunks_checked: u64,
    pub message: String,
}

/// Byte-addressable forensic image (raw, E01, …).
pub trait ImageReader: Send {
    fn kind(&self) -> ImageKind;
    fn byte_length(&self) -> u64;
    fn read_at(&mut self, offset: u64, buf: &mut [u8]) -> LabResult<usize>;
    fn verify_integrity(&mut self, progress: &mut dyn ProgressSink) -> LabResult<IntegrityReport>;
    fn crc_errors(&self) -> u64 {
        0
    }
}
