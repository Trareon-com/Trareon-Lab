//! F6: valid fixtures pass; invalid fixtures fail with classified reasons.

use lab_core::schema_validate::{validate_fixture_file, FailureKind, SchemaKind};
use std::path::PathBuf;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/contracts")
        .join(name)
}

#[test]
fn valid_evidence_and_case_fixtures_pass() {
    validate_fixture_file(SchemaKind::EvidenceObject, &fixture("evidence.valid.json"))
        .expect("evidence.valid");
    validate_fixture_file(SchemaKind::Case, &fixture("case.valid.json")).expect("case.valid");
}

#[test]
fn invalid_fixtures_fail_with_classified_kinds() {
    let missing = validate_fixture_file(
        SchemaKind::EvidenceObject,
        &fixture("evidence.invalid-missing-field.json"),
    )
    .expect_err("missing");
    assert_eq!(missing.kind, FailureKind::Missing);

    let enum_err = validate_fixture_file(
        SchemaKind::EvidenceObject,
        &fixture("evidence.invalid-enum.json"),
    )
    .expect_err("enum");
    assert_eq!(enum_err.kind, FailureKind::Enum);

    let integrity = validate_fixture_file(
        SchemaKind::EvidenceObject,
        &fixture("evidence.invalid-integrity-link.json"),
    )
    .expect_err("integrity");
    assert_eq!(integrity.kind, FailureKind::Integrity);

    let case_missing = validate_fixture_file(
        SchemaKind::Case,
        &fixture("case.invalid-missing-field.json"),
    )
    .expect_err("case missing");
    assert_eq!(case_missing.kind, FailureKind::Missing);
}

#[test]
fn valid_bookmark_and_transfer_fixtures_pass() {
    validate_fixture_file(SchemaKind::Bookmark, &fixture("bookmark.valid.json"))
        .expect("bookmark.valid");
    validate_fixture_file(SchemaKind::TransferPackage, &fixture("transfer.valid.json"))
        .expect("transfer.valid");
}

#[test]
fn invalid_bookmark_fixtures_fail_with_classified_kinds() {
    let missing = validate_fixture_file(
        SchemaKind::Bookmark,
        &fixture("bookmark.invalid-missing-field.json"),
    )
    .expect_err("bookmark missing");
    assert_eq!(missing.kind, FailureKind::Missing);

    let enum_err =
        validate_fixture_file(SchemaKind::Bookmark, &fixture("bookmark.invalid-enum.json"))
            .expect_err("bookmark enum");
    assert_eq!(enum_err.kind, FailureKind::Enum);

    let integrity = validate_fixture_file(
        SchemaKind::Bookmark,
        &fixture("bookmark.invalid-integrity-link.json"),
    )
    .expect_err("bookmark integrity");
    assert_eq!(integrity.kind, FailureKind::Integrity);
}

#[test]
fn invalid_transfer_fixtures_fail_with_classified_kinds() {
    let missing = validate_fixture_file(
        SchemaKind::TransferPackage,
        &fixture("transfer.invalid-missing-field.json"),
    )
    .expect_err("transfer missing");
    assert_eq!(missing.kind, FailureKind::Missing);

    let enum_err = validate_fixture_file(
        SchemaKind::TransferPackage,
        &fixture("transfer.invalid-enum.json"),
    )
    .expect_err("transfer enum");
    assert_eq!(enum_err.kind, FailureKind::Enum);

    let integrity = validate_fixture_file(
        SchemaKind::TransferPackage,
        &fixture("transfer.invalid-integrity-link.json"),
    )
    .expect_err("transfer integrity");
    assert_eq!(integrity.kind, FailureKind::Integrity);
}
