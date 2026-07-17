//! Case metadata SQLite sidecar (admin tables only; forensic index is separate).

use std::path::{Path, PathBuf};

use lab_core::{LabError, LabResult};
use rusqlite::{Connection, OptionalExtension};

const CURRENT_SCHEMA_VERSION: i32 = 1;

/// Versioned case metadata database.
pub struct CaseDb {
    path: PathBuf,
    conn: Connection,
    schema_version: i32,
}

impl CaseDb {
    /// Open (or create) the case metadata DB and apply pending migrations.
    pub fn open_and_migrate(path: &Path) -> LabResult<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| LabError::Internal {
                detail: format!("create case dir: {e}"),
            })?;
        }

        let conn = Connection::open(path).map_err(|e| LabError::Internal {
            detail: format!("open case db: {e}"),
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

    /// Borrow the underlying connection for case admin tables.
    pub fn connection(&self) -> &Connection {
        &self.conn
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
                detail: format!("apply v1 migration: {e}"),
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
