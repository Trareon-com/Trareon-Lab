//! F4: exclusive lock blocks second open; dead-PID recovery → RECOVERY_REQUIRED.

use lab_case::lifecycle::CaseState;
use lab_case::lock::{CaseLock, LockOutcome};
use tempfile::tempdir;

#[test]
fn second_open_blocked_while_lock_held() {
    let dir = tempdir().expect("tempdir");
    let case_dir = dir.path();

    let lock = CaseLock::acquire(case_dir, "11111111-1111-1111-1111-111111111111")
        .expect("first acquire");
    assert!(matches!(lock, CaseLock::Held(_)));

    let second = CaseLock::acquire(case_dir, "11111111-1111-1111-1111-111111111111")
        .expect("second acquire attempt");
    assert!(matches!(second, CaseLock::Conflict { .. }));
}

#[test]
fn stale_dead_pid_lock_recovers_to_recovery_required() {
    let dir = tempdir().expect("tempdir");
    let case_dir = dir.path();

    // Write a lock claiming a PID that cannot be alive on this host.
    CaseLock::write_stale_for_test(case_dir, "11111111-1111-1111-1111-111111111111", 999_999_999)
        .expect("stale lock");

    let outcome = CaseLock::acquire(case_dir, "11111111-1111-1111-1111-111111111111")
        .expect("recover");
    match outcome {
        CaseLock::Recovered { state, .. } => {
            assert_eq!(state, CaseState::RecoveryRequired);
        }
        other => panic!("expected Recovered, got {other:?}"),
    }
}

#[test]
fn lock_outcome_conflict_uses_typed_error_code() {
    let err = LockOutcome::conflict_error("held by pid 1");
    assert_eq!(err.code(), "CASE_LOCK_CONFLICT");
}
