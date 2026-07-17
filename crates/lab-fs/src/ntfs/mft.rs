//! Streaming MFT record iterator.

use std::collections::HashMap;

use lab_core::{LabError, LabResult, ProgressEvent, ProgressSink};
use lab_storage::ImageReader;

use crate::ntfs::attribute::{parse_attributes, Attribute, DataAttr};
use crate::ntfs::boot::NtfsVolume;
use crate::ntfs::runlist::DataRun;

/// MFT record flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MftRecordFlags {
    pub in_use: bool,
    pub is_directory: bool,
}

#[derive(Debug, Clone)]
pub struct MftRecord {
    pub record_number: u64,
    pub sequence: u16,
    pub flags: MftRecordFlags,
    pub base_record: Option<u64>,
    pub link_count: u16,
    pub attributes: Vec<Attribute>,
    pub raw: Vec<u8>,
}

/// Lazy MFT walker with hot-record cache.
pub struct MftIterator<'a> {
    image: &'a mut dyn ImageReader,
    volume: NtfsVolume,
    mft_runs: Vec<DataRun>,
    record_size: u32,
    total_records: u64,
    cache: HashMap<u64, MftRecord>,
}

impl<'a> MftIterator<'a> {
    pub fn open(image: &'a mut dyn ImageReader, volume: NtfsVolume) -> LabResult<Self> {
        let record_size = volume.boot.mft_record_size;
        let cluster = volume.boot.cluster_size();
        let mft_offset = volume.boot.mft_cluster.saturating_mul(cluster);
        let mut first = vec![0u8; record_size as usize];
        let n = image.read_at(mft_offset, &mut first)?;
        if n < record_size as usize {
            return Err(LabError::Internal {
                detail: "cannot read $MFT record 0".into(),
            });
        }
        apply_fixup(&mut first)?;
        let attrs = parse_attributes(&first, first_attr_offset(&first)?)?;
        let mft_runs = attrs
            .iter()
            .find_map(|a| match a {
                Attribute::Data(DataAttr {
                    name,
                    resident: false,
                    runs,
                    ..
                }) if name.is_empty() => Some(runs.clone()),
                _ => None,
            })
            .unwrap_or_else(|| {
                vec![DataRun {
                    vcn_length: 16,
                    lcn: Some(volume.boot.mft_cluster),
                }]
            });

        let total_clusters: u64 = mft_runs.iter().map(|r| r.vcn_length).sum();
        let total_bytes = total_clusters.saturating_mul(cluster);
        let total_records = (total_bytes / record_size as u64).max(1);

        Ok(Self {
            image,
            volume,
            mft_runs,
            record_size,
            total_records,
            cache: HashMap::new(),
        })
    }

    pub fn total_records(&self) -> u64 {
        self.total_records
    }

    pub fn read_record(&mut self, index: u64) -> LabResult<Option<MftRecord>> {
        if let Some(r) = self.cache.get(&index) {
            return Ok(Some(r.clone()));
        }
        if index >= self.total_records {
            return Ok(None);
        }
        let offset = self.record_byte_offset(index)?;
        let mut raw = vec![0u8; self.record_size as usize];
        let n = self.image.read_at(offset, &mut raw)?;
        if n < 4 || &raw[0..4] != b"FILE" {
            return Ok(None);
        }
        apply_fixup(&mut raw)?;
        let sequence = u16::from_le_bytes(raw[0x10..0x12].try_into().unwrap());
        let link_count = u16::from_le_bytes(raw[0x12..0x14].try_into().unwrap());
        let flags_raw = u16::from_le_bytes(raw[0x16..0x18].try_into().unwrap());
        let base = u64::from_le_bytes(raw[0x20..0x28].try_into().unwrap());
        let base_record = if base == 0 {
            None
        } else {
            Some(base & 0x0000_FFFF_FFFF)
        };
        let first = first_attr_offset(&raw)?;
        let attributes = parse_attributes(&raw, first).unwrap_or_default();
        let rec = MftRecord {
            record_number: index,
            sequence,
            flags: MftRecordFlags {
                in_use: flags_raw & 0x01 != 0,
                is_directory: flags_raw & 0x02 != 0,
            },
            base_record,
            link_count,
            attributes,
            raw,
        };
        self.cache.insert(index, rec.clone());
        if self.cache.len() > 256 {
            // ponytail: simple hot cache ceiling 256; upgrade to LRU if needed
            self.cache.retain(|k, _| *k == index || *k < 16);
        }
        Ok(Some(rec))
    }

    fn record_byte_offset(&self, index: u64) -> LabResult<u64> {
        let cluster = self.volume.boot.cluster_size();
        let record_size = self.record_size as u64;
        let byte_off = index * record_size;
        let mut remaining = byte_off;
        for run in &self.mft_runs {
            let run_bytes = run.vcn_length.saturating_mul(cluster);
            if remaining < run_bytes {
                let lcn = run.lcn.ok_or_else(|| LabError::Internal {
                    detail: "sparse $MFT run unexpected".into(),
                })?;
                return Ok(lcn.saturating_mul(cluster) + remaining);
            }
            remaining -= run_bytes;
        }
        Err(LabError::Internal {
            detail: format!("MFT record {index} beyond runlist"),
        })
    }

    pub fn for_each_record(
        &mut self,
        progress: &mut dyn ProgressSink,
        mut f: impl FnMut(&MftRecord),
    ) -> LabResult<()> {
        for i in 0..self.total_records {
            if progress.is_cancelled() {
                break;
            }
            if i % 64 == 0 {
                progress.report(ProgressEvent::new(
                    "ntfs.mft",
                    i,
                    Some(self.total_records),
                    format!("MFT record {i}"),
                ));
            }
            if let Some(rec) = self.read_record(i)? {
                f(&rec);
            }
        }
        Ok(())
    }
}

fn first_attr_offset(record: &[u8]) -> LabResult<usize> {
    if record.len() < 0x16 {
        return Err(LabError::Internal {
            detail: "mft record too short".into(),
        });
    }
    Ok(u16::from_le_bytes(record[0x14..0x16].try_into().unwrap()) as usize)
}

fn apply_fixup(record: &mut [u8]) -> LabResult<()> {
    if record.len() < 8 {
        return Ok(());
    }
    let usa_off = u16::from_le_bytes(record[4..6].try_into().unwrap()) as usize;
    let usa_count = u16::from_le_bytes(record[6..8].try_into().unwrap()) as usize;
    if usa_off == 0 || usa_count < 2 || usa_off + usa_count * 2 > record.len() {
        return Ok(());
    }
    let sector_size = 512usize;
    for i in 1..usa_count {
        let sector_end = i * sector_size;
        if sector_end > record.len() || sector_end < 2 {
            break;
        }
        let fix = u16::from_le_bytes(
            record[usa_off + i * 2..usa_off + i * 2 + 2]
                .try_into()
                .unwrap(),
        );
        let pos = sector_end - 2;
        record[pos] = (fix & 0xFF) as u8;
        record[pos + 1] = (fix >> 8) as u8;
    }
    Ok(())
}
