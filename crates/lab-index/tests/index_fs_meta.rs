//! Day 19: FS metadata lands in index.

use lab_index::{index_fs_entries, IndexDb};
use tempfile::tempdir;

#[test]
fn fs_metadata_indexed_and_listed() {
    let dir = tempdir().unwrap();
    let mut db = IndexDb::open_and_migrate(&dir.path().join("idx.sqlite")).unwrap();
    let case = "11111111-1111-4111-8111-111111111111";
    let n = index_fs_entries(
        &mut db,
        case,
        "2026-07-17T00:00:00Z",
        &[
            (
                "fs_path".into(),
                r"\Windows\System32\notepad.exe".into(),
                "notepad.exe".into(),
            ),
            ("fs_path".into(), r"\readme.txt".into(), "readme.txt".into()),
        ],
    )
    .unwrap();
    assert_eq!(n, 2);
    assert_eq!(db.count_for_case(case).unwrap(), 2);
    let page = db.list_for_case(case, 10, 0).unwrap();
    assert_eq!(page[0].display_text, "notepad.exe");
    assert_eq!(page[1].entry_kind, "fs_path");
}
