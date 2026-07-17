use lab_transfer::{export_signed_package, import_verify_package, LocalKeypair, TransferPackage};

fn sample(keys: &LocalKeypair) -> TransferPackage {
    TransferPackage {
        schema_version: "transfer-package-1".into(),
        transfer_uuid: "44444444-4444-4444-8444-444444444444".into(),
        source_case_uuid: "11111111-1111-4111-8111-111111111111".into(),
        created_at_utc: "2026-07-17T00:00:00Z".into(),
        destination: "Partner".into(),
        purpose: "review".into(),
        authority_note: "approved".into(),
        selected_bookmark_digests: vec!["a".repeat(64)],
        result: "export_ready".into(),
        signature: lab_transfer::SignatureBlock {
            algorithm: "Ed25519".into(),
            key_id: keys.key_id.clone(),
            payload_digest: String::new(),
            signature: String::new(),
            trust_state: "NOT_CHECKED".into(),
        },
    }
}

#[test]
fn export_import_ok_and_tamper_rejected() {
    let keys = LocalKeypair::generate("lab-local-1");
    let pkg = export_signed_package(sample(&keys), &keys).unwrap();
    let pubk = keys.public_bytes();
    assert!(import_verify_package(&pkg, &pubk, false).is_ok());

    let mut bad = pkg.clone();
    bad.signature.signature = "00".repeat(64);
    let err = import_verify_package(&bad, &pubk, false).unwrap_err();
    assert!(
        format!("{err:?}").contains("INTEGRITY_FAILED")
            || format!("{err:?}").contains("Invalid")
            || format!("{err:?}").contains("sig")
            || format!("{err:?}").contains("rejected")
            || format!("{err:?}").contains("Integrity")
    );
}
