//! Cryptographic digests and signature envelopes.

pub mod digest;
pub mod signature;

pub use digest::{DigestAlg, DigestLabel, DigestValue, digest_bytes};
pub use signature::{TrustState, verify_ed25519_envelope};
