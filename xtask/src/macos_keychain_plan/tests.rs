use super::{commands, Request};

#[test]
fn prints_keychain_import_plan() {
    let lines = commands(&Request {
        work_dir: "/tmp/dropsquash-signing".into(),
    })
    .unwrap();

    assert_eq!(lines.len(), 8);
    assert!(lines[0].contains("mkdir -p /tmp/dropsquash-signing"));
    assert!(lines[1].contains("$APPLE_CERTIFICATE"));
    assert!(lines[1].contains("base64 --decode"));
    assert!(lines[2].contains("security create-keychain"));
    assert!(lines[5].contains("security list-keychains"));
    assert!(lines[6].contains("security import"));
    assert!(lines[7].contains("security set-key-partition-list"));
}

#[test]
fn uses_environment_references_without_secret_values() {
    let text = commands(&Request {
        work_dir: "/tmp/dropsquash-signing".into(),
    })
    .unwrap()
    .join("\n");

    assert!(text.contains("$APPLE_KEYCHAIN_PASSWORD"));
    assert!(text.contains("$APPLE_CERTIFICATE_PASSWORD"));
    assert!(!text.contains("cert-passphrase"));
    assert!(!text.contains("base64-secret"));
}

#[test]
fn quotes_spaces_in_work_dir() {
    let text = commands(&Request {
        work_dir: "/tmp/drop squash/signing".into(),
    })
    .unwrap()
    .join("\n");

    assert!(text.contains("'/tmp/drop squash/signing'"));
    assert!(text.contains("'/tmp/drop squash/signing/DropSquash-signing.p12'"));
}

#[test]
fn rejects_relative_work_dir() {
    let error = commands(&Request {
        work_dir: "tmp/signing".into(),
    })
    .unwrap_err();

    assert!(error.contains("absolute"));
}

#[test]
fn rejects_nix_store_work_dir() {
    let error = commands(&Request {
        work_dir: "/nix/store/signing".into(),
    })
    .unwrap_err();

    assert!(error.contains("/nix/store"));
}
