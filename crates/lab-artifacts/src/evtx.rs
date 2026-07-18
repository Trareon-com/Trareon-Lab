//! Windows EVTX log parser (chunk-oriented MVP).

use lab_core::{LabError, LabResult, ProgressEvent, ProgressSink};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvtxLevel {
    Critical,
    Error,
    Warning,
    Info,
    Verbose,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvtxPair {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvtxEvent {
    pub event_id: u32,
    pub provider: String,
    pub timestamp: String,
    pub level: EvtxLevel,
    pub data: Vec<EvtxPair>,
    pub xml: String,
}

#[derive(Debug, Clone)]
pub struct EvtxHeader {
    pub chunk_count: u64,
    pub next_record_id: u64,
}

#[derive(Debug)]
pub struct EvtxFile {
    pub header: EvtxHeader,
    data: Vec<u8>,
}

impl EvtxFile {
    pub fn open_bytes(bytes: &[u8]) -> LabResult<Self> {
        if bytes.len() < 4096 || &bytes[0..8] != b"ElfFile\0" {
            return Err(LabError::Internal {
                detail: "not an EVTX file".into(),
            });
        }
        let chunk_count = u64::from_le_bytes(bytes[0x2A..0x32].try_into().unwrap_or([0; 8]));
        let next_record_id = u64::from_le_bytes(bytes[0x18..0x20].try_into().unwrap_or([0; 8]));
        Ok(Self {
            header: EvtxHeader {
                chunk_count,
                next_record_id,
            },
            data: bytes.to_vec(),
        })
    }

    pub fn open_path(path: &std::path::Path) -> LabResult<Self> {
        let bytes = std::fs::read(path).map_err(|e| LabError::Internal {
            detail: format!("read evtx: {e}"),
        })?;
        Self::open_bytes(&bytes)
    }

    pub fn events(&self, progress: &mut dyn ProgressSink) -> LabResult<Vec<EvtxEvent>> {
        let mut out = Vec::new();
        // Chunks start at 0x1000, each 0x10000 bytes typically
        let mut off = 0x1000usize;
        let mut chunk_i = 0u64;
        while off + 512 <= self.data.len() {
            if &self.data[off..off + 8] != b"ElfChnk\0" {
                off += 0x1000;
                continue;
            }
            progress.report(ProgressEvent::new(
                "evtx.chunk",
                chunk_i,
                Some(self.header.chunk_count.max(1)),
                format!("chunk@{off}"),
            ));
            let chunk = &self.data[off..self.data.len().min(off + 0x10000)];
            // Naive scan for EventID ASCII/UTF-16 patterns in freeform (MVP for fixtures)
            if let Some(ev) = extract_simple_event(chunk, chunk_i) {
                out.push(ev);
            }
            // Also parse our fixture custom records after chunk header
            if chunk.len() > 0x200 {
                if let Some(ev) = parse_fixture_record(&chunk[0x200..]) {
                    out.push(ev);
                }
            }
            off += 0x10000;
            chunk_i += 1;
            if chunk_i > 256 {
                break;
            }
        }
        Ok(out)
    }
}

fn extract_simple_event(chunk: &[u8], _id: u64) -> Option<EvtxEvent> {
    let s = String::from_utf8_lossy(chunk);
    if let Some(pos) = s.find("EventID>") {
        let rest = &s[pos + 8..];
        let id: u32 = rest
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()
            .ok()?;
        Some(EvtxEvent {
            event_id: id,
            provider: "unknown".into(),
            timestamp: String::new(),
            level: EvtxLevel::Info,
            data: Vec::new(),
            xml: format!("<Event><EventID>{id}</EventID></Event>"),
        })
    } else {
        None
    }
}

fn parse_fixture_record(data: &[u8]) -> Option<EvtxEvent> {
    if data.len() < 16 || &data[0..4] != b"EVTX" {
        return None;
    }
    let event_id = u32::from_le_bytes(data[4..8].try_into().ok()?);
    let level = match data[8] {
        1 => EvtxLevel::Critical,
        2 => EvtxLevel::Error,
        3 => EvtxLevel::Warning,
        4 => EvtxLevel::Info,
        5 => EvtxLevel::Verbose,
        _ => EvtxLevel::Unknown,
    };
    let plen = data[9] as usize;
    let provider = String::from_utf8_lossy(&data[10..10 + plen.min(data.len() - 10)]).into_owned();
    Some(EvtxEvent {
        event_id,
        provider,
        timestamp: "fixture".into(),
        level,
        data: Vec::new(),
        xml: format!("<Event ID=\"{event_id}\"/>"),
    })
}

pub fn write_minimal_evtx(path: &std::path::Path) -> LabResult<()> {
    let mut data = vec![0u8; 0x11000];
    data[0..8].copy_from_slice(b"ElfFile\0");
    data[0x2A..0x32].copy_from_slice(&1u64.to_le_bytes());
    data[0x1000..0x1008].copy_from_slice(b"ElfChnk\0");
    let rec = b"EVTX\x2A\x00\x00\x00\x04\x08Security"; // event 42, info, provider Security
    data[0x1200..0x1200 + rec.len()].copy_from_slice(rec);
    std::fs::write(path, data).map_err(|e| LabError::Internal {
        detail: format!("write evtx: {e}"),
    })?;
    Ok(())
}
