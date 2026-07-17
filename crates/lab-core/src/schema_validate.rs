//! Foundation record validation against `schemas/*.json` contracts.
//!
//! Uses typed serde checks that mirror the JSON Schema 2020-12 documents
//! (required fields, enums, const schema_version, non-negative integrity fields).

use std::fs;
use std::path::Path;

use serde::Deserialize;
use serde_json::Value;

use crate::error::LabError;

/// Which foundation schema to apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaKind {
    Case,
    EvidenceObject,
    ProvenanceEvent,
    AuditEvent,
    CoverageRecord,
    Bookmark,
    TransferPackage,
}

impl SchemaKind {
    fn label(self) -> &'static str {
        match self {
            Self::Case => "case",
            Self::EvidenceObject => "evidence-object",
            Self::ProvenanceEvent => "provenance-event",
            Self::AuditEvent => "audit-event",
            Self::CoverageRecord => "coverage-record",
            Self::Bookmark => "bookmark",
            Self::TransferPackage => "transfer-package",
        }
    }

    fn schema_path(self) -> std::path::PathBuf {
        let name = match self {
            Self::Case => "case.schema.json",
            Self::EvidenceObject => "evidence-object.schema.json",
            Self::ProvenanceEvent => "provenance-event.schema.json",
            Self::AuditEvent => "audit-event.schema.json",
            Self::CoverageRecord => "coverage-record.schema.json",
            Self::Bookmark => "bookmark.schema.json",
            Self::TransferPackage => "transfer-package.schema.json",
        };
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../schemas")
            .join(name)
    }
}

/// Classified schema failure for tests and UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailureKind {
    Missing,
    Enum,
    Integrity,
    Other,
}

/// Structured validation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaFailure {
    pub kind: FailureKind,
    pub detail: String,
}

impl From<SchemaFailure> for LabError {
    fn from(value: SchemaFailure) -> Self {
        LabError::SchemaInvalid {
            schema: "foundation".into(),
            detail: format!("{:?}: {}", value.kind, value.detail),
        }
    }
}

fn fail(kind: FailureKind, detail: impl Into<String>) -> SchemaFailure {
    SchemaFailure {
        kind,
        detail: detail.into(),
    }
}

fn ensure_schema_file(kind: SchemaKind) -> Result<(), SchemaFailure> {
    let path = kind.schema_path();
    if !path.is_file() {
        return Err(fail(
            FailureKind::Other,
            format!("schema missing: {}", path.display()),
        ));
    }
    Ok(())
}

fn require_str<'a>(
    obj: &'a serde_json::Map<String, Value>,
    key: &str,
) -> Result<&'a str, SchemaFailure> {
    match obj.get(key) {
        None => Err(fail(
            FailureKind::Missing,
            format!("missing required field `{key}`"),
        )),
        Some(Value::String(s)) => Ok(s.as_str()),
        Some(_) => Err(fail(FailureKind::Other, format!("`{key}` must be string"))),
    }
}

fn require_i64(obj: &serde_json::Map<String, Value>, key: &str) -> Result<i64, SchemaFailure> {
    match obj.get(key) {
        None => Err(fail(
            FailureKind::Missing,
            format!("missing required field `{key}`"),
        )),
        Some(Value::Number(n)) => n
            .as_i64()
            .ok_or_else(|| fail(FailureKind::Other, format!("`{key}` must be integer"))),
        Some(_) => Err(fail(FailureKind::Other, format!("`{key}` must be integer"))),
    }
}

fn require_enum(value: &str, allowed: &[&str], field: &str) -> Result<(), SchemaFailure> {
    if allowed.contains(&value) {
        Ok(())
    } else {
        Err(fail(
            FailureKind::Enum,
            format!("`{field}` value `{value}` not in enum"),
        ))
    }
}

fn as_object(value: &Value) -> Result<&serde_json::Map<String, Value>, SchemaFailure> {
    value
        .as_object()
        .ok_or_else(|| fail(FailureKind::Other, "root must be object"))
}

fn validate_evidence(obj: &serde_json::Map<String, Value>) -> Result<(), SchemaFailure> {
    let sv = require_str(obj, "schema_version")?;
    if sv != "evidence-object-1" {
        return Err(fail(
            FailureKind::Enum,
            format!("schema_version const mismatch: {sv}"),
        ));
    }
    let _ = require_str(obj, "evidence_uuid")?;
    let _ = require_str(obj, "case_uuid")?;
    require_enum(
        require_str(obj, "source_designation")?,
        &[
            "original",
            "forensic_copy",
            "verified_working_copy",
            "logical_view",
            "derivative",
        ],
        "source_designation",
    )?;
    let _ = require_str(obj, "evidence_class")?;
    let byte_length = require_i64(obj, "byte_length")?;
    if byte_length < 0 {
        return Err(fail(
            FailureKind::Integrity,
            format!("byte_length integrity minimum violated: {byte_length}"),
        ));
    }
    require_enum(
        require_str(obj, "validation_state")?,
        &[
            "VerifiedMatch",
            "VerifiedMismatch",
            "ComputedUnanchored",
            "SignatureInvalid",
            "IncompleteInput",
            "Unsupported",
        ],
        "validation_state",
    )?;
    Ok(())
}

fn looks_like_uuid(s: &str) -> bool {
    let parts: Vec<&str> = s.split('-').collect();
    parts.len() == 5
        && parts[0].len() == 8
        && parts[1].len() == 4
        && parts[2].len() == 4
        && parts[3].len() == 4
        && parts[4].len() == 12
        && s.chars().all(|c| c.is_ascii_hexdigit() || c == '-')
}

fn validate_case(obj: &serde_json::Map<String, Value>) -> Result<(), SchemaFailure> {
    let sv = require_str(obj, "schema_version")?;
    if sv != "case-1" {
        return Err(fail(
            FailureKind::Enum,
            format!("schema_version const mismatch: {sv}"),
        ));
    }
    let case_uuid = require_str(obj, "case_uuid")?;
    if !looks_like_uuid(case_uuid) {
        return Err(fail(
            FailureKind::Integrity,
            format!("case_uuid integrity/format violated: {case_uuid}"),
        ));
    }
    require_enum(
        require_str(obj, "state")?,
        &[
            "CREATED",
            "OPEN",
            "READ_ONLY",
            "RECOVERY_REQUIRED",
            "CLOSED",
            "ARCHIVED",
        ],
        "state",
    )?;
    let _ = require_str(obj, "created_at_utc")?;
    let title = require_str(obj, "title")?;
    if title.is_empty() {
        return Err(fail(FailureKind::Integrity, "title minLength violated"));
    }
    Ok(())
}

fn validate_generic_required(
    obj: &serde_json::Map<String, Value>,
    required: &[&str],
    schema_version: &str,
) -> Result<(), SchemaFailure> {
    let sv = require_str(obj, "schema_version")?;
    if sv != schema_version {
        return Err(fail(
            FailureKind::Enum,
            format!("schema_version const mismatch: {sv}"),
        ));
    }
    for key in required {
        if *key == "schema_version" {
            continue;
        }
        if !obj.contains_key(*key) {
            return Err(fail(
                FailureKind::Missing,
                format!("missing required field `{key}`"),
            ));
        }
    }
    Ok(())
}

fn is_sha256_hex(s: &str) -> bool {
    s.len() == 64 && s.chars().all(|c| c.is_ascii_hexdigit())
}

fn validate_bookmark(obj: &serde_json::Map<String, Value>) -> Result<(), SchemaFailure> {
    validate_generic_required(
        obj,
        &[
            "schema_version",
            "bookmark_uuid",
            "case_uuid",
            "target_kind",
            "target_ref",
            "citation",
            "author_role",
            "created_at_utc",
            "review_state",
        ],
        "bookmark-1",
    )?;
    let bookmark_uuid = require_str(obj, "bookmark_uuid")?;
    if !looks_like_uuid(bookmark_uuid) {
        return Err(fail(
            FailureKind::Integrity,
            format!("bookmark_uuid integrity/format violated: {bookmark_uuid}"),
        ));
    }
    let case_uuid = require_str(obj, "case_uuid")?;
    if !looks_like_uuid(case_uuid) {
        return Err(fail(
            FailureKind::Integrity,
            format!("case_uuid integrity/format violated: {case_uuid}"),
        ));
    }
    require_enum(
        require_str(obj, "target_kind")?,
        &[
            "evidence_object",
            "artifact_hit",
            "timeline_event",
            "fs_path",
            "byte_range",
        ],
        "target_kind",
    )?;
    let target_ref = require_str(obj, "target_ref")?;
    if target_ref.is_empty() {
        return Err(fail(
            FailureKind::Integrity,
            "target_ref minLength violated",
        ));
    }
    let citation = require_str(obj, "citation")?;
    if citation.is_empty() {
        return Err(fail(FailureKind::Integrity, "citation minLength violated"));
    }
    let author_role = require_str(obj, "author_role")?;
    if author_role.is_empty() {
        return Err(fail(
            FailureKind::Integrity,
            "author_role minLength violated",
        ));
    }
    let _ = require_str(obj, "created_at_utc")?;
    require_enum(
        require_str(obj, "review_state")?,
        &["open", "accepted", "rejected", "superseded"],
        "review_state",
    )?;
    if let Some(Value::Object(range)) = obj.get("byte_range") {
        let start = require_i64(range, "start")?;
        let end = require_i64(range, "end")?;
        if start < 0 || end < start {
            return Err(fail(
                FailureKind::Integrity,
                format!("byte_range integrity violated: start={start} end={end}"),
            ));
        }
    }
    Ok(())
}

fn validate_transfer_package(obj: &serde_json::Map<String, Value>) -> Result<(), SchemaFailure> {
    validate_generic_required(
        obj,
        &[
            "schema_version",
            "transfer_uuid",
            "source_case_uuid",
            "created_at_utc",
            "destination",
            "purpose",
            "authority_note",
            "selected_bookmark_digests",
            "result",
            "signature",
        ],
        "transfer-package-1",
    )?;
    let transfer_uuid = require_str(obj, "transfer_uuid")?;
    if !looks_like_uuid(transfer_uuid) {
        return Err(fail(
            FailureKind::Integrity,
            format!("transfer_uuid integrity/format violated: {transfer_uuid}"),
        ));
    }
    let source_case_uuid = require_str(obj, "source_case_uuid")?;
    if !looks_like_uuid(source_case_uuid) {
        return Err(fail(
            FailureKind::Integrity,
            format!("source_case_uuid integrity/format violated: {source_case_uuid}"),
        ));
    }
    for key in ["destination", "purpose", "authority_note"] {
        let value = require_str(obj, key)?;
        if value.is_empty() {
            return Err(fail(
                FailureKind::Integrity,
                format!("`{key}` minLength violated"),
            ));
        }
    }
    let digests = obj
        .get("selected_bookmark_digests")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            fail(
                FailureKind::Other,
                "`selected_bookmark_digests` must be array",
            )
        })?;
    if digests.is_empty() {
        return Err(fail(
            FailureKind::Integrity,
            "selected_bookmark_digests minItems violated",
        ));
    }
    for (idx, digest) in digests.iter().enumerate() {
        let Some(s) = digest.as_str() else {
            return Err(fail(
                FailureKind::Other,
                format!("selected_bookmark_digests[{idx}] must be string"),
            ));
        };
        if !is_sha256_hex(s) {
            return Err(fail(
                FailureKind::Integrity,
                format!("selected_bookmark_digests[{idx}] must be 64-char hex"),
            ));
        }
    }
    require_enum(
        require_str(obj, "result")?,
        &["export_ready", "imported", "rejected"],
        "result",
    )?;
    let signature = obj
        .get("signature")
        .and_then(Value::as_object)
        .ok_or_else(|| fail(FailureKind::Missing, "missing required field `signature`"))?;
    let algorithm = require_str(signature, "algorithm")?;
    if algorithm != "Ed25519" {
        return Err(fail(
            FailureKind::Enum,
            format!("signature.algorithm const mismatch: {algorithm}"),
        ));
    }
    for key in ["key_id", "payload_digest", "signature"] {
        let value = require_str(signature, key)?;
        if value.is_empty() {
            return Err(fail(
                FailureKind::Integrity,
                format!("signature.{key} minLength violated"),
            ));
        }
    }
    let payload_digest = require_str(signature, "payload_digest")?;
    if !is_sha256_hex(payload_digest) {
        return Err(fail(
            FailureKind::Integrity,
            "signature.payload_digest must be 64-char hex",
        ));
    }
    require_enum(
        require_str(signature, "trust_state")?,
        &[
            "VALID_TRUSTED",
            "VALID_UNTRUSTED",
            "INVALID",
            "EXPIRED_OR_REVOKED",
            "NOT_SIGNED",
            "NOT_CHECKED",
        ],
        "signature.trust_state",
    )?;
    Ok(())
}

/// Validate an in-memory JSON value against a foundation schema.
pub fn validate_value(kind: SchemaKind, value: &Value) -> Result<(), SchemaFailure> {
    ensure_schema_file(kind)?;
    let obj = as_object(value)?;
    match kind {
        SchemaKind::EvidenceObject => validate_evidence(obj),
        SchemaKind::Case => validate_case(obj),
        SchemaKind::ProvenanceEvent => validate_generic_required(
            obj,
            &["schema_version", "event_uuid", "case_uuid"],
            "provenance-event-1",
        ),
        SchemaKind::AuditEvent => validate_generic_required(
            obj,
            &["schema_version", "event_uuid", "case_uuid"],
            "audit-event-1",
        ),
        SchemaKind::CoverageRecord => validate_generic_required(
            obj,
            &["schema_version", "coverage_uuid", "case_uuid"],
            "coverage-record-1",
        ),
        SchemaKind::Bookmark => validate_bookmark(obj),
        SchemaKind::TransferPackage => validate_transfer_package(obj),
    }
}

/// Validate a fixture or record file on disk.
pub fn validate_fixture_file(kind: SchemaKind, path: &Path) -> Result<(), SchemaFailure> {
    let raw = fs::read_to_string(path).map_err(|e| {
        fail(
            FailureKind::Other,
            format!("read fixture {}: {e}", path.display()),
        )
    })?;
    let value: Value = serde_json::from_str(&raw)
        .map_err(|e| fail(FailureKind::Other, format!("parse fixture: {e}")))?;
    validate_value(kind, &value).map_err(|mut f| {
        f.detail = format!("{} [{}]", f.detail, kind.label());
        f
    })
}

/// Typed evidence decode helper for callers that prefer structs.
#[derive(Debug, Deserialize)]
pub struct EvidenceObjectRecord {
    pub schema_version: String,
    pub evidence_uuid: String,
    pub case_uuid: String,
    pub source_designation: String,
    pub evidence_class: String,
    pub byte_length: i64,
    pub validation_state: String,
}
