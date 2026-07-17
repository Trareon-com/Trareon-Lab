//! Day 13: import raw image → evidence + provenance.

use lab_case::CaseDb;
use lab_storage::import_raw_image;
use std::io::Write;
use tempfile::{tempdir, NamedTempFile};

#[test]
fn import_raw_registers_evidence_and_provenance() {
    let dir = tempdir().expect("dir");
    let db = CaseDb::open_and_migrate(&dir.path().join("case.sqlite")).expect("db");
    let case_uuid = "11111111-1111-4111-8111-111111111111";

    let mut image = NamedTempFile::new().expect("img");
    image.write_all(b"IMPORT-DAY13").expect("write");
    image.flush().expect("flush");

    let result = import_raw_image(
        &db,
        case_uuid,
        image.path(),
        "22222222-2222-4222-8222-222222222222",
        "33333333-3333-4333-8333-333333333333",
        "2026-07-17T12:00:00Z",
    )
    .expect("import");

    assert_eq!(result.byte_length, 12);
    assert_eq!(result.sha256_hex.len(), 64);
    assert_eq!(db.count_evidence_objects(case_uuid).unwrap(), 1);
    assert_eq!(db.count_provenance_events(case_uuid).unwrap(), 1);
}
