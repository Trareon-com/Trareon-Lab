//! Forensic index SQLite store (searchable rows per case).

use std::path::{Path, PathBuf};

use lab_core::{LabError, LabResult};
use rusqlite::{params, Connection, OptionalExtension};

const CURRENT_SCHEMA_VERSION: i32 = 1;

/// One searchable index row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexEntry {
    pub case_uuid: String,
    pub entry_kind: String,
    pub target_ref: String,
    pub display_text: String,
    pub sort_key: String,
    pub created_at_utc: String,
}

/// Versioned forensic index database.
pub struct IndexDb {
    path: PathBuf,
    conn: Connection,
    schema_version: i32,
}

impl IndexDb {
    /// Open (or create) the index DB and apply pending migrations.
    pub fn open_and_migrate(path: &Path) -> LabResult<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| LabError::Internal {
                detail: format!("create index dir: {e}"),
            })?;
        }

        let conn = Connection::open(path).map_err(|e| LabError::Internal {
            detail: format!("open index db: {e}"),
        })?;

        conn.execute_batch(
            "PRAGMA foreign_keys = ON;
             CREATE TABLE IF NOT EXISTS schema_meta (
               key TEXT PRIMARY KEY NOT NULL,
               value TEXT NOT NULL
             );",
        )
        .map_err(|e| LabError::Internal {
            detail: format!("init schema_meta: {e}"),
        })?;

        let mut db = Self {
            path: path.to_path_buf(),
            conn,
            schema_version: 0,
        };
        db.schema_version = db.read_schema_version()?;
        db.migrate_to_current()?;
        Ok(db)
    }

    /// Current applied schema version.
    pub fn schema_version(&self) -> i32 {
        self.schema_version
    }

    /// Filesystem path of the SQLite file.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Insert one index entry.
    pub fn insert_entry(&self, entry: &IndexEntry) -> LabResult<()> {
        self.conn
            .execute(
                "INSERT INTO index_entry(
                    case_uuid, entry_kind, target_ref, display_text, sort_key, created_at_utc
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    entry.case_uuid,
                    entry.entry_kind,
                    entry.target_ref,
                    entry.display_text,
                    entry.sort_key,
                    entry.created_at_utc,
                ],
            )
            .map_err(|e| LabError::Internal {
                detail: format!("insert index_entry: {e}"),
            })?;
        Ok(())
    }

    /// Bulk insert entries inside a single transaction.
    pub fn insert_entries_batch(&mut self, entries: &[IndexEntry]) -> LabResult<()> {
        let tx = self.conn.transaction().map_err(|e| LabError::Internal {
            detail: format!("begin index batch: {e}"),
        })?;
        {
            let mut stmt = tx
                .prepare(
                    "INSERT INTO index_entry(
                        case_uuid, entry_kind, target_ref, display_text, sort_key, created_at_utc
                     ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                )
                .map_err(|e| LabError::Internal {
                    detail: format!("prepare index batch: {e}"),
                })?;
            for entry in entries {
                stmt.execute(params![
                    entry.case_uuid,
                    entry.entry_kind,
                    entry.target_ref,
                    entry.display_text,
                    entry.sort_key,
                    entry.created_at_utc,
                ])
                .map_err(|e| LabError::Internal {
                    detail: format!("batch insert index_entry: {e}"),
                })?;
            }
        }
        tx.commit().map_err(|e| LabError::Internal {
            detail: format!("commit index batch: {e}"),
        })?;
        Ok(())
    }

    /// Count rows for a case.
    pub fn count_for_case(&self, case_uuid: &str) -> LabResult<u64> {
        let count: i64 = self
            .conn
            .query_row(
                "SELECT COUNT(*) FROM index_entry WHERE case_uuid = ?1",
                [case_uuid],
                |row| row.get(0),
            )
            .map_err(|e| LabError::Internal {
                detail: format!("count index_entry: {e}"),
            })?;
        Ok(count as u64)
    }

    /// Fetch a page of entries ordered by sort_key.
    pub fn list_for_case(
        &self,
        case_uuid: &str,
        limit: usize,
        offset: usize,
    ) -> LabResult<Vec<IndexEntry>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT case_uuid, entry_kind, target_ref, display_text, sort_key, created_at_utc
                 FROM index_entry
                 WHERE case_uuid = ?1
                 ORDER BY sort_key ASC
                 LIMIT ?2 OFFSET ?3",
            )
            .map_err(|e| LabError::Internal {
                detail: format!("prepare list index_entry: {e}"),
            })?;
        let rows = stmt
            .query_map(params![case_uuid, limit as i64, offset as i64], |row| {
                Ok(IndexEntry {
                    case_uuid: row.get(0)?,
                    entry_kind: row.get(1)?,
                    target_ref: row.get(2)?,
                    display_text: row.get(3)?,
                    sort_key: row.get(4)?,
                    created_at_utc: row.get(5)?,
                })
            })
            .map_err(|e| LabError::Internal {
                detail: format!("query index_entry: {e}"),
            })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row.map_err(|e| LabError::Internal {
                detail: format!("row index_entry: {e}"),
            })?);
        }
        Ok(out)
    }

    fn read_schema_version(&self) -> LabResult<i32> {
        let value: Option<String> = self
            .conn
            .query_row(
                "SELECT value FROM schema_meta WHERE key = 'schema_version'",
                [],
                |row| row.get(0),
            )
            .optional()
            .map_err(|e| LabError::Internal {
                detail: format!("read schema_version: {e}"),
            })?;

        match value {
            None => Ok(0),
            Some(v) => v.parse::<i32>().map_err(|e| LabError::Internal {
                detail: format!("parse schema_version: {e}"),
            }),
        }
    }

    fn migrate_to_current(&mut self) -> LabResult<()> {
        while self.schema_version < CURRENT_SCHEMA_VERSION {
            let next = self.schema_version + 1;
            match next {
                1 => self.apply_v1()?,
                other => {
                    return Err(LabError::Internal {
                        detail: format!("unknown migration target {other}"),
                    });
                }
            }
            self.set_schema_version(next)?;
            self.schema_version = next;
        }
        Ok(())
    }

    fn apply_v1(&self) -> LabResult<()> {
        let sql = include_str!("../migrations/001_init.sql");
        self.conn
            .execute_batch(sql)
            .map_err(|e| LabError::Internal {
                detail: format!("apply index v1 migration: {e}"),
            })
    }

    fn set_schema_version(&self, version: i32) -> LabResult<()> {
        self.conn
            .execute(
                "INSERT INTO schema_meta(key, value) VALUES('schema_version', ?1)
                 ON CONFLICT(key) DO UPDATE SET value = excluded.value",
                [version.to_string()],
            )
            .map_err(|e| LabError::Internal {
                detail: format!("set schema_version: {e}"),
            })?;
        Ok(())
    }
}
