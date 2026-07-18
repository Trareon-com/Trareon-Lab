//! USN Journal ($UsnJrnl:$J) parser — USN_RECORD_V2/V3 subset.

use lab_core::LabResult;

/// USN reason flag constants.
#[allow(non_snake_case)]
pub mod UsnReason {
    pub const DATA_OVERWRITE: u32 = 0x0000_0001;
    pub const DATA_EXTEND: u32 = 0x0000_0002;
    pub const FILE_CREATE: u32 = 0x0000_0100;
    pub const FILE_DELETE: u32 = 0x0000_0200;
    pub const RENAME_OLD: u32 = 0x0000_1000;
    pub const RENAME_NEW: u32 = 0x0000_2000;
    pub const CLOSE: u32 = 0x8000_0000;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsnRecord {
    pub usn: u64,
    pub timestamp: u64,
    pub reason: u32,
    pub file_name: String,
    pub file_ref: u64,
    pub parent_ref: u64,
}

/// Parse a raw `$J` stream buffer into USN records (V2/V3).
pub fn parse_usn_journal(data: &[u8]) -> LabResult<Vec<UsnRecord>> {
    let mut out = Vec::new();
    let mut off = 0usize;
    while off + 60 < data.len() {
        if data[off..off + 4].iter().all(|&b| b == 0) {
            off += 8;
            continue;
        }
        let record_len = u32::from_le_bytes(data[off..off + 4].try_into().unwrap()) as usize;
        if record_len < 60 || off + record_len > data.len() {
            break;
        }
        let major = u16::from_le_bytes(data[off + 4..off + 6].try_into().unwrap());
        if major != 2 && major != 3 {
            off += record_len;
            continue;
        }
        let file_ref = u64::from_le_bytes(data[off + 8..off + 16].try_into().unwrap());
        let parent_ref = u64::from_le_bytes(data[off + 16..off + 24].try_into().unwrap());
        let usn = u64::from_le_bytes(data[off + 24..off + 32].try_into().unwrap());
        let timestamp = u64::from_le_bytes(data[off + 32..off + 40].try_into().unwrap());
        let reason = u32::from_le_bytes(data[off + 40..off + 44].try_into().unwrap());
        let name_len = u16::from_le_bytes(data[off + 56..off + 58].try_into().unwrap()) as usize;
        let name_off = u16::from_le_bytes(data[off + 58..off + 60].try_into().unwrap()) as usize;
        let file_name = if name_off + name_len <= record_len {
            let nb = &data[off + name_off..off + name_off + name_len];
            let u16s: Vec<u16> = nb
                .chunks_exact(2)
                .map(|c| u16::from_le_bytes([c[0], c[1]]))
                .collect();
            String::from_utf16_lossy(&u16s)
        } else {
            String::new()
        };
        out.push(UsnRecord {
            usn,
            timestamp,
            reason,
            file_name,
            file_ref: file_ref & 0x0000_FFFF_FFFF,
            parent_ref: parent_ref & 0x0000_FFFF_FFFF,
        });
        off += record_len;
    }
    Ok(out)
}
