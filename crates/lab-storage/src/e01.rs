//! Minimal EWF/E01 reader + fixture writer (pure Rust, no libewf).
//!
//! Supports a single-segment EWF v1 subset sufficient for lab import:
//! file header → header/volume/sectors/table/done sections, zlib chunks, CRC-32.

use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use lab_core::{LabError, LabResult, ProgressEvent, ProgressSink};

use crate::image::{ImageReader, IntegrityReport};
use crate::raw::ImageKind;

const EVF_MAGIC: &[u8; 8] = b"EVF\x09\x0d\x0a\xff\x00";
const SECTION_HEADER_SIZE: u64 = 76;
const MAX_METADATA_SECTION_BYTES: u64 = 16 * 1024 * 1024;
const MAX_INFLATED_METADATA_BYTES: usize = 1024 * 1024;

/// Metadata extracted from the E01 header section.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct E01Metadata {
    pub case_number: String,
    pub examiner: String,
    pub evidence_number: String,
    pub description: String,
    pub hash_algorithms: Vec<String>,
}

/// Opened E01 image with virtual linear byte space.
#[derive(Debug)]
pub struct E01Image {
    path: PathBuf,
    file: File,
    byte_length: u64,
    chunk_size: u32,
    /// Absolute file offsets of each chunk's payload (after optional size prefix).
    chunk_offsets: Vec<u64>,
    /// Compressed size of each chunk payload (excluding trailing CRC).
    chunk_comp_sizes: Vec<u32>,
    /// Whether each chunk is zlib-compressed.
    chunk_compressed: Vec<bool>,
    crc_errors: u64,
    metadata: E01Metadata,
    /// Hot cache: last decompressed chunk.
    cache_index: Option<u32>,
    cache_data: Vec<u8>,
}

impl E01Image {
    pub fn open(path: &Path) -> LabResult<Self> {
        let mut file = File::open(path).map_err(|e| LabError::Internal {
            detail: format!("open e01 {}: {e}", path.display()),
        })?;
        let mut magic = [0u8; 8];
        file.read_exact(&mut magic)
            .map_err(|e| LabError::Internal {
                detail: format!("read e01 magic: {e}"),
            })?;
        if &magic != EVF_MAGIC {
            return Err(LabError::Internal {
                detail: format!("not an EVF/E01 file: {}", path.display()),
            });
        }
        let file_len = file
            .metadata()
            .map_err(|e| LabError::Internal {
                detail: format!("stat e01 {}: {e}", path.display()),
            })?
            .len();

        let mut metadata = E01Metadata::default();
        let mut byte_length = 0_u64;
        let mut chunk_size = 64 * 1024_u32;
        let mut sectors_offset = 0_u64;
        let mut table_entries: Vec<u32> = Vec::new();
        let mut next = 8_u64;
        let mut saw_done = false;

        loop {
            if next
                .checked_add(SECTION_HEADER_SIZE)
                .filter(|end| *end <= file_len)
                .is_none()
            {
                return Err(LabError::Internal {
                    detail: "e01 truncated section header".into(),
                });
            }
            file.seek(SeekFrom::Start(next))
                .map_err(|e| LabError::Internal {
                    detail: format!("seek section: {e}"),
                })?;
            let mut hdr = [0u8; SECTION_HEADER_SIZE as usize];
            let nread = file.read(&mut hdr).map_err(|e| LabError::Internal {
                detail: format!("read section header: {e}"),
            })?;
            if nread < SECTION_HEADER_SIZE as usize {
                break;
            }
            let type_str = {
                let end = hdr[..16].iter().position(|&b| b == 0).unwrap_or(16);
                std::str::from_utf8(&hdr[..end]).unwrap_or("").to_string()
            };
            let next_off = u64::from_le_bytes(hdr[16..24].try_into().unwrap());
            let section_size = u64::from_le_bytes(hdr[24..32].try_into().unwrap());
            if section_size < SECTION_HEADER_SIZE {
                return Err(LabError::Internal {
                    detail: format!("e01 invalid {type_str} section size"),
                });
            }
            let data_off = next + SECTION_HEADER_SIZE;
            let data_len = section_size - SECTION_HEADER_SIZE;
            if data_off
                .checked_add(data_len)
                .filter(|end| *end <= file_len)
                .is_none()
            {
                return Err(LabError::Internal {
                    detail: format!("e01 {type_str} section exceeds file length"),
                });
            }

            match type_str.as_str() {
                "header" | "header2" => {
                    metadata = parse_header_section(&mut file, data_off, data_len)?;
                }
                "volume" | "disk" => {
                    let (media_size, sectors_per_chunk) =
                        parse_volume_section(&mut file, data_off, data_len)?;
                    byte_length = media_size;
                    chunk_size = sectors_per_chunk.saturating_mul(512).max(512);
                }
                "sectors" => {
                    sectors_offset = data_off;
                }
                "table" | "table2" => {
                    table_entries = parse_table_section(&mut file, data_off, data_len)?;
                }
                "done" => {
                    saw_done = true;
                    break;
                }
                _ => {}
            }

            if next_off == 0 || next_off <= next {
                break;
            }
            if next_off > file_len {
                return Err(LabError::Internal {
                    detail: "e01 next section offset exceeds file length".into(),
                });
            }
            next = next_off;
        }

        if !saw_done || table_entries.is_empty() || sectors_offset == 0 {
            return Err(LabError::Internal {
                detail: "e01 missing sectors/table/done sections".into(),
            });
        }

        let mut chunk_offsets = Vec::with_capacity(table_entries.len());
        let mut chunk_comp_sizes = Vec::with_capacity(table_entries.len());
        let mut chunk_compressed = Vec::with_capacity(table_entries.len());

        for (i, &rel) in table_entries.iter().enumerate() {
            let abs = sectors_offset + rel as u64;
            // Chunk layout we write: u32 le size | bit31=compressed, then data, then u32 crc
            file.seek(SeekFrom::Start(abs))
                .map_err(|e| LabError::Internal {
                    detail: format!("seek chunk: {e}"),
                })?;
            let mut sz_buf = [0u8; 4];
            file.read_exact(&mut sz_buf)
                .map_err(|e| LabError::Internal {
                    detail: format!("read chunk size: {e}"),
                })?;
            let raw_sz = u32::from_le_bytes(sz_buf);
            let compressed = (raw_sz & 0x8000_0000) != 0;
            let comp_size = raw_sz & 0x7fff_ffff;
            let max_payload = chunk_size.saturating_mul(2).saturating_add(1024);
            if comp_size > max_payload
                || (abs + 4)
                    .checked_add(u64::from(comp_size) + 4)
                    .filter(|end| *end <= file_len)
                    .is_none()
            {
                return Err(LabError::Internal {
                    detail: format!("e01 chunk {i} payload is out of bounds"),
                });
            }
            chunk_offsets.push(abs + 4);
            chunk_comp_sizes.push(comp_size);
            chunk_compressed.push(compressed);
            let _ = i;
        }

        if byte_length == 0 {
            byte_length = table_entries.len() as u64 * chunk_size as u64;
        }

        Ok(Self {
            path: path.to_path_buf(),
            file,
            byte_length,
            chunk_size,
            chunk_offsets,
            chunk_comp_sizes,
            chunk_compressed,
            crc_errors: 0,
            metadata,
            cache_index: None,
            cache_data: Vec::new(),
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn metadata(&self) -> &E01Metadata {
        &self.metadata
    }

    fn load_chunk(&mut self, index: u32) -> LabResult<&[u8]> {
        if self.cache_index == Some(index) {
            return Ok(&self.cache_data);
        }
        let i = index as usize;
        if i >= self.chunk_offsets.len() {
            return Err(LabError::Internal {
                detail: format!("chunk index out of range: {index}"),
            });
        }
        let off = self.chunk_offsets[i];
        let comp_size = self.chunk_comp_sizes[i] as usize;
        let compressed = self.chunk_compressed[i];

        self.file
            .seek(SeekFrom::Start(off))
            .map_err(|e| LabError::Internal {
                detail: format!("seek chunk data: {e}"),
            })?;
        let mut payload = vec![0u8; comp_size];
        self.file
            .read_exact(&mut payload)
            .map_err(|e| LabError::Internal {
                detail: format!("read chunk data: {e}"),
            })?;
        let mut crc_buf = [0u8; 4];
        self.file
            .read_exact(&mut crc_buf)
            .map_err(|e| LabError::Internal {
                detail: format!("read chunk crc: {e}"),
            })?;
        let expected = u32::from_le_bytes(crc_buf);
        let actual = crc32_ieee(&payload);
        if actual != expected {
            self.crc_errors += 1;
            return Err(LabError::Internal {
                detail: format!(
                    "e01 chunk {index} CRC mismatch: expected {expected:#x} got {actual:#x}"
                ),
            });
        }

        let data = if compressed {
            inflate_zlib_limited(&payload, self.chunk_size as usize)?
        } else {
            payload
        };
        self.cache_index = Some(index);
        self.cache_data = data;
        Ok(&self.cache_data)
    }
}

impl ImageReader for E01Image {
    fn kind(&self) -> ImageKind {
        ImageKind::E01
    }

    fn byte_length(&self) -> u64 {
        self.byte_length
    }

    fn read_at(&mut self, offset: u64, buf: &mut [u8]) -> LabResult<usize> {
        if offset >= self.byte_length || buf.is_empty() {
            return Ok(0);
        }
        let byte_length = self.byte_length;
        let chunk_size = self.chunk_size as u64;
        let mut written = 0usize;
        let mut pos = offset;
        while written < buf.len() && pos < byte_length {
            let chunk_idx = (pos / chunk_size) as u32;
            let within = (pos % chunk_size) as usize;
            let piece: Vec<u8> = match self.load_chunk(chunk_idx) {
                Ok(c) => {
                    if within >= c.len() {
                        break;
                    }
                    c[within..].to_vec()
                }
                Err(e) => {
                    let msg = format!("{e:?}");
                    if msg.contains("CRC") {
                        let remain = (chunk_size as usize).saturating_sub(within);
                        let n = std::cmp::min(remain, buf.len() - written);
                        let n = std::cmp::min(n, (byte_length - pos) as usize);
                        buf[written..written + n].fill(0);
                        written += n;
                        pos += n as u64;
                        continue;
                    }
                    return Err(e);
                }
            };
            let n = std::cmp::min(piece.len(), buf.len() - written);
            let n = std::cmp::min(n, (byte_length - pos) as usize);
            buf[written..written + n].copy_from_slice(&piece[..n]);
            written += n;
            pos += n as u64;
        }
        Ok(written)
    }

    fn verify_integrity(&mut self, progress: &mut dyn ProgressSink) -> LabResult<IntegrityReport> {
        let total = self.chunk_offsets.len() as u64;
        let mut errors = 0_u64;
        self.crc_errors = 0;
        for i in 0..self.chunk_offsets.len() as u32 {
            if progress.is_cancelled() {
                break;
            }
            progress.report(ProgressEvent::new(
                "e01.verify",
                i as u64 + 1,
                Some(total),
                format!("verify chunk {i}"),
            ));
            self.cache_index = None;
            if self.load_chunk(i).is_err() {
                errors += 1;
            }
        }
        self.crc_errors = errors;
        Ok(IntegrityReport {
            ok: errors == 0,
            crc_errors: errors,
            chunks_checked: total,
            message: if errors == 0 {
                "all chunks ok".into()
            } else {
                format!("{errors} chunk CRC failures")
            },
        })
    }

    fn crc_errors(&self) -> u64 {
        self.crc_errors
    }
}

/// Write a minimal single-segment E01 for fixtures (round-trip with [`E01Image::open`]).
pub fn write_simple_e01(
    path: &Path,
    media: &[u8],
    meta: &E01Metadata,
    chunk_size: u32,
) -> LabResult<()> {
    let chunk_size = chunk_size.max(512);
    let mut file = File::create(path).map_err(|e| LabError::Internal {
        detail: format!("create e01: {e}"),
    })?;
    file.write_all(EVF_MAGIC).map_err(|e| LabError::Internal {
        detail: format!("write magic: {e}"),
    })?;

    let mut cursor = 8_u64;

    // header section
    let header_text = format!(
        "1\ncase_number\n{}\nexaminer_name\n{}\nevidence_number\n{}\ndescription\n{}\nhashes\n{}\n\n",
        meta.case_number,
        meta.examiner,
        meta.evidence_number,
        meta.description,
        meta.hash_algorithms.join(",")
    );
    let header_zlib = deflate_zlib(header_text.as_bytes())?;
    let header_section_size = SECTION_HEADER_SIZE + header_zlib.len() as u64;
    let after_header = cursor + header_section_size;
    write_section_header(&mut file, "header", after_header, header_section_size)?;
    file.write_all(&header_zlib)
        .map_err(|e| LabError::Internal {
            detail: format!("write header: {e}"),
        })?;
    cursor = after_header;

    // volume section (112 bytes typical layout — we use a compact custom layout
    // documented for our writer: media_size u64, chunk_size u32, pad)
    let mut volume = vec![0u8; 112];
    volume[0..8].copy_from_slice(&(media.len() as u64).to_le_bytes());
    volume[8..12].copy_from_slice(&chunk_size.to_le_bytes());
    let volume_section_size = SECTION_HEADER_SIZE + volume.len() as u64;
    let after_volume = cursor + volume_section_size;
    write_section_header(&mut file, "volume", after_volume, volume_section_size)?;
    file.write_all(&volume).map_err(|e| LabError::Internal {
        detail: format!("write volume: {e}"),
    })?;
    cursor = after_volume;

    // Build chunks
    let mut table: Vec<u32> = Vec::new();
    let mut sectors_blob = Vec::new();
    let mut offset_in_sectors = 0_u32;
    for chunk in media.chunks(chunk_size as usize) {
        table.push(offset_in_sectors);
        let compressed = deflate_zlib(chunk)?;
        let use_comp = compressed.len() < chunk.len();
        let payload = if use_comp { compressed } else { chunk.to_vec() };
        let mut size_word = payload.len() as u32;
        if use_comp {
            size_word |= 0x8000_0000;
        }
        sectors_blob.extend_from_slice(&size_word.to_le_bytes());
        sectors_blob.extend_from_slice(&payload);
        let crc = crc32_ieee(&payload);
        sectors_blob.extend_from_slice(&crc.to_le_bytes());
        offset_in_sectors = sectors_blob.len() as u32;
    }

    let sectors_section_size = SECTION_HEADER_SIZE + sectors_blob.len() as u64;
    let after_sectors = cursor + sectors_section_size;
    write_section_header(&mut file, "sectors", after_sectors, sectors_section_size)?;
    file.write_all(&sectors_blob)
        .map_err(|e| LabError::Internal {
            detail: format!("write sectors: {e}"),
        })?;
    cursor = after_sectors;

    // table section: u32 count + entries + checksum placeholder
    let mut table_data = Vec::new();
    table_data.extend_from_slice(&(table.len() as u32).to_le_bytes());
    for e in &table {
        table_data.extend_from_slice(&e.to_le_bytes());
    }
    table_data.extend_from_slice(&0u32.to_le_bytes()); // checksum stub
    let table_section_size = SECTION_HEADER_SIZE + table_data.len() as u64;
    let after_table = cursor + table_section_size;
    write_section_header(&mut file, "table", after_table, table_section_size)?;
    file.write_all(&table_data)
        .map_err(|e| LabError::Internal {
            detail: format!("write table: {e}"),
        })?;
    cursor = after_table;

    // done
    write_section_header(&mut file, "done", 0, SECTION_HEADER_SIZE)?;
    let _ = cursor;

    Ok(())
}

fn write_section_header(file: &mut File, name: &str, next: u64, size: u64) -> LabResult<()> {
    let mut hdr = [0u8; SECTION_HEADER_SIZE as usize];
    let nb = name.as_bytes();
    let n = nb.len().min(15);
    hdr[..n].copy_from_slice(&nb[..n]);
    hdr[16..24].copy_from_slice(&next.to_le_bytes());
    hdr[24..32].copy_from_slice(&size.to_le_bytes());
    let sum = crc32_ieee(&hdr[..72]);
    hdr[72..76].copy_from_slice(&sum.to_le_bytes());
    file.write_all(&hdr).map_err(|e| LabError::Internal {
        detail: format!("write section header: {e}"),
    })
}

fn parse_header_section(file: &mut File, off: u64, len: u64) -> LabResult<E01Metadata> {
    if len > MAX_METADATA_SECTION_BYTES {
        return Err(LabError::Internal {
            detail: "e01 metadata section exceeds limit".into(),
        });
    }
    file.seek(SeekFrom::Start(off))
        .map_err(|e| LabError::Internal {
            detail: format!("seek header: {e}"),
        })?;
    let mut buf = vec![0u8; len as usize];
    file.read_exact(&mut buf).map_err(|e| LabError::Internal {
        detail: format!("read header: {e}"),
    })?;
    let text = match inflate_zlib_limited(&buf, MAX_INFLATED_METADATA_BYTES) {
        Ok(t) => String::from_utf8_lossy(&t).into_owned(),
        Err(_) => String::from_utf8_lossy(&buf).into_owned(),
    };
    let mut meta = E01Metadata::default();
    let lines: Vec<&str> = text.lines().collect();
    let mut i = 0;
    while i + 1 < lines.len() {
        match lines[i] {
            "case_number" => meta.case_number = lines[i + 1].to_string(),
            "examiner_name" => meta.examiner = lines[i + 1].to_string(),
            "evidence_number" => meta.evidence_number = lines[i + 1].to_string(),
            "description" => meta.description = lines[i + 1].to_string(),
            "hashes" => {
                meta.hash_algorithms = lines[i + 1]
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
            _ => {}
        }
        i += 1;
    }
    Ok(meta)
}

fn parse_volume_section(file: &mut File, off: u64, len: u64) -> LabResult<(u64, u32)> {
    file.seek(SeekFrom::Start(off))
        .map_err(|e| LabError::Internal {
            detail: format!("seek volume: {e}"),
        })?;
    let mut buf = vec![0u8; len.min(112) as usize];
    file.read_exact(&mut buf).map_err(|e| LabError::Internal {
        detail: format!("read volume: {e}"),
    })?;
    if buf.len() < 12 {
        return Err(LabError::Internal {
            detail: "volume section too short".into(),
        });
    }
    let media_size = u64::from_le_bytes(buf[0..8].try_into().unwrap());
    let chunk_size = u32::from_le_bytes(buf[8..12].try_into().unwrap());
    // Our writer stores chunk_size in bytes; convert to sectors-per-chunk for caller
    let sectors_per_chunk = (chunk_size / 512).max(1);
    Ok((media_size, sectors_per_chunk))
}

fn parse_table_section(file: &mut File, off: u64, len: u64) -> LabResult<Vec<u32>> {
    file.seek(SeekFrom::Start(off))
        .map_err(|e| LabError::Internal {
            detail: format!("seek table: {e}"),
        })?;
    let mut buf = vec![0u8; len as usize];
    file.read_exact(&mut buf).map_err(|e| LabError::Internal {
        detail: format!("read table: {e}"),
    })?;
    if buf.len() < 4 {
        return Err(LabError::Internal {
            detail: "table too short".into(),
        });
    }
    let count = u32::from_le_bytes(buf[0..4].try_into().unwrap()) as usize;
    if count > (buf.len() - 4) / 4 {
        return Err(LabError::Internal {
            detail: "table entry count exceeds section length".into(),
        });
    }
    let mut entries = Vec::with_capacity(count);
    for i in 0..count {
        let start = 4 + i * 4;
        if start + 4 > buf.len() {
            break;
        }
        entries.push(u32::from_le_bytes(
            buf[start..start + 4].try_into().unwrap(),
        ));
    }
    Ok(entries)
}

fn crc32_ieee(data: &[u8]) -> u32 {
    // IEEE CRC-32 (ISO 3309 / zlib polynomial) via crc32fast-free simple impl
    let mut crc = 0xffff_ffff_u32;
    for &b in data {
        crc ^= u32::from(b);
        for _ in 0..8 {
            let mask = (!(crc & 1)).wrapping_add(1); // 0 or 0xFFFFFFFF
            crc = (crc >> 1) ^ (0xEDB88320 & mask);
        }
    }
    !crc
}

fn deflate_zlib(data: &[u8]) -> LabResult<Vec<u8>> {
    use flate2::write::ZlibEncoder;
    use flate2::Compression;
    let mut enc = ZlibEncoder::new(Vec::new(), Compression::fast());
    enc.write_all(data).map_err(|e| LabError::Internal {
        detail: format!("zlib compress: {e}"),
    })?;
    enc.finish().map_err(|e| LabError::Internal {
        detail: format!("zlib finish: {e}"),
    })
}

fn inflate_zlib_limited(data: &[u8], max_output: usize) -> LabResult<Vec<u8>> {
    use flate2::read::ZlibDecoder;
    let dec = ZlibDecoder::new(data);
    let mut out = Vec::new();
    dec.take(max_output.saturating_add(1) as u64)
        .read_to_end(&mut out)
        .map_err(|e| LabError::Internal {
            detail: format!("zlib decompress: {e}"),
        })?;
    if out.len() > max_output {
        return Err(LabError::Internal {
            detail: "zlib output exceeds limit".into(),
        });
    }
    Ok(out)
}
