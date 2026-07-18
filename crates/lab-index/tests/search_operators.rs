use lab_index::{IndexDb, IndexEntry, SearchPlan, SearchQuery};
use tempfile::tempdir;

fn seeded_db() -> (tempfile::TempDir, IndexDb) {
    let dir = tempdir().unwrap();
    let mut db = IndexDb::open_and_migrate(&dir.path().join("search.sqlite")).unwrap();
    let rows = [
        ("/docs/foo bar.txt", "foo bar report"),
        ("/logs/foo-only", "system.log"),
        ("/bytes/deadbeef.bin", "payload"),
        ("/other/bar", "closing report"),
    ]
    .into_iter()
    .enumerate()
    .map(|(i, (target_ref, display_text))| IndexEntry {
        case_uuid: "case-1".into(),
        entry_kind: "fs_path".into(),
        target_ref: target_ref.into(),
        display_text: display_text.into(),
        sort_key: format!("{i:08}"),
        created_at_utc: "2026-07-18T00:00:00Z".into(),
    })
    .collect::<Vec<_>>();
    db.insert_entries_batch(&rows).unwrap();
    (dir, db)
}

#[test]
fn phrase_boolean_wildcard_and_hex_search() {
    let (_dir, db) = seeded_db();

    let phrase = db
        .search("case-1", &SearchQuery::parse("\"foo bar\"").unwrap(), 10)
        .unwrap();
    assert_eq!(phrase.entries.len(), 1);

    let boolean = db
        .search(
            "case-1",
            &SearchQuery::parse("foo AND bar OR hex:deadbeef").unwrap(),
            10,
        )
        .unwrap();
    assert_eq!(boolean.entries.len(), 2);

    let prefix = db
        .search("case-1", &SearchQuery::parse("sys*").unwrap(), 10)
        .unwrap();
    assert_eq!(prefix.entries[0].display_text, "system.log");

    let suffix = db
        .search("case-1", &SearchQuery::parse("*report").unwrap(), 10)
        .unwrap();
    assert_eq!(suffix.entries.len(), 2);
}

#[test]
fn truncation_and_invalid_hex_are_explicit() {
    let (_dir, db) = seeded_db();
    let result = db
        .search("case-1", &SearchQuery::parse("foo OR bar").unwrap(), 1)
        .unwrap();
    assert_eq!(result.entries.len(), 1);
    assert!(result.truncated);
    assert!(result
        .truncation_reason
        .as_deref()
        .unwrap()
        .contains("coverage is partial"));

    assert!(SearchQuery::parse("hex:abc").is_err());
}

#[test]
fn search_plan_round_trips_as_json() {
    let plan = SearchPlan {
        query: "\"foo bar\"".into(),
        scope: "case:case-1".into(),
        encodings: vec!["UTF-8".into()],
        error_count: 0,
        hit_count: 1,
        tags: vec!["reviewed".into()],
    };
    let json = serde_json::to_string(&plan).unwrap();
    assert_eq!(serde_json::from_str::<SearchPlan>(&json).unwrap(), plan);
}
