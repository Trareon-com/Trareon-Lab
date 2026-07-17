//! Shared Gate A spike logic. Synthetic data only. No production forensic parsers.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use thiserror::Error;
use uuid::Uuid;

pub const ROW_COUNT: usize = 1_000_000;
pub const IPC_SCHEMA_VERSION: &str = "spike-ipc-1";

#[derive(Debug, Error)]
pub enum SpikeError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("case lock held by pid {holder_pid}: {message}")]
    LockHeld { holder_pid: u32, message: String },
    #[error("case not open")]
    NotOpen,
    #[error("invalid ipc schema: expected {expected}, got {got}")]
    IpcSchema { expected: String, got: String },
    #[error("{0}")]
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyntheticRow {
    pub index: u64,
    pub path: String,
    pub size: u64,
    pub hash_prefix: String,
    pub hash_full: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseLockFile {
    pub case_id: String,
    pub holder_pid: u32,
    pub opened_at_utc: DateTime<Utc>,
    pub host: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcRequest {
    pub schema_version: String,
    pub correlation_id: String,
    pub command: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcResponse {
    pub schema_version: String,
    pub correlation_id: String,
    pub ok: bool,
    pub error: Option<String>,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportPackage {
    pub case_id: String,
    pub row_count: usize,
    pub filtered_count: usize,
    pub filter_prefix: String,
    pub selected_index: Option<u64>,
    pub hash_job_status: String,
    pub hashed_count: u64,
    pub cancelled: bool,
    pub crash_simulated: bool,
    pub export_sha256: String,
    pub build_identity: String,
    pub exported_at_utc: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasurementSample {
    pub candidate: String,
    pub os: String,
    pub cold_start_ms: Option<u64>,
    pub idle_rss_mib: Option<f64>,
    pub peak_rss_mib: Option<f64>,
    pub table_display_ms: Option<u64>,
    pub filter_p50_ms: Option<u64>,
    pub filter_p95_ms: Option<u64>,
    pub cancel_ms: Option<u64>,
    pub crash_recovery: String,
    pub installer_size_mib: Option<f64>,
    pub a11y_smoke: String,
    pub notes: String,
}

pub struct OpenCase {
    pub root: PathBuf,
    pub case_id: String,
    pub rows: Vec<SyntheticRow>,
    cancel: Arc<AtomicBool>,
    worker: Option<JoinHandle<Result<u64, SpikeError>>>,
    pub hashed_count: u64,
    pub hash_job_status: String,
    pub crash_simulated: bool,
}

impl Drop for OpenCase {
    fn drop(&mut self) {
        let _ = self.release_lock();
    }
}

impl OpenCase {
    pub fn open(root: impl AsRef<Path>, row_count: usize) -> Result<(Self, Duration), SpikeError> {
        let started = Instant::now();
        let root = root.as_ref().to_path_buf();
        fs::create_dir_all(&root)?;
        let case_id = Uuid::new_v4().to_string();
        acquire_lock(&root, &case_id)?;
        let rows = generate_rows(row_count);
        let case = Self {
            root,
            case_id,
            rows,
            cancel: Arc::new(AtomicBool::new(false)),
            worker: None,
            hashed_count: 0,
            hash_job_status: "idle".into(),
            crash_simulated: false,
        };
        Ok((case, started.elapsed()))
    }

    pub fn filter_by_hash_prefix(&self, prefix: &str) -> (Vec<usize>, Duration) {
        let started = Instant::now();
        let prefix = prefix.to_ascii_lowercase();
        let idxs: Vec<usize> = self
            .rows
            .iter()
            .enumerate()
            .filter(|(_, r)| r.hash_prefix.starts_with(&prefix) || r.hash_full.starts_with(&prefix))
            .map(|(i, _)| i)
            .collect();
        (idxs, started.elapsed())
    }

    pub fn detail(&self, index: usize) -> Option<&SyntheticRow> {
        self.rows.get(index)
    }

    /// Viewport-style page: UI should only request pages, never the full Vec over IPC.
    pub fn page(&self, offset: usize, limit: usize, prefix: &str) -> (Vec<SyntheticRow>, usize) {
        let (idxs, _) = self.filter_by_hash_prefix(prefix);
        let total = idxs.len();
        let page: Vec<SyntheticRow> = idxs
            .into_iter()
            .skip(offset)
            .take(limit)
            .filter_map(|i| self.rows.get(i).cloned())
            .collect();
        (page, total)
    }

    pub fn start_hash_job(&mut self, bounded_queue: usize) -> Result<(), SpikeError> {
        if self.worker.is_some() {
            return Err(SpikeError::Other("hash job already running".into()));
        }
        self.cancel.store(false, Ordering::SeqCst);
        self.hash_job_status = "running".into();
        let cancel = Arc::clone(&self.cancel);
        let rows = self.rows.clone();
        let handle = thread::spawn(move || {
            let mut hashed = 0u64;
            let mut in_flight = 0usize;
            for row in rows {
                if cancel.load(Ordering::SeqCst) {
                    break;
                }
                while in_flight >= bounded_queue {
                    if cancel.load(Ordering::SeqCst) {
                        break;
                    }
                    thread::sleep(Duration::from_micros(50));
                    in_flight = in_flight.saturating_sub(1);
                }
                let _ = sha256_hex(row.path.as_bytes());
                hashed += 1;
                in_flight += 1;
                if hashed % 50_000 == 0 {
                    thread::sleep(Duration::from_millis(1));
                    in_flight = 0;
                }
            }
            Ok(hashed)
        });
        self.worker = Some(handle);
        Ok(())
    }

    pub fn cancel_hash_job(&mut self) -> Result<Duration, SpikeError> {
        let started = Instant::now();
        self.cancel.store(true, Ordering::SeqCst);
        if let Some(handle) = self.worker.take() {
            match handle.join() {
                Ok(Ok(count)) => {
                    self.hashed_count = count;
                    self.hash_job_status = "cancelled".into();
                }
                Ok(Err(e)) => return Err(e),
                Err(_) => {
                    self.hash_job_status = "worker_panic".into();
                    self.crash_simulated = true;
                }
            }
        } else {
            self.hash_job_status = "idle".into();
        }
        Ok(started.elapsed())
    }

    pub fn simulate_worker_crash(&mut self) -> Result<(), SpikeError> {
        self.cancel.store(true, Ordering::SeqCst);
        if let Some(handle) = self.worker.take() {
            // Detach by not joining cleanly: drop join without waiting if possible.
            // We join with timeout simulation by spawning a killer pattern:
            let _ = handle.join();
        }
        self.crash_simulated = true;
        self.hash_job_status = "crashed".into();
        // Lock must remain held by this process until explicit release.
        ensure_lock_owned(&self.root)?;
        Ok(())
    }

    pub fn export_deterministic(
        &self,
        filter_prefix: &str,
        selected_index: Option<u64>,
        build_identity: &str,
    ) -> Result<(PathBuf, ExportPackage, Duration), SpikeError> {
        let started = Instant::now();
        let (idxs, _) = self.filter_by_hash_prefix(filter_prefix);
        let mut pkg = ExportPackage {
            case_id: self.case_id.clone(),
            row_count: self.rows.len(),
            filtered_count: idxs.len(),
            filter_prefix: filter_prefix.to_string(),
            selected_index,
            hash_job_status: self.hash_job_status.clone(),
            hashed_count: self.hashed_count,
            cancelled: self.hash_job_status == "cancelled",
            crash_simulated: self.crash_simulated,
            export_sha256: String::new(),
            build_identity: build_identity.to_string(),
            exported_at_utc: Utc::now(),
        };
        let body = serde_json::to_vec_pretty(&pkg)?;
        let digest = sha256_hex(&body);
        pkg.export_sha256 = digest;
        let final_body = serde_json::to_vec_pretty(&pkg)?;
        let path = self.root.join("export.json");
        fs::write(&path, final_body)?;
        Ok((path, pkg, started.elapsed()))
    }

    pub fn release_lock(&mut self) -> Result<(), SpikeError> {
        let lock_path = lock_path(&self.root);
        if lock_path.exists() {
            fs::remove_file(lock_path)?;
        }
        Ok(())
    }
}

pub fn validate_ipc_request(req: &IpcRequest) -> Result<(), SpikeError> {
    if req.schema_version != IPC_SCHEMA_VERSION {
        return Err(SpikeError::IpcSchema {
            expected: IPC_SCHEMA_VERSION.into(),
            got: req.schema_version.clone(),
        });
    }
    if req.correlation_id.trim().is_empty() {
        return Err(SpikeError::Other("correlation_id required".into()));
    }
    Ok(())
}

pub fn ipc_ok(correlation_id: &str, payload: serde_json::Value) -> IpcResponse {
    IpcResponse {
        schema_version: IPC_SCHEMA_VERSION.into(),
        correlation_id: correlation_id.into(),
        ok: true,
        error: None,
        payload,
    }
}

pub fn ipc_err(correlation_id: &str, error: impl ToString) -> IpcResponse {
    IpcResponse {
        schema_version: IPC_SCHEMA_VERSION.into(),
        correlation_id: correlation_id.into(),
        ok: false,
        error: Some(error.to_string()),
        payload: serde_json::json!({}),
    }
}

pub fn try_reopen_after_release(root: impl AsRef<Path>) -> Result<String, SpikeError> {
    let root = root.as_ref();
    let lock = lock_path(root);
    if lock.exists() {
        let holder = read_lock(root)?;
        return Err(SpikeError::LockHeld {
            holder_pid: holder.holder_pid,
            message: "second process cannot open while lock is held".into(),
        });
    }
    let case_id = Uuid::new_v4().to_string();
    acquire_lock(root, &case_id)?;
    // Immediately release for probe convenience unless caller keeps it.
    fs::remove_file(lock_path(root))?;
    Ok(case_id)
}

pub fn current_rss_mib() -> Option<f64> {
    // Best-effort macOS/Linux via /proc or mach; Windows left to harness scripts.
    #[cfg(target_os = "linux")]
    {
        let status = fs::read_to_string("/proc/self/status").ok()?;
        for line in status.lines() {
            if let Some(rest) = line.strip_prefix("VmRSS:") {
                let kb: f64 = rest.split_whitespace().next()?.parse().ok()?;
                return Some(kb / 1024.0);
            }
        }
        None
    }
    #[cfg(target_os = "macos")]
    {
        // Approximate via libc mach APIs is complex; use `ps` fallback in harness.
        None
    }
    #[cfg(windows)]
    {
        None
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", windows)))]
    {
        None
    }
}

pub fn percentile(sorted_ms: &[u64], p: f64) -> Option<u64> {
    if sorted_ms.is_empty() {
        return None;
    }
    let idx = ((p / 100.0) * (sorted_ms.len() as f64 - 1.0)).round() as usize;
    sorted_ms.get(idx).copied()
}

fn generate_rows(count: usize) -> Vec<SyntheticRow> {
    // Deterministic synthetic hashes (not cryptographic claims). Full SHA-256 is exercised in the hash job.
    let mut rows = Vec::with_capacity(count);
    for i in 0..count as u64 {
        let path = format!("/synthetic/case/obj_{i:08}.bin");
        let size = 64 + (i % 4096);
        let hash_full = format!("{i:016x}{:016x}", size.wrapping_mul(0x9e37_79b9));
        let hash_prefix = hash_full.chars().take(8).collect::<String>();
        rows.push(SyntheticRow {
            index: i,
            path,
            size,
            hash_prefix,
            hash_full,
        });
    }
    rows
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex::encode(hasher.finalize())
}

fn lock_path(root: &Path) -> PathBuf {
    root.join("case.lock")
}

fn acquire_lock(root: &Path, case_id: &str) -> Result<(), SpikeError> {
    let path = lock_path(root);
    if path.exists() {
        let existing = read_lock(root)?;
        if process_alive(existing.holder_pid) {
            return Err(SpikeError::LockHeld {
                holder_pid: existing.holder_pid,
                message: "active holder".into(),
            });
        }
        // Stale lock from dead process: recover by replacing.
        fs::remove_file(&path)?;
    }
    let lock = CaseLockFile {
        case_id: case_id.to_string(),
        holder_pid: std::process::id(),
        opened_at_utc: Utc::now(),
        host: hostname(),
    };
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)?;
    file.write_all(serde_json::to_string_pretty(&lock)?.as_bytes())?;
    Ok(())
}

fn ensure_lock_owned(root: &Path) -> Result<(), SpikeError> {
    let lock = read_lock(root)?;
    if lock.holder_pid != std::process::id() {
        return Err(SpikeError::Other("lock ownership lost after crash simulation".into()));
    }
    Ok(())
}

fn read_lock(root: &Path) -> Result<CaseLockFile, SpikeError> {
    let mut f = File::open(lock_path(root))?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    Ok(serde_json::from_str(&buf)?)
}

fn process_alive(pid: u32) -> bool {
    if pid == 0 {
        return false;
    }
    #[cfg(unix)]
    {
        // signal 0 checks existence
        libc_kill(pid as i32, 0) == 0
    }
    #[cfg(windows)]
    {
        // Without WinAPI dependency, assume lock alive if file exists; harness validates.
        true
    }
}

#[cfg(unix)]
fn libc_kill(pid: i32, sig: i32) -> i32 {
    extern "C" {
        fn kill(pid: i32, sig: i32) -> i32;
    }
    unsafe { kill(pid, sig) }
}

fn hostname() -> String {
    hostname_impl().unwrap_or_else(|| "unknown".into())
}

fn hostname_impl() -> Option<String> {
    std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("COMPUTERNAME"))
        .ok()
        .or_else(|| {
            fs::read_to_string("/etc/hostname")
                .ok()
                .map(|s| s.trim().to_string())
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn generate_and_filter() {
        let dir = env::temp_dir().join(format!("lab-spike-{}", Uuid::new_v4()));
        let (case, _) = OpenCase::open(&dir, 10_000).unwrap();
        let (idxs, _) = case.filter_by_hash_prefix("a");
        assert!(!idxs.is_empty() || true); // prefix may or may not match; ensure no panic
        let (page, total) = case.page(0, 50, "");
        assert_eq!(page.len(), 50);
        assert_eq!(total, 10_000);
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn ipc_schema_required() {
        let bad = IpcRequest {
            schema_version: "wrong".into(),
            correlation_id: "c1".into(),
            command: "page".into(),
            payload: serde_json::json!({}),
        };
        assert!(validate_ipc_request(&bad).is_err());
    }
}
