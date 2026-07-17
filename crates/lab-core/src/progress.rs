//! Cooperative progress reporting for long forensic operations.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

/// One progress tick from a long-running job.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgressEvent {
    pub stage: &'static str,
    pub done: u64,
    pub total: Option<u64>,
    pub message: String,
}

impl ProgressEvent {
    pub fn new(stage: &'static str, done: u64, total: Option<u64>, message: impl Into<String>) -> Self {
        Self {
            stage,
            done,
            total,
            message: message.into(),
        }
    }

    /// Ratio in \[0.0, 1.0\] when total is known and non-zero; otherwise 0.0.
    pub fn ratio(&self) -> f64 {
        match self.total {
            Some(t) if t > 0 => (self.done as f64 / t as f64).clamp(0.0, 1.0),
            _ => 0.0,
        }
    }
}

/// Sink for progress events; may also signal cooperative cancel.
pub trait ProgressSink: Send {
    fn report(&mut self, ev: ProgressEvent);
    fn is_cancelled(&self) -> bool {
        false
    }
}

/// No-op sink (tests / callers that ignore progress).
#[derive(Debug, Default, Clone, Copy)]
pub struct NullProgress;

impl ProgressSink for NullProgress {
    fn report(&mut self, _ev: ProgressEvent) {}
}

/// Callback sink.
pub struct FnProgress<F>(pub F);

impl<F> ProgressSink for FnProgress<F>
where
    F: FnMut(ProgressEvent) + Send,
{
    fn report(&mut self, ev: ProgressEvent) {
        (self.0)(ev);
    }
}

/// Shared progress + cancel flag for worker/UI polling.
#[derive(Debug, Default)]
pub struct SharedProgress {
    cancel: AtomicBool,
    latest: Mutex<Option<ProgressEvent>>,
}

impl SharedProgress {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn request_cancel(&self) {
        self.cancel.store(true, Ordering::SeqCst);
    }

    pub fn latest(&self) -> Option<ProgressEvent> {
        self.latest.lock().ok().and_then(|g| g.clone())
    }
}

impl ProgressSink for SharedProgress {
    fn report(&mut self, ev: ProgressEvent) {
        if let Ok(mut g) = self.latest.lock() {
            *g = Some(ev);
        }
    }

    fn is_cancelled(&self) -> bool {
        self.cancel.load(Ordering::SeqCst)
    }
}

impl ProgressSink for Arc<SharedProgress> {
    fn report(&mut self, ev: ProgressEvent) {
        if let Ok(mut g) = self.latest.lock() {
            *g = Some(ev);
        }
    }

    fn is_cancelled(&self) -> bool {
        self.cancel.load(Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ratio_and_fn_progress() {
        let mut seen = Vec::new();
        {
            let mut sink = FnProgress(|ev: ProgressEvent| seen.push(ev));
            sink.report(ProgressEvent::new("hash", 50, Some(100), "halfway"));
        }
        assert_eq!(seen.len(), 1);
        assert!((seen[0].ratio() - 0.5).abs() < f64::EPSILON);
        assert!(!NullProgress.is_cancelled());
    }

    #[test]
    fn shared_progress_cancel() {
        let p = SharedProgress::new();
        p.request_cancel();
        assert!(p.is_cancelled());
        let mut sink = Arc::clone(&p);
        sink.report(ProgressEvent::new("x", 1, Some(2), "t"));
        assert_eq!(p.latest().unwrap().done, 1);
    }
}
