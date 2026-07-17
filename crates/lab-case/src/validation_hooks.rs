//! Second-method verification and blind PT participant hooks (FR-VAL-009/010).

use lab_core::{LabError, LabResult};
use rusqlite::params;

use crate::CaseDb;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecondMethodVerification {
    pub verification_uuid: String,
    pub case_uuid: String,
    pub method_a_id: String,
    pub method_b_id: String,
    pub claim_ref: String,
    pub output_a_digest: Option<String>,
    pub output_b_digest: Option<String>,
    pub discrepancy: String,
    pub disposition: String,
    pub residual_risk: Option<String>,
    pub created_at_utc: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlindPtParticipant {
    pub package_uuid: String,
    pub case_uuid: String,
    pub scheme_id: String,
    pub round_id: String,
    pub import_digest: String,
    pub expected_results_embargoed: bool,
    pub submission_locked: bool,
    pub result_export_digest: Option<String>,
    pub created_at_utc: String,
}

impl CaseDb {
    pub fn upsert_second_method(&self, v: &SecondMethodVerification) -> LabResult<()> {
        self.connection()
            .execute(
                "INSERT INTO second_method_verification(
                    verification_uuid, case_uuid, method_a_id, method_b_id, claim_ref,
                    output_a_digest, output_b_digest, discrepancy, disposition,
                    residual_risk, created_at_utc
                 ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)
                 ON CONFLICT(verification_uuid) DO UPDATE SET
                    discrepancy=excluded.discrepancy,
                    disposition=excluded.disposition,
                    residual_risk=excluded.residual_risk,
                    output_a_digest=excluded.output_a_digest,
                    output_b_digest=excluded.output_b_digest",
                params![
                    v.verification_uuid,
                    v.case_uuid,
                    v.method_a_id,
                    v.method_b_id,
                    v.claim_ref,
                    v.output_a_digest,
                    v.output_b_digest,
                    v.discrepancy,
                    v.disposition,
                    v.residual_risk,
                    v.created_at_utc,
                ],
            )
            .map_err(|e| LabError::Internal {
                detail: format!("upsert second_method: {e}"),
            })?;
        Ok(())
    }

    pub fn list_second_methods(&self, case_uuid: &str) -> LabResult<Vec<SecondMethodVerification>> {
        let mut stmt = self
            .connection()
            .prepare(
                "SELECT verification_uuid, case_uuid, method_a_id, method_b_id, claim_ref,
                        output_a_digest, output_b_digest, discrepancy, disposition,
                        residual_risk, created_at_utc
                 FROM second_method_verification WHERE case_uuid=?1
                 ORDER BY created_at_utc",
            )
            .map_err(|e| LabError::Internal {
                detail: format!("prepare second_method: {e}"),
            })?;
        let rows = stmt
            .query_map([case_uuid], |row| {
                Ok(SecondMethodVerification {
                    verification_uuid: row.get(0)?,
                    case_uuid: row.get(1)?,
                    method_a_id: row.get(2)?,
                    method_b_id: row.get(3)?,
                    claim_ref: row.get(4)?,
                    output_a_digest: row.get(5)?,
                    output_b_digest: row.get(6)?,
                    discrepancy: row.get(7)?,
                    disposition: row.get(8)?,
                    residual_risk: row.get(9)?,
                    created_at_utc: row.get(10)?,
                })
            })
            .map_err(|e| LabError::Internal {
                detail: format!("query second_method: {e}"),
            })?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r.map_err(|e| LabError::Internal {
                detail: format!("row second_method: {e}"),
            })?);
        }
        Ok(out)
    }

    pub fn upsert_blind_pt(&self, p: &BlindPtParticipant) -> LabResult<()> {
        self.connection()
            .execute(
                "INSERT INTO blind_pt_participant(
                    package_uuid, case_uuid, scheme_id, round_id, import_digest,
                    expected_results_embargoed, submission_locked, result_export_digest,
                    created_at_utc
                 ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)
                 ON CONFLICT(package_uuid) DO UPDATE SET
                    submission_locked=excluded.submission_locked,
                    result_export_digest=excluded.result_export_digest,
                    expected_results_embargoed=excluded.expected_results_embargoed",
                params![
                    p.package_uuid,
                    p.case_uuid,
                    p.scheme_id,
                    p.round_id,
                    p.import_digest,
                    i32::from(p.expected_results_embargoed),
                    i32::from(p.submission_locked),
                    p.result_export_digest,
                    p.created_at_utc,
                ],
            )
            .map_err(|e| LabError::Internal {
                detail: format!("upsert blind_pt: {e}"),
            })?;
        Ok(())
    }

    pub fn get_blind_pt(&self, package_uuid: &str) -> LabResult<Option<BlindPtParticipant>> {
        let mut stmt = self
            .connection()
            .prepare(
                "SELECT package_uuid, case_uuid, scheme_id, round_id, import_digest,
                        expected_results_embargoed, submission_locked, result_export_digest,
                        created_at_utc
                 FROM blind_pt_participant WHERE package_uuid=?1",
            )
            .map_err(|e| LabError::Internal {
                detail: format!("prepare blind_pt: {e}"),
            })?;
        let mut rows = stmt
            .query_map([package_uuid], |row| {
                Ok(BlindPtParticipant {
                    package_uuid: row.get(0)?,
                    case_uuid: row.get(1)?,
                    scheme_id: row.get(2)?,
                    round_id: row.get(3)?,
                    import_digest: row.get(4)?,
                    expected_results_embargoed: row.get::<_, i32>(5)? != 0,
                    submission_locked: row.get::<_, i32>(6)? != 0,
                    result_export_digest: row.get(7)?,
                    created_at_utc: row.get(8)?,
                })
            })
            .map_err(|e| LabError::Internal {
                detail: format!("query blind_pt: {e}"),
            })?;
        match rows.next() {
            None => Ok(None),
            Some(r) => Ok(Some(r.map_err(|e| LabError::Internal {
                detail: format!("row blind_pt: {e}"),
            })?)),
        }
    }
}
