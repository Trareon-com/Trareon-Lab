//! Immutable analysis run manifests for exact rerun / compare.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Pinned CASE/UCO interchange profile version for Lab exports.
pub const CASE_UCO_PROFILE_VERSION: &str = "trareon-case-uco-2026.07";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunManifest {
    pub run_uuid: String,
    pub case_uuid: String,
    pub build_version: String,
    pub started_at_utc: String,
    pub locale: String,
    pub timezone: String,
    pub hash_set_pin: Option<String>,
    pub rule_set_pin: Option<String>,
    pub config_json: String,
    pub input_hashes: Vec<String>,
    pub output_hash: Option<String>,
    pub error_summary: Option<String>,
    pub coverage_status: String,
}

impl RunManifest {
    pub fn content_digest(&self) -> String {
        let payload = serde_json::to_vec(self).unwrap_or_default();
        let mut h = Sha256::new();
        h.update(&payload);
        hex::encode(h.finalize())
    }
}

/// Diff two manifests for Compare runs UI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunCompareLine {
    pub field: String,
    pub left: String,
    pub right: String,
}

pub fn compare_runs(a: &RunManifest, b: &RunManifest) -> Vec<RunCompareLine> {
    let mut lines = Vec::new();
    macro_rules! cmp {
        ($field:ident) => {
            if a.$field != b.$field {
                lines.push(RunCompareLine {
                    field: stringify!($field).into(),
                    left: format!("{:?}", a.$field),
                    right: format!("{:?}", b.$field),
                });
            }
        };
    }
    cmp!(build_version);
    cmp!(locale);
    cmp!(timezone);
    cmp!(hash_set_pin);
    cmp!(rule_set_pin);
    cmp!(config_json);
    cmp!(input_hashes);
    cmp!(output_hash);
    cmp!(error_summary);
    cmp!(coverage_status);
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_detects_output_diff() {
        let a = RunManifest {
            run_uuid: "1".into(),
            case_uuid: "c".into(),
            build_version: "0.1.0".into(),
            started_at_utc: "t".into(),
            locale: "en".into(),
            timezone: "UTC".into(),
            hash_set_pin: None,
            rule_set_pin: None,
            config_json: "{}".into(),
            input_hashes: vec!["aa".into()],
            output_hash: Some("out1".into()),
            error_summary: None,
            coverage_status: "Complete".into(),
        };
        let mut b = a.clone();
        b.output_hash = Some("out2".into());
        let diff = compare_runs(&a, &b);
        assert!(diff.iter().any(|l| l.field == "output_hash"));
        let _ = a.run_uuid;
    }

    #[test]
    fn profile_constant_non_empty() {
        assert!(!CASE_UCO_PROFILE_VERSION.is_empty());
    }
}
