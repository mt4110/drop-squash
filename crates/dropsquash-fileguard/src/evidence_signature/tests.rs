use super::{sign_evidence, verify_evidence_signature};

#[test]
fn signs_and_verifies_a_stable_payload() {
    let directory = tempfile::tempdir().unwrap();
    let key = directory.path().join("evidence.pk8");
    let signature = sign_evidence(b"verified output hash", &key).unwrap();

    verify_evidence_signature(b"verified output hash", &signature).unwrap();
    assert!(key.is_file());
}

#[test]
fn rejects_a_modified_payload() {
    let directory = tempfile::tempdir().unwrap();
    let signature = sign_evidence(b"verified output hash", &directory.path().join("key")).unwrap();

    assert!(verify_evidence_signature(b"modified output hash", &signature).is_err());
}
