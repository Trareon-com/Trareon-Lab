use lab_artifacts::{write_minimal_evtx, write_minimal_hive, EvtxFile, HiveFile};
use lab_core::NullProgress;

#[test]
fn registry_opens_fixture() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("SYSTEM");
    write_minimal_hive(&path).unwrap();
    let hive = HiveFile::open_path(&path).unwrap();
    assert_eq!(hive.base_block.file_name, "SYSTEM");
    let keys = hive.enumerate_keys(&mut NullProgress, 10).unwrap();
    assert!(!keys.is_empty());
}

#[test]
fn evtx_reads_event_id() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("Security.evtx");
    write_minimal_evtx(&path).unwrap();
    let f = EvtxFile::open_path(&path).unwrap();
    let events = f.events(&mut NullProgress).unwrap();
    assert!(events.iter().any(|e| e.event_id == 42), "{events:?}");
}
