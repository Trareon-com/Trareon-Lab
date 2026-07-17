//! Day 6: persist 10k index rows across reopen.

use lab_index::{IndexDb, IndexEntry};
use tempfile::tempdir;

#[test]
fn ten_thousand_rows_survive_reopen() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("index.sqlite");
    let case_uuid = "11111111-1111-4111-8111-111111111111";

    {
        let mut db = IndexDb::open_and_migrate(&path).expect("open");
        assert_eq!(db.schema_version(), 1);

        let mut entries = Vec::with_capacity(10_000);
        for i in 0..10_000 {
            entries.push(IndexEntry {
                case_uuid: case_uuid.to_string(),
                entry_kind: "fs_path".to_string(),
                target_ref: format!("/Windows/System32/file-{i}.dll"),
                display_text: format!("file-{i}.dll"),
                sort_key: format!("{i:08}"),
                created_at_utc: "2026-07-17T00:00:00Z".to_string(),
            });
        }
        db.insert_entries_batch(&entries).expect("batch insert");
        assert_eq!(db.count_for_case(case_uuid).expect("count"), 10_000);
        let first = db.list_for_case(case_uuid, 1, 0).expect("first");
        assert_eq!(first[0].display_text, "file-0.dll");
        // Drop closes the connection.
    }

    let reopened = IndexDb::open_and_migrate(&path).expect("reopen");
    assert_eq!(reopened.schema_version(), 1);
    assert_eq!(
        reopened.count_for_case(case_uuid).expect("count reopen"),
        10_000
    );
    let last = reopened
        .list_for_case(case_uuid, 1, 9_999)
        .expect("last page");
    assert_eq!(last.len(), 1);
    assert_eq!(last[0].display_text, "file-9999.dll");
    assert_eq!(last[0].sort_key, "00009999");
}
