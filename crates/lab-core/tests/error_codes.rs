//! F2: typed errors expose stable machine-readable codes.

use lab_core::error::LabError;

#[test]
fn fsnap_rejected_maps_to_stable_code() {
    let err = LabError::FsNapRejected {
        reason: "path_traversal".into(),
    };
    assert_eq!(err.code(), "FSNAP_REJECTED");
}
