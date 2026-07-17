//! Day 18: read synthetic file bytes → CAS; roundtrip bytes must match.

use lab_fs::{
    ingest_synth_file_to_cas, read_synth_file_bytes, write_fs_content_synthetic, SynthFileContent,
};
use lab_store::CasStore;
use sha2::{Digest, Sha256};
use tempfile::tempdir;

#[test]
fn read_file_into_cas_bytes_match() {
    let dir = tempdir().expect("dir");
    let content_path = dir.path().join("payloads.trfsct");
    let payload = b"TRA REON-DAY18-FILE-BYTES\nline-2\n".to_vec();

    write_fs_content_synthetic(
        &content_path,
        &[
            SynthFileContent {
                record_number: 200,
                bytes: payload.clone(),
            },
            SynthFileContent {
                record_number: 201,
                bytes: b"other".to_vec(),
            },
        ],
    )
    .expect("write content");

    let raw = read_synth_file_bytes(&content_path, 200).expect("read");
    assert_eq!(raw, payload);

    let cas = CasStore::open(dir.path()).expect("cas");
    let ingested = ingest_synth_file_to_cas(&content_path, 200, &cas).expect("ingest");
    assert_eq!(ingested.record_number, 200);
    assert_eq!(ingested.byte_length, payload.len() as u64);
    assert_eq!(ingested.digest_hex, hex::encode(Sha256::digest(&payload)));

    let from_cas = cas.get(&ingested.digest_hex).expect("get");
    assert_eq!(from_cas, payload);
}

#[test]
fn missing_record_fails_closed() {
    let dir = tempdir().expect("dir");
    let content_path = dir.path().join("emptyish.trfsct");
    write_fs_content_synthetic(
        &content_path,
        &[SynthFileContent {
            record_number: 1,
            bytes: b"x".to_vec(),
        }],
    )
    .expect("write");
    let err = read_synth_file_bytes(&content_path, 999).expect_err("missing");
    assert!(format!("{err:?}").contains("missing"));
}

#[test]
fn bad_content_magic_fails_closed() {
    let dir = tempdir().expect("dir");
    let path = dir.path().join("bad.trfsct");
    std::fs::write(&path, b"NOTMAGIC").expect("write");
    let err = read_synth_file_bytes(&path, 1).expect_err("magic");
    assert!(format!("{err:?}").contains("magic"));
}
