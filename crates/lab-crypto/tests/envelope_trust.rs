//! F7: Ed25519 envelope trust states; legacy MD5 labeled non-authoritative.

use ed25519_dalek::{Signer, SigningKey};
use lab_crypto::digest::{DigestAlg, DigestLabel, digest_bytes};
use lab_crypto::signature::{TrustState, verify_ed25519_envelope};
use rand_core::OsRng;

#[test]
fn ed25519_envelope_valid_trusted_vs_invalid() {
    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();
    let payload = br#"{"case_uuid":"11111111-1111-4111-8111-111111111111"}"#;
    let payload_digest = digest_bytes(DigestAlg::Sha256, payload);
    let signature = signing_key.sign(payload_digest.hex.as_bytes());

    let trusted = verify_ed25519_envelope(
        payload,
        &payload_digest.hex,
        signature.to_bytes().as_slice(),
        verifying_key.as_bytes(),
        true,
    )
    .expect("verify trusted");
    assert_eq!(trusted, TrustState::ValidTrusted);

    let untrusted = verify_ed25519_envelope(
        payload,
        &payload_digest.hex,
        signature.to_bytes().as_slice(),
        verifying_key.as_bytes(),
        false,
    )
    .expect("verify untrusted");
    assert_eq!(untrusted, TrustState::ValidUntrusted);

    let mut bad_sig = signature.to_bytes();
    bad_sig[0] ^= 0xff;
    let invalid = verify_ed25519_envelope(
        payload,
        &payload_digest.hex,
        &bad_sig,
        verifying_key.as_bytes(),
        true,
    )
    .expect("verify invalid");
    assert_eq!(invalid, TrustState::Invalid);
}

#[test]
fn legacy_md5_labeled_non_authoritative() {
    let labeled = digest_bytes(DigestAlg::Md5Legacy, b"legacy-correlation");
    assert_eq!(labeled.label, DigestLabel::NonAuthoritativeLegacyCorrelation);
    assert_eq!(labeled.algorithm, "md5");
    assert_eq!(labeled.hex.len(), 32);
}
