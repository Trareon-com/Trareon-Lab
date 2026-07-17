//! NSRL / known-bad hash set database.

use std::collections::HashMap;
use std::path::Path;

use lab_core::{LabError, LabResult, ProgressEvent, ProgressSink};
use rusqlite::{params, Connection};

/// Result of looking up a file hash.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HashLookupResult {
    KnownGood { name: String, product: String },
    KnownBad { threat: String },
    Unknown,
    NotChecked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashMatch {
    pub hash: String,
    pub set_name: String,
    pub threat: ThreatLevel,
    pub details: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreatLevel {
    Good,
    Suspicious,
    Malicious,
}

/// Bloom filter for fast negative NSRL checks.
#[derive(Debug, Clone)]
pub struct BloomFilter {
    bits: Vec<u64>,
    k: u32,
}

impl BloomFilter {
    pub fn new(bits: usize, k: u32) -> Self {
        Self {
            bits: vec![0u64; bits.div_ceil(64)],
            k,
        }
    }

    fn indexes(&self, hash_hex: &str) -> Vec<usize> {
        let h = fnv1a64(hash_hex.as_bytes());
        let mut out = Vec::with_capacity(self.k as usize);
        let nbits = self.bits.len() * 64;
        for i in 0..self.k {
            let idx = h.wrapping_mul(0x9E37_79B9_7F4A_7C15).wrapping_add(i as u64) as usize % nbits;
            out.push(idx);
        }
        out
    }

    pub fn insert(&mut self, hash_hex: &str) {
        for idx in self.indexes(hash_hex) {
            self.bits[idx / 64] |= 1u64 << (idx % 64);
        }
    }

    pub fn maybe_contains(&self, hash_hex: &str) -> bool {
        self.indexes(hash_hex)
            .into_iter()
            .all(|idx| self.bits[idx / 64] & (1u64 << (idx % 64)) != 0)
    }
}

fn fnv1a64(data: &[u8]) -> u64 {
    let mut h = 0xcbf29ce484222325u64;
    for &b in data {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

/// Hash set DB: bloom + SQLite.
pub struct HashSetDb {
    db: Connection,
    nsrl_bloom: BloomFilter,
    known_bad: HashMap<String, String>,
}

impl HashSetDb {
    pub fn open(path: &Path) -> LabResult<Self> {
        let db = Connection::open(path).map_err(|e| LabError::Internal {
            detail: format!("hashset open: {e}"),
        })?;
        db.execute_batch(
            "CREATE TABLE IF NOT EXISTS nsrl (
                sha256 TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                product TEXT NOT NULL
             );
             CREATE TABLE IF NOT EXISTS known_bad (
                sha256 TEXT PRIMARY KEY,
                threat TEXT NOT NULL
             );",
        )
        .map_err(|e| LabError::Internal {
            detail: format!("hashset migrate: {e}"),
        })?;
        Ok(Self {
            db,
            nsrl_bloom: BloomFilter::new(1 << 20, 4),
            known_bad: HashMap::new(),
        })
    }

    /// Import NSRL-like CSV: `sha256,name,product` (header optional).
    pub fn import_nsrl(&mut self, path: &Path, progress: &mut dyn ProgressSink) -> LabResult<u64> {
        let text = std::fs::read_to_string(path).map_err(|e| LabError::Internal {
            detail: format!("read nsrl: {e}"),
        })?;
        let mut count = 0u64;
        let tx = self.db.unchecked_transaction().map_err(|e| LabError::Internal {
            detail: format!("tx: {e}"),
        })?;
        for (i, line) in text.lines().enumerate() {
            if progress.is_cancelled() {
                break;
            }
            let line = line.trim();
            if line.is_empty() || line.starts_with("sha256") {
                continue;
            }
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() < 3 {
                continue;
            }
            let sha = parts[0].trim().to_ascii_lowercase();
            let name = parts[1].trim();
            let product = parts[2].trim();
            tx.execute(
                "INSERT OR REPLACE INTO nsrl(sha256,name,product) VALUES (?1,?2,?3)",
                params![sha, name, product],
            )
            .map_err(|e| LabError::Internal {
                detail: format!("insert nsrl: {e}"),
            })?;
            self.nsrl_bloom.insert(&sha);
            count += 1;
            if i % 100 == 0 {
                progress.report(ProgressEvent::new(
                    "hashset.nsrl",
                    count,
                    None,
                    format!("imported {count}"),
                ));
            }
        }
        tx.commit().map_err(|e| LabError::Internal {
            detail: format!("commit: {e}"),
        })?;
        Ok(count)
    }

    pub fn import_known_bad(&mut self, path: &Path) -> LabResult<u64> {
        let text = std::fs::read_to_string(path).map_err(|e| LabError::Internal {
            detail: format!("read known_bad: {e}"),
        })?;
        let mut count = 0u64;
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = line.splitn(2, ',').collect();
            if parts.is_empty() {
                continue;
            }
            let sha = parts[0].trim().to_ascii_lowercase();
            let threat = parts.get(1).copied().unwrap_or("malicious").trim();
            self.db
                .execute(
                    "INSERT OR REPLACE INTO known_bad(sha256,threat) VALUES (?1,?2)",
                    params![sha, threat],
                )
                .map_err(|e| LabError::Internal {
                    detail: format!("insert bad: {e}"),
                })?;
            self.known_bad.insert(sha, threat.to_string());
            count += 1;
        }
        Ok(count)
    }

    pub fn lookup(&self, sha256: &str) -> LabResult<HashLookupResult> {
        let sha = sha256.to_ascii_lowercase();
        if let Some(threat) = self.known_bad.get(&sha) {
            return Ok(HashLookupResult::KnownBad {
                threat: threat.clone(),
            });
        }
        // Check SQLite for known_bad too (after restart)
        if let Ok(threat) = self.db.query_row(
            "SELECT threat FROM known_bad WHERE sha256=?1",
            params![sha],
            |r| r.get::<_, String>(0),
        ) {
            return Ok(HashLookupResult::KnownBad { threat });
        }
        if !self.nsrl_bloom.maybe_contains(&sha) {
            return Ok(HashLookupResult::Unknown);
        }
        match self.db.query_row(
            "SELECT name, product FROM nsrl WHERE sha256=?1",
            params![sha],
            |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)),
        ) {
            Ok((name, product)) => Ok(HashLookupResult::KnownGood { name, product }),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(HashLookupResult::Unknown),
            Err(e) => Err(LabError::Internal {
                detail: format!("nsrl lookup: {e}"),
            }),
        }
    }

    pub fn batch_lookup(&self, hashes: &[String]) -> LabResult<Vec<HashLookupResult>> {
        hashes.iter().map(|h| self.lookup(h)).collect()
    }
}
