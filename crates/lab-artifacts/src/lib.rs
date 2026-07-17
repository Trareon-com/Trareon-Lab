//! OS artifact parsers (synthetic corpora) — Days 26–32.

use lab_core::{LabError, LabResult};
use serde::{Deserialize, Serialize};

fn fail(d: impl Into<String>) -> LabError {
    LabError::Internal { detail: d.into() }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactHit {
    pub kind: String,
    pub summary: String,
    pub occurred_at_utc: String,
    pub provenance_ref: String,
    pub source_path: String,
}

#[derive(Debug, Deserialize)]
struct PrefetchCorpus {
    schema: String,
    hits: Vec<PrefetchHit>,
}

#[derive(Debug, Deserialize)]
struct PrefetchHit {
    exe_name: String,
    run_count: u32,
    last_run_utc: String,
    path: String,
}

pub fn parse_prefetch_synth(json: &str) -> LabResult<Vec<ArtifactHit>> {
    let corpus: PrefetchCorpus =
        serde_json::from_str(json).map_err(|e| fail(format!("prefetch json: {e}")))?;
    if corpus.schema != "prefetch-synth-1" {
        return Err(fail("prefetch bad schema"));
    }
    Ok(corpus
        .hits
        .into_iter()
        .map(|h| ArtifactHit {
            kind: "windows.prefetch".into(),
            summary: format!("{} runs={}", h.exe_name, h.run_count),
            occurred_at_utc: h.last_run_utc,
            provenance_ref: format!("prefetch:{}", h.exe_name),
            source_path: h.path,
        })
        .collect())
}

#[derive(Debug, Deserialize)]
struct LnkCorpus {
    schema: String,
    links: Vec<LnkHit>,
}

#[derive(Debug, Deserialize)]
struct LnkHit {
    target_path: String,
    created_utc: String,
    link_path: String,
}

pub fn parse_lnk_synth(json: &str) -> LabResult<Vec<ArtifactHit>> {
    let corpus: LnkCorpus =
        serde_json::from_str(json).map_err(|e| fail(format!("lnk json: {e}")))?;
    if corpus.schema != "lnk-synth-1" {
        return Err(fail("lnk bad schema"));
    }
    Ok(corpus
        .links
        .into_iter()
        .map(|h| ArtifactHit {
            kind: "windows.lnk".into(),
            summary: format!("lnk -> {}", h.target_path),
            occurred_at_utc: h.created_utc,
            provenance_ref: format!("lnk:{}", h.link_path),
            source_path: h.link_path,
        })
        .collect())
}

#[derive(Debug, Deserialize)]
struct JumpListCorpus {
    schema: String,
    entries: Vec<JumpHit>,
}

#[derive(Debug, Deserialize)]
struct JumpHit {
    app_id: String,
    target: String,
    accessed_utc: String,
}

pub fn parse_jumplist_synth(json: &str) -> LabResult<Vec<ArtifactHit>> {
    let corpus: JumpListCorpus =
        serde_json::from_str(json).map_err(|e| fail(format!("jumplist json: {e}")))?;
    if corpus.schema != "jumplist-synth-1" {
        return Err(fail("jumplist bad schema"));
    }
    Ok(corpus
        .entries
        .into_iter()
        .map(|h| ArtifactHit {
            kind: "windows.jumplist".into(),
            summary: format!("{} :: {}", h.app_id, h.target),
            occurred_at_utc: h.accessed_utc,
            provenance_ref: format!("jumplist:{}", h.app_id),
            source_path: h.target,
        })
        .collect())
}

#[derive(Debug, Deserialize)]
struct UlogCorpus {
    schema: String,
    events: Vec<UlogHit>,
}

#[derive(Debug, Deserialize)]
struct UlogHit {
    subsystem: String,
    message: String,
    timestamp_utc: String,
}

pub fn parse_unified_log_synth(json: &str) -> LabResult<Vec<ArtifactHit>> {
    let corpus: UlogCorpus =
        serde_json::from_str(json).map_err(|e| fail(format!("ulog json: {e}")))?;
    if corpus.schema != "ulog-synth-1" {
        return Err(fail("ulog bad schema"));
    }
    Ok(corpus
        .events
        .into_iter()
        .map(|h| ArtifactHit {
            kind: "macos.unified_log".into(),
            summary: format!("{}: {}", h.subsystem, h.message),
            occurred_at_utc: h.timestamp_utc,
            provenance_ref: format!("ulog:{}", h.subsystem),
            source_path: "unified_log".into(),
        })
        .collect())
}

#[derive(Debug, Deserialize)]
struct SyslogCorpus {
    schema: String,
    lines: Vec<SysHit>,
}

#[derive(Debug, Deserialize)]
struct SysHit {
    facility: String,
    message: String,
    timestamp_utc: String,
}

pub fn parse_linux_auth_syslog_synth(json: &str) -> LabResult<Vec<ArtifactHit>> {
    let corpus: SyslogCorpus =
        serde_json::from_str(json).map_err(|e| fail(format!("syslog json: {e}")))?;
    if corpus.schema != "linux-auth-synth-1" {
        return Err(fail("syslog bad schema"));
    }
    Ok(corpus
        .lines
        .into_iter()
        .map(|h| ArtifactHit {
            kind: "linux.auth_syslog".into(),
            summary: format!("{}: {}", h.facility, h.message),
            occurred_at_utc: h.timestamp_utc,
            provenance_ref: format!("auth:{}", h.facility),
            source_path: "/var/log/auth.log".into(),
        })
        .collect())
}

/// Day 28: normalize artifact hits into timeline events.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub occurred_at_utc: String,
    pub kind: String,
    pub summary: String,
    pub provenance_ref: String,
}

pub fn artifacts_to_timeline(hits: &[ArtifactHit]) -> Vec<TimelineEvent> {
    let mut events: Vec<_> = hits
        .iter()
        .map(|h| TimelineEvent {
            occurred_at_utc: h.occurred_at_utc.clone(),
            kind: h.kind.clone(),
            summary: h.summary.clone(),
            provenance_ref: h.provenance_ref.clone(),
        })
        .collect();
    events.sort_by(|a, b| a.occurred_at_utc.cmp(&b.occurred_at_utc));
    events
}
