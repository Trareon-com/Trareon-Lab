use lab_core::NullProgress;
use lab_hashset::{hash_set_version_pin, HashLookupResult, HashSetDb, HASH_SET_FORMAT_VERSION};
use std::io::Write;

#[test]
fn nsrl_and_known_bad_lookup() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("hashes.db");
    let mut db = HashSetDb::open(&db_path).unwrap();

    let nsrl = dir.path().join("nsrl.csv");
    {
        let mut f = std::fs::File::create(&nsrl).unwrap();
        writeln!(f, "sha256,name,product").unwrap();
        writeln!(
            f,
            "aabbccddeeff00112233445566778899aabbccddeeff00112233445566778899,notepad.exe,Windows"
        )
        .unwrap();
    }
    let n = db.import_nsrl(&nsrl, &mut NullProgress).unwrap();
    assert_eq!(n, 1);

    let bad = dir.path().join("bad.csv");
    std::fs::write(
        &bad,
        "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef,trojan\n",
    )
    .unwrap();
    assert_eq!(db.import_known_bad(&bad).unwrap(), 1);

    match db
        .lookup("aabbccddeeff00112233445566778899aabbccddeeff00112233445566778899")
        .unwrap()
    {
        HashLookupResult::KnownGood { name, .. } => assert_eq!(name, "notepad.exe"),
        other => panic!("{other:?}"),
    }
    match db
        .lookup("deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef")
        .unwrap()
    {
        HashLookupResult::KnownBad { threat } => assert_eq!(threat, "trojan"),
        other => panic!("{other:?}"),
    }
    assert_eq!(
        db.lookup("0000000000000000000000000000000000000000000000000000000000000000")
            .unwrap(),
        HashLookupResult::Unknown
    );
}

#[test]
fn hash_set_pin_contains_format_version_and_file_sha256() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("hashes.db");
    std::fs::write(&path, b"abc").unwrap();

    assert_eq!(
        hash_set_version_pin(&path).unwrap(),
        concat!(
            "trareon-hashset-db-1:sha256:",
            "ba7816bf8f01cfea414140de5dae2223",
            "b00361a396177a9cb410ff61f20015ad"
        )
    );
    assert_eq!(HASH_SET_FORMAT_VERSION, "trareon-hashset-db-1");
}
