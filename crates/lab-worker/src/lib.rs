//! Bounded cancellable workers.

pub mod queue;

pub use queue::{JobProgress, JobStatus, WorkerQueue};
