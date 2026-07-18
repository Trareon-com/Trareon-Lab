//! YARA scanning via the pinned YARA-X engine.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use lab_core::{LabError, LabResult, ProgressEvent, ProgressSink};
use yara_x::{Compiler, MetaValue, Rules, Scanner};

/// Scanner implementation and version recorded in run manifests.
pub const YARA_ENGINE_ID: &str = "yara-x/1.19.0";

const BUILTIN_RULES: &str = r#"
rule Trareon_Eicar_Substring {
  meta:
    severity = "HIGH"
  strings:
    $eicar = "EICAR-STANDARD-ANTIVIRUS-TEST-FILE"
  condition:
    $eicar
}
"#;

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
    rules: Rules,
}

impl YaraEngine {
    /// Load `.yar` files from a directory plus built-in starter rules.
    pub fn load_dir(dir: Option<&Path>) -> LabResult<Self> {
        let mut sources = vec![BUILTIN_RULES.to_string()];
        if let Some(dir) = dir {
            if dir.is_dir() {
                let mut paths = std::fs::read_dir(dir)
                    .map_err(|e| LabError::Internal {
                        detail: format!("yara dir: {e}"),
                    })?
                    .map(|entry| {
                        entry
                            .map(|entry| entry.path())
                            .map_err(|e| LabError::Internal {
                                detail: format!("yara entry: {e}"),
                            })
                    })
                    .collect::<LabResult<Vec<_>>>()?;
                paths.sort();
                for path in paths.into_iter().filter(|path| {
                    matches!(
                        path.extension().and_then(|extension| extension.to_str()),
                        Some("yar" | "yara")
                    )
                }) {
                    sources.push(std::fs::read_to_string(&path).map_err(|e| {
                        LabError::Internal {
                            detail: format!("read YARA rule {}: {e}", path.display()),
                        }
                    })?);
                }
            }
        }
        Self::compile(sources)
    }

    pub fn from_source(source: &str) -> LabResult<Self> {
        Self::compile(vec![source.to_string()])
    }

    pub fn scan_bytes(&self, data: &[u8]) -> LabResult<Vec<YaraHit>> {
        let mut scanner = Scanner::new(&self.rules);
        let results = scanner.scan(data).map_err(|e| LabError::Internal {
            detail: format!("YARA-X scan: {e}"),
        })?;
        let mut hits = Vec::new();
        for rule in results.matching_rules() {
            let meta: HashMap<String, String> = rule
                .metadata()
                .map(|(key, value)| (key.to_string(), meta_value_to_string(value)))
                .collect();
            let severity = meta
                .get("severity")
                .cloned()
                .unwrap_or_else(|| "MEDIUM".to_string());
            let first_match = rule
                .patterns()
                .include_private(true)
                .flat_map(|pattern| pattern.matches())
                .min_by_key(|matched| matched.range().start);
            if let Some(matched) = first_match {
                hits.push(YaraHit {
                    rule_name: rule.identifier().to_string(),
                    meta,
                    offset: matched.range().start as u64,
                    matched_data: matched.data()[..matched.data().len().min(64)].to_vec(),
                    severity,
                });
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

    fn compile(sources: Vec<String>) -> LabResult<Self> {
        let mut compiler = Compiler::new();
        for source in &sources {
            compiler
                .add_source(source.as_str())
                .map_err(|e| LabError::Internal {
                    detail: format!("YARA-X compile: {e}"),
                })?;
        }
        Ok(Self {
            rules: compiler.build(),
        })
    }
}

fn meta_value_to_string(value: MetaValue<'_>) -> String {
    match value {
        MetaValue::Integer(value) => value.to_string(),
        MetaValue::Float(value) => value.to_string(),
        MetaValue::Bool(value) => value.to_string(),
        MetaValue::String(value) => value.to_string(),
        MetaValue::Bytes(value) => String::from_utf8_lossy(value.as_ref()).into_owned(),
    }
}
