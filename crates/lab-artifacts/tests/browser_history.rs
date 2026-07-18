use lab_artifacts::browser;
use lab_core::NullProgress;
use rusqlite::Connection;

#[test]
fn chrome_history_from_sqlite() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("History");
    let conn = Connection::open(&path).unwrap();
    conn.execute_batch(
        "CREATE TABLE urls (
            id INTEGER PRIMARY KEY,
            url TEXT,
            title TEXT,
            visit_count INTEGER,
            typed_count INTEGER,
            last_visit_time INTEGER
         );
         INSERT INTO urls(url,title,visit_count,typed_count,last_visit_time)
         VALUES ('https://example.com','Example',3,1,13300000000000000);",
    )
    .unwrap();
    drop(conn);
    let hist = browser::chrome::parse_history(&path, &mut NullProgress).unwrap();
    assert_eq!(hist.len(), 1);
    assert_eq!(hist[0].url, "https://example.com");
}
