//! Cryptographic digests and signature envelopes.

pub mod digest;
pub mod signature;

pub use digest::{digest_bytes, DigestAlg, DigestLabel, DigestValue};
pub use signature::{verify_ed25519_envelope, TrustState};
