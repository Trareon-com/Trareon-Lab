//! F11: same case seal → identical export digest; draft labeled non-final.

use lab_core::export::{export_case_skeleton, ExportMode};

#[test]
fn sealed_export_digest_is_deterministic() {
    let a = export_case_skeleton(
        "11111111-1111-4111-8111-111111111111",
        "Case A",
        2,
        3,
        ExportMode::Sealed,
    )
    .unwrap();
    let b = export_case_skeleton(
        "11111111-1111-4111-8111-111111111111",
        "Case A",
        2,
        3,
        ExportMode::Sealed,
    )
    .unwrap();
    assert_eq!(a.digest_sha256, b.digest_sha256);
    assert!(!a.labeled_non_final);
}

#[test]
fn draft_export_labeled_non_final() {
    let draft = export_case_skeleton(
        "11111111-1111-4111-8111-111111111111",
        "Case A",
        1,
        1,
        ExportMode::Draft,
    )
    .unwrap();
    assert!(draft.labeled_non_final);
    assert!(draft.body.contains("draft_non_final"));
}
