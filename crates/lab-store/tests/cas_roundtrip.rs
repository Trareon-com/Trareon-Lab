//! F5: content-addressed put/get; overwrite denied.

use lab_core::LabError;
use lab_store::cas::CasStore;
use sha2::{Digest, Sha256};
use tempfile::tempdir;

#[test]
fn put_bytes_sha256_path_get_identical_overwrite_denied() {
    let dir = tempdir().expect("tempdir");
    let store = CasStore::open(dir.path()).expect("open");

    let expected = hex::encode(Sha256::digest(b"hello-evidence"));
    let digest = store.put(b"hello-evidence").expect("put");
    assert_eq!(digest, expected);

    let got = store.get(&digest).expect("get");
    assert_eq!(got, b"hello-evidence");

    let err = store.put(b"hello-evidence").expect_err("overwrite denied");
    assert!(matches!(err, LabError::IntegrityFailed { .. }));
    assert_eq!(err.code(), "INTEGRITY_FAILED");
}
