//! Bounded worker queue with cooperative cancellation and progress.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use lab_core::{LabError, LabResult, ProgressEvent};
use lab_store::CasStore;

/// Terminal / observable job status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobStatus {
    Queued,
    Running,
    Completed,
    CompletedIdempotent,
    Cancelled,
    Failed,
}

/// Latest progress snapshot for a job.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JobProgress {
    pub stage: String,
    pub done: u64,
    pub total: Option<u64>,
    pub message: String,
}

impl From<ProgressEvent> for JobProgress {
    fn from(ev: ProgressEvent) -> Self {
        Self {
            stage: ev.stage.to_string(),
            done: ev.done,
            total: ev.total,
            message: ev.message,
        }
    }
}

struct Job {
    id: String,
    payload: Vec<u8>,
    delay: Duration,
    cancel: Arc<AtomicBool>,
    status: JobStatus,
    progress: Option<JobProgress>,
}

struct QueueInner {
    jobs: HashMap<String, Job>,
    next_id: u64,
}

/// Single-case bounded worker.
pub struct WorkerQueue {
    inner: Arc<Mutex<QueueInner>>,
    wake: Arc<Condvar>,
    workers: Vec<JoinHandle<()>>,
    stop: Arc<AtomicBool>,
}

impl WorkerQueue {
    pub fn new(store: Arc<CasStore>, worker_count: usize) -> Self {
        let inner = Arc::new(Mutex::new(QueueInner {
            jobs: HashMap::new(),
            next_id: 1,
        }));
        let wake = Arc::new(Condvar::new());
        let stop = Arc::new(AtomicBool::new(false));
        let mut workers = Vec::new();
        for _ in 0..worker_count.max(1) {
            let inner_w = Arc::clone(&inner);
            let wake_w = Arc::clone(&wake);
            let stop_w = Arc::clone(&stop);
            let store_w = Arc::clone(&store);
            workers.push(thread::spawn(move || {
                worker_loop(inner_w, wake_w, stop_w, store_w)
            }));
        }
        Self {
            inner,
            wake,
            workers,
            stop,
        }
    }

    pub fn enqueue_put(&self, bytes: &[u8]) -> LabResult<String> {
        self.enqueue(bytes, Duration::ZERO)
    }

    pub fn enqueue_slow_put(&self, bytes: &[u8], delay: Duration) -> LabResult<String> {
        self.enqueue(bytes, delay)
    }

    fn enqueue(&self, bytes: &[u8], delay: Duration) -> LabResult<String> {
        let mut guard = self.inner.lock().map_err(|_| LabError::Internal {
            detail: "worker queue poisoned".into(),
        })?;
        let id = format!("job-{}", guard.next_id);
        guard.next_id += 1;
        guard.jobs.insert(
            id.clone(),
            Job {
                id: id.clone(),
                payload: bytes.to_vec(),
                delay,
                cancel: Arc::new(AtomicBool::new(false)),
                status: JobStatus::Queued,
                progress: None,
            },
        );
        self.wake.notify_one();
        Ok(id)
    }

    pub fn cancel(&self, job_id: &str) -> LabResult<()> {
        let mut guard = self.inner.lock().map_err(|_| LabError::Internal {
            detail: "worker queue poisoned".into(),
        })?;
        let job = guard
            .jobs
            .get_mut(job_id)
            .ok_or_else(|| LabError::Internal {
                detail: format!("unknown job {job_id}"),
            })?;
        job.cancel.store(true, Ordering::SeqCst);
        if matches!(job.status, JobStatus::Queued) {
            job.status = JobStatus::Cancelled;
        }
        self.wake.notify_all();
        Ok(())
    }

    /// Status plus optional latest progress.
    pub fn status(&self, job_id: &str) -> LabResult<(JobStatus, Option<JobProgress>)> {
        let guard = self.inner.lock().map_err(|_| LabError::Internal {
            detail: "worker queue poisoned".into(),
        })?;
        let job = guard.jobs.get(job_id).ok_or_else(|| LabError::Internal {
            detail: format!("unknown job {job_id}"),
        })?;
        Ok((job.status, job.progress.clone()))
    }

    /// Update progress for a running job (used by analysis drivers).
    pub fn report_progress(&self, job_id: &str, progress: JobProgress) -> LabResult<()> {
        let mut guard = self.inner.lock().map_err(|_| LabError::Internal {
            detail: "worker queue poisoned".into(),
        })?;
        let job = guard
            .jobs
            .get_mut(job_id)
            .ok_or_else(|| LabError::Internal {
                detail: format!("unknown job {job_id}"),
            })?;
        job.progress = Some(progress);
        Ok(())
    }

    pub fn wait(&self, job_id: &str, timeout: Duration) -> LabResult<JobStatus> {
        let deadline = Instant::now() + timeout;
        loop {
            {
                let guard = self.inner.lock().map_err(|_| LabError::Internal {
                    detail: "worker queue poisoned".into(),
                })?;
                if let Some(job) = guard.jobs.get(job_id) {
                    if matches!(
                        job.status,
                        JobStatus::Completed
                            | JobStatus::CompletedIdempotent
                            | JobStatus::Cancelled
                            | JobStatus::Failed
                    ) {
                        return Ok(job.status);
                    }
                }
            }
            if Instant::now() >= deadline {
                return Err(LabError::Internal {
                    detail: format!("wait timeout for {job_id}"),
                });
            }
            thread::sleep(Duration::from_millis(10));
        }
    }
}

impl Drop for WorkerQueue {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        self.wake.notify_all();
        while let Some(h) = self.workers.pop() {
            let _ = h.join();
        }
    }
}

fn worker_loop(
    inner: Arc<Mutex<QueueInner>>,
    wake: Arc<Condvar>,
    stop: Arc<AtomicBool>,
    store: Arc<CasStore>,
) {
    loop {
        if stop.load(Ordering::SeqCst) {
            break;
        }
        let next = {
            let mut guard = inner.lock().unwrap();
            let queued = guard
                .jobs
                .values_mut()
                .find(|j| j.status == JobStatus::Queued);
            if let Some(job) = queued {
                job.status = JobStatus::Running;
                job.progress = Some(JobProgress {
                    stage: "worker.put".into(),
                    done: 0,
                    total: Some(1),
                    message: "starting".into(),
                });
                Some((
                    job.id.clone(),
                    job.payload.clone(),
                    job.delay,
                    Arc::clone(&job.cancel),
                ))
            } else {
                let (_guard, _) = wake.wait_timeout(guard, Duration::from_millis(50)).unwrap();
                None
            }
        };

        let Some((id, payload, delay, cancel)) = next else {
            continue;
        };

        if cancel.load(Ordering::SeqCst) {
            set_status(&inner, &id, JobStatus::Cancelled);
            continue;
        }

        let slice = Duration::from_millis(25);
        let mut remaining = delay;
        let total_delay_ms = delay.as_millis() as u64;
        let mut elapsed_ms = 0_u64;
        while remaining > Duration::ZERO {
            if cancel.load(Ordering::SeqCst) {
                set_status(&inner, &id, JobStatus::Cancelled);
                break;
            }
            let step = remaining.min(slice);
            thread::sleep(step);
            remaining = remaining.saturating_sub(step);
            elapsed_ms = elapsed_ms.saturating_add(step.as_millis() as u64);
            set_progress(
                &inner,
                &id,
                JobProgress {
                    stage: "worker.delay".into(),
                    done: elapsed_ms,
                    total: Some(total_delay_ms.max(1)),
                    message: "waiting".into(),
                },
            );
        }
        if cancel.load(Ordering::SeqCst) {
            set_status(&inner, &id, JobStatus::Cancelled);
            continue;
        }

        set_progress(
            &inner,
            &id,
            JobProgress {
                stage: "worker.put".into(),
                done: 1,
                total: Some(1),
                message: "writing CAS".into(),
            },
        );

        let status = match store.put(&payload) {
            Ok(_) => JobStatus::Completed,
            Err(lab_core::LabError::IntegrityFailed { .. }) => JobStatus::CompletedIdempotent,
            Err(_) => JobStatus::Failed,
        };
        set_status(&inner, &id, status);
    }
}

fn set_status(inner: &Mutex<QueueInner>, id: &str, status: JobStatus) {
    if let Ok(mut guard) = inner.lock() {
        if let Some(job) = guard.jobs.get_mut(id) {
            job.status = status;
        }
    }
}

fn set_progress(inner: &Mutex<QueueInner>, id: &str, progress: JobProgress) {
    if let Ok(mut guard) = inner.lock() {
        if let Some(job) = guard.jobs.get_mut(id) {
            job.progress = Some(progress);
        }
    }
}
