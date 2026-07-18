//! APFS container superblock parsing.

use lab_core::LabResult;
use lab_storage::ImageReader;

use super::{fail, read_exact_at, verify_object, APFS_NX_MAGIC};

const NX_MIN_BLOCK_SIZE: u32 = 4096;
const NX_MAX_BLOCK_SIZE: u32 = 65_536;
const NX_MAX_FILE_SYSTEMS: usize = 100;
const NX_FS_OID_OFFSET: usize = 0xb8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApfsContainerSuperblock {
    pub block_size: u32,
    pub block_count: u64,
    pub uuid: [u8; 16],
    pub transaction_id: u64,
    pub object_map_oid: u64,
    pub volume_count: u32,
    pub volume_oids: Vec<u64>,
}

/// Detect the `NXSB` magic at byte 32 of block zero.
pub fn detect_apfs(image: &mut dyn ImageReader) -> LabResult<bool> {
    let mut header = [0_u8; 36];
    read_exact_at(image, 0, &mut header, "container header")?;
    Ok(&header[32..36] == APFS_NX_MAGIC)
}

/// Parse and validate the APFS container superblock in block zero.
pub fn parse_apfs_container(image: &mut dyn ImageReader) -> LabResult<ApfsContainerSuperblock> {
    let mut header = [0_u8; 40];
    read_exact_at(image, 0, &mut header, "container header")?;
    if &header[32..36] != APFS_NX_MAGIC {
        return Err(fail("APFS container bad NXSB magic"));
    }

    let block_size = u32::from_le_bytes(
        header[36..40]
            .try_into()
            .map_err(|_| fail("APFS container missing block size"))?,
    );
    if !(NX_MIN_BLOCK_SIZE..=NX_MAX_BLOCK_SIZE).contains(&block_size)
        || !block_size.is_power_of_two()
    {
        return Err(fail(format!("APFS invalid block size {block_size}")));
    }

    let mut block = vec![0_u8; block_size as usize];
    read_exact_at(image, 0, &mut block, "container superblock")?;
    verify_object(&block, 1, "container superblock")?;

    let block_count = le_u64(&block, 0x28, "block count")?;
    if block_count == 0 {
        return Err(fail("APFS container has zero blocks"));
    }
    let declared_bytes = block_count
        .checked_mul(block_size as u64)
        .ok_or_else(|| fail("APFS container size overflow"))?;
    if declared_bytes > image.byte_length() {
        return Err(fail("APFS container extends beyond image"));
    }

    let max_file_systems = le_u32(&block, 0xb4, "maximum volume count")? as usize;
    if max_file_systems > NX_MAX_FILE_SYSTEMS {
        return Err(fail(format!(
            "APFS maximum volume count {max_file_systems} exceeds format limit"
        )));
    }
    let mut volume_oids = Vec::new();
    for index in 0..max_file_systems {
        let oid = le_u64(
            &block,
            NX_FS_OID_OFFSET + index * 8,
            "volume object identifier",
        )?;
        if oid != 0 {
            volume_oids.push(oid);
        }
    }

    let object_map_oid = le_u64(&block, 0xa0, "container object map")?;
    if !volume_oids.is_empty() && object_map_oid == 0 {
        return Err(fail("APFS container with volumes has no object map"));
    }

    let mut uuid = [0_u8; 16];
    uuid.copy_from_slice(&block[0x48..0x58]);
    Ok(ApfsContainerSuperblock {
        block_size,
        block_count,
        uuid,
        transaction_id: le_u64(&block, 0x10, "transaction identifier")?,
        object_map_oid,
        volume_count: volume_oids.len() as u32,
        volume_oids,
    })
}

fn le_u32(buf: &[u8], offset: usize, label: &str) -> LabResult<u32> {
    let bytes = buf
        .get(offset..offset + 4)
        .ok_or_else(|| fail(format!("APFS container missing {label}")))?;
    Ok(u32::from_le_bytes(bytes.try_into().map_err(|_| {
        fail(format!("APFS container invalid {label}"))
    })?))
}

fn le_u64(buf: &[u8], offset: usize, label: &str) -> LabResult<u64> {
    let bytes = buf
        .get(offset..offset + 8)
        .ok_or_else(|| fail(format!("APFS container missing {label}")))?;
    Ok(u64::from_le_bytes(bytes.try_into().map_err(|_| {
        fail(format!("APFS container invalid {label}"))
    })?))
}
