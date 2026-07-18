//! APFS parser for a corpus-validated, read-only on-disk subset.
//!
//! The container superblock magic (`NXSB`) is at byte 32 of block zero,
//! immediately after the 32-byte APFS object header.

mod enumerate;
mod fixture;
mod superblock;

use lab_core::{LabError, LabResult};
use lab_storage::ImageReader;

pub use enumerate::{enumerate_apfs, ApfsFileEntry, ApfsVolume};
pub use fixture::write_minimal_apfs_image;
pub use superblock::{detect_apfs, parse_apfs_container, ApfsContainerSuperblock};

pub const APFS_NX_MAGIC: &[u8; 4] = b"NXSB";
pub const APFS_VOLUME_MAGIC: &[u8; 4] = b"APSB";

const OBJECT_HEADER_SIZE: usize = 32;
const OBJECT_TYPE_MASK: u32 = 0x0000_ffff;

fn fail(detail: impl Into<String>) -> LabError {
    LabError::Internal {
        detail: detail.into(),
    }
}

fn read_exact_at(
    image: &mut dyn ImageReader,
    offset: u64,
    buf: &mut [u8],
    label: &str,
) -> LabResult<()> {
    let end = offset
        .checked_add(buf.len() as u64)
        .ok_or_else(|| fail(format!("APFS {label} offset overflow")))?;
    if end > image.byte_length() {
        return Err(fail(format!("APFS {label} truncated")));
    }

    let mut done = 0;
    while done < buf.len() {
        let n = image.read_at(offset + done as u64, &mut buf[done..])?;
        if n == 0 {
            return Err(fail(format!("APFS {label} truncated")));
        }
        done += n;
    }
    Ok(())
}

fn read_object(
    image: &mut dyn ImageReader,
    block_size: u32,
    block_count: u64,
    paddr: u64,
    expected_type: u32,
    label: &str,
) -> LabResult<Vec<u8>> {
    if paddr >= block_count {
        return Err(fail(format!("APFS {label} block out of range")));
    }
    let offset = paddr
        .checked_mul(block_size as u64)
        .ok_or_else(|| fail(format!("APFS {label} offset overflow")))?;
    let mut block = vec![0; block_size as usize];
    read_exact_at(image, offset, &mut block, label)?;
    verify_object(&block, expected_type, label)?;
    Ok(block)
}

fn verify_object(block: &[u8], expected_type: u32, label: &str) -> LabResult<()> {
    if block.len() < OBJECT_HEADER_SIZE || !block.len().is_multiple_of(4) {
        return Err(fail(format!("APFS {label} invalid object size")));
    }
    let stored = u64::from_le_bytes(
        block[0..8]
            .try_into()
            .map_err(|_| fail(format!("APFS {label} missing checksum")))?,
    );
    if stored != object_checksum(&block[8..]) {
        return Err(fail(format!("APFS {label} checksum mismatch")));
    }
    let object_type = u32::from_le_bytes(
        block[24..28]
            .try_into()
            .map_err(|_| fail(format!("APFS {label} missing object type")))?,
    ) & OBJECT_TYPE_MASK;
    if object_type != expected_type {
        return Err(fail(format!(
            "APFS {label} object type {object_type:#x}, expected {expected_type:#x}"
        )));
    }
    Ok(())
}

fn object_checksum(data: &[u8]) -> u64 {
    const MOD: u64 = u32::MAX as u64;
    let mut sum1 = 0_u64;
    let mut sum2 = 0_u64;
    for chunk in data.chunks_exact(4) {
        let word = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]) as u64;
        sum1 = (sum1 + word) % MOD;
        sum2 = (sum2 + sum1) % MOD;
    }
    let low = MOD - ((sum1 + sum2) % MOD);
    let high = MOD - ((sum1 + low) % MOD);
    low | (high << 32)
}
