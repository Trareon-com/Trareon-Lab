//! Windows Registry hive parser (REGF).

use lab_core::{LabError, LabResult, ProgressEvent, ProgressSink};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistryBaseBlock {
    pub file_name: String,
    pub timestamp: u64,
    pub major: u32,
    pub minor: u32,
    pub root_cell: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistryValue {
    pub name: String,
    pub data_type: u32,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistryKey {
    pub name: String,
    pub path: String,
    pub values: Vec<RegistryValue>,
    pub subkey_names: Vec<String>,
}

#[derive(Debug)]
pub struct HiveFile {
    pub base_block: RegistryBaseBlock,
    data: Vec<u8>,
}

impl HiveFile {
    pub fn open_bytes(bytes: &[u8]) -> LabResult<Self> {
        if bytes.len() < 0x1000 || &bytes[0..4] != b"regf" {
            return Err(LabError::Internal {
                detail: "not a REGF hive".into(),
            });
        }
        let file_name = utf16le_z(&bytes[0x30..0x30 + 64]);
        let timestamp = u64::from_le_bytes(bytes[0x0C..0x14].try_into().unwrap());
        let major = u32::from_le_bytes(bytes[0x14..0x18].try_into().unwrap());
        let minor = u32::from_le_bytes(bytes[0x18..0x1C].try_into().unwrap());
        let root_cell = u32::from_le_bytes(bytes[0x24..0x28].try_into().unwrap());
        Ok(Self {
            base_block: RegistryBaseBlock {
                file_name,
                timestamp,
                major,
                minor,
                root_cell,
            },
            data: bytes.to_vec(),
        })
    }

    pub fn open_path(path: &std::path::Path) -> LabResult<Self> {
        let bytes = std::fs::read(path).map_err(|e| LabError::Internal {
            detail: format!("read hive: {e}"),
        })?;
        Self::open_bytes(&bytes)
    }

    /// Walk keys under root (depth-limited) with progress.
    pub fn enumerate_keys(
        &self,
        progress: &mut dyn ProgressSink,
        max_keys: usize,
    ) -> LabResult<Vec<RegistryKey>> {
        let mut out = Vec::new();
        let root = self.base_block.root_cell;
        self.walk_key(root, String::new(), &mut out, progress, max_keys, 0)?;
        Ok(out)
    }

    fn walk_key(
        &self,
        cell_offset: u32,
        path: String,
        out: &mut Vec<RegistryKey>,
        progress: &mut dyn ProgressSink,
        max_keys: usize,
        depth: u32,
    ) -> LabResult<()> {
        if out.len() >= max_keys || depth > 32 {
            return Ok(());
        }
        progress.report(ProgressEvent::new(
            "registry.hbin",
            cell_offset as u64,
            Some(self.data.len() as u64),
            path.clone(),
        ));
        let nk = match self.read_nk(cell_offset)? {
            Some(k) => k,
            None => return Ok(()),
        };
        let key_path = if path.is_empty() {
            nk.name.clone()
        } else {
            format!("{path}\\{}", nk.name)
        };
        out.push(RegistryKey {
            name: nk.name.clone(),
            path: key_path.clone(),
            values: nk.values,
            subkey_names: nk.subkeys.iter().map(|(_, n)| n.clone()).collect(),
        });
        for (off, name) in nk.subkeys {
            let child_path = format!("{key_path}\\{name}");
            let _ = name;
            self.walk_key(off, child_path, out, progress, max_keys, depth + 1)?;
        }
        Ok(())
    }

    fn cell_bytes(&self, offset: u32) -> LabResult<&[u8]> {
        // Cell offset is relative to first hbin (0x1000)
        let abs = 0x1000usize + offset as usize;
        if abs + 4 > self.data.len() {
            return Err(LabError::Internal {
                detail: "cell OOB".into(),
            });
        }
        let size =
            i32::from_le_bytes(self.data[abs..abs + 4].try_into().unwrap()).unsigned_abs() as usize;
        if abs + size > self.data.len() {
            return Err(LabError::Internal {
                detail: "cell size OOB".into(),
            });
        }
        Ok(&self.data[abs + 4..abs + size])
    }

    fn read_nk(&self, offset: u32) -> LabResult<Option<NkParsed>> {
        let cell = self.cell_bytes(offset)?;
        if cell.len() < 0x50 || &cell[0..2] != b"nk" {
            return Ok(None);
        }
        let name_len = u16::from_le_bytes(cell[0x48..0x4A].try_into().unwrap()) as usize;
        let name = if 0x50 + name_len <= cell.len() {
            String::from_utf8_lossy(&cell[0x50..0x50 + name_len]).into_owned()
        } else {
            String::new()
        };
        let subkey_count = u32::from_le_bytes(cell[0x14..0x18].try_into().unwrap());
        let subkey_list = u32::from_le_bytes(cell[0x1C..0x20].try_into().unwrap());
        let value_count = u32::from_le_bytes(cell[0x24..0x28].try_into().unwrap());
        let value_list = u32::from_le_bytes(cell[0x28..0x2C].try_into().unwrap());

        let mut subkeys = Vec::new();
        if subkey_count > 0 && subkey_list != 0xFFFF_FFFF {
            if let Ok(list) = self.cell_bytes(subkey_list) {
                if list.len() >= 4 && (&list[0..2] == b"lf" || &list[0..2] == b"lh") {
                    let count = u16::from_le_bytes(list[2..4].try_into().unwrap()) as usize;
                    for i in 0..count {
                        let base = 4 + i * 8;
                        if base + 8 > list.len() {
                            break;
                        }
                        let off = u32::from_le_bytes(list[base..base + 4].try_into().unwrap());
                        subkeys.push((off, format!("sub{i}")));
                    }
                } else if list.len() >= 4 && &list[0..2] == b"li" {
                    let count = u16::from_le_bytes(list[2..4].try_into().unwrap()) as usize;
                    for i in 0..count {
                        let base = 4 + i * 4;
                        if base + 4 > list.len() {
                            break;
                        }
                        let off = u32::from_le_bytes(list[base..base + 4].try_into().unwrap());
                        subkeys.push((off, format!("sub{i}")));
                    }
                }
            }
        }

        let mut values = Vec::new();
        if value_count > 0 && value_list != 0xFFFF_FFFF {
            if let Ok(list) = self.cell_bytes(value_list) {
                for i in 0..value_count as usize {
                    let base = i * 4;
                    if base + 4 > list.len() {
                        break;
                    }
                    let voff = u32::from_le_bytes(list[base..base + 4].try_into().unwrap());
                    if let Ok(Some(v)) = self.read_vk(voff) {
                        values.push(v);
                    }
                }
            }
        }

        Ok(Some(NkParsed {
            name,
            subkeys,
            values,
        }))
    }

    fn read_vk(&self, offset: u32) -> LabResult<Option<RegistryValue>> {
        let cell = self.cell_bytes(offset)?;
        if cell.len() < 20 || &cell[0..2] != b"vk" {
            return Ok(None);
        }
        let name_len = u16::from_le_bytes(cell[2..4].try_into().unwrap()) as usize;
        let data_len = u32::from_le_bytes(cell[4..8].try_into().unwrap());
        let data_off = u32::from_le_bytes(cell[8..12].try_into().unwrap());
        let data_type = u32::from_le_bytes(cell[12..16].try_into().unwrap());
        let name = if name_len == 0 {
            "(default)".into()
        } else if 20 + name_len <= cell.len() {
            String::from_utf8_lossy(&cell[20..20 + name_len]).into_owned()
        } else {
            String::new()
        };
        let data = if data_len & 0x8000_0000 != 0 {
            // resident in data_off
            data_off.to_le_bytes()[..(data_len & 0x7FFF_FFFF).min(4) as usize].to_vec()
        } else if data_len > 0 {
            self.cell_bytes(data_off)
                .map(|c| c[..c.len().min(data_len as usize)].to_vec())
                .unwrap_or_default()
        } else {
            Vec::new()
        };
        Ok(Some(RegistryValue {
            name,
            data_type,
            data,
        }))
    }
}

struct NkParsed {
    name: String,
    subkeys: Vec<(u32, String)>,
    values: Vec<RegistryValue>,
}

fn utf16le_z(bytes: &[u8]) -> String {
    let u16s: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|c| u16::from_le_bytes([c[0], c[1]]))
        .take_while(|u| *u != 0)
        .collect();
    String::from_utf16_lossy(&u16s)
}

/// Build a tiny valid-enough hive for tests.
pub fn write_minimal_hive(path: &std::path::Path) -> LabResult<()> {
    let mut data = vec![0u8; 0x2000];
    data[0..4].copy_from_slice(b"regf");
    data[0x14..0x18].copy_from_slice(&1u32.to_le_bytes());
    data[0x18..0x1C].copy_from_slice(&3u32.to_le_bytes());
    data[0x24..0x28].copy_from_slice(&0x20u32.to_le_bytes()); // root cell at 0x20 in hbin
                                                              // file name
    let name = "SYSTEM\0"
        .encode_utf16()
        .flat_map(|u| u.to_le_bytes())
        .collect::<Vec<_>>();
    data[0x30..0x30 + name.len()].copy_from_slice(&name);
    // hbin header
    data[0x1000..0x1004].copy_from_slice(b"hbin");
    data[0x1004..0x1008].copy_from_slice(&0u32.to_le_bytes());
    data[0x1008..0x100C].copy_from_slice(&0x1000u32.to_le_bytes());
    // nk cell at relative 0x20 → abs 0x1020
    let nk_abs = 0x1020;
    let nk_size: i32 = -0x70;
    data[nk_abs..nk_abs + 4].copy_from_slice(&nk_size.to_le_bytes());
    data[nk_abs + 4..nk_abs + 6].copy_from_slice(b"nk");
    data[nk_abs + 4 + 0x48..nk_abs + 4 + 0x4A].copy_from_slice(&4u16.to_le_bytes());
    data[nk_abs + 4 + 0x50..nk_abs + 4 + 0x54].copy_from_slice(b"ROOT");
    std::fs::write(path, data).map_err(|e| LabError::Internal {
        detail: format!("write hive: {e}"),
    })?;
    Ok(())
}
