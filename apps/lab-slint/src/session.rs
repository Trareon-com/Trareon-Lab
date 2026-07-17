//! Live lab session: case DB + index + honest GUI refresh helpers.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use lab_case::{BookmarkRecord, CaseDb, CaseState, EvidenceObject};
use lab_core::LabResult;
use lab_index::{IndexDb, IndexEntry};
use lab_storage::import_raw_image;
use uuid::Uuid;

use crate::{ArtifactHitRow, EvidenceFileRow, FindingRow, NavScreen, UiSnapshot};

const CASE_DB_NAME: &str = "case.sqlite";
const INDEX_DB_NAME: &str = "index.sqlite";
const CASE_ID_NAME: &str = "case_uuid.txt";

/// Open or create a case folder and bind it to the UI snapshot.
pub struct LabSession {
    pub case_dir: PathBuf,
    pub case_uuid: String,
    pub db: CaseDb,
    pub index: IndexDb,
}

impl LabSession {
    /// Open (or create) `case.sqlite` + `index.sqlite` under `case_dir`.
    pub fn open_or_create(case_dir: &Path) -> LabResult<Self> {
        fs::create_dir_all(case_dir).map_err(|e| lab_core::LabError::Internal {
            detail: format!("create case dir: {e}"),
        })?;
        let case_uuid = read_or_write_case_uuid(case_dir)?;
        let db = CaseDb::open_and_migrate(&case_dir.join(CASE_DB_NAME))?;
        let index = IndexDb::open_and_migrate(&case_dir.join(INDEX_DB_NAME))?;
        Ok(Self {
            case_dir: case_dir.to_path_buf(),
            case_uuid,
            db,
            index,
        })
    }

    pub fn title(&self) -> String {
        self.case_dir
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("case")
            .to_string()
    }

    pub fn refresh_snapshot(&self, snap: &mut UiSnapshot) -> LabResult<()> {
        let evidence = self.db.count_evidence_objects(&self.case_uuid)? as i32;
        let coverage = self.db.count_coverage_records(&self.case_uuid)? as i32;
        let title = self.title();
        snap.open_case(title, evidence, coverage);
        snap.case_state = CaseState::Open;
        snap.about_disclosure = sellable_disclosure();

        let objects = self.db.list_evidence_objects(&self.case_uuid)?;
        snap.evidence_files = evidence_rows_from_objects(&objects);
        snap.selected_file_index = None;
        snap.navigate_to(NavScreen::CaseHome);

        let bookmarks = self.db.list_bookmarks(&self.case_uuid)?;
        snap.set_bookmark_count(bookmarks.len() as i32);
        snap.set_findings(findings_from_bookmarks(&bookmarks));

        // Honest v1: no synthetic timeline/artifact theater.
        snap.timeline_labels.clear();
        snap.search_results.clear();
        snap.artifact_hits.clear();
        snap.report_state = "draft".into();
        Ok(())
    }

    pub fn import_raw_path(&mut self, path: &Path) -> LabResult<String> {
        let now = now_utc();
        let evidence_uuid = Uuid::new_v4().to_string();
        let provenance_uuid = Uuid::new_v4().to_string();
        let imported = import_raw_image(
            &self.db,
            &self.case_uuid,
            path,
            &evidence_uuid,
            &provenance_uuid,
            &now,
        )?;

        let path_disp = path.display().to_string();
        self.index.insert_entry(&IndexEntry {
            case_uuid: self.case_uuid.clone(),
            entry_kind: "evidence_path".into(),
            target_ref: path_disp,
            display_text: format!(
                "{} · sha256={} · {} bytes",
                imported.display_name, imported.sha256_hex, imported.byte_length
            ),
            sort_key: format!("{}:{}", now, imported.display_name),
            created_at_utc: now.clone(),
        })?;
        let hash_prefix = imported
            .sha256_hex
            .get(..12)
            .unwrap_or(&imported.sha256_hex)
            .to_string();
        self.index.insert_entry(&IndexEntry {
            case_uuid: self.case_uuid.clone(),
            entry_kind: "evidence_hash".into(),
            target_ref: imported.sha256_hex.clone(),
            display_text: imported.display_name.clone(),
            sort_key: format!("{now}:hash:{hash_prefix}"),
            created_at_utc: now,
        })?;

        Ok(imported.display_name)
    }

    pub fn search(&self, query: &str) -> LabResult<(Vec<String>, Vec<ArtifactHitRow>)> {
        let hits = self
            .index
            .query_path_name_hash(&self.case_uuid, query, 200)?;
        let results: Vec<String> = hits
            .iter()
            .map(|h| format!("{} · {}", h.entry_kind, h.display_text))
            .collect();
        let artifacts: Vec<ArtifactHitRow> = hits
            .iter()
            .map(|h| ArtifactHitRow {
                kind: h.entry_kind.clone(),
                summary: h.display_text.clone(),
                provenance_ref: h.target_ref.clone(),
            })
            .collect();
        Ok((results, artifacts))
    }

    pub fn bookmark_selection(
        &self,
        snap: &UiSnapshot,
        citation: impl Into<String>,
    ) -> LabResult<()> {
        let now = now_utc();
        let (target_kind, target_ref) = if let Some(idx) = snap.selected_file_index {
            let row = snap.evidence_files.get(idx);
            (
                "evidence_object".to_string(),
                row.map(|r| r.path.clone())
                    .unwrap_or_else(|| format!("index:{idx}")),
            )
        } else if let Some(hit) = snap.artifact_hits.first() {
            ("index_hit".to_string(), hit.provenance_ref.clone())
        } else {
            ("case".to_string(), self.case_uuid.clone())
        };
        self.db.upsert_bookmark(&BookmarkRecord {
            bookmark_uuid: Uuid::new_v4().to_string(),
            case_uuid: self.case_uuid.clone(),
            target_kind,
            target_ref,
            citation: citation.into(),
            author_role: "examiner".into(),
            created_at_utc: now,
            review_state: "open".into(),
            note: None,
        })?;
        Ok(())
    }

    pub fn reload_bookmarks_into(&self, snap: &mut UiSnapshot) -> LabResult<()> {
        let bookmarks = self.db.list_bookmarks(&self.case_uuid)?;
        snap.set_bookmark_count(bookmarks.len() as i32);
        snap.set_findings(findings_from_bookmarks(&bookmarks));
        Ok(())
    }

    pub fn reload_evidence_into(&self, snap: &mut UiSnapshot) -> LabResult<()> {
        let objects = self.db.list_evidence_objects(&self.case_uuid)?;
        let evidence = objects.len() as i32;
        let coverage = self.db.count_coverage_records(&self.case_uuid)? as i32;
        snap.evidence_count = evidence;
        snap.coverage_count = coverage;
        snap.evidence_files = evidence_rows_from_objects(&objects);
        snap.selected_file_index = None;
        snap.active_screen = NavScreen::Evidence;
        snap.open_case_focused = false;
        Ok(())
    }
}

fn evidence_rows_from_objects(objects: &[EvidenceObject]) -> Vec<EvidenceFileRow> {
    objects
        .iter()
        .map(|o| EvidenceFileRow {
            path: o.evidence_uuid.clone(),
            name: format!("{} [{}]", o.display_name, o.validation_state),
            size: 0,
            deleted: false,
        })
        .collect()
}

fn findings_from_bookmarks(bookmarks: &[BookmarkRecord]) -> Vec<FindingRow> {
    bookmarks
        .iter()
        .map(|b| FindingRow {
            claim: b.citation.clone(),
            bookmark_uuid: b.bookmark_uuid.clone(),
        })
        .collect()
}

fn read_or_write_case_uuid(case_dir: &Path) -> LabResult<String> {
    let id_path = case_dir.join(CASE_ID_NAME);
    if id_path.exists() {
        let raw = fs::read_to_string(&id_path).map_err(|e| lab_core::LabError::Internal {
            detail: format!("read case_uuid: {e}"),
        })?;
        let id = raw.trim().to_string();
        if id.is_empty() {
            return Err(lab_core::LabError::Internal {
                detail: "case_uuid.txt is empty".into(),
            });
        }
        return Ok(id);
    }
    let id = Uuid::new_v4().to_string();
    fs::write(&id_path, format!("{id}\n")).map_err(|e| lab_core::LabError::Internal {
        detail: format!("write case_uuid: {e}"),
    })?;
    Ok(id)
}

fn now_utc() -> String {
    // ponytail: epoch-seconds stamp is sortable enough for v1 ledgers; swap to chrono RFC3339 if auditors need wall-clock UTC.
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("ts-{secs}")
}

pub fn sellable_disclosure() -> String {
    "Trareon Lab v1 · offline case DB · raw/dd import + hash/provenance · path/name/hash search · bookmarks · unsigned builds — see docs/SELLING-UNSIGNED.md · NOT court-ready / NOT ISO-accredited".into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn open_session_imports_raw_and_searches() {
        let dir = tempdir().unwrap();
        let case_dir = dir.path().join("demo-case");
        let mut session = LabSession::open_or_create(&case_dir).expect("session");

        let raw = dir.path().join("disk.dd");
        fs::write(&raw, b"RAWIMAGE-BYTES-FOR-HASH").unwrap();
        session.import_raw_path(&raw).expect("import");

        let mut snap = UiSnapshot::default();
        session.refresh_snapshot(&mut snap).expect("refresh");
        assert_eq!(snap.evidence_count, 1);
        assert!(!snap.evidence_files.is_empty());

        let (results, arts) = session.search("disk").expect("search");
        assert!(!results.is_empty());
        assert!(!arts.is_empty());
        assert_eq!(arts[0].kind, "evidence_path");
    }

    #[test]
    fn sellable_flow_import_search_bookmark_survives_reopen() {
        let dir = tempdir().unwrap();
        let case_dir = dir.path().join("sellable-case");
        let mut session = LabSession::open_or_create(&case_dir).expect("open");

        assert!(case_dir.join("case.sqlite").is_file());
        assert!(case_dir.join("index.sqlite").is_file());
        assert!(case_dir.join("case_uuid.txt").is_file());

        let raw = dir.path().join("evidence.bin");
        fs::write(&raw, b"SELLABLE-SMOKE-RAW-BYTES").unwrap();
        let imported = session.import_raw_path(&raw).expect("import");
        assert!(
            imported.contains("evidence.bin") || imported.contains("evidence"),
            "unexpected import label: {imported}"
        );

        let mut snap = UiSnapshot::default();
        session.refresh_snapshot(&mut snap).expect("refresh");
        assert_eq!(snap.evidence_count, 1);
        snap.selected_file_index = Some(0);

        let (hits, arts) = session.search("evidence").expect("search");
        assert!(!hits.is_empty(), "expected index hit for imported evidence");
        snap.artifact_hits = arts;

        session
            .bookmark_selection(&snap, "Smoke citation: evidence.bin")
            .expect("bookmark");
        session
            .reload_bookmarks_into(&mut snap)
            .expect("reload bookmarks");
        assert_eq!(snap.bookmark_count, 1);
        assert_eq!(snap.findings.len(), 1);
        assert!(snap.findings[0].claim.contains("Smoke citation"));

        drop(session);
        let reopened = LabSession::open_or_create(&case_dir).expect("reopen");
        let mut snap2 = UiSnapshot::default();
        reopened
            .refresh_snapshot(&mut snap2)
            .expect("refresh reopen");
        assert_eq!(snap2.evidence_count, 1);
        assert_eq!(snap2.bookmark_count, 1);
        assert!(
            snap2.about_disclosure.contains("NOT court-ready"),
            "buyer-facing honesty must stay in disclosure"
        );
    }
}
