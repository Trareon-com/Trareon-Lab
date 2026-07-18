//! NTFS attribute parsers.

use crate::ntfs::runlist::{decode_runlist, DataRun};
use lab_core::{LabError, LabResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttributeType {
    StandardInformation,
    AttributeList,
    FileName,
    Data,
    IndexRoot,
    IndexAllocation,
    Bitmap,
    End,
    Other(u32),
}

impl AttributeType {
    pub fn from_u32(v: u32) -> Self {
        match v {
            0x10 => Self::StandardInformation,
            0x20 => Self::AttributeList,
            0x30 => Self::FileName,
            0x80 => Self::Data,
            0x90 => Self::IndexRoot,
            0xA0 => Self::IndexAllocation,
            0xB0 => Self::Bitmap,
            0xFFFF_FFFF => Self::End,
            other => Self::Other(other),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Timestamps {
    pub created: u64,
    pub modified: u64,
    pub mft_modified: u64,
    pub accessed: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StdInfo {
    pub timestamps: Timestamps,
    pub file_attributes: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileNameAttr {
    pub parent_record: u64,
    pub parent_sequence: u16,
    pub timestamps: Timestamps,
    pub allocated_size: u64,
    pub real_size: u64,
    pub flags: u32,
    pub name_space: u8,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataAttr {
    pub name: String,
    pub resident: bool,
    pub resident_data: Vec<u8>,
    pub runs: Vec<DataRun>,
    pub data_size: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Attribute {
    StandardInformation(StdInfo),
    FileName(FileNameAttr),
    Data(DataAttr),
    IndexRoot(Vec<u8>),
    IndexAllocation(Vec<DataRun>),
    Bitmap(Vec<u8>),
    Raw {
        ty: u32,
        resident: bool,
        data: Vec<u8>,
        runs: Vec<DataRun>,
    },
}

/// Parse all attributes from an MFT record body (after USA fixup applied).
pub fn parse_attributes(record: &[u8], first_attr_offset: usize) -> LabResult<Vec<Attribute>> {
    let mut attrs = Vec::new();
    let mut off = first_attr_offset;
    while off + 8 <= record.len() {
        let ty = u32::from_le_bytes(record[off..off + 4].try_into().unwrap());
        if ty == 0xFFFF_FFFF {
            break;
        }
        let len = u32::from_le_bytes(record[off + 4..off + 8].try_into().unwrap()) as usize;
        if len < 16 || off + len > record.len() {
            return Err(LabError::Internal {
                detail: format!("bad attribute length {len} at {off}"),
            });
        }
        let slice = &record[off..off + len];
        let non_resident = slice[8] != 0;
        let name_len = slice[9] as usize;
        let name_off = u16::from_le_bytes(slice[10..12].try_into().unwrap()) as usize;
        let name = if name_len > 0 && name_off + name_len * 2 <= slice.len() {
            utf16le_to_string(&slice[name_off..name_off + name_len * 2])
        } else {
            String::new()
        };

        let attr = match AttributeType::from_u32(ty) {
            AttributeType::StandardInformation if !non_resident => {
                Attribute::StandardInformation(parse_stdinfo(resident_content(slice)?)?)
            }
            AttributeType::FileName if !non_resident => {
                Attribute::FileName(parse_filename(resident_content(slice)?)?)
            }
            AttributeType::Data => {
                if non_resident {
                    let (runs, data_size) = parse_nonresident(slice)?;
                    Attribute::Data(DataAttr {
                        name,
                        resident: false,
                        resident_data: Vec::new(),
                        runs,
                        data_size,
                    })
                } else {
                    let data = resident_content(slice)?.to_vec();
                    let data_size = data.len() as u64;
                    Attribute::Data(DataAttr {
                        name,
                        resident: true,
                        resident_data: data,
                        runs: Vec::new(),
                        data_size,
                    })
                }
            }
            AttributeType::IndexRoot if !non_resident => {
                Attribute::IndexRoot(resident_content(slice)?.to_vec())
            }
            AttributeType::IndexAllocation if non_resident => {
                let (runs, _) = parse_nonresident(slice)?;
                Attribute::IndexAllocation(runs)
            }
            AttributeType::Bitmap if !non_resident => {
                Attribute::Bitmap(resident_content(slice)?.to_vec())
            }
            _ => {
                if non_resident {
                    let (runs, _) = parse_nonresident(slice)?;
                    Attribute::Raw {
                        ty,
                        resident: false,
                        data: Vec::new(),
                        runs,
                    }
                } else {
                    Attribute::Raw {
                        ty,
                        resident: true,
                        data: resident_content(slice)?.to_vec(),
                        runs: Vec::new(),
                    }
                }
            }
        };
        attrs.push(attr);
        off += len;
    }
    Ok(attrs)
}

fn resident_content(attr: &[u8]) -> LabResult<&[u8]> {
    if attr.len() < 24 {
        return Err(LabError::Internal {
            detail: "resident attr too short".into(),
        });
    }
    let content_len = u32::from_le_bytes(attr[16..20].try_into().unwrap()) as usize;
    let content_off = u16::from_le_bytes(attr[20..22].try_into().unwrap()) as usize;
    if content_off + content_len > attr.len() {
        return Err(LabError::Internal {
            detail: "resident content OOB".into(),
        });
    }
    Ok(&attr[content_off..content_off + content_len])
}

fn parse_nonresident(attr: &[u8]) -> LabResult<(Vec<DataRun>, u64)> {
    if attr.len() < 64 {
        return Err(LabError::Internal {
            detail: "non-resident attr too short".into(),
        });
    }
    let data_size = u64::from_le_bytes(attr[48..56].try_into().unwrap());
    let runlist_off = u16::from_le_bytes(attr[32..34].try_into().unwrap()) as usize;
    if runlist_off >= attr.len() {
        return Err(LabError::Internal {
            detail: "runlist offset OOB".into(),
        });
    }
    let runs = decode_runlist(&attr[runlist_off..])?;
    Ok((runs, data_size))
}

fn parse_stdinfo(data: &[u8]) -> LabResult<StdInfo> {
    if data.len() < 48 {
        return Err(LabError::Internal {
            detail: "stdinfo too short".into(),
        });
    }
    Ok(StdInfo {
        timestamps: Timestamps {
            created: u64::from_le_bytes(data[0..8].try_into().unwrap()),
            modified: u64::from_le_bytes(data[8..16].try_into().unwrap()),
            mft_modified: u64::from_le_bytes(data[16..24].try_into().unwrap()),
            accessed: u64::from_le_bytes(data[24..32].try_into().unwrap()),
        },
        file_attributes: u32::from_le_bytes(data[32..36].try_into().unwrap()),
    })
}

fn parse_filename(data: &[u8]) -> LabResult<FileNameAttr> {
    if data.len() < 66 {
        return Err(LabError::Internal {
            detail: "filename attr too short".into(),
        });
    }
    let parent = u64::from_le_bytes(data[0..8].try_into().unwrap());
    let parent_record = parent & 0x0000_FFFF_FFFF;
    let parent_sequence = (parent >> 48) as u16;
    let name_len = data[64] as usize;
    let name_space = data[65];
    let name_bytes = 66 + name_len * 2;
    if name_bytes > data.len() {
        return Err(LabError::Internal {
            detail: "filename name OOB".into(),
        });
    }
    Ok(FileNameAttr {
        parent_record,
        parent_sequence,
        timestamps: Timestamps {
            created: u64::from_le_bytes(data[8..16].try_into().unwrap()),
            modified: u64::from_le_bytes(data[16..24].try_into().unwrap()),
            mft_modified: u64::from_le_bytes(data[24..32].try_into().unwrap()),
            accessed: u64::from_le_bytes(data[32..40].try_into().unwrap()),
        },
        allocated_size: u64::from_le_bytes(data[40..48].try_into().unwrap()),
        real_size: u64::from_le_bytes(data[48..56].try_into().unwrap()),
        flags: u32::from_le_bytes(data[56..60].try_into().unwrap()),
        name_space,
        name: utf16le_to_string(&data[66..name_bytes]),
    })
}

fn utf16le_to_string(bytes: &[u8]) -> String {
    let u16s: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|c| u16::from_le_bytes([c[0], c[1]]))
        .collect();
    String::from_utf16_lossy(&u16s)
}
