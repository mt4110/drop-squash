use super::{commands, Request};

#[test]
fn prints_keychain_cleanup_plan() {
    let lines = commands(&Request {
        work_dir: "/tmp/dropsquash-signing".into(),
    })
    .unwrap();

    assert_eq!(lines.len(), 3);
    assert_eq!(
        lines[0],
        "security delete-keychain /tmp/dropsquash-signing/DropSquash-signing.keychain-db"
    );
    assert_eq!(
        lines[1],
        "rm -f /tmp/dropsquash-signing/DropSquash-signing.p12"
    );
    assert_eq!(
        lines[2],
        "rm -f /tmp/dropsquash-signing/DropSquash-signing.keychain-db"
    );
}

#[test]
fn does_not_print_secret_references() {
    let text = commands(&Request {
        work_dir: "/tmp/dropsquash-signing".into(),
    })
    .unwrap()
    .join("\n");

    assert!(!text.contains("APPLE_CERTIFICATE"));
    assert!(!text.contains("APPLE_KEYCHAIN_PASSWORD"));
}

#[test]
fn quotes_spaces_in_work_dir() {
    let text = commands(&Request {
        work_dir: "/tmp/drop squash/signing".into(),
    })
    .unwrap()
    .join("\n");

    assert!(text.contains("'/tmp/drop squash/signing/DropSquash-signing.p12'"));
    assert!(text.contains("'/tmp/drop squash/signing/DropSquash-signing.keychain-db'"));
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
