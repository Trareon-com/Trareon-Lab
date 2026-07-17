//! Bookmark CRUD on case DB (Day 33).

use lab_core::{LabError, LabResult};
use rusqlite::params;

use crate::CaseDb;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BookmarkRecord {
    pub bookmark_uuid: String,
    pub case_uuid: String,
    pub target_kind: String,
    pub target_ref: String,
    pub citation: String,
    pub author_role: String,
    pub created_at_utc: String,
    pub review_state: String,
    pub note: Option<String>,
}

impl CaseDb {
    pub fn upsert_bookmark(&self, b: &BookmarkRecord) -> LabResult<()> {
        self.connection()
            .execute(
                "INSERT INTO bookmark(
                    bookmark_uuid, case_uuid, target_kind, target_ref, citation,
                    author_role, created_at_utc, review_state, note
                 ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)
                 ON CONFLICT(bookmark_uuid) DO UPDATE SET
                    citation=excluded.citation,
                    review_state=excluded.review_state,
                    note=excluded.note",
                params![
                    b.bookmark_uuid,
                    b.case_uuid,
                    b.target_kind,
                    b.target_ref,
                    b.citation,
                    b.author_role,
                    b.created_at_utc,
                    b.review_state,
                    b.note,
                ],
            )
            .map_err(|e| LabError::Internal {
                detail: format!("upsert bookmark: {e}"),
            })?;
        Ok(())
    }

    pub fn list_bookmarks(&self, case_uuid: &str) -> LabResult<Vec<BookmarkRecord>> {
        let mut stmt = self
            .connection()
            .prepare(
                "SELECT bookmark_uuid, case_uuid, target_kind, target_ref, citation,
                        author_role, created_at_utc, review_state, note
                 FROM bookmark WHERE case_uuid=?1 ORDER BY created_at_utc",
            )
            .map_err(|e| LabError::Internal {
                detail: format!("prepare bookmarks: {e}"),
            })?;
        let rows = stmt
            .query_map([case_uuid], |row| {
                Ok(BookmarkRecord {
                    bookmark_uuid: row.get(0)?,
                    case_uuid: row.get(1)?,
                    target_kind: row.get(2)?,
                    target_ref: row.get(3)?,
                    citation: row.get(4)?,
                    author_role: row.get(5)?,
                    created_at_utc: row.get(6)?,
                    review_state: row.get(7)?,
                    note: row.get(8)?,
                })
            })
            .map_err(|e| LabError::Internal {
                detail: format!("query bookmarks: {e}"),
            })?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r.map_err(|e| LabError::Internal {
                detail: format!("row bookmark: {e}"),
            })?);
        }
        Ok(out)
    }

    pub fn supersede_bookmark(&self, bookmark_uuid: &str) -> LabResult<()> {
        let n = self
            .connection()
            .execute(
                "UPDATE bookmark SET review_state='superseded' WHERE bookmark_uuid=?1",
                [bookmark_uuid],
            )
            .map_err(|e| LabError::Internal {
                detail: format!("supersede bookmark: {e}"),
            })?;
        if n == 0 {
            return Err(LabError::Internal {
                detail: "bookmark not found".into(),
            });
        }
        Ok(())
    }
}
