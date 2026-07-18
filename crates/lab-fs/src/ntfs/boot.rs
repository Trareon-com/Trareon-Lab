//! NTFS $Boot sector.

use lab_core::{LabError, LabResult};
use lab_storage::ImageReader;

/// Parsed NTFS boot sector / BPB fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NtfsBootRecord {
    pub bytes_per_sector: u16,
    pub sectors_per_cluster: u8,
    pub mft_cluster: u64,
    pub mft_mirror_cluster: u64,
    pub mft_record_size: u32,
    pub index_record_size: u32,
    pub volume_serial: u64,
}

impl NtfsBootRecord {
    pub fn cluster_size(&self) -> u64 {
        self.bytes_per_sector as u64 * self.sectors_per_cluster as u64
    }
}

/// Opened NTFS volume geometry over an image.
#[derive(Debug, Clone)]
pub struct NtfsVolume {
    pub boot: NtfsBootRecord,
}

impl NtfsVolume {
    pub fn open(image: &mut dyn ImageReader) -> LabResult<Self> {
        let boot = parse_boot(image)?;
        Ok(Self { boot })
    }
}

pub fn parse_boot(image: &mut dyn ImageReader) -> LabResult<NtfsBootRecord> {
    let mut sector = [0u8; 512];
    let n = image.read_at(0, &mut sector)?;
    if n < 512 {
        return Err(LabError::Internal {
            detail: "image too small for NTFS boot sector".into(),
        });
    }
    if &sector[3..11] != b"NTFS    " {
        return Err(LabError::Internal {
            detail: "OEM id is not NTFS".into(),
        });
    }
    let bytes_per_sector = u16::from_le_bytes(sector[0x0B..0x0D].try_into().unwrap());
    let sectors_per_cluster = sector[0x0D];
    if bytes_per_sector == 0 || sectors_per_cluster == 0 {
        return Err(LabError::Internal {
            detail: "invalid NTFS geometry".into(),
        });
    }
    let mft_cluster = u64::from_le_bytes(sector[0x30..0x38].try_into().unwrap());
    let mft_mirror_cluster = u64::from_le_bytes(sector[0x38..0x40].try_into().unwrap());
    let clusters_per_mft = sector[0x40] as i8;
    let mft_record_size =
        record_size_from_clusters(clusters_per_mft, bytes_per_sector, sectors_per_cluster)?;
    let clusters_per_index = sector[0x44] as i8;
    let index_record_size =
        record_size_from_clusters(clusters_per_index, bytes_per_sector, sectors_per_cluster)?;
    let volume_serial = u64::from_le_bytes(sector[0x48..0x50].try_into().unwrap());

    Ok(NtfsBootRecord {
        bytes_per_sector,
        sectors_per_cluster,
        mft_cluster,
        mft_mirror_cluster,
        mft_record_size,
        index_record_size,
        volume_serial,
    })
}

fn record_size_from_clusters(
    clusters: i8,
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
) -> LabResult<u32> {
    let cluster = bytes_per_sector as u32 * sectors_per_cluster as u32;
    if clusters > 0 {
        Ok(cluster * clusters as u32)
    } else if clusters < 0 {
        // size = 2^|clusters|
        let shift = (-clusters) as u32;
        if shift > 20 {
            return Err(LabError::Internal {
                detail: "mft record size exponent too large".into(),
            });
        }
        Ok(1u32 << shift)
    } else {
        Err(LabError::Internal {
            detail: "zero clusters-per-mft-record".into(),
        })
    }
}
