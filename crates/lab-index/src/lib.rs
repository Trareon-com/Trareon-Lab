//! Forensic index SQLite store (searchable rows per case).

use std::path::{Path, PathBuf};

use lab_core::{LabError, LabResult};
use rusqlite::{params, params_from_iter, types::Value, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};

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

/// One parsed search operand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchTerm {
    Phrase(String),
    Text {
        value: String,
        wildcard_prefix: bool,
        wildcard_suffix: bool,
    },
    Hex(String),
}

/// Search expression represented as OR groups containing AND operands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchQuery {
    pub query: String,
    pub clauses: Vec<Vec<SearchTerm>>,
}

impl SearchQuery {
    /// Parse phrases, `AND`/`OR` (AND has precedence), wildcards, and `hex:` operands.
    pub fn parse(input: &str) -> LabResult<Self> {
        enum Token {
            Term(SearchTerm),
            And,
            Or,
        }

        let invalid = |detail: String| LabError::SchemaInvalid {
            schema: "search_query".into(),
            detail,
        };
        let parse_term = |raw: String, phrase: bool| -> LabResult<SearchTerm> {
            if raw.is_empty() {
                return Err(invalid("empty search operand".into()));
            }
            if phrase {
                return Ok(SearchTerm::Phrase(raw));
            }
            if raw
                .get(..4)
                .is_some_and(|prefix| prefix.eq_ignore_ascii_case("hex:"))
            {
                let hex = &raw[4..];
                if hex.is_empty()
                    || !hex.len().is_multiple_of(2)
                    || !hex.bytes().all(|b| b.is_ascii_hexdigit())
                {
                    return Err(invalid(
                        "hex: requires a non-empty, even number of hexadecimal digits".into(),
                    ));
                }
                return Ok(SearchTerm::Hex(hex.to_ascii_lowercase()));
            }
            let wildcard_prefix = raw.starts_with('*');
            let wildcard_suffix = raw.ends_with('*');
            let value = raw.trim_matches('*').to_string();
            Ok(SearchTerm::Text {
                value,
                wildcard_prefix,
                wildcard_suffix,
            })
        };

        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch.is_whitespace() {
                continue;
            }
            if ch == '"' {
                let mut phrase = String::new();
                let mut closed = false;
                while let Some(next) = chars.next() {
                    match next {
                        '"' => {
                            closed = true;
                            break;
                        }
                        '\\' => match chars.next() {
                            Some(escaped @ ('"' | '\\')) => phrase.push(escaped),
                            Some(other) => {
                                phrase.push('\\');
                                phrase.push(other);
                            }
                            None => phrase.push('\\'),
                        },
                        other => phrase.push(other),
                    }
                }
                if !closed {
                    return Err(invalid("unterminated quoted phrase".into()));
                }
                tokens.push(Token::Term(parse_term(phrase, true)?));
                continue;
            }

            let mut word = String::from(ch);
            while let Some(next) = chars.peek() {
                if next.is_whitespace() {
                    break;
                }
                word.push(chars.next().expect("peeked character"));
            }
            if word.eq_ignore_ascii_case("AND") {
                tokens.push(Token::And);
            } else if word.eq_ignore_ascii_case("OR") {
                tokens.push(Token::Or);
            } else {
                tokens.push(Token::Term(parse_term(word, false)?));
            }
        }

        let mut clauses = vec![Vec::new()];
        let mut expecting_term = true;
        for token in tokens {
            match token {
                Token::Term(term) => {
                    clauses.last_mut().expect("initial clause").push(term);
                    expecting_term = false;
                }
                Token::And => {
                    if expecting_term {
                        return Err(invalid("AND requires an operand on each side".into()));
                    }
                    expecting_term = true;
                }
                Token::Or => {
                    if expecting_term {
                        return Err(invalid("OR requires an operand on each side".into()));
                    }
                    clauses.push(Vec::new());
                    expecting_term = true;
                }
            }
        }
        if expecting_term && clauses.iter().any(|clause| !clause.is_empty()) {
            return Err(invalid("query cannot end with a boolean operator".into()));
        }

        Ok(Self {
            query: input.to_string(),
            clauses,
        })
    }
}

/// A bounded search page with explicit coverage status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchResult {
    pub entries: Vec<IndexEntry>,
    pub truncated: bool,
    pub truncation_reason: Option<String>,
}

/// Persistable record of the search inputs and observed result counts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchPlan {
    pub query: String,
    pub scope: String,
    pub encodings: Vec<String>,
    pub error_count: u64,
    pub hit_count: u64,
    pub tags: Vec<String>,
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

/// Index filesystem enumeration metadata into the search DB (Day 19).
pub fn index_fs_entries(
    db: &mut IndexDb,
    case_uuid: &str,
    created_at_utc: &str,
    entries: &[(String, String, String)],
) -> LabResult<u64> {
    // tuples: (entry_kind, target_ref, display_text)
    let mut batch = Vec::with_capacity(entries.len());
    for (i, (kind, target_ref, display_text)) in entries.iter().enumerate() {
        batch.push(IndexEntry {
            case_uuid: case_uuid.to_string(),
            entry_kind: kind.clone(),
            target_ref: target_ref.clone(),
            display_text: display_text.clone(),
            sort_key: format!("{i:08}"),
            created_at_utc: created_at_utc.to_string(),
        });
    }
    db.insert_entries_batch(&batch)?;
    Ok(batch.len() as u64)
}

impl IndexDb {
    /// Execute a parsed search and fetch at most `limit` rows.
    ///
    /// Paths and names are stored as UTF-8 SQLite `TEXT`. Hex operands search the
    /// ASCII-hex representation in `target_ref` and `display_text`; schema v1 has
    /// no separate content column.
    pub fn search(
        &self,
        case_uuid: &str,
        query: &SearchQuery,
        limit: usize,
    ) -> LabResult<SearchResult> {
        let mut sql = String::from(
            "SELECT case_uuid, entry_kind, target_ref, display_text, sort_key, created_at_utc
             FROM index_entry WHERE case_uuid = ?",
        );
        let mut bindings = vec![Value::Text(case_uuid.to_string())];

        if query.clauses.iter().any(|clause| !clause.is_empty()) {
            sql.push_str(" AND (");
            for (clause_index, clause) in query.clauses.iter().enumerate() {
                if clause_index > 0 {
                    sql.push_str(" OR ");
                }
                sql.push('(');
                for (term_index, term) in clause.iter().enumerate() {
                    if term_index > 0 {
                        sql.push_str(" AND ");
                    }
                    sql.push_str(
                        "(LOWER(target_ref) LIKE ? ESCAPE '\\' OR \
                         LOWER(display_text) LIKE ? ESCAPE '\\')",
                    );
                    let pattern = search_like_pattern(term);
                    bindings.push(Value::Text(pattern.clone()));
                    bindings.push(Value::Text(pattern));
                }
                sql.push(')');
            }
            sql.push(')');
        }
        sql.push_str(" ORDER BY sort_key ASC LIMIT ?");
        let fetch_limit = limit.saturating_add(1).min(i64::MAX as usize);
        bindings.push(Value::Integer(fetch_limit as i64));

        let mut stmt = self.conn.prepare(&sql).map_err(|e| LabError::Internal {
            detail: format!("prepare search query: {e}"),
        })?;
        let rows = stmt
            .query_map(params_from_iter(bindings), |row| {
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
                detail: format!("execute search query: {e}"),
            })?;
        let mut entries = Vec::new();
        for row in rows {
            entries.push(row.map_err(|e| LabError::Internal {
                detail: format!("read search result: {e}"),
            })?);
        }

        let truncated = entries.len() > limit;
        entries.truncate(limit);
        Ok(SearchResult {
            entries,
            truncated,
            truncation_reason: truncated.then(|| {
                format!(
                    "result limit {limit} reached; coverage is partial—raise the limit or narrow the query"
                )
            }),
        })
    }

    /// Day 41: search by path/name substring (hash treated as target_ref match).
    pub fn query_path_name_hash(
        &self,
        case_uuid: &str,
        needle: &str,
        limit: usize,
    ) -> LabResult<Vec<IndexEntry>> {
        let like = format!("%{needle}%");
        let mut stmt = self
            .conn
            .prepare(
                "SELECT case_uuid, entry_kind, target_ref, display_text, sort_key, created_at_utc
                 FROM index_entry
                 WHERE case_uuid=?1 AND (target_ref LIKE ?2 OR display_text LIKE ?2)
                 ORDER BY sort_key LIMIT ?3",
            )
            .map_err(|e| LabError::Internal {
                detail: format!("prepare query: {e}"),
            })?;
        let rows = stmt
            .query_map(params![case_uuid, like, limit as i64], |row| {
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
                detail: format!("query: {e}"),
            })?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r.map_err(|e| LabError::Internal {
                detail: format!("row: {e}"),
            })?);
        }
        Ok(out)
    }
}

fn search_like_pattern(term: &SearchTerm) -> String {
    let (value, wildcard_prefix, wildcard_suffix) = match term {
        SearchTerm::Phrase(value) | SearchTerm::Hex(value) => (value.as_str(), true, true),
        SearchTerm::Text {
            value,
            wildcard_prefix,
            wildcard_suffix,
        } => (value.as_str(), *wildcard_prefix, *wildcard_suffix),
    };
    let escaped = value
        .to_lowercase()
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_");
    match (wildcard_prefix, wildcard_suffix) {
        (true, true) => format!("%{escaped}%"),
        (true, false) => format!("%{escaped}"),
        (false, true) => format!("{escaped}%"),
        (false, false) => format!("%{escaped}%"),
    }
}
