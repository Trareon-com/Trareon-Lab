//! Offline transfer package export/import (Day 34).

use ed25519_dalek::{Signer, SigningKey};
use lab_core::{LabError, LabResult};
use lab_crypto::{verify_ed25519_envelope, TrustState};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TransferPackage {
    pub schema_version: String,
    pub transfer_uuid: String,
    pub source_case_uuid: String,
    pub created_at_utc: String,
    pub destination: String,
    pub purpose: String,
    pub authority_note: String,
    pub selected_bookmark_digests: Vec<String>,
    pub result: String,
    pub signature: SignatureBlock,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignatureBlock {
    pub algorithm: String,
    pub key_id: String,
    pub payload_digest: String,
    pub signature: String,
    pub trust_state: String,
}

fn fail(d: impl Into<String>) -> LabError {
    LabError::Internal { detail: d.into() }
}

fn payload_bytes(pkg: &TransferPackage) -> LabResult<Vec<u8>> {
    // Sign everything except signature block contents that circularly depend on digest.
    let mut unsigned = pkg.clone();
    unsigned.signature.signature = String::new();
    unsigned.signature.payload_digest = String::new();
    unsigned.signature.trust_state = "NOT_CHECKED".into();
    serde_json::to_vec(&unsigned).map_err(|e| fail(format!("serialize: {e}")))
}

pub struct LocalKeypair {
    pub signing: SigningKey,
    pub key_id: String,
}

impl LocalKeypair {
    pub fn generate(key_id: impl Into<String>) -> Self {
        Self {
            signing: SigningKey::generate(&mut OsRng),
            key_id: key_id.into(),
        }
    }

    pub fn public_bytes(&self) -> [u8; 32] {
        self.signing.verifying_key().to_bytes()
    }
}

pub fn export_signed_package(
    mut pkg: TransferPackage,
    keys: &LocalKeypair,
) -> LabResult<TransferPackage> {
    pkg.schema_version = "transfer-package-1".into();
    pkg.result = "export_ready".into();
    pkg.signature.algorithm = "Ed25519".into();
    pkg.signature.key_id = keys.key_id.clone();
    let payload = payload_bytes(&pkg)?;
    let digest = hex::encode(Sha256::digest(&payload));
    pkg.signature.payload_digest = digest.clone();
    let sig = keys.signing.sign(digest.as_bytes());
    pkg.signature.signature = hex::encode(sig.to_bytes());
    pkg.signature.trust_state = TrustState::ValidUntrusted.as_str().into();
    Ok(pkg)
}

pub fn import_verify_package(
    pkg: &TransferPackage,
    public_key: &[u8; 32],
    key_trusted: bool,
) -> LabResult<TrustState> {
    if pkg.schema_version != "transfer-package-1" {
        return Err(fail("bad transfer schema"));
    }
    let payload = payload_bytes(pkg)?;
    let sig_bytes =
        hex::decode(&pkg.signature.signature).map_err(|e| LabError::IntegrityFailed {
            detail: format!("sig hex: {e}"),
        })?;
    let state = verify_ed25519_envelope(
        &payload,
        &pkg.signature.payload_digest,
        &sig_bytes,
        public_key,
        key_trusted,
    )?;
    if state == TrustState::Invalid {
        return Err(LabError::IntegrityFailed {
            detail: "INTEGRITY_FAILED: transfer signature rejected".into(),
        });
    }
    Ok(state)
}
