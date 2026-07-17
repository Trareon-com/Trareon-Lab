//! Append-only audit / provenance / coverage ledgers on the case DB.

use lab_core::{LabError, LabResult};
use rusqlite::params;

use crate::CaseDb;

/// Audit event row (append-only).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditEvent {
    pub event_uuid: String,
    pub case_uuid: String,
    pub created_at_utc: String,
    pub actor_role: String,
    pub action: String,
    pub detail_json: String,
}

/// Provenance event row (append-only).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvenanceEvent {
    pub event_uuid: String,
    pub case_uuid: String,
    pub created_at_utc: String,
    pub evidence_uuid: String,
    pub activity: String,
    pub detail_json: String,
}

/// Coverage record row (append-only; corrections are new rows).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoverageRecord {
    pub coverage_uuid: String,
    pub case_uuid: String,
    pub created_at_utc: String,
    pub scope: String,
    pub status: String,
    pub detail_json: String,
}

impl CaseDb {
    /// Append an audit event. Never updates or deletes prior rows.
    pub fn append_audit_event(&self, event: &AuditEvent) -> LabResult<()> {
        self.connection()
            .execute(
                "INSERT INTO audit_event(
                    event_uuid, case_uuid, created_at_utc, actor_role, action, detail_json
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    event.event_uuid,
                    event.case_uuid,
                    event.created_at_utc,
                    event.actor_role,
                    event.action,
                    event.detail_json,
                ],
            )
            .map_err(|e| LabError::Internal {
                detail: format!("append audit_event: {e}"),
            })?;
        Ok(())
    }

    /// Append a provenance event.
    pub fn append_provenance_event(&self, event: &ProvenanceEvent) -> LabResult<()> {
        self.connection()
            .execute(
                "INSERT INTO provenance_event(
                    event_uuid, case_uuid, created_at_utc, evidence_uuid, activity, detail_json
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    event.event_uuid,
                    event.case_uuid,
                    event.created_at_utc,
                    event.evidence_uuid,
                    event.activity,
                    event.detail_json,
                ],
            )
            .map_err(|e| LabError::Internal {
                detail: format!("append provenance_event: {e}"),
            })?;
        Ok(())
    }

    /// Append a coverage record.
    pub fn append_coverage_record(&self, record: &CoverageRecord) -> LabResult<()> {
        self.connection()
            .execute(
                "INSERT INTO coverage_record(
                    coverage_uuid, case_uuid, created_at_utc, scope, status, detail_json
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    record.coverage_uuid,
                    record.case_uuid,
                    record.created_at_utc,
                    record.scope,
                    record.status,
                    record.detail_json,
                ],
            )
            .map_err(|e| LabError::Internal {
                detail: format!("append coverage_record: {e}"),
            })?;
        Ok(())
    }

    /// Count ledger rows for a case.
    pub fn count_audit_events(&self, case_uuid: &str) -> LabResult<u64> {
        self.count_table("audit_event", case_uuid)
    }

    pub fn count_provenance_events(&self, case_uuid: &str) -> LabResult<u64> {
        self.count_table("provenance_event", case_uuid)
    }

    pub fn count_coverage_records(&self, case_uuid: &str) -> LabResult<u64> {
        self.count_table("coverage_record", case_uuid)
    }

    /// Reject mutation helpers — append-only surface has no update/delete API.
    pub fn try_update_audit_event_forbidden(&self, _event_uuid: &str) -> LabResult<()> {
        Err(LabError::Internal {
            detail: "audit_event is append-only; updates are forbidden".into(),
        })
    }

    pub fn try_delete_audit_event_forbidden(&self, _event_uuid: &str) -> LabResult<()> {
        Err(LabError::Internal {
            detail: "audit_event is append-only; deletes are forbidden".into(),
        })
    }

    fn count_table(&self, table: &str, case_uuid: &str) -> LabResult<u64> {
        let sql = match table {
            "audit_event" => "SELECT COUNT(*) FROM audit_event WHERE case_uuid = ?1",
            "provenance_event" => "SELECT COUNT(*) FROM provenance_event WHERE case_uuid = ?1",
            "coverage_record" => "SELECT COUNT(*) FROM coverage_record WHERE case_uuid = ?1",
            other => {
                return Err(LabError::Internal {
                    detail: format!("unknown ledger table: {other}"),
                });
            }
        };
        let count: i64 = self
            .connection()
            .query_row(sql, [case_uuid], |row| row.get(0))
            .map_err(|e| LabError::Internal {
                detail: format!("count {table}: {e}"),
            })?;
        Ok(count as u64)
    }
}
