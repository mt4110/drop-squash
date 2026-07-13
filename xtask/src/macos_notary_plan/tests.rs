use std::io::Write;

use super::{command, Credentials, Request};

#[test]
fn prints_api_key_notarytool_submit_command() {
    let directory = tempfile::tempdir().unwrap();
    let target = write_dmg(directory.path(), b"signed");

    let text = command(&Request {
        target,
        credentials: Credentials::ApiKey,
    })
    .unwrap();

    assert!(text.starts_with("xcrun notarytool submit "));
    assert!(text.contains("--wait"));
    assert!(text.contains("--key $APPLE_API_KEY_PATH"));
    assert!(text.contains("--key-id $APPLE_API_KEY"));
    assert!(text.contains("--issuer $APPLE_API_ISSUER"));
}

#[test]
fn prints_apple_id_notarytool_submit_command() {
    let directory = tempfile::tempdir().unwrap();
    let target = write_dmg(directory.path(), b"signed");

    let text = command(&Request {
        target,
        credentials: Credentials::AppleId,
    })
    .unwrap();

    assert!(text.contains("--apple-id $APPLE_ID"));
    assert!(text.contains("--password $APPLE_PASSWORD"));
    assert!(text.contains("--team-id $APPLE_TEAM_ID"));
}

#[test]
fn plan_does_not_print_secret_values() {
    let directory = tempfile::tempdir().unwrap();
    let target = write_dmg(directory.path(), b"signed");

    let text = command(&Request {
        target,
        credentials: Credentials::AppleId,
    })
    .unwrap();

    assert!(!text.contains("abcd-efgh"));
    assert!(!text.contains("APPLE_API_KEY_P8"));
}

#[test]
fn rejects_noncanonical_target() {
    let directory = tempfile::tempdir().unwrap();
    let target = directory.path().join("Other.dmg");
    std::fs::File::create(&target)
        .unwrap()
        .write_all(&dmg_bytes(b"signed"))
        .unwrap();

    let error = command(&Request {
        target,
        credentials: Credentials::ApiKey,
    })
    .unwrap_err();

    assert!(error.contains("DropSquash.dmg"));
}

#[test]
fn rejects_target_with_nix_store_reference() {
    let directory = tempfile::tempdir().unwrap();
    let target = write_dmg(directory.path(), b"/nix/store/abc");

    let error = command(&Request {
        target,
        credentials: Credentials::ApiKey,
    })
    .unwrap_err();

    assert!(error.contains("/nix/store"));
}

fn write_dmg(directory: &std::path::Path, prefix: &[u8]) -> std::path::PathBuf {
    std::fs::create_dir_all(directory).unwrap();
    let path = directory.join("DropSquash.dmg");
    std::fs::File::create(&path)
        .unwrap()
        .write_all(&dmg_bytes(prefix))
        .unwrap();
    path
}

fn dmg_bytes(prefix: &[u8]) -> Vec<u8> {
    let mut bytes = prefix.to_vec();
    let mut trailer = vec![0; 512];
    trailer[..4].copy_from_slice(b"koly");
    bytes.extend(trailer);
    bytes
}
