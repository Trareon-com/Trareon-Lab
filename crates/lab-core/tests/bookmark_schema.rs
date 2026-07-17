//! Bookmark schema fixture tests (Official Day 04 alias).
//!
//! Thin alias over `schema_fixtures` bookmark cases so the Official plan
//! verification command `cargo test -p lab-core bookmark_schema` resolves.

use lab_core::schema_validate::{validate_fixture_file, FailureKind, SchemaKind};
use std::path::PathBuf;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/contracts")
        .join(name)
}

#[test]
fn bookmark_valid_fixture_passes() {
    validate_fixture_file(SchemaKind::Bookmark, &fixture("bookmark.valid.json"))
        .expect("bookmark.valid");
}

#[test]
fn bookmark_invalid_fixtures_fail_closed() {
    let missing = validate_fixture_file(
        SchemaKind::Bookmark,
        &fixture("bookmark.invalid-missing-field.json"),
    )
    .expect_err("missing field must fail");
    assert_eq!(missing.kind, FailureKind::Missing);

    let bad_enum =
        validate_fixture_file(SchemaKind::Bookmark, &fixture("bookmark.invalid-enum.json"))
            .expect_err("bad enum must fail");
    assert_eq!(bad_enum.kind, FailureKind::Enum);

    let integrity = validate_fixture_file(
        SchemaKind::Bookmark,
        &fixture("bookmark.invalid-integrity-link.json"),
    )
    .expect_err("integrity link must fail");
    assert_eq!(integrity.kind, FailureKind::Integrity);
}

#[test]
fn transfer_valid_and_invalid_fixtures() {
    validate_fixture_file(SchemaKind::TransferPackage, &fixture("transfer.valid.json"))
        .expect("transfer.valid");
    validate_fixture_file(
        SchemaKind::TransferPackage,
        &fixture("transfer.invalid-missing-field.json"),
    )
    .expect_err("transfer missing field");
}
