//! E01 round-trip and import tests.

use lab_case::CaseDb;
use lab_storage::{
    detect_image_kind, import_raw_image, write_simple_e01, E01Image, E01Metadata, ImageKind,
    ImageReader,
};
use std::io::Write;

#[test]
fn e01_round_trip_matches_raw() {
    let dir = tempfile::tempdir().unwrap();
    let media = {
        let mut v = vec![0u8; 130_000];
        v[0] = 0xEB;
        v[1000] = 0x42;
        v[129_999] = 0xAA;
        v
    };
    let e01_path = dir.path().join("disk.E01");
    let meta = E01Metadata {
        case_number: "CASE-1".into(),
        examiner: "Ada".into(),
        evidence_number: "E-1".into(),
        description: "fixture".into(),
        hash_algorithms: vec!["SHA256".into()],
    };
    write_simple_e01(&e01_path, &media, &meta, 64 * 1024).unwrap();

    assert_eq!(detect_image_kind(&e01_path).unwrap(), ImageKind::E01);
    let mut e01 = E01Image::open(&e01_path).unwrap();
    assert_eq!(e01.metadata().case_number, "CASE-1");
    assert_eq!(e01.byte_length(), media.len() as u64);

    let mut out = vec![0u8; media.len()];
    let n = e01.read_at(0, &mut out).unwrap();
    assert_eq!(n, media.len());
    assert_eq!(out, media);

    let mut progress = lab_core::NullProgress;
    let report = e01.verify_integrity(&mut progress).unwrap();
    assert!(report.ok, "{}", report.message);
}

#[test]
fn import_e01_records_metadata() {
    let dir = tempfile::tempdir().unwrap();
    let media = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
    let e01_path = dir.path().join("small.E01");
    write_simple_e01(
        &e01_path,
        &media,
        &E01Metadata {
            case_number: "C".into(),
            examiner: "E".into(),
            evidence_number: "1".into(),
            description: "d".into(),
            hash_algorithms: vec!["MD5".into(), "SHA1".into()],
        },
        512,
    )
    .unwrap();

    let db = CaseDb::open_and_migrate(&dir.path().join("case.sqlite")).unwrap();
    let ok = import_raw_image(
        &db,
        "case-uuid",
        &e01_path,
        "ev-1",
        "prov-1",
        "2026-01-01T00:00:00Z",
    )
    .expect("import");
    assert_eq!(ok.byte_length, media.len() as u64);
    assert_eq!(ok.sha256_hex.len(), 64);
    assert_eq!(db.count_provenance_events("case-uuid").unwrap(), 1);
}

#[test]
fn write_fixture_under_fixtures_dir() {
    // Keep a tiny checked-in style fixture generation path for CI.
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("fixture.E01");
    write_simple_e01(
        &path,
        b"NTFS_BOOT_PLACEHOLDER_DATA_XXXX",
        &E01Metadata {
            case_number: "FIXTURE".into(),
            examiner: "ci".into(),
            evidence_number: "0".into(),
            description: "generated".into(),
            hash_algorithms: vec!["SHA256".into()],
        },
        512,
    )
    .unwrap();
    assert!(path.metadata().unwrap().len() > 8);
    let _ = std::io::stdout().write_all(b"");
}
