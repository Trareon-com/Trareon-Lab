//! Case session: wires CaseDb / storage / index / transfer into UiSnapshot.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use lab_case::{AuditEvent, CaseDb, CaseLock, CoverageRecord};
use lab_core::{
    compare_runs, export_case_html, export_case_pdfa, export_case_skeleton, export_case_uco,
    CoverageStatus, ExportMode, IntegrityState, LabResult, RunManifest, ScopeAction, ScopeBounds,
    ScopeGuard, ScopeOverride, CASE_UCO_PROFILE_VERSION,
};
use lab_crypto::TrustState;
use lab_index::{IndexDb, IndexEntry, SearchPlan, SearchQuery};
use lab_storage::{import_raw_image, open_image};
use lab_transfer::{
    export_signed_package, import_verify_package, LocalKeypair, SignatureBlock, TransferPackage,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::ui_model::{EvidenceFileRow, UiSnapshot};

/// Active laboratory session backed by on-disk case artifacts.
pub struct LabSession {
    pub case_dir: PathBuf,
    pub case_uuid: String,
    pub case_title: String,
    pub db: CaseDb,
    pub index: Option<IndexDb>,
    pub lock: CaseLock,
    pub scope: ScopeGuard,
    pub runs: Vec<RunManifest>,
    pub transfer_keypair: LocalKeypair,
    pub last_transfer: Option<TransferPackage>,
    pub search_plans: Vec<SearchPlan>,
    pub notes: Vec<String>,
    pub discrepancies: Vec<DiscrepancyRow>,
    pub report_author: String,
    pub report_approver: String,
    pub original_report_id: Option<String>,
    pub report_version: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscrepancyRow {
    pub id: String,
    pub summary: String,
    pub status: String,
    pub note: String,
}

fn now_utc() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("{secs}Z")
}

fn uuid_v4_like(seed: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let mut h = DefaultHasher::new();
    seed.hash(&mut h);
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0)
        .hash(&mut h);
    COUNTER.fetch_add(1, Ordering::Relaxed).hash(&mut h);
    let n = h.finish();
    format!(
        "{:08x}-{:04x}-4{:03x}-8{:03x}-{:012x}",
        (n >> 32) as u32,
        ((n >> 16) & 0xffff) as u16,
        ((n >> 4) & 0x0fff) as u16,
        (n & 0x0fff) as u16,
        n % 0x0000_ffffffffffff
    )
}

fn json_str(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}

const CASE_ID_NAME: &str = "case_uuid.txt";

fn read_or_write_case_uuid(case_dir: &Path) -> LabResult<String> {
    std::fs::create_dir_all(case_dir).map_err(|e| lab_core::LabError::Internal {
        detail: format!("mkdir case: {e}"),
    })?;
    let path = case_dir.join(CASE_ID_NAME);
    if let Ok(existing) = std::fs::read_to_string(&path) {
        let trimmed = existing.trim();
        if !trimmed.is_empty() {
            return Ok(trimmed.to_string());
        }
    }
    let uuid = uuid_v4_like("case");
    std::fs::write(&path, format!("{uuid}\n")).map_err(|e| lab_core::LabError::Internal {
        detail: format!("write case_uuid: {e}"),
    })?;
    Ok(uuid)
}

impl LabSession {
    /// Open an existing case folder, or create one if `case.sqlite` is missing.
    pub fn open_or_create(case_dir: &Path) -> LabResult<Self> {
        let title = case_dir
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("case")
            .to_string();
        let case_uuid = read_or_write_case_uuid(case_dir)?;
        if case_dir.join("case.sqlite").exists() {
            Self::open(case_dir, &case_uuid, &title)
        } else {
            Self::create(case_dir, &case_uuid, &title)
        }
    }

    /// Create a new case directory with empty CaseDb + lock.
    pub fn create(case_dir: &Path, case_uuid: &str, title: &str) -> LabResult<Self> {
        std::fs::create_dir_all(case_dir).map_err(|e| lab_core::LabError::Internal {
            detail: format!("mkdir case: {e}"),
        })?;
        let lock = CaseLock::acquire(case_dir, case_uuid)?;
        let db = CaseDb::open_and_migrate(&case_dir.join("case.sqlite"))?;
        let index = IndexDb::open_and_migrate(&case_dir.join("index.sqlite")).ok();
        let session = Self {
            case_dir: case_dir.to_path_buf(),
            case_uuid: case_uuid.to_string(),
            case_title: title.to_string(),
            db,
            index,
            lock,
            scope: ScopeGuard::new(ScopeBounds {
                case_uuid: case_uuid.to_string(),
                enforced: true,
                path_prefixes: Vec::new(),
                ..Default::default()
            }),
            runs: Vec::new(),
            transfer_keypair: LocalKeypair::generate("lab-local-1"),
            last_transfer: None,
            search_plans: Vec::new(),
            notes: Vec::new(),
            discrepancies: Vec::new(),
            report_author: String::new(),
            report_approver: String::new(),
            original_report_id: None,
            report_version: 1,
        };
        session.save_discrepancies()?;
        session.append_audit("system", "case_create", "{}")?;
        Ok(session)
    }

    /// Open an existing case directory.
    pub fn open(case_dir: &Path, case_uuid: &str, title: &str) -> LabResult<Self> {
        let lock = CaseLock::acquire(case_dir, case_uuid)?;
        let db = CaseDb::open_and_migrate(&case_dir.join("case.sqlite"))?;
        let index = IndexDb::open_and_migrate(&case_dir.join("index.sqlite")).ok();
        let discrepancies = Self::load_discrepancies(case_dir)?;
        Ok(Self {
            case_dir: case_dir.to_path_buf(),
            case_uuid: case_uuid.to_string(),
            case_title: title.to_string(),
            db,
            index,
            lock,
            scope: ScopeGuard::new(ScopeBounds {
                case_uuid: case_uuid.to_string(),
                enforced: true,
                path_prefixes: Vec::new(),
                ..Default::default()
            }),
            runs: Vec::new(),
            transfer_keypair: LocalKeypair::generate("lab-local-1"),
            last_transfer: None,
            search_plans: Vec::new(),
            notes: Vec::new(),
            discrepancies,
            report_author: String::new(),
            report_approver: String::new(),
            original_report_id: None,
            report_version: 1,
        })
    }

    pub fn append_audit(&self, actor: &str, action: &str, detail_json: &str) -> LabResult<()> {
        self.db.append_audit_event(&AuditEvent {
            event_uuid: uuid_v4_like(action),
            case_uuid: self.case_uuid.clone(),
            created_at_utc: now_utc(),
            actor_role: actor.into(),
            action: action.into(),
            detail_json: detail_json.into(),
        })
    }

    pub fn apply_scope_override(&mut self, ov: ScopeOverride) -> LabResult<()> {
        let detail = format!(
            "{{\"reason\":{},\"approver\":{},\"expires\":{}}}",
            json_str(&ov.reason),
            json_str(&ov.approver_note),
            ov.expires_at_utc
                .as_ref()
                .map(|s| json_str(s))
                .unwrap_or_else(|| "null".into())
        );
        self.append_audit("examiner", "scope_override", &detail)?;
        let mut bounds = self.scope.bounds().clone();
        bounds.enforced = false;
        self.scope = ScopeGuard::new(bounds);
        Ok(())
    }

    pub fn sync_snapshot(&self, snap: &mut UiSnapshot) -> LabResult<()> {
        let evidence = self.db.list_evidence_objects(&self.case_uuid)?;
        let coverage = self.db.list_coverage_records(&self.case_uuid)?;
        snap.open_case(
            &self.case_title,
            evidence.len() as i32,
            coverage.len() as i32,
        );
        snap.scope = self.scope.bounds().clone();
        snap.coverage_processed = coverage
            .iter()
            .filter(|c| c.status == "processed" || c.status == "complete")
            .count() as i32;
        snap.coverage_skipped = coverage.iter().filter(|c| c.status == "skipped").count() as i32;
        snap.coverage_failed = coverage.iter().filter(|c| c.status == "failed").count() as i32;
        snap.coverage_unsupported = coverage
            .iter()
            .filter(|c| c.status == "unsupported")
            .count() as i32;
        snap.coverage_status = if coverage.is_empty() {
            CoverageStatus::NotRun
        } else if snap.coverage_failed > 0 {
            CoverageStatus::Failed
        } else if snap.coverage_skipped > 0 || snap.coverage_unsupported > 0 {
            CoverageStatus::CompletedWithLimitations
        } else {
            CoverageStatus::Complete
        };

        let mut rows = Vec::new();
        for e in &evidence {
            let path = self
                .db
                .evidence_source_path(&self.case_uuid, &e.evidence_uuid)?
                .unwrap_or_default();
            let integrity = match e.validation_state.as_str() {
                "VerifiedMatch" => IntegrityState::VerifiedMatch,
                "VerifiedMismatch" => IntegrityState::VerifiedMismatch,
                "SignatureInvalid" => IntegrityState::SignatureInvalid,
                "IncompleteInput" => IntegrityState::IncompleteInput,
                "ComputedUnanchored" => IntegrityState::ComputedUnanchored,
                _ => IntegrityState::NotRun,
            };
            let designation = match e.evidence_class.as_str() {
                "disk_image" => "forensic_copy",
                "logical" => "logical",
                other => other,
            };
            rows.push(EvidenceFileRow {
                path,
                name: e.display_name.clone(),
                size: 0,
                deleted: false,
                integrity,
                designation: designation.into(),
            });
        }
        snap.set_evidence_files(rows);
        snap.recompute_report_gate();
        Ok(())
    }

    /// Import a raw/E01 image into the case ledgers.
    pub fn import_image(&mut self, path: &Path) -> LabResult<()> {
        self.scope
            .allow_path(&path.display().to_string(), ScopeAction::Process)?;
        let evidence_uuid = uuid_v4_like("evidence");
        let provenance_uuid = uuid_v4_like("provenance");
        let result = import_raw_image(
            &self.db,
            &self.case_uuid,
            path,
            &evidence_uuid,
            &provenance_uuid,
            &now_utc(),
        )?;
        self.db.append_coverage_record(&CoverageRecord {
            coverage_uuid: uuid_v4_like("coverage"),
            case_uuid: self.case_uuid.clone(),
            created_at_utc: now_utc(),
            scope: format!("import:{}", result.display_name),
            status: "processed".into(),
            detail_json: format!(
                "{{\"sha256\":\"{}\",\"bytes\":{}}}",
                result.sha256_hex, result.byte_length
            ),
        })?;
        if let Some(index) = self.index.as_mut() {
            let _ = index.insert_entry(&IndexEntry {
                case_uuid: self.case_uuid.clone(),
                entry_kind: "evidence".into(),
                target_ref: result.evidence_uuid.clone(),
                display_text: result.display_name.clone(),
                sort_key: result.display_name.clone(),
                created_at_utc: now_utc(),
            });
        }
        self.append_audit(
            "examiner",
            "import_image",
            &format!(
                "{{\"path\":{},\"evidence_uuid\":\"{}\"}}",
                json_str(&path.display().to_string()),
                result.evidence_uuid
            ),
        )?;
        Ok(())
    }

    /// Read a hex window from a local evidence path.
    pub fn read_hex(
        &self,
        path: &Path,
        offset: u64,
        len: usize,
    ) -> LabResult<(Vec<String>, String)> {
        self.scope
            .allow_path(&path.display().to_string(), ScopeAction::Preview)?;
        let mut img = open_image(path)?;
        let mut buf = vec![0u8; len.min(512)];
        let n = img.read_at(offset, &mut buf)?;
        if n == 0 {
            return Ok((
                vec!["-- empty read --".into()],
                format!("Error/Limited · offset {offset} · 0 bytes"),
            ));
        }
        let mut lines = Vec::new();
        let mut i = 0;
        while i < n {
            let end = (i + 16).min(n);
            let mut hex = String::new();
            let mut ascii = String::new();
            for (j, b) in buf[i..end].iter().enumerate() {
                if j == 8 {
                    hex.push(' ');
                }
                hex.push_str(&format!("{b:02X} "));
                ascii.push(if (0x20..0x7f).contains(b) {
                    *b as char
                } else {
                    '.'
                });
            }
            lines.push(format!("{:08X}  {hex:<49} {ascii}", offset + i as u64));
            i = end;
        }
        let status = format!(
            "{} · offset {offset} · {n} bytes · {}",
            path.file_name().and_then(|s| s.to_str()).unwrap_or("image"),
            IntegrityState::ComputedUnanchored.as_str()
        );
        Ok((lines, status))
    }

    pub fn search(&mut self, query: &str, snap: &mut UiSnapshot) -> LabResult<()> {
        self.scope.allow_path("/", ScopeAction::Search)?;
        let Some(index) = self.index.as_ref() else {
            snap.set_search(query, Vec::new());
            return Ok(());
        };
        let parsed = SearchQuery::parse(query)?;
        let result = index.search(&self.case_uuid, &parsed, 200)?;
        let mut labels: Vec<String> = result
            .entries
            .iter()
            .map(|h| format!("{} · {}", h.entry_kind, h.display_text))
            .collect();
        if result.truncated {
            labels.push(format!(
                "[coverage] truncated: {}",
                result
                    .truncation_reason
                    .clone()
                    .unwrap_or_else(|| "limit".into())
            ));
        }
        let plan = SearchPlan {
            query: query.to_string(),
            scope: self.case_uuid.clone(),
            encodings: vec!["utf-8".into()],
            error_count: 0,
            hit_count: result.entries.len() as u64,
            tags: if result.truncated {
                vec!["partial".into()]
            } else {
                vec!["complete".into()]
            },
        };
        self.search_plans.push(plan);
        snap.set_search(query, labels);
        Ok(())
    }

    pub fn record_run(&mut self, tool: &str, input_hash: &str, output_hash: &str) -> RunManifest {
        let m = RunManifest {
            run_uuid: uuid_v4_like("run"),
            case_uuid: self.case_uuid.clone(),
            build_version: env!("CARGO_PKG_VERSION").into(),
            started_at_utc: now_utc(),
            locale: "en".into(),
            timezone: "UTC".into(),
            hash_set_pin: None,
            rule_set_pin: Some(tool.into()),
            config_json: "{}".into(),
            input_hashes: vec![input_hash.into()],
            output_hash: Some(output_hash.into()),
            error_summary: None,
            coverage_status: "Complete".into(),
        };
        self.runs.push(m.clone());
        m
    }

    pub fn compare_last_runs(&self) -> Vec<String> {
        if self.runs.len() < 2 {
            return vec!["need two runs to compare".into()];
        }
        let a = &self.runs[self.runs.len() - 2];
        let b = &self.runs[self.runs.len() - 1];
        compare_runs(a, b)
            .into_iter()
            .map(|l| format!("{}: {} → {}", l.field, l.left, l.right))
            .collect()
    }

    pub fn export_transfer_package(&mut self, label: &str) -> LabResult<TrustState> {
        self.scope.allow_path("/", ScopeAction::Export)?;
        let pkg = TransferPackage {
            schema_version: "transfer-package-1".into(),
            transfer_uuid: uuid_v4_like("xfer"),
            source_case_uuid: self.case_uuid.clone(),
            created_at_utc: now_utc(),
            destination: label.into(),
            purpose: "review".into(),
            authority_note: "lab session export".into(),
            selected_bookmark_digests: vec!["a".repeat(64)],
            result: "export_ready".into(),
            signature: SignatureBlock {
                algorithm: String::new(),
                key_id: String::new(),
                payload_digest: String::new(),
                signature: String::new(),
                trust_state: "NOT_CHECKED".into(),
            },
        };
        let signed = export_signed_package(pkg, &self.transfer_keypair)?;
        let trust = import_verify_package(&signed, &self.transfer_keypair.public_bytes(), true)?;
        self.last_transfer = Some(signed);
        self.append_audit("examiner", "transfer_export", "{\"ok\":true}")?;
        Ok(trust)
    }

    /// Tamper the last package and expect integrity failure.
    pub fn verify_last_transfer_tampered(&self) -> LabResult<()> {
        let mut pkg = self
            .last_transfer
            .clone()
            .ok_or_else(|| lab_core::LabError::Internal {
                detail: "no transfer package".into(),
            })?;
        pkg.purpose.push_str("-tamper");
        match import_verify_package(&pkg, &self.transfer_keypair.public_bytes(), true) {
            Err(_) => Ok(()),
            Ok(TrustState::Invalid) => Ok(()),
            Ok(other) => Err(lab_core::LabError::Internal {
                detail: format!("expected Invalid after tamper, got {other:?}"),
            }),
        }
    }

    pub fn try_finalize(
        &mut self,
        snap: &mut UiSnapshot,
        author: &str,
        approver: &str,
    ) -> LabResult<()> {
        self.scope.allow_path("/", ScopeAction::Report)?;
        snap.require_sod = true;
        if snap.require_sod && !author.is_empty() && author == approver {
            snap.report_blockers.push("sod_self_approve".into());
            snap.report_finalizable = false;
            return Err(lab_core::LabError::Internal {
                detail: "SoD: author cannot self-approve".into(),
            });
        }
        snap.recompute_report_gate();
        if !snap.report_finalizable {
            return Err(lab_core::LabError::Internal {
                detail: format!("finalize blocked: {:?}", snap.report_blockers),
            });
        }
        self.report_author = author.into();
        self.report_approver = approver.into();
        snap.set_report_state("final");
        self.append_audit(
            "approver",
            "report_finalize",
            &format!(
                "{{\"author\":{},\"approver\":{},\"version\":{}}}",
                json_str(author),
                json_str(approver),
                self.report_version
            ),
        )?;
        Ok(())
    }

    pub fn amend_report(&mut self, snap: &mut UiSnapshot) -> LabResult<()> {
        let original = format!("report-v{}", self.report_version);
        self.original_report_id = Some(original.clone());
        self.report_version += 1;
        snap.set_report_state("draft");
        self.append_audit(
            "examiner",
            "report_amendment",
            &format!(
                "{{\"original_id\":{},\"new_version\":{}}}",
                json_str(&original),
                self.report_version
            ),
        )?;
        Ok(())
    }

    pub fn append_note(&mut self, text: &str) -> LabResult<()> {
        let line = format!("{} · {text}", now_utc());
        self.notes.push(line);
        self.append_audit(
            "examiner",
            "note_append",
            &format!("{{\"text\":{}}}", json_str(text)),
        )?;
        Ok(())
    }

    /// Record a fail-closed parser outcome in the append-only coverage ledger.
    pub fn record_parser_failure(&self, parser: &str, scope: &str, reason: &str) -> LabResult<()> {
        self.db.append_coverage_record(&CoverageRecord {
            coverage_uuid: uuid_v4_like("parser-failure"),
            case_uuid: self.case_uuid.clone(),
            created_at_utc: now_utc(),
            scope: format!("{parser}:{scope}"),
            status: "failed".into(),
            detail_json: format!("{{\"reason\":{}}}", json_str(reason)),
        })
    }

    pub fn export_formats(&self, mode: ExportMode) -> LabResult<Vec<(String, String)>> {
        self.scope.allow_path("/", ScopeAction::Export)?;
        let evidence = self.db.count_evidence_objects(&self.case_uuid)? as u32;
        let coverage = self.db.count_coverage_records(&self.case_uuid)? as u32;
        let skel =
            export_case_skeleton(&self.case_uuid, &self.case_title, evidence, coverage, mode)?;
        let html = export_case_html(&self.case_uuid, &self.case_title, &[], mode)?;
        let pdf = export_case_pdfa(&self.case_uuid, &self.case_title, "Lab report", mode)?;
        let uco = export_case_uco(&self.case_uuid, &self.case_title, "Lab report", mode)?;
        Ok(vec![
            ("json".into(), skel.digest_sha256),
            (
                format!("html:{}", html.validation_level),
                html.digest_sha256,
            ),
            (format!("pdfa:{}", pdf.validation_level), pdf.digest_sha256),
            (
                format!(
                    "case_uco:{}:{}",
                    CASE_UCO_PROFILE_VERSION, uco.validation_level
                ),
                uco.digest_sha256,
            ),
        ])
    }

    /// Backup case sqlite files to a destination directory (hashed copy).
    pub fn backup_case(&self, dest: &Path) -> LabResult<String> {
        std::fs::create_dir_all(dest).map_err(|e| lab_core::LabError::Internal {
            detail: format!("backup mkdir: {e}"),
        })?;
        let src = self.case_dir.join("case.sqlite");
        let dst = dest.join("case.sqlite");
        std::fs::copy(&src, &dst).map_err(|e| lab_core::LabError::Internal {
            detail: format!("backup copy: {e}"),
        })?;
        let bytes = std::fs::read(&dst).map_err(|e| lab_core::LabError::Internal {
            detail: format!("backup read: {e}"),
        })?;
        let digest = hex::encode(Sha256::digest(&bytes));
        for name in ["index.sqlite", "discrepancies.json"] {
            let source = self.case_dir.join(name);
            if source.is_file() {
                std::fs::copy(&source, dest.join(name)).map_err(|e| {
                    lab_core::LabError::Internal {
                        detail: format!("backup {name}: {e}"),
                    }
                })?;
            }
        }
        self.append_audit(
            "examiner",
            "case_backup",
            &format!("{{\"sha256\":\"{digest}\"}}"),
        )?;
        Ok(digest)
    }

    /// Restore a backup into a new case directory, refusing to overwrite a case database.
    pub fn restore_case(
        backup_dir: &Path,
        case_dir: &Path,
        case_uuid: &str,
        title: &str,
    ) -> LabResult<Self> {
        std::fs::create_dir_all(case_dir).map_err(|e| lab_core::LabError::Internal {
            detail: format!("restore mkdir: {e}"),
        })?;
        if case_dir.join("case.sqlite").exists() {
            return Err(lab_core::LabError::Internal {
                detail: "restore destination already contains case.sqlite".into(),
            });
        }
        std::fs::copy(backup_dir.join("case.sqlite"), case_dir.join("case.sqlite")).map_err(
            |e| lab_core::LabError::Internal {
                detail: format!("restore case.sqlite: {e}"),
            },
        )?;
        for name in ["index.sqlite", "discrepancies.json"] {
            let source = backup_dir.join(name);
            if source.is_file() {
                std::fs::copy(&source, case_dir.join(name)).map_err(|e| {
                    lab_core::LabError::Internal {
                        detail: format!("restore {name}: {e}"),
                    }
                })?;
            }
        }
        let session = Self::open(case_dir, case_uuid, title)?;
        session.append_audit("system", "case_restore", "{}")?;
        Ok(session)
    }

    pub fn sanitize_derivatives(&self) -> LabResult<()> {
        let deriv = self.case_dir.join("derivatives");
        if deriv.exists() {
            std::fs::remove_dir_all(&deriv).map_err(|e| lab_core::LabError::Internal {
                detail: format!("sanitize: {e}"),
            })?;
        }
        self.append_audit("examiner", "sanitize_derivatives", "{\"ok\":true}")?;
        Ok(())
    }

    pub fn record_discrepancy(&mut self, summary: &str) -> LabResult<String> {
        let id = uuid_v4_like("disc");
        self.discrepancies.push(DiscrepancyRow {
            id: id.clone(),
            summary: summary.into(),
            status: "open".into(),
            note: String::new(),
        });
        self.save_discrepancies()?;
        Ok(id)
    }

    pub fn resolve_discrepancy(&mut self, id: &str, note: &str) -> LabResult<()> {
        if let Some(d) = self.discrepancies.iter_mut().find(|d| d.id == id) {
            d.status = "resolved".into();
            d.note = note.into();
        }
        self.save_discrepancies()
    }

    fn load_discrepancies(case_dir: &Path) -> LabResult<Vec<DiscrepancyRow>> {
        let path = case_dir.join("discrepancies.json");
        if !path.is_file() {
            return Ok(Vec::new());
        }
        let bytes = std::fs::read(&path).map_err(|e| lab_core::LabError::Internal {
            detail: format!("read discrepancies: {e}"),
        })?;
        serde_json::from_slice(&bytes).map_err(|e| lab_core::LabError::Internal {
            detail: format!("parse discrepancies: {e}"),
        })
    }

    fn save_discrepancies(&self) -> LabResult<()> {
        let bytes = serde_json::to_vec_pretty(&self.discrepancies).map_err(|e| {
            lab_core::LabError::Internal {
                detail: format!("serialize discrepancies: {e}"),
            }
        })?;
        std::fs::write(self.case_dir.join("discrepancies.json"), bytes).map_err(|e| {
            lab_core::LabError::Internal {
                detail: format!("write discrepancies: {e}"),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn session_import_and_counts() {
        let dir = tempdir().unwrap();
        let case_dir = dir.path().join("case");
        let mut session =
            LabSession::create(&case_dir, "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa", "T").unwrap();
        let img = dir.path().join("sample.dd");
        std::fs::write(&img, b"TRA REON LAB HEX TEST DATA!!!!").unwrap();
        session.import_image(&img).unwrap();
        let mut snap = UiSnapshot::default();
        session.sync_snapshot(&mut snap).unwrap();
        assert_eq!(snap.evidence_count, 1);
        assert_eq!(snap.coverage_count, 1);
        let (lines, status) = session.read_hex(&img, 0, 32).unwrap();
        assert!(!lines.is_empty());
        assert!(status.contains("bytes"));
    }

    #[test]
    fn sod_blocks_self_approve() {
        let dir = tempdir().unwrap();
        let case_dir = dir.path().join("case");
        let mut session =
            LabSession::create(&case_dir, "bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb", "T").unwrap();
        let mut snap = UiSnapshot {
            intake_accepted: true,
            coverage_status: CoverageStatus::Complete,
            evidence_files: vec![EvidenceFileRow {
                path: String::new(),
                name: "e".into(),
                size: 0,
                deleted: false,
                integrity: IntegrityState::VerifiedMatch,
                designation: "forensic_copy".into(),
            }],
            ..Default::default()
        };
        snap.recompute_report_gate();
        assert!(session.try_finalize(&mut snap, "alice", "alice").is_err());
    }

    #[test]
    fn transfer_tamper_invalid() {
        let dir = tempdir().unwrap();
        let case_dir = dir.path().join("case");
        let mut session =
            LabSession::create(&case_dir, "cccccccc-cccc-4ccc-8ccc-cccccccccccc", "T").unwrap();
        let trust = session.export_transfer_package("pt").unwrap();
        assert!(matches!(
            trust,
            TrustState::ValidTrusted | TrustState::ValidUntrusted
        ));
        session.verify_last_transfer_tampered().unwrap();
    }

    #[test]
    fn parser_failure_is_coverage_failure_with_reason() {
        let dir = tempdir().unwrap();
        let session = LabSession::create(
            &dir.path().join("case"),
            "dddddddd-dddd-4ddd-8ddd-dddddddddddd",
            "T",
        )
        .unwrap();
        session
            .record_parser_failure("e01", "hostile.E01", "parse timeout")
            .unwrap();
        let records = session
            .db
            .list_coverage_records(&session.case_uuid)
            .unwrap();
        assert_eq!(records[0].status, "failed");
        assert!(records[0].detail_json.contains("parse timeout"));
    }

    #[test]
    fn notes_are_append_only_and_ordered() {
        let dir = tempdir().unwrap();
        let mut session = LabSession::create(
            &dir.path().join("case"),
            "eeeeeeee-eeee-4eee-8eee-eeeeeeeeeeee",
            "T",
        )
        .unwrap();
        session.append_note("first").unwrap();
        session.append_note("second").unwrap();
        assert!(session.notes[0].ends_with("first"));
        assert!(session.notes[1].ends_with("second"));
        assert_eq!(
            session.db.count_audit_events(&session.case_uuid).unwrap(),
            3
        );
    }

    #[test]
    fn discrepancies_persist_and_backup_restores_case() {
        let dir = tempdir().unwrap();
        let case_dir = dir.path().join("case");
        let backup_dir = dir.path().join("backup");
        let restored_dir = dir.path().join("restored");
        let case_uuid = "ffffffff-ffff-4fff-8fff-ffffffffffff";
        let mut session = LabSession::create(&case_dir, case_uuid, "T").unwrap();
        let id = session.record_discrepancy("hash mismatch").unwrap();
        session.resolve_discrepancy(&id, "verified source").unwrap();
        session.backup_case(&backup_dir).unwrap();
        drop(session);

        let reopened = LabSession::open(&case_dir, case_uuid, "T").unwrap();
        assert_eq!(reopened.discrepancies[0].status, "resolved");
        drop(reopened);

        let restored =
            LabSession::restore_case(&backup_dir, &restored_dir, case_uuid, "T").unwrap();
        assert_eq!(restored.discrepancies[0].note, "verified source");
        assert!(restored_dir.join("case.sqlite").is_file());
    }
}
