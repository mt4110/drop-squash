use std::collections::BTreeMap;
use std::io::Write;

use super::check;

#[test]
fn accepts_identity_and_apple_id_notarization() {
    let env = env([
        (
            "APPLE_SIGNING_IDENTITY",
            "Developer ID Application: Example",
        ),
        ("APPLE_ID", "dev@example.com"),
        ("APPLE_PASSWORD", "@env:APPLE_APP_PASSWORD"),
        ("APPLE_TEAM_ID", "ABCDE12345"),
    ]);

    assert!(check(&env).is_ok());
}

#[test]
fn ignores_empty_github_actions_flag() {
    let env = env([
        ("GITHUB_ACTIONS", " "),
        (
            "APPLE_SIGNING_IDENTITY",
            "Developer ID Application: Example",
        ),
        ("APPLE_ID", "dev@example.com"),
        ("APPLE_PASSWORD", "@env:APPLE_APP_PASSWORD"),
        ("APPLE_TEAM_ID", "ABCDE12345"),
    ]);

    assert!(check(&env).is_ok());
}

#[test]
fn accepts_certificate_and_api_key_notarization() {
    let directory = tempfile::tempdir().unwrap();
    let key_path = write_api_key(directory.path(), TEST_API_KEY);
    let env = env([
        ("APPLE_CERTIFICATE", TEST_CERTIFICATE),
        ("APPLE_CERTIFICATE_PASSWORD", "cert-passphrase-123"),
        ("APPLE_API_KEY", "ABCDEF1234"),
        ("APPLE_API_ISSUER", "12345678-1234-1234-1234-123456789abc"),
        ("APPLE_API_KEY_PATH", key_path.to_str().unwrap()),
    ]);

    assert!(check(&env).is_ok());
}

#[test]
fn accepts_certificate_signing_in_ci() {
    let env = env([
        ("GITHUB_ACTIONS", "true"),
        ("APPLE_CERTIFICATE", TEST_CERTIFICATE),
        ("APPLE_CERTIFICATE_PASSWORD", "cert-passphrase-123"),
        ("APPLE_ID", "dev@example.com"),
        ("APPLE_PASSWORD", "abcd-efgh-ijkl-mnop"),
        ("APPLE_TEAM_ID", "ABCDE12345"),
    ]);

    assert!(check(&env).is_ok());
}

#[test]
fn rejects_identity_only_signing_in_ci() {
    let env = env([
        ("GITHUB_ACTIONS", "true"),
        (
            "APPLE_SIGNING_IDENTITY",
            "Developer ID Application: Example",
        ),
        ("APPLE_ID", "dev@example.com"),
        ("APPLE_PASSWORD", "abcd-efgh-ijkl-mnop"),
        ("APPLE_TEAM_ID", "ABCDE12345"),
    ]);
    let error = check(&env).unwrap_err();

    assert!(error.contains("CI macOS signing requires"));
}

#[test]
fn rejects_placeholder_certificate() {
    let env = env([
        ("APPLE_CERTIFICATE", "base64"),
        ("APPLE_CERTIFICATE_PASSWORD", "cert-passphrase-123"),
        ("APPLE_ID", "dev@example.com"),
        ("APPLE_PASSWORD", "abcd-efgh-ijkl-mnop"),
        ("APPLE_TEAM_ID", "ABCDE12345"),
    ]);
    let error = check(&env).unwrap_err();

    assert!(error.contains("APPLE_CERTIFICATE"));
}

#[test]
fn rejects_placeholder_secret_values() {
    let env = env([
        ("APPLE_CERTIFICATE", TEST_CERTIFICATE),
        ("APPLE_CERTIFICATE_PASSWORD", "password"),
        ("APPLE_ID", "dev@example.com"),
        ("APPLE_PASSWORD", "abcd-efgh-ijkl-mnop"),
        ("APPLE_TEAM_ID", "ABCDE12345"),
    ]);
    let error = check(&env).unwrap_err();

    assert!(error.contains("APPLE_CERTIFICATE_PASSWORD"));
}

#[test]
fn rejects_placeholder_apple_password() {
    let env = env([
        ("APPLE_SIGNING_IDENTITY", "Developer ID Application"),
        ("APPLE_ID", "dev@example.com"),
        ("APPLE_PASSWORD", "app-password"),
        ("APPLE_TEAM_ID", "ABCDE12345"),
    ]);
    let error = check(&env).unwrap_err();

    assert!(error.contains("APPLE_PASSWORD"));
}

#[test]
fn rejects_non_developer_id_identity() {
    let env = env([
        ("APPLE_SIGNING_IDENTITY", "Mac Developer: Example"),
        ("APPLE_ID", "dev@example.com"),
        ("APPLE_PASSWORD", "abcd-efgh-ijkl-mnop"),
        ("APPLE_TEAM_ID", "ABCDE12345"),
    ]);
    let error = check(&env).unwrap_err();

    assert!(error.contains("Developer ID Application"));
}

#[test]
fn rejects_missing_notarization_group() {
    let env = env([("APPLE_SIGNING_IDENTITY", "Developer ID Application")]);
    let error = check(&env).unwrap_err();

    assert!(error.contains("notarization requires"));
}

#[test]
fn rejects_malformed_team_id() {
    let env = env([
        ("APPLE_SIGNING_IDENTITY", "Developer ID Application"),
        ("APPLE_ID", "dev@example.com"),
        ("APPLE_PASSWORD", "abcd-efgh-ijkl-mnop"),
        ("APPLE_TEAM_ID", "not-a-team"),
    ]);
    let error = check(&env).unwrap_err();

    assert!(error.contains("APPLE_TEAM_ID"));
}

#[test]
fn rejects_missing_signing_source() {
    let env = env([
        ("APPLE_ID", "dev@example.com"),
        ("APPLE_PASSWORD", "@env:APPLE_APP_PASSWORD"),
        ("APPLE_TEAM_ID", "ABCDE12345"),
    ]);
    let error = check(&env).unwrap_err();

    assert!(error.contains("macOS signing requires"));
}

#[test]
fn rejects_missing_api_key_file() {
    let env = env([
        ("APPLE_SIGNING_IDENTITY", "Developer ID Application"),
        ("APPLE_API_KEY", "ABCDEF1234"),
        ("APPLE_API_ISSUER", "12345678-1234-1234-1234-123456789abc"),
        ("APPLE_API_KEY_PATH", "/missing/AuthKey_TEST.p8"),
    ]);
    let error = check(&env).unwrap_err();

    assert!(error.contains("APPLE_API_KEY_PATH"));
}

#[test]
fn rejects_api_key_path_without_p8_extension() {
    let directory = tempfile::tempdir().unwrap();
    let key_path = directory.path().join("AuthKey_TEST.txt");
    std::fs::File::create(&key_path).unwrap();
    let env = env([
        ("APPLE_SIGNING_IDENTITY", "Developer ID Application"),
        ("APPLE_API_KEY", "ABCDEF1234"),
        ("APPLE_API_ISSUER", "12345678-1234-1234-1234-123456789abc"),
        ("APPLE_API_KEY_PATH", key_path.to_str().unwrap()),
    ]);
    let error = check(&env).unwrap_err();

    assert!(error.contains("APPLE_API_KEY_PATH"));
}

#[test]
fn rejects_empty_api_key_file() {
    let directory = tempfile::tempdir().unwrap();
    let key_path = write_api_key(directory.path(), "");
    let env = env([
        ("APPLE_SIGNING_IDENTITY", "Developer ID Application"),
        ("APPLE_API_KEY", "ABCDEF1234"),
        ("APPLE_API_ISSUER", "12345678-1234-1234-1234-123456789abc"),
        ("APPLE_API_KEY_PATH", key_path.to_str().unwrap()),
    ]);
    let error = check(&env).unwrap_err();

    assert!(error.contains("private key data"));
}

#[test]
fn rejects_placeholder_api_key_file() {
    let directory = tempfile::tempdir().unwrap();
    let key_path = write_api_key(directory.path(), "private-key-placeholder");
    let env = env([
        ("APPLE_SIGNING_IDENTITY", "Developer ID Application"),
        ("APPLE_API_KEY", "ABCDEF1234"),
        ("APPLE_API_ISSUER", "12345678-1234-1234-1234-123456789abc"),
        ("APPLE_API_KEY_PATH", key_path.to_str().unwrap()),
    ]);
    let error = check(&env).unwrap_err();

    assert!(error.contains("private key data"));
}

#[test]
fn rejects_placeholder_api_key_id() {
    let directory = tempfile::tempdir().unwrap();
    let key_path = write_api_key(directory.path(), TEST_API_KEY);
    let env = env([
        ("APPLE_SIGNING_IDENTITY", "Developer ID Application"),
        ("APPLE_API_KEY", "TEST"),
        ("APPLE_API_ISSUER", "12345678-1234-1234-1234-123456789abc"),
        ("APPLE_API_KEY_PATH", key_path.to_str().unwrap()),
    ]);
    let error = check(&env).unwrap_err();

    assert!(error.contains("APPLE_API_KEY"));
}

#[test]
fn rejects_repeated_api_key_id() {
    let directory = tempfile::tempdir().unwrap();
    let key_path = write_api_key(directory.path(), TEST_API_KEY);
    let env = env([
        ("APPLE_SIGNING_IDENTITY", "Developer ID Application"),
        ("APPLE_API_KEY", "0000000000"),
        ("APPLE_API_ISSUER", "12345678-1234-1234-1234-123456789abc"),
        ("APPLE_API_KEY_PATH", key_path.to_str().unwrap()),
    ]);
    let error = check(&env).unwrap_err();

    assert!(error.contains("APPLE_API_KEY"));
}

#[test]
fn rejects_placeholder_api_issuer() {
    let directory = tempfile::tempdir().unwrap();
    let key_path = write_api_key(directory.path(), TEST_API_KEY);
    let env = env([
        ("APPLE_SIGNING_IDENTITY", "Developer ID Application"),
        ("APPLE_API_KEY", "ABCDEF1234"),
        ("APPLE_API_ISSUER", "issuer"),
        ("APPLE_API_KEY_PATH", key_path.to_str().unwrap()),
    ]);
    let error = check(&env).unwrap_err();

    assert!(error.contains("APPLE_API_ISSUER"));
}

#[test]
fn rejects_zero_api_issuer() {
    let directory = tempfile::tempdir().unwrap();
    let key_path = write_api_key(directory.path(), TEST_API_KEY);
    let env = env([
        ("APPLE_SIGNING_IDENTITY", "Developer ID Application"),
        ("APPLE_API_KEY", "ABCDEF1234"),
        ("APPLE_API_ISSUER", "00000000-0000-0000-0000-000000000000"),
        ("APPLE_API_KEY_PATH", key_path.to_str().unwrap()),
    ]);
    let error = check(&env).unwrap_err();

    assert!(error.contains("APPLE_API_ISSUER"));
}

fn env<const N: usize>(pairs: [(&str, &str); N]) -> BTreeMap<String, String> {
    pairs
        .into_iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect()
}

fn write_api_key(directory: &std::path::Path, contents: &str) -> std::path::PathBuf {
    let key_path = directory.join("AuthKey_TEST.p8");
    std::fs::File::create(&key_path)
        .unwrap()
        .write_all(contents.as_bytes())
        .unwrap();
    key_path
}

const TEST_CERTIFICATE: &str = "QUJDREVGR0hJSktMTU5PUFFSU1RVVldYWVo=";
const TEST_API_KEY: &str = "-----BEGIN PRIVATE KEY-----\nabc123\n-----END PRIVATE KEY-----\n";
