//! Day 33 bookmarks + Day 37 findings.

use lab_case::{BookmarkRecord, CaseDb};
use tempfile::tempdir;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    pub finding_uuid: String,
    pub claim: String,
    pub bookmark_uuid: String,
}

#[test]
fn bookmark_crud_and_finding_link() {
    let dir = tempdir().unwrap();
    let db = CaseDb::open_and_migrate(&dir.path().join("c.sqlite")).unwrap();
    assert_eq!(db.schema_version(), 4);
    let case = "11111111-1111-4111-8111-111111111111";
    db.upsert_bookmark(&BookmarkRecord {
        bookmark_uuid: "33333333-3333-4333-8333-333333333333".into(),
        case_uuid: case.into(),
        target_kind: "fs_path".into(),
        target_ref: r"\a.txt".into(),
        citation: "cite".into(),
        author_role: "examiner".into(),
        created_at_utc: "2026-07-17T00:00:00Z".into(),
        review_state: "open".into(),
        note: None,
    })
    .unwrap();
    let list = db.list_bookmarks(case).unwrap();
    assert_eq!(list.len(), 1);
    db.supersede_bookmark("33333333-3333-4333-8333-333333333333")
        .unwrap();
    assert_eq!(
        db.list_bookmarks(case).unwrap()[0].review_state,
        "superseded"
    );
    let finding = Finding {
        finding_uuid: "f1".into(),
        claim: "exe ran".into(),
        bookmark_uuid: list[0].bookmark_uuid.clone(),
    };
    assert_eq!(
        finding.bookmark_uuid,
        "33333333-3333-4333-8333-333333333333"
    );
}
