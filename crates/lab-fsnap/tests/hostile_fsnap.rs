//! F8: hostile .fsnap preflight rejects; happy path imports deterministically.

use lab_core::LabError;
use lab_fsnap::import::import_package;
use lab_fsnap::preflight::{PreflightLimits, preflight_package};
use serde_json::json;
use std::fs;
use tempfile::tempdir;

fn write_pkg(root: &std::path::Path, manifest: serde_json::Value, files: &[(&str, &[u8])]) {
    fs::create_dir_all(root).unwrap();
    fs::write(root.join("manifest.json"), serde_json::to_vec_pretty(&manifest).unwrap()).unwrap();
    for (name, bytes) in files {
        let path = root.join(name);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, bytes).unwrap();
    }
}

#[test]
fn rejects_path_traversal_duplicate_bomb_and_partial() {
    let dir = tempdir().unwrap();

    // Path traversal
    let trav = dir.path().join("trav");
    write_pkg(
        &trav,
        json!({
            "package_version": "1.0",
            "members": ["../escape.bin"]
        }),
        &[("../escape.bin", b"x")],
    );
    let err = preflight_package(&trav, PreflightLimits::default()).expect_err("traversal");
    assert!(matches!(err, LabError::FsNapRejected { .. }));

    // Duplicate members
    let dup = dir.path().join("dup");
    write_pkg(
        &dup,
        json!({
            "package_version": "1.0",
            "members": ["a.bin", "a.bin"]
        }),
        &[("a.bin", b"x")],
    );
    let err = preflight_package(&dup, PreflightLimits::default()).expect_err("dup");
    assert!(matches!(err, LabError::FsNapRejected { .. }));

    // Bomb limit (declared member too large)
    let bomb = dir.path().join("bomb");
    write_pkg(
        &bomb,
        json!({
            "package_version": "1.0",
            "members": ["big.bin"]
        }),
        &[("big.bin", &[0u8; 64])],
    );
    let err = preflight_package(
        &bomb,
        PreflightLimits {
            max_member_bytes: 16,
            max_total_bytes: 1024,
        },
    )
    .expect_err("bomb");
    assert!(matches!(err, LabError::FsNapRejected { .. }));

    // Partial package (member listed but missing on disk)
    let partial = dir.path().join("partial");
    write_pkg(
        &partial,
        json!({
            "package_version": "1.0",
            "members": ["missing.bin"]
        }),
        &[],
    );
    let err = preflight_package(&partial, PreflightLimits::default()).expect_err("partial");
    assert!(matches!(err, LabError::FsNapRejected { .. }));
}

#[test]
fn happy_path_deterministic_import_result() {
    let dir = tempdir().unwrap();
    let pkg = dir.path().join("ok");
    write_pkg(
        &pkg,
        json!({
            "package_version": "1.0",
            "members": ["a.bin"]
        }),
        &[("a.bin", b"hello")],
    );
    preflight_package(&pkg, PreflightLimits::default()).expect("preflight");
    let case_uuid = "11111111-1111-4111-8111-111111111111";
    let a = import_package(&pkg, case_uuid, "00000000-0000-4000-8000-000000000001").unwrap();
    let b = import_package(&pkg, case_uuid, "00000000-0000-4000-8000-000000000001").unwrap();
    assert_eq!(a, b);
    assert_eq!(a["result"], "accepted");
    assert_eq!(a["schema_version"], "fsnap-import-result-1");
    assert_eq!(a["case_uuid"], case_uuid);
}
