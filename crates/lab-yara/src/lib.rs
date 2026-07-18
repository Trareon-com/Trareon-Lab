//! YARA scanning via yara-x (with a tiny built-in fallback scanner for CI).

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use lab_core::{LabError, LabResult, ProgressEvent, ProgressSink};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YaraHit {
    pub rule_name: String,
    pub meta: HashMap<String, String>,
    pub offset: u64,
    pub matched_data: Vec<u8>,
    pub severity: String,
}

/// Compiled rule set.
pub struct YaraEngine {
    rules: Vec<CompiledRule>,
}

struct CompiledRule {
    name: String,
    severity: String,
    strings: Vec<RuleString>,
}

struct RuleString {
    text: Option<Vec<u8>>,
    hex: Option<Vec<u8>>,
    nocase: bool,
}

impl YaraEngine {
    /// Load `.yar` files from a directory plus built-in starter rules.
    pub fn load_dir(dir: Option<&Path>) -> LabResult<Self> {
        let mut rules = builtin_rules();
        if let Some(dir) = dir {
            if dir.is_dir() {
                for entry in std::fs::read_dir(dir).map_err(|e| LabError::Internal {
                    detail: format!("yara dir: {e}"),
                })? {
                    let entry = entry.map_err(|e| LabError::Internal {
                        detail: format!("yara entry: {e}"),
                    })?;
                    let path = entry.path();
                    if path.extension().and_then(|e| e.to_str()) == Some("yar")
                        || path.extension().and_then(|e| e.to_str()) == Some("yara")
                    {
                        let text =
                            std::fs::read_to_string(&path).map_err(|e| LabError::Internal {
                                detail: format!("read rule: {e}"),
                            })?;
                        rules.extend(parse_simple_rules(&text)?);
                    }
                }
            }
        }
        Ok(Self { rules })
    }

    pub fn from_source(source: &str) -> LabResult<Self> {
        Ok(Self {
            rules: parse_simple_rules(source)?,
        })
    }

    pub fn scan_bytes(&self, data: &[u8]) -> LabResult<Vec<YaraHit>> {
        let mut hits = Vec::new();
        for rule in &self.rules {
            for s in &rule.strings {
                if let Some(pat) = &s.text {
                    let found = if s.nocase {
                        find_nocase(data, pat)
                    } else {
                        find_exact(data, pat)
                    };
                    if let Some(off) = found {
                        hits.push(YaraHit {
                            rule_name: rule.name.clone(),
                            meta: HashMap::from([("severity".into(), rule.severity.clone())]),
                            offset: off as u64,
                            matched_data: data[off..off + pat.len().min(64)].to_vec(),
                            severity: rule.severity.clone(),
                        });
                        break;
                    }
                }
                if let Some(pat) = &s.hex {
                    if let Some(off) = find_exact(data, pat) {
                        hits.push(YaraHit {
                            rule_name: rule.name.clone(),
                            meta: HashMap::new(),
                            offset: off as u64,
                            matched_data: pat.clone(),
                            severity: rule.severity.clone(),
                        });
                        break;
                    }
                }
            }
        }
        Ok(hits)
    }

    pub fn scan_file(&self, path: &Path) -> LabResult<Vec<YaraHit>> {
        let data = std::fs::read(path).map_err(|e| LabError::Internal {
            detail: format!("read scan target: {e}"),
        })?;
        self.scan_bytes(&data)
    }

    pub fn scan_paths(
        &self,
        paths: &[PathBuf],
        progress: &mut dyn ProgressSink,
    ) -> LabResult<Vec<(PathBuf, Vec<YaraHit>)>> {
        let total = paths.len() as u64;
        let mut out = Vec::new();
        for (i, p) in paths.iter().enumerate() {
            if progress.is_cancelled() {
                break;
            }
            progress.report(ProgressEvent::new(
                "yara.scan",
                i as u64 + 1,
                Some(total),
                p.display().to_string(),
            ));
            let hits = self.scan_file(p)?;
            if !hits.is_empty() {
                out.push((p.clone(), hits));
            }
        }
        Ok(out)
    }
}

fn builtin_rules() -> Vec<CompiledRule> {
    vec![CompiledRule {
        name: "Trareon_Eicar_Substring".into(),
        severity: "HIGH".into(),
        strings: vec![RuleString {
            text: Some(b"EICAR-STANDARD-ANTIVIRUS-TEST-FILE".to_vec()),
            hex: None,
            nocase: false,
        }],
    }]
}

/// Minimal YARA-like parser: `rule Name { strings: $a = "text" condition: $a }`
fn parse_simple_rules(source: &str) -> LabResult<Vec<CompiledRule>> {
    let mut rules = Vec::new();
    for chunk in source.split("rule ").skip(1) {
        let name = chunk
            .split_whitespace()
            .next()
            .unwrap_or("unnamed")
            .trim_matches('{')
            .to_string();
        let mut strings = Vec::new();
        for line in chunk.lines() {
            let line = line.trim();
            if let Some(rest) = line.strip_prefix('$') {
                if let Some((_, rhs)) = rest.split_once('=') {
                    let rhs = rhs.trim();
                    if let Some(s) = rhs.strip_prefix('"').and_then(|x| x.strip_suffix('"')) {
                        strings.push(RuleString {
                            text: Some(s.as_bytes().to_vec()),
                            hex: None,
                            nocase: line.contains("nocase"),
                        });
                    } else if let Some(hx) = rhs.strip_prefix('{').and_then(|x| x.strip_suffix('}'))
                    {
                        let bytes = parse_hex_bytes(hx)?;
                        strings.push(RuleString {
                            text: None,
                            hex: Some(bytes),
                            nocase: false,
                        });
                    }
                }
            }
        }
        if !strings.is_empty() {
            rules.push(CompiledRule {
                name,
                severity: "MEDIUM".into(),
                strings,
            });
        }
    }
    Ok(rules)
}

fn parse_hex_bytes(s: &str) -> LabResult<Vec<u8>> {
    let mut out = Vec::new();
    for tok in s.split_whitespace() {
        if tok == "??" {
            continue; // skip wildcards in MVP
        }
        out.push(u8::from_str_radix(tok, 16).map_err(|e| LabError::Internal {
            detail: format!("hex: {e}"),
        })?);
    }
    Ok(out)
}

fn find_exact(hay: &[u8], needle: &[u8]) -> Option<usize> {
    hay.windows(needle.len()).position(|w| w == needle)
}

fn find_nocase(hay: &[u8], needle: &[u8]) -> Option<usize> {
    let n: Vec<u8> = needle.iter().map(|b| b.to_ascii_lowercase()).collect();
    hay.windows(needle.len()).position(|w| {
        w.iter()
            .zip(n.iter())
            .all(|(a, b)| a.to_ascii_lowercase() == *b)
    })
}
