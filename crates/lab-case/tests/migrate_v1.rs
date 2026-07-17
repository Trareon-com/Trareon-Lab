//! F3: empty case dir migrates v0→current and reopens.

use lab_case::db::CaseDb;
use tempfile::tempdir;

#[test]
fn migrate_empty_case_dir_v0_to_current_and_reopen() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("case.db");

    {
        let db = CaseDb::open_and_migrate(&path).expect("migrate");
        assert_eq!(db.schema_version(), 4);
    }

    let db = CaseDb::open_and_migrate(&path).expect("reopen");
    assert_eq!(db.schema_version(), 4);
}
