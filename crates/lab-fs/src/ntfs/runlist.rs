//! NTFS data runlist decoder (VCN → LCN mapping pairs).

use lab_core::{LabError, LabResult};

/// One run: `vcn` length starting at `lcn` (`None` = sparse).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataRun {
    pub vcn_length: u64,
    pub lcn: Option<u64>,
}

/// Decode NTFS mapping pairs until a zero-length terminator.
pub fn decode_runlist(data: &[u8]) -> LabResult<Vec<DataRun>> {
    let mut runs = Vec::new();
    let mut i = 0usize;
    let mut current_lcn: i64 = 0;
    while i < data.len() {
        let header = data[i];
        i += 1;
        if header == 0 {
            break;
        }
        let len_size = (header & 0x0F) as usize;
        let off_size = ((header >> 4) & 0x0F) as usize;
        if len_size == 0 || i + len_size + off_size > data.len() {
            return Err(LabError::Internal {
                detail: "truncated runlist".into(),
            });
        }
        let vcn_length = read_le_int(&data[i..i + len_size]) as u64;
        i += len_size;
        let lcn = if off_size == 0 {
            None
        } else {
            let delta = read_le_signed(&data[i..i + off_size]);
            i += off_size;
            current_lcn = current_lcn.wrapping_add(delta);
            if current_lcn < 0 {
                return Err(LabError::Internal {
                    detail: "negative LCN in runlist".into(),
                });
            }
            Some(current_lcn as u64)
        };
        runs.push(DataRun { vcn_length, lcn });
    }
    Ok(runs)
}

fn read_le_int(bytes: &[u8]) -> u64 {
    let mut v = 0u64;
    for (i, b) in bytes.iter().enumerate() {
        v |= (*b as u64) << (8 * i);
    }
    v
}

fn read_le_signed(bytes: &[u8]) -> i64 {
    let mut v = read_le_int(bytes) as i64;
    let bits = bytes.len() * 8;
    if bits < 64 {
        let sign_bit = 1i64 << (bits - 1);
        if v & sign_bit != 0 {
            v |= !((1i64 << bits) - 1);
        }
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_simple_run() {
        // length 1 byte = 4 clusters, offset 1 byte = +16
        let runs = decode_runlist(&[0x11, 0x04, 0x10, 0x00]).unwrap();
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].vcn_length, 4);
        assert_eq!(runs[0].lcn, Some(16));
    }
}
