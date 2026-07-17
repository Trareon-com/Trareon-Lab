//! Deterministic `.fsnap` import result (no silent repair of source package).

use std::fs;
use std::path::Path;

use lab_core::LabResult;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use crate::preflight::{PreflightLimits, preflight_package};

/// Import after successful preflight. `import_uuid` is caller-supplied for determinism in tests.
pub fn import_package(
    package_dir: &Path,
    case_uuid: &str,
    import_uuid: &str,
) -> LabResult<Value> {
    let ok = preflight_package(package_dir, PreflightLimits::default())?;
    let mut member_results = Vec::new();
    for name in &ok.members {
        let bytes = fs::read(package_dir.join(name)).map_err(|e| {
            lab_core::LabError::FsNapRejected {
                reason: format!("read_member:{name}:{e}"),
            }
        })?;
        let digest = hex::encode(Sha256::digest(&bytes));
        member_results.push(json!({
            "member": name,
            "sha256": digest,
            "byte_length": bytes.len(),
            "status": "accepted"
        }));
    }

    Ok(json!({
        "schema_version": "fsnap-import-result-1",
        "import_uuid": import_uuid,
        "case_uuid": case_uuid,
        "package_version": ok.package_version,
        "result": "accepted",
        "member_results": member_results
    }))
}
