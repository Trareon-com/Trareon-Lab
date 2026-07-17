//! FR-VAL-009/010 persistence hooks.

use lab_case::{BlindPtParticipant, CaseDb, SecondMethodVerification};
use tempfile::tempdir;

#[test]
fn second_method_and_blind_pt_roundtrip() {
    let dir = tempdir().unwrap();
    let db = CaseDb::open_and_migrate(&dir.path().join("case.db")).unwrap();
    assert!(db.schema_version() >= 5);

    let v = SecondMethodVerification {
        verification_uuid: "11111111-1111-1111-1111-111111111111".into(),
        case_uuid: "22222222-2222-2222-2222-222222222222".into(),
        method_a_id: "storage.raw.hash".into(),
        method_b_id: "storage.raw.hash.recompute".into(),
        claim_ref: "evidence:raw-1".into(),
        output_a_digest: Some("aa".into()),
        output_b_digest: Some("bb".into()),
        discrepancy: "present".into(),
        disposition: "both_disclosed".into(),
        residual_risk: Some("examiner must disclose both digests".into()),
        created_at_utc: "2026-07-17T00:00:00Z".into(),
    };
    db.upsert_second_method(&v).unwrap();
    let listed = db.list_second_methods(&v.case_uuid).unwrap();
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].discrepancy, "present");
    assert_ne!(listed[0].disposition, "majority_vote");

    let p = BlindPtParticipant {
        package_uuid: "33333333-3333-3333-3333-333333333333".into(),
        case_uuid: v.case_uuid.clone(),
        scheme_id: "demo-scheme".into(),
        round_id: "2026-Q3".into(),
        import_digest: "cc".into(),
        expected_results_embargoed: true,
        submission_locked: false,
        result_export_digest: None,
        created_at_utc: "2026-07-17T00:00:00Z".into(),
    };
    db.upsert_blind_pt(&p).unwrap();
    let got = db.get_blind_pt(&p.package_uuid).unwrap().expect("present");
    assert!(got.expected_results_embargoed);
    assert!(!got.submission_locked);
}
