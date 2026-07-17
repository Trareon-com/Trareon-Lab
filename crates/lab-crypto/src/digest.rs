//! Digests per `docs/CRYPTOGRAPHIC-PROFILE.md`.

use md5::{Digest as Md5Digest, Md5};
use sha2::{Digest as Sha2Digest, Sha256};

/// Supported digest algorithms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DigestAlg {
    Sha256,
    /// Legacy correlation only — never authoritative integrity.
    Md5Legacy,
}

/// Label distinguishing authoritative vs legacy digests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DigestLabel {
    Authoritative,
    NonAuthoritativeLegacyCorrelation,
}

/// Computed digest with algorithm identifier and forensic label.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigestValue {
    pub algorithm: &'static str,
    pub hex: String,
    pub label: DigestLabel,
}

/// Hash `bytes` and return hex digest with profile label.
pub fn digest_bytes(alg: DigestAlg, bytes: &[u8]) -> DigestValue {
    match alg {
        DigestAlg::Sha256 => DigestValue {
            algorithm: "sha256",
            hex: hex::encode(<Sha256 as Sha2Digest>::digest(bytes)),
            label: DigestLabel::Authoritative,
        },
        DigestAlg::Md5Legacy => DigestValue {
            algorithm: "md5",
            hex: hex::encode(<Md5 as Md5Digest>::digest(bytes)),
            label: DigestLabel::NonAuthoritativeLegacyCorrelation,
        },
    }
}
