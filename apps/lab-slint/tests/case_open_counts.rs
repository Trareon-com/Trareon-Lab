//! Day 9: open case loads real evidence/coverage counts from the case DB.

use lab_case::{CaseDb, CoverageRecord, EvidenceObject};
use lab_slint::UiSnapshot;
use tempfile::tempdir;

fn open_case_from_db(db: &CaseDb, title: &str, case_uuid: &str) -> UiSnapshot {
    let evidence = db.count_evidence_objects(case_uuid).expect("evidence") as i32;
    let coverage = db.count_coverage_records(case_uuid).expect("coverage") as i32;
    let mut snap = UiSnapshot::default();
    snap.open_case(title, evidence, coverage);
    snap
}

#[test]
fn open_case_reads_evidence_and_coverage_counts_from_db() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("case.sqlite");
    let case_uuid = "11111111-1111-4111-8111-111111111111";
    let db = CaseDb::open_and_migrate(&path).expect("open");

    db.append_evidence_object(&EvidenceObject {
        evidence_uuid: "22222222-2222-4222-8222-222222222222".into(),
        case_uuid: case_uuid.into(),
        created_at_utc: "2026-07-17T00:00:00Z".into(),
        display_name: "evidence.raw".into(),
        evidence_class: "disk_image".into(),
        validation_state: "VerifiedMatch".into(),
    })
    .expect("evidence 1");
    db.append_evidence_object(&EvidenceObject {
        evidence_uuid: "33333333-3333-4333-8333-333333333333".into(),
        case_uuid: case_uuid.into(),
        created_at_utc: "2026-07-17T00:01:00Z".into(),
        display_name: "logical.zip".into(),
        evidence_class: "logical".into(),
        validation_state: "ComputedUnanchored".into(),
    })
    .expect("evidence 2");
    db.append_coverage_record(&CoverageRecord {
        coverage_uuid: "cccccccc-cccc-4ccc-8ccc-cccccccccccc".into(),
        case_uuid: case_uuid.into(),
        created_at_utc: "2026-07-17T00:02:00Z".into(),
        scope: "fs:ntfs".into(),
        status: "partial".into(),
        detail_json: "{}".into(),
    })
    .expect("coverage");

    let snap = open_case_from_db(&db, "CASE-FROM-DB", case_uuid);
    assert_eq!(snap.case_title, "CASE-FROM-DB");
    assert_eq!(snap.evidence_count, 2);
    assert_eq!(snap.coverage_count, 1);
}
