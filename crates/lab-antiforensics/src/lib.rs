//! Anti-forensics detectors (timestomping, USN, MFT anomalies).

use lab_core::{LabResult, ProgressEvent, ProgressSink};
use lab_fs::{Attribute, FileNameAttr, MftRecord, StdInfo, UsnRecord};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Timestamps {
    pub created: u64,
    pub modified: u64,
    pub accessed: u64,
    pub mft_modified: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimestampDiscrepancy {
    None,
    Modified,
    Timestomped,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimestompHit {
    pub record_number: u64,
    pub file_path: String,
    pub si_timestamps: Timestamps,
    pub fn_timestamps: Timestamps,
    pub discrepancy: TimestampDiscrepancy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsnAnomaly {
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MftAnomaly {
    pub record_number: u64,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AntiForensicsResult {
    pub timestomping_hits: Vec<TimestompHit>,
    pub usn_anomalies: Vec<UsnAnomaly>,
    pub mft_anomalies: Vec<MftAnomaly>,
}

pub fn detect_timestomping(records: &[MftRecord]) -> Vec<TimestompHit> {
    let mut hits = Vec::new();
    for rec in records {
        let mut si: Option<&StdInfo> = None;
        let mut fnattr: Option<&FileNameAttr> = None;
        for a in &rec.attributes {
            match a {
                Attribute::StandardInformation(s) => si = Some(s),
                Attribute::FileName(f) if fnattr.is_none() => fnattr = Some(f),
                _ => {}
            }
        }
        let (Some(si), Some(fnattr)) = (si, fnattr) else {
            continue;
        };
        let si_ts = Timestamps {
            created: si.timestamps.created,
            modified: si.timestamps.modified,
            accessed: si.timestamps.accessed,
            mft_modified: si.timestamps.mft_modified,
        };
        let fn_ts = Timestamps {
            created: fnattr.timestamps.created,
            modified: fnattr.timestamps.modified,
            accessed: fnattr.timestamps.accessed,
            mft_modified: fnattr.timestamps.mft_modified,
        };
        let mut discrepancy = TimestampDiscrepancy::None;
        // Impossible: modified < created in SI
        if si_ts.modified < si_ts.created {
            discrepancy = TimestampDiscrepancy::Timestomped;
        } else if si_ts.modified != fn_ts.modified || si_ts.created != fn_ts.created {
            discrepancy = TimestampDiscrepancy::Modified;
        }
        // All SI identical often indicates automated wipe/stomp
        if si_ts.created == si_ts.modified
            && si_ts.modified == si_ts.accessed
            && si_ts.accessed == si_ts.mft_modified
            && si_ts.created != 0
        {
            discrepancy = TimestampDiscrepancy::Timestomped;
        }
        if discrepancy != TimestampDiscrepancy::None {
            hits.push(TimestompHit {
                record_number: rec.record_number,
                file_path: fnattr.name.clone(),
                si_timestamps: si_ts,
                fn_timestamps: fn_ts,
                discrepancy,
            });
        }
    }
    hits
}

pub fn detect_usn_journal_manipulation(usn: &[UsnRecord]) -> Vec<UsnAnomaly> {
    let mut out = Vec::new();
    if usn.is_empty() {
        out.push(UsnAnomaly {
            kind: "usn_empty".into(),
            detail: "USN journal stream empty or wiped".into(),
        });
        return out;
    }
    let mut prev = usn[0].timestamp;
    for r in usn.iter().skip(1) {
        if r.timestamp + 10_000_000_000 < prev {
            // large backward jump (100 seconds in 100ns units)
            out.push(UsnAnomaly {
                kind: "usn_time_gap".into(),
                detail: format!("timestamp jumped backward at usn {}", r.usn),
            });
        }
        if r.usn < usn.iter().map(|x| x.usn).max().unwrap_or(0) / 2 && r.usn + 1000 < prev {
            // soft signal only
        }
        prev = r.timestamp;
    }
    out
}

pub fn detect_mft_manipulation(records: &[MftRecord]) -> Vec<MftAnomaly> {
    let mut out = Vec::new();
    let mut seq_map = std::collections::HashMap::new();
    let ids: std::collections::HashSet<u64> = records.iter().map(|r| r.record_number).collect();
    for rec in records {
        if let Some(base) = rec.base_record {
            if !ids.contains(&base) {
                out.push(MftAnomaly {
                    record_number: rec.record_number,
                    kind: "dangling_base".into(),
                    detail: format!("base record {base} missing"),
                });
            }
        }
        if let Some(prev) = seq_map.insert(rec.record_number, rec.sequence) {
            if prev == rec.sequence {
                out.push(MftAnomaly {
                    record_number: rec.record_number,
                    kind: "duplicate_sequence".into(),
                    detail: format!("duplicate sequence {prev}"),
                });
            }
        }
        if rec.sequence == 0 {
            out.push(MftAnomaly {
                record_number: rec.record_number,
                kind: "invalid_sequence".into(),
                detail: "sequence number is zero".into(),
            });
        }
    }
    out
}

pub fn analyze(
    records: &[MftRecord],
    usn: &[UsnRecord],
    progress: &mut dyn ProgressSink,
) -> LabResult<AntiForensicsResult> {
    progress.report(ProgressEvent::new("antiforensics", 0, Some(3), "timestomp"));
    let timestomping_hits = detect_timestomping(records);
    progress.report(ProgressEvent::new("antiforensics", 1, Some(3), "usn"));
    let usn_anomalies = detect_usn_journal_manipulation(usn);
    progress.report(ProgressEvent::new("antiforensics", 2, Some(3), "mft"));
    let mft_anomalies = detect_mft_manipulation(records);
    progress.report(ProgressEvent::new("antiforensics", 3, Some(3), "done"));
    Ok(AntiForensicsResult {
        timestomping_hits,
        usn_anomalies,
        mft_anomalies,
    })
}
