//! Day 43: windowed navigation over 1M synthetic index rows.

use lab_index::{IndexDb, IndexEntry};
use std::time::Instant;
use tempfile::tempdir;

#[test]
fn windowed_nav_one_million_rows() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("big.sqlite");
    let case = "11111111-1111-4111-8111-111111111111";
    let mut db = IndexDb::open_and_migrate(&path).unwrap();

    let t0 = Instant::now();
    // Insert in chunks to keep memory bounded.
    for chunk in 0..100 {
        let mut batch = Vec::with_capacity(10_000);
        for i in 0..10_000 {
            let n = chunk * 10_000 + i;
            batch.push(IndexEntry {
                case_uuid: case.into(),
                entry_kind: "fs_path".into(),
                target_ref: format!("/p/{n}"),
                display_text: format!("f-{n}"),
                sort_key: format!("{n:08}"),
                created_at_utc: "2026-07-17T00:00:00Z".into(),
            });
        }
        db.insert_entries_batch(&batch).unwrap();
    }
    let insert_ms = t0.elapsed().as_millis();
    assert_eq!(db.count_for_case(case).unwrap(), 1_000_000);

    let t1 = Instant::now();
    let page = db.list_for_case(case, 100, 500_000).unwrap();
    let page_ms = t1.elapsed().as_millis();
    assert_eq!(page.len(), 100);
    assert_eq!(page[0].display_text, "f-500000");

    let found = db.query_path_name_hash(case, "f-999999", 5).unwrap();
    assert_eq!(found[0].display_text, "f-999999");

    eprintln!("day43_1m insert_ms={insert_ms} page_ms={page_ms}");
    assert!(insert_ms < 300_000, "insert too slow: {insert_ms}");
    assert!(page_ms < 5_000, "page too slow: {page_ms}");
}
