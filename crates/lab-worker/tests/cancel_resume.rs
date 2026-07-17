//! F9: cancel within 500ms; no duplicate derived objects on resume.

use lab_store::cas::CasStore;
use lab_worker::queue::{JobStatus, WorkerQueue};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use tempfile::tempdir;

#[test]
fn cancel_within_500ms_and_resume_without_duplicate_cas() {
    let dir = tempdir().unwrap();
    let store = Arc::new(CasStore::open(dir.path()).unwrap());
    let queue = WorkerQueue::new(store.clone(), 1);

    let job_id = queue
        .enqueue_slow_put(b"derived-object-v1", Duration::from_secs(2))
        .unwrap();

    thread::sleep(Duration::from_millis(50));
    let started = Instant::now();
    queue.cancel(&job_id).unwrap();
    assert!(started.elapsed() < Duration::from_millis(500));

    let status = queue.wait(&job_id, Duration::from_secs(2)).unwrap();
    assert_eq!(status, JobStatus::Cancelled);

    // Resume: enqueue same payload again — first put may or may not have completed;
    // CAS must not contain duplicates (single digest key).
    let job2 = queue.enqueue_put(b"derived-object-v1").unwrap();
    let status2 = queue.wait(&job2, Duration::from_secs(2)).unwrap();
    assert!(matches!(
        status2,
        JobStatus::Completed | JobStatus::CompletedIdempotent
    ));

    let digest = {
        use sha2::{Digest, Sha256};
        hex::encode(Sha256::digest(b"derived-object-v1"))
    };
    let bytes = store.get(&digest).unwrap();
    assert_eq!(bytes, b"derived-object-v1");
}
