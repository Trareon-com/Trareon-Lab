//! Browser artifact parsers (Chromium + Firefox).

use lab_core::{LabError, LabResult, ProgressEvent, ProgressSink};
use rusqlite::Connection;

use crate::ArtifactHit;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrowserHistory {
    pub url: String,
    pub title: String,
    pub visit_count: u32,
    pub last_visit: String,
    pub typed_count: u32,
    pub transition: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrowserCookie {
    pub host: String,
    pub name: String,
    pub value: String,
    pub creation_utc: String,
    pub expires_utc: String,
    pub secure: bool,
    pub httponly: bool,
}

pub mod chrome {
    use super::*;

    pub fn parse_history(
        path: &std::path::Path,
        progress: &mut dyn ProgressSink,
    ) -> LabResult<Vec<BrowserHistory>> {
        parse_chromium_history(path, progress)
    }

    pub fn parse_downloads(path: &std::path::Path) -> LabResult<Vec<ArtifactHit>> {
        let conn = open_ro(path)?;
        let mut stmt = conn
            .prepare("SELECT target_path, tab_url, start_time FROM downloads LIMIT 1000")
            .map_err(|e| LabError::Internal {
                detail: format!("downloads prep: {e}"),
            })?;
        let rows = stmt
            .query_map([], |r| {
                Ok(ArtifactHit {
                    kind: "browser.chrome.download".into(),
                    summary: format!("{} <- {}", r.get::<_, String>(0)?, r.get::<_, String>(1)?),
                    occurred_at_utc: chrome_time(r.get::<_, i64>(2).unwrap_or(0)),
                    provenance_ref: "chrome:downloads".into(),
                    source_path: path.display().to_string(),
                })
            })
            .map_err(|e| LabError::Internal {
                detail: format!("downloads query: {e}"),
            })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row.map_err(|e| LabError::Internal {
                detail: format!("row: {e}"),
            })?);
        }
        Ok(out)
    }
}

pub mod edge {
    use super::*;

    pub fn parse_history(
        path: &std::path::Path,
        progress: &mut dyn ProgressSink,
    ) -> LabResult<Vec<BrowserHistory>> {
        // Edge is Chromium-based
        parse_chromium_history(path, progress)
    }
}

pub mod firefox {
    use super::*;

    pub fn parse_places(
        path: &std::path::Path,
        progress: &mut dyn ProgressSink,
    ) -> LabResult<Vec<BrowserHistory>> {
        progress.report(ProgressEvent::new("browser.firefox", 0, Some(1), "places"));
        let conn = open_ro(path)?;
        let mut stmt = conn
            .prepare(
                "SELECT p.url, COALESCE(p.title,''), p.visit_count, COALESCE(p.last_visit_date,0)
                 FROM moz_places p WHERE p.url IS NOT NULL LIMIT 5000",
            )
            .map_err(|e| LabError::Internal {
                detail: format!("places: {e}"),
            })?;
        let rows = stmt
            .query_map([], |r| {
                Ok(BrowserHistory {
                    url: r.get(0)?,
                    title: r.get(1)?,
                    visit_count: r.get::<_, i64>(2).unwrap_or(0) as u32,
                    last_visit: firefox_time(r.get::<_, i64>(3).unwrap_or(0)),
                    typed_count: 0,
                    transition: "UNKNOWN".into(),
                })
            })
            .map_err(|e| LabError::Internal {
                detail: format!("places map: {e}"),
            })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row.map_err(|e| LabError::Internal {
                detail: format!("row: {e}"),
            })?);
        }
        Ok(out)
    }
}

fn parse_chromium_history(
    path: &std::path::Path,
    progress: &mut dyn ProgressSink,
) -> LabResult<Vec<BrowserHistory>> {
    progress.report(ProgressEvent::new(
        "browser.chromium",
        0,
        Some(1),
        "history",
    ));
    let conn = open_ro(path)?;
    let mut stmt = conn
        .prepare(
            "SELECT u.url, COALESCE(u.title,''), u.visit_count, u.typed_count, u.last_visit_time
             FROM urls u LIMIT 5000",
        )
        .map_err(|e| LabError::Internal {
            detail: format!("history prep: {e}"),
        })?;
    let rows = stmt
        .query_map([], |r| {
            Ok(BrowserHistory {
                url: r.get(0)?,
                title: r.get(1)?,
                visit_count: r.get::<_, i64>(2).unwrap_or(0) as u32,
                typed_count: r.get::<_, i64>(3).unwrap_or(0) as u32,
                last_visit: chrome_time(r.get::<_, i64>(4).unwrap_or(0)),
                transition: "LINK".into(),
            })
        })
        .map_err(|e| LabError::Internal {
            detail: format!("history map: {e}"),
        })?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row.map_err(|e| LabError::Internal {
            detail: format!("row: {e}"),
        })?);
    }
    Ok(out)
}

fn open_ro(path: &std::path::Path) -> LabResult<Connection> {
    let uri = format!("file:{}?mode=ro", path.display());
    Connection::open_with_flags(
        &uri,
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY | rusqlite::OpenFlags::SQLITE_OPEN_URI,
    )
    .or_else(|_| Connection::open(path))
    .map_err(|e| LabError::Internal {
        detail: format!("sqlite open {}: {e}", path.display()),
    })
}

fn chrome_time(v: i64) -> String {
    // microseconds since 1601-01-01
    if v == 0 {
        return String::new();
    }
    format!("chrome_time:{v}")
}

fn firefox_time(v: i64) -> String {
    // microseconds since unix epoch
    if v == 0 {
        return String::new();
    }
    format!("firefox_time:{v}")
}
