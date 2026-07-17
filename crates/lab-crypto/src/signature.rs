//! Ed25519 signature envelope verification and trust states.

use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use lab_core::{LabError, LabResult};

use crate::digest::{digest_bytes, DigestAlg};

/// Exact trust states from the cryptographic profile / signature-envelope schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrustState {
    ValidTrusted,
    ValidUntrusted,
    Invalid,
    ExpiredOrRevoked,
    NotSigned,
    NotChecked,
}

impl TrustState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ValidTrusted => "VALID_TRUSTED",
            Self::ValidUntrusted => "VALID_UNTRUSTED",
            Self::Invalid => "INVALID",
            Self::ExpiredOrRevoked => "EXPIRED_OR_REVOKED",
            Self::NotSigned => "NOT_SIGNED",
            Self::NotChecked => "NOT_CHECKED",
        }
    }
}

/// Verify Ed25519 signature over the SHA-256 payload digest bytes (hex UTF-8).
///
/// `key_trusted` selects VALID_TRUSTED vs VALID_UNTRUSTED when the signature verifies.
pub fn verify_ed25519_envelope(
    payload: &[u8],
    expected_payload_digest_hex: &str,
    signature_bytes: &[u8],
    public_key_bytes: &[u8],
    key_trusted: bool,
) -> LabResult<TrustState> {
    let computed = digest_bytes(DigestAlg::Sha256, payload);
    if computed.hex != expected_payload_digest_hex {
        return Ok(TrustState::Invalid);
    }

    let verifying_key = VerifyingKey::from_bytes(public_key_bytes.try_into().map_err(|_| {
        LabError::IntegrityFailed {
            detail: "ed25519 public key must be 32 bytes".into(),
        }
    })?)
    .map_err(|e| LabError::IntegrityFailed {
        detail: format!("ed25519 public key: {e}"),
    })?;

    let signature =
        Signature::from_slice(signature_bytes).map_err(|e| LabError::IntegrityFailed {
            detail: format!("ed25519 signature: {e}"),
        })?;

    match verifying_key.verify(computed.hex.as_bytes(), &signature) {
        Ok(()) => Ok(if key_trusted {
            TrustState::ValidTrusted
        } else {
            TrustState::ValidUntrusted
        }),
        Err(_) => Ok(TrustState::Invalid),
    }
}
