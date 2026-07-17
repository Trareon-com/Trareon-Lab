//! Day 7: audit/provenance/coverage ledgers are append-only.

use lab_case::{AuditEvent, CaseDb, CoverageRecord, ProvenanceEvent};
use tempfile::tempdir;

#[test]
fn append_only_ledgers_persist_and_forbid_mutation() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("case.sqlite");
    let case_uuid = "11111111-1111-4111-8111-111111111111";

    let db = CaseDb::open_and_migrate(&path).expect("open");
    assert_eq!(db.schema_version(), 2);

    db.append_audit_event(&AuditEvent {
        event_uuid: "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa".into(),
        case_uuid: case_uuid.into(),
        created_at_utc: "2026-07-17T00:00:00Z".into(),
        actor_role: "examiner".into(),
        action: "case.open".into(),
        detail_json: "{}".into(),
    })
    .expect("audit");

    db.append_provenance_event(&ProvenanceEvent {
        event_uuid: "bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb".into(),
        case_uuid: case_uuid.into(),
        created_at_utc: "2026-07-17T00:01:00Z".into(),
        evidence_uuid: "22222222-2222-4222-8222-222222222222".into(),
        activity: "import".into(),
        detail_json: "{\"source\":\"raw\"}".into(),
    })
    .expect("provenance");

    db.append_coverage_record(&CoverageRecord {
        coverage_uuid: "cccccccc-cccc-4ccc-8ccc-cccccccccccc".into(),
        case_uuid: case_uuid.into(),
        created_at_utc: "2026-07-17T00:02:00Z".into(),
        scope: "fs:ntfs".into(),
        status: "partial".into(),
        detail_json: "{\"pct\":12}".into(),
    })
    .expect("coverage");

    assert_eq!(db.count_audit_events(case_uuid).unwrap(), 1);
    assert_eq!(db.count_provenance_events(case_uuid).unwrap(), 1);
    assert_eq!(db.count_coverage_records(case_uuid).unwrap(), 1);

    // Corrections are new appends, not updates.
    db.append_coverage_record(&CoverageRecord {
        coverage_uuid: "dddddddd-dddd-4ddd-8ddd-dddddddddddd".into(),
        case_uuid: case_uuid.into(),
        created_at_utc: "2026-07-17T00:03:00Z".into(),
        scope: "fs:ntfs".into(),
        status: "extended".into(),
        detail_json: "{\"pct\":18}".into(),
    })
    .expect("coverage append correction");
    assert_eq!(db.count_coverage_records(case_uuid).unwrap(), 2);

    assert!(db
        .try_update_audit_event_forbidden("aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa")
        .is_err());
    assert!(db
        .try_delete_audit_event_forbidden("aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa")
        .is_err());

    drop(db);
    let reopened = CaseDb::open_and_migrate(&path).expect("reopen");
    assert_eq!(reopened.count_audit_events(case_uuid).unwrap(), 1);
    assert_eq!(reopened.count_provenance_events(case_uuid).unwrap(), 1);
    assert_eq!(reopened.count_coverage_records(case_uuid).unwrap(), 2);
}
