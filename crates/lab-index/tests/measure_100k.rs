//! Day 10: measure 100k index insert + reopen (record numbers in docs).

use lab_index::{IndexDb, IndexEntry};
use std::time::Instant;
use tempfile::tempdir;

#[test]
fn measure_one_hundred_thousand_index_rows() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("index-100k.sqlite");
    let case_uuid = "11111111-1111-4111-8111-111111111111";

    let insert_started = Instant::now();
    {
        let mut db = IndexDb::open_and_migrate(&path).expect("open");
        let mut entries = Vec::with_capacity(100_000);
        for i in 0..100_000 {
            entries.push(IndexEntry {
                case_uuid: case_uuid.to_string(),
                entry_kind: "fs_path".to_string(),
                target_ref: format!("/path/{i}"),
                display_text: format!("f-{i}"),
                sort_key: format!("{i:08}"),
                created_at_utc: "2026-07-17T00:00:00Z".to_string(),
            });
        }
        db.insert_entries_batch(&entries).expect("batch");
        assert_eq!(db.count_for_case(case_uuid).unwrap(), 100_000);
    }
    let insert_ms = insert_started.elapsed().as_millis();

    let reopen_started = Instant::now();
    let reopened = IndexDb::open_and_migrate(&path).expect("reopen");
    assert_eq!(reopened.count_for_case(case_uuid).unwrap(), 100_000);
    let last = reopened.list_for_case(case_uuid, 1, 99_999).unwrap();
    assert_eq!(last[0].display_text, "f-99999");
    let reopen_ms = reopen_started.elapsed().as_millis();

    // Soft ceiling so CI stays useful; absolute numbers vary by runner.
    assert!(
        insert_ms < 120_000,
        "100k insert took {insert_ms}ms (expected <120s)"
    );
    assert!(
        reopen_ms < 30_000,
        "100k reopen/count took {reopen_ms}ms (expected <30s)"
    );

    eprintln!("day10_index_100k insert_ms={insert_ms} reopen_ms={reopen_ms}");
}
