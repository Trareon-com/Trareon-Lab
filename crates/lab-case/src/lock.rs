//! Exclusive per-case process lock (`case.lock`).

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use lab_core::{LabError, LabResult};

use crate::lifecycle::CaseState;

const LOCK_FILE: &str = "case.lock";

/// Outcome of attempting to acquire the case lock.
#[derive(Debug)]
pub enum CaseLock {
    /// Lock held by this process; drop releases it.
    Held(HeldLock),
    /// Another live process owns the lock.
    Conflict { detail: String },
    /// Stale lock from a dead PID was cleared; case must enter recovery.
    Recovered { state: CaseState, lock: HeldLock },
}

/// RAII guard for an exclusive case lock.
#[derive(Debug)]
pub struct HeldLock {
    path: PathBuf,
    _file: File,
}

impl Drop for HeldLock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

/// Helpers for typed lock conflict errors (tests + callers).
pub struct LockOutcome;

impl LockOutcome {
    pub fn conflict_error(detail: impl Into<String>) -> LabError {
        LabError::CaseLockConflict {
            detail: detail.into(),
        }
    }
}

#[derive(Debug)]
struct LockRecord {
    case_uuid: String,
    holder_pid: u32,
    host: String,
    opened_at_utc: String,
}

impl CaseLock {
    /// Acquire exclusive lock for `case_dir`, or report conflict / recovery.
    pub fn acquire(case_dir: &Path, case_uuid: &str) -> LabResult<Self> {
        fs::create_dir_all(case_dir).map_err(|e| LabError::Internal {
            detail: format!("create case dir: {e}"),
        })?;
        let path = case_dir.join(LOCK_FILE);

        if path.exists() {
            let existing = read_lock(&path)?;
            if existing.case_uuid != case_uuid {
                return Err(LabError::CaseLockConflict {
                    detail: format!(
                        "lock case_uuid mismatch: {} != {case_uuid}",
                        existing.case_uuid
                    ),
                });
            }
            if process_alive(existing.holder_pid) {
                return Ok(Self::Conflict {
                    detail: format!(
                        "held by pid {} on {} since {}",
                        existing.holder_pid, existing.host, existing.opened_at_utc
                    ),
                });
            }
            fs::remove_file(&path).map_err(|e| LabError::Internal {
                detail: format!("remove stale lock: {e}"),
            })?;
            let held = write_lock(&path, case_uuid)?;
            return Ok(Self::Recovered {
                state: CaseState::RecoveryRequired,
                lock: held,
            });
        }

        let held = write_lock(&path, case_uuid)?;
        Ok(Self::Held(held))
    }

    /// Test helper: plant a lock file with a chosen PID.
    pub fn write_stale_for_test(case_dir: &Path, case_uuid: &str, pid: u32) -> LabResult<()> {
        fs::create_dir_all(case_dir).map_err(|e| LabError::Internal {
            detail: format!("create case dir: {e}"),
        })?;
        let path = case_dir.join(LOCK_FILE);
        let body = format!(
            "{{\"case_uuid\":\"{case_uuid}\",\"holder_pid\":{pid},\"host\":\"test-host\",\"opened_at_utc\":\"1970-01-01T00:00:00Z\"}}\n"
        );
        fs::write(&path, body).map_err(|e| LabError::Internal {
            detail: format!("write stale lock: {e}"),
        })?;
        Ok(())
    }
}

fn write_lock(path: &Path, case_uuid: &str) -> LabResult<HeldLock> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| LabError::CaseLockConflict {
            detail: format!("create lock: {e}"),
        })?;
    let host = hostname();
    let pid = std::process::id();
    let body = format!(
        "{{\"case_uuid\":\"{case_uuid}\",\"holder_pid\":{pid},\"host\":\"{host}\",\"opened_at_utc\":\"1970-01-01T00:00:00Z\"}}\n"
    );
    file.write_all(body.as_bytes())
        .map_err(|e| LabError::Internal {
            detail: format!("write lock: {e}"),
        })?;
    file.sync_all().map_err(|e| LabError::Internal {
        detail: format!("sync lock: {e}"),
    })?;
    Ok(HeldLock {
        path: path.to_path_buf(),
        _file: file,
    })
}

fn read_lock(path: &Path) -> LabResult<LockRecord> {
    let mut raw = String::new();
    File::open(path)
        .and_then(|mut f| f.read_to_string(&mut raw))
        .map_err(|e| LabError::Internal {
            detail: format!("read lock: {e}"),
        })?;
    parse_lock(&raw)
}

fn parse_lock(raw: &str) -> LabResult<LockRecord> {
    let case_uuid = extract_string(raw, "case_uuid")?;
    let host = extract_string(raw, "host")?;
    let opened_at_utc = extract_string(raw, "opened_at_utc")?;
    let holder_pid = extract_u32(raw, "holder_pid")?;
    Ok(LockRecord {
        case_uuid,
        holder_pid,
        host,
        opened_at_utc,
    })
}

fn extract_string(raw: &str, key: &str) -> LabResult<String> {
    let pattern = format!("\"{key}\":\"");
    let start = raw.find(&pattern).ok_or_else(|| LabError::Internal {
        detail: format!("lock missing {key}"),
    })? + pattern.len();
    let end = raw[start..].find('"').ok_or_else(|| LabError::Internal {
        detail: format!("lock truncated {key}"),
    })? + start;
    Ok(raw[start..end].to_string())
}

fn extract_u32(raw: &str, key: &str) -> LabResult<u32> {
    let pattern = format!("\"{key}\":");
    let start = raw.find(&pattern).ok_or_else(|| LabError::Internal {
        detail: format!("lock missing {key}"),
    })? + pattern.len();
    let rest = raw[start..].trim_start();
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    digits.parse().map_err(|e| LabError::Internal {
        detail: format!("lock bad {key}: {e}"),
    })
}

fn process_alive(pid: u32) -> bool {
    if pid == 0 {
        return false;
    }
    #[cfg(unix)]
    {
        // Safety: kill(pid, 0) only checks existence.
        unsafe { libc::kill(pid as i32, 0) == 0 }
    }
    #[cfg(not(unix))]
    {
        // Fail closed: assume alive so stale recovery requires explicit platform support.
        let _ = pid;
        true
    }
}

fn hostname() -> String {
    std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("COMPUTERNAME"))
        .unwrap_or_else(|_| "unknown-host".into())
}
