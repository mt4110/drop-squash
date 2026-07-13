use super::dev_environment::reject_parallel_version_manager;
use super::secret_files::{is_secret_file, reject_secret_files, require_local_agent_ignore};
use super::workflow::{
    forbidden_release_workflow_values, missing_ci_workflow_gates, missing_desktop_workflow_gates,
    missing_release_workflow_gates, missing_security_workflow_gates,
};

#[test]
fn accepts_release_workflow_with_required_gates() {
    let missing = missing_release_workflow_gates(
        r#"
tags:
  - "v*.*.*"
permissions:
  contents: read
environment: production
with:
  components: rustfmt, clippy
run: cargo fmt --all -- --check
run: cargo clippy --workspace --all-targets -- -D warnings
run: cargo test --workspace
run: cargo run -p xtask -- file-size-check
run: cargo run -p xtask -- website-check
run: cargo run -p xtask -- manual-qa-check
run: cargo run -p xtask -- release-check
uses: pnpm/action-setup@v4
  version: 10.34.0
uses: actions/setup-node@v4
  node-version: 24.16.0
run: pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci
run: cargo run -p xtask -- normalize-dmg target/release/bundle/dmg
run: cargo run -p xtask -- artifact-check target/release/bundle/dmg/DropSquash.dmg
name: dropsquash-unsigned-dmg
run: cargo run -p xtask -- checksum target/release/bundle/dmg/DropSquash.dmg --output SHA256SUMS
uses: actions/upload-artifact@v4
name: dropsquash-unsigned-dmg-checksum
name: Prepare App Store Connect key file
APPLE_API_KEY_P8: ${{ secrets.APPLE_API_KEY_P8 }}
install -m 600 /dev/null "$key_path"
APPLE_API_KEY_PATH=$key_path
APPLE_CERTIFICATE: ${{ secrets.APPLE_CERTIFICATE }}
APPLE_CERTIFICATE_PASSWORD: ${{ secrets.APPLE_CERTIFICATE_PASSWORD }}
APPLE_KEYCHAIN_PASSWORD: ${{ secrets.APPLE_KEYCHAIN_PASSWORD }}
APPLE_CODESIGN_IDENTITY: ${{ vars.APPLE_CODESIGN_IDENTITY }}
run: cargo run -p xtask -- macos-signing-check
run: cargo run -p xtask -- macos-signing-plan target/release/bundle/dmg/DropSquash.dmg "$RUNNER_TEMP/dropsquash-signed"
run: cargo run -p xtask -- macos-keychain-plan "$RUNNER_TEMP/dropsquash-signing"
name: Import macOS signing certificate
security list-keychains -d user -s "$keychain"
run: cargo run -p xtask -- signed-dmg-prepare target/release/bundle/dmg/DropSquash.dmg "$RUNNER_TEMP/dropsquash-signed"
run: cargo run -p xtask -- signed-dmg-copy target/release/bundle/dmg/DropSquash.dmg "$RUNNER_TEMP/dropsquash-signed"
run: cargo run -p xtask -- macos-codesign-plan "$RUNNER_TEMP/dropsquash-signed/DropSquash.dmg" "Developer ID Application: ..."
codesign --force --options runtime --timestamp --sign "$APPLE_CODESIGN_IDENTITY"
run: cargo run -p xtask -- macos-codesign-verify-plan "$RUNNER_TEMP/dropsquash-signed/DropSquash.dmg"
codesign --verify --deep --strict --verbose=4 "$RUNNER_TEMP/dropsquash-signed/DropSquash.dmg"
codesign -dv --verbose=4 "$RUNNER_TEMP/dropsquash-signed/DropSquash.dmg"
run: cargo run -p xtask -- macos-notary-plan "$RUNNER_TEMP/dropsquash-signed/DropSquash.dmg" --api-key
name: Notarize macOS DMG
xcrun notarytool submit "$RUNNER_TEMP/dropsquash-signed/DropSquash.dmg" --wait --key "$APPLE_API_KEY_PATH" --key-id "$APPLE_API_KEY" --issuer "$APPLE_API_ISSUER"
run: cargo run -p xtask -- macos-stapler-plan "$RUNNER_TEMP/dropsquash-signed/DropSquash.dmg"
name: Staple macOS DMG
xcrun stapler staple "$RUNNER_TEMP/dropsquash-signed/DropSquash.dmg"
xcrun stapler validate "$RUNNER_TEMP/dropsquash-signed/DropSquash.dmg"
run: cargo run -p xtask -- macos-spctl-plan "$RUNNER_TEMP/dropsquash-signed/DropSquash.dmg"
name: Assess macOS Gatekeeper
spctl --assess --type open --verbose=4 "$RUNNER_TEMP/dropsquash-signed/DropSquash.dmg"
name: Check signed DMG artifact
run: cargo run -p xtask -- signed-dmg-check "$RUNNER_TEMP/dropsquash-signed/DropSquash.dmg" target/release/bundle/dmg/DropSquash.dmg
name: Write signed DMG checksum
run: cargo run -p xtask -- checksum "$RUNNER_TEMP/dropsquash-signed/DropSquash.dmg" --output "$RUNNER_TEMP/dropsquash-signed/SHA256SUMS"
run: cargo run -p xtask -- macos-keychain-cleanup-plan "$RUNNER_TEMP/dropsquash-signing"
name: Cleanup macOS signing keychain
always() && matrix.os == 'macos-latest'
name: Block unsigned Phase 0 release
echo "Signed release packaging is not implemented."
exit 1
"#,
    );

    assert!(missing.is_empty());
}

#[test]
fn reports_missing_release_workflow_gates() {
    let missing = missing_release_workflow_gates("run: cargo run -p xtask -- release-check");

    assert_eq!(
        missing,
        vec![
            "tags:",
            "\"v*.*.*\"",
            "permissions:",
            "contents: read",
            "environment: production",
            "components: rustfmt, clippy",
            "cargo fmt --all -- --check",
            "cargo clippy --workspace --all-targets -- -D warnings",
            "cargo test --workspace",
            "cargo run -p xtask -- file-size-check",
            "cargo run -p xtask -- website-check",
            "cargo run -p xtask -- manual-qa-check",
            "pnpm/action-setup@v4",
            "version: 10.34.0",
            "actions/setup-node@v4",
            "node-version: 24.16.0",
            "pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci",
            "cargo run -p xtask -- normalize-dmg target/release/bundle/dmg",
            "cargo run -p xtask -- artifact-check target/release/bundle/dmg/DropSquash.dmg",
            "dropsquash-unsigned-dmg",
            "cargo run -p xtask -- checksum target/release/bundle/dmg/DropSquash.dmg --output SHA256SUMS",
            "actions/upload-artifact@v4",
            "dropsquash-unsigned-dmg-checksum",
            "Prepare App Store Connect key file",
            "APPLE_API_KEY_P8: ${{ secrets.APPLE_API_KEY_P8 }}",
            "install -m 600 /dev/null \"$key_path\"",
            "APPLE_API_KEY_PATH=$key_path",
            "APPLE_CERTIFICATE: ${{ secrets.APPLE_CERTIFICATE }}",
            "APPLE_CERTIFICATE_PASSWORD: ${{ secrets.APPLE_CERTIFICATE_PASSWORD }}",
            "APPLE_KEYCHAIN_PASSWORD: ${{ secrets.APPLE_KEYCHAIN_PASSWORD }}",
            "APPLE_CODESIGN_IDENTITY: ${{ vars.APPLE_CODESIGN_IDENTITY }}",
            "cargo run -p xtask -- macos-signing-check",
            "cargo run -p xtask -- macos-signing-plan target/release/bundle/dmg/DropSquash.dmg \"$RUNNER_TEMP/dropsquash-signed\"",
            "cargo run -p xtask -- macos-keychain-plan \"$RUNNER_TEMP/dropsquash-signing\"",
            "Import macOS signing certificate",
            "security list-keychains -d user -s \"$keychain\"",
            "cargo run -p xtask -- signed-dmg-prepare target/release/bundle/dmg/DropSquash.dmg \"$RUNNER_TEMP/dropsquash-signed\"",
            "cargo run -p xtask -- signed-dmg-copy target/release/bundle/dmg/DropSquash.dmg \"$RUNNER_TEMP/dropsquash-signed\"",
            "cargo run -p xtask -- macos-codesign-plan \"$RUNNER_TEMP/dropsquash-signed/DropSquash.dmg\" \"Developer ID Application: ...\"",
            "codesign --force --options runtime --timestamp --sign \"$APPLE_CODESIGN_IDENTITY\"",
            "cargo run -p xtask -- macos-codesign-verify-plan \"$RUNNER_TEMP/dropsquash-signed/DropSquash.dmg\"",
            "codesign --verify --deep --strict --verbose=4 \"$RUNNER_TEMP/dropsquash-signed/DropSquash.dmg\"",
            "codesign -dv --verbose=4 \"$RUNNER_TEMP/dropsquash-signed/DropSquash.dmg\"",
            "cargo run -p xtask -- macos-notary-plan \"$RUNNER_TEMP/dropsquash-signed/DropSquash.dmg\" --api-key",
            "Notarize macOS DMG",
            "xcrun notarytool submit \"$RUNNER_TEMP/dropsquash-signed/DropSquash.dmg\" --wait --key \"$APPLE_API_KEY_PATH\" --key-id \"$APPLE_API_KEY\" --issuer \"$APPLE_API_ISSUER\"",
            "cargo run -p xtask -- macos-stapler-plan \"$RUNNER_TEMP/dropsquash-signed/DropSquash.dmg\"",
            "Staple macOS DMG",
            "xcrun stapler staple \"$RUNNER_TEMP/dropsquash-signed/DropSquash.dmg\"",
            "xcrun stapler validate \"$RUNNER_TEMP/dropsquash-signed/DropSquash.dmg\"",
            "cargo run -p xtask -- macos-spctl-plan \"$RUNNER_TEMP/dropsquash-signed/DropSquash.dmg\"",
            "Assess macOS Gatekeeper",
            "spctl --assess --type open --verbose=4 \"$RUNNER_TEMP/dropsquash-signed/DropSquash.dmg\"",
            "Check signed DMG artifact",
            "cargo run -p xtask -- signed-dmg-check \"$RUNNER_TEMP/dropsquash-signed/DropSquash.dmg\" target/release/bundle/dmg/DropSquash.dmg",
            "Write signed DMG checksum",
            "cargo run -p xtask -- checksum \"$RUNNER_TEMP/dropsquash-signed/DropSquash.dmg\" --output \"$RUNNER_TEMP/dropsquash-signed/SHA256SUMS\"",
            "cargo run -p xtask -- macos-keychain-cleanup-plan \"$RUNNER_TEMP/dropsquash-signing\"",
            "Cleanup macOS signing keychain",
            "always() && matrix.os == 'macos-latest'",
            "Block unsigned Phase 0 release",
            "Signed release packaging is not implemented.",
            "exit 1"
        ]
    );
}

#[test]
fn release_workflow_file_has_required_gates() {
    let text = std::fs::read_to_string("../.github/workflows/release.yml").unwrap();

    assert!(missing_release_workflow_gates(&text).is_empty());
}

#[test]
fn rejects_release_workflow_local_signing_identity_secret() {
    let found = forbidden_release_workflow_values(
        "APPLE_SIGNING_IDENTITY: ${{ secrets.APPLE_SIGNING_IDENTITY }}",
    );

    assert_eq!(
        found,
        vec!["APPLE_SIGNING_IDENTITY: ${{ secrets.APPLE_SIGNING_IDENTITY }}"]
    );
}

#[test]
fn rejects_release_workflow_api_key_path_secret() {
    let found =
        forbidden_release_workflow_values("APPLE_API_KEY_PATH: ${{ secrets.APPLE_API_KEY_PATH }}");

    assert_eq!(
        found,
        vec!["APPLE_API_KEY_PATH: ${{ secrets.APPLE_API_KEY_PATH }}"]
    );
}

#[test]
fn release_workflow_file_uses_ci_certificate_signing() {
    let text = std::fs::read_to_string("../.github/workflows/release.yml").unwrap();

    assert!(forbidden_release_workflow_values(&text).is_empty());
}

#[test]
fn accepts_ci_workflow_with_required_gates() {
    let missing = missing_ci_workflow_gates(
        r#"
run: cargo fmt --all -- --check
permissions:
  contents: read
run: cargo run -p xtask -- file-size-check
run: cargo run -p xtask -- website-check
run: cargo run -p xtask -- release-check
run: cargo clippy --workspace --all-targets -- -D warnings
run: cargo test --workspace
uses: cachix/install-nix-action@v31
run: nix flake check --no-build --all-systems
"#,
    );

    assert!(missing.is_empty());
}

#[test]
fn ci_workflow_file_has_required_gates() {
    let text = std::fs::read_to_string("../.github/workflows/ci.yml").unwrap();

    assert!(missing_ci_workflow_gates(&text).is_empty());
}

#[test]
fn reports_missing_ci_workflow_gates() {
    let missing = missing_ci_workflow_gates("run: cargo fmt --all -- --check");

    assert_eq!(
        missing,
        vec![
            "permissions:",
            "contents: read",
            "cargo run -p xtask -- file-size-check",
            "cargo run -p xtask -- website-check",
            "cargo run -p xtask -- release-check",
            "cargo clippy --workspace --all-targets -- -D warnings",
            "cargo test --workspace",
            "cachix/install-nix-action@v31",
            "nix flake check --no-build --all-systems"
        ]
    );
}

#[test]
fn accepts_desktop_workflow_with_required_gates() {
    let missing = missing_desktop_workflow_gates(
        r#"
run: pnpm --dir apps/desktop/web install --frozen-lockfile
permissions:
  contents: read
run: pnpm --dir apps/desktop/web test
run: pnpm --dir apps/desktop/web build
uses: dtolnay/rust-toolchain@1.95.0
run: cargo test -p dropsquash-desktop
"#,
    );

    assert!(missing.is_empty());
}

#[test]
fn reports_missing_desktop_workflow_gates() {
    let missing = missing_desktop_workflow_gates("run: pnpm --dir apps/desktop/web test");

    assert_eq!(
        missing,
        vec![
            "permissions:",
            "contents: read",
            "pnpm --dir apps/desktop/web install --frozen-lockfile",
            "pnpm --dir apps/desktop/web build",
            "dtolnay/rust-toolchain@1.95.0",
            "cargo test -p dropsquash-desktop"
        ]
    );
}

#[test]
fn desktop_workflow_file_has_required_gates() {
    let text = std::fs::read_to_string("../.github/workflows/desktop-ci.yml").unwrap();

    assert!(missing_desktop_workflow_gates(&text).is_empty());
}

#[test]
fn accepts_security_workflow_with_required_gates() {
    let missing = missing_security_workflow_gates(
        r#"
run: cargo audit
permissions:
  contents: read
run: cargo deny check
run: cargo run -p xtask -- media-policy-check
run: cargo run -p xtask -- privacy-policy-check
"#,
    );

    assert!(missing.is_empty());
}

#[test]
fn security_workflow_file_has_required_gates() {
    let text = std::fs::read_to_string("../.github/workflows/security.yml").unwrap();

    assert!(missing_security_workflow_gates(&text).is_empty());
}

#[test]
fn reports_missing_security_workflow_gates() {
    let missing = missing_security_workflow_gates("run: cargo audit");

    assert_eq!(
        missing,
        vec![
            "permissions:",
            "contents: read",
            "cargo deny check",
            "cargo run -p xtask -- media-policy-check",
            "cargo run -p xtask -- privacy-policy-check"
        ]
    );
}

#[test]
fn accepts_public_release_files() {
    assert!(!is_secret_file("release.yml", Some("yml")));
    assert!(!is_secret_file("pricing.html", Some("html")));
    assert!(!is_secret_file("README.md", Some("md")));
}

#[test]
fn rejects_environment_files() {
    assert!(is_secret_file(".env", None));
    assert!(is_secret_file(".envrc", Some("envrc")));
    assert!(is_secret_file(".env.local", Some("local")));
    assert!(is_secret_file(".env.production", Some("production")));
}

#[test]
fn rejects_signing_secret_extensions() {
    assert!(is_secret_file("Distribution.p12", Some("p12")));
    assert!(is_secret_file("AuthKey_TEST.p8", Some("p8")));
    assert!(is_secret_file("DeveloperID.cer", Some("cer")));
    assert!(is_secret_file("DeveloperID.cert", Some("cert")));
    assert!(is_secret_file("DeveloperID.crt", Some("crt")));
    assert!(is_secret_file(
        "profile.mobileprovision",
        Some("mobileprovision")
    ));
    assert!(is_secret_file(
        "profile.provisionprofile",
        Some("provisionprofile")
    ));
}

#[test]
fn scans_repository_tree_for_secret_like_files() {
    let directory = tempfile::tempdir().unwrap();
    write(directory.path(), "docs/release.md", "safe");
    write(directory.path(), "secrets/AuthKey_TEST.p8", "private");

    let error = reject_secret_files(directory.path()).unwrap_err();

    assert!(error.contains("AuthKey_TEST.p8"));
}

#[test]
fn rejects_committed_local_media_and_benchmark_evidence() {
    let directory = tempfile::tempdir().unwrap();
    write(
        directory.path(),
        "bench/results.csv",
        "private sample results",
    );

    let error = reject_secret_files(directory.path()).unwrap_err();

    assert!(error.contains("local evidence file"));
    assert!(error.contains("results.csv"));
}

#[test]
fn rejects_committed_release_artifacts_and_private_recordings() {
    let directory = tempfile::tempdir().unwrap();
    write(directory.path(), "release/DropSquash.dmg", "artifact");

    let error = reject_secret_files(directory.path()).unwrap_err();

    assert!(error.contains("DropSquash.dmg"));
}

#[test]
fn rejects_committed_release_archive_outputs() {
    let directory = tempfile::tempdir().unwrap();
    write(directory.path(), "release/DropSquash.zip", "artifact");

    let error = reject_secret_files(directory.path()).unwrap_err();

    assert!(error.contains("local evidence file"));
    assert!(error.contains("DropSquash.zip"));
}

#[test]
fn rejects_committed_checksum_output() {
    let directory = tempfile::tempdir().unwrap();
    write(directory.path(), "SHA256SUMS", "checksum");

    let error = reject_secret_files(directory.path()).unwrap_err();

    assert!(error.contains("local evidence file"));
    assert!(error.contains("SHA256SUMS"));
}

#[test]
fn ignores_local_agent_state_when_scanning_for_secrets() {
    let directory = tempfile::tempdir().unwrap();
    write(directory.path(), "docs/release.md", "safe");
    write(directory.path(), ".codex/.env", "local-only");

    reject_secret_files(directory.path()).unwrap();
}

#[test]
fn ignores_local_nix_outputs_when_scanning_for_secrets() {
    let directory = tempfile::tempdir().unwrap();
    write(directory.path(), "docs/release.md", "safe");
    write(directory.path(), ".direnv/AuthKey_TEST.p8", "local-only");
    write(directory.path(), "result/AuthKey_TEST.p8", "local-only");
    write(
        directory.path(),
        "result-build/AuthKey_TEST.p8",
        "local-only",
    );

    reject_secret_files(directory.path()).unwrap();
}

#[test]
fn requires_local_agent_state_to_be_ignored() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join(".gitignore");
    std::fs::write(&path, "/target/\n/.codex/\n/.direnv/\n/result\n/result-*\n").unwrap();

    require_local_agent_ignore(&path).unwrap();
}

#[test]
fn reports_missing_local_agent_ignore_rule() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join(".gitignore");
    std::fs::write(&path, "/target/\n").unwrap();

    let error = require_local_agent_ignore(&path).unwrap_err();

    assert!(error.contains("/.codex/"));
}

#[test]
fn reports_missing_nix_output_ignore_rule() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join(".gitignore");
    std::fs::write(&path, "/target/\n/.codex/\n/.direnv/\n/result\n").unwrap();

    let error = require_local_agent_ignore(&path).unwrap_err();

    assert!(error.contains("/result-*"));
}

#[test]
fn rejects_parallel_version_manager_config() {
    let directory = tempfile::tempdir().unwrap();
    write(directory.path(), "docs/release.md", "safe");
    write(
        directory.path(),
        ".mise.toml",
        "[tools]\nrust = \"1.95.0\"\n",
    );

    let error = reject_parallel_version_manager(directory.path()).unwrap_err();

    assert!(error.contains("parallel version manager"));
    assert!(error.contains(".mise.toml"));
}

#[test]
fn ignores_private_parallel_version_manager_notes() {
    let directory = tempfile::tempdir().unwrap();
    write(directory.path(), "docs/release.md", "safe");
    write(directory.path(), ".private_docs/.mise.toml", "local note");
    write(directory.path(), ".codex/.mise.toml", "local note");

    reject_parallel_version_manager(directory.path()).unwrap();
}

fn write(root: &std::path::Path, name: &str, text: &str) {
    let path = root.join(name);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, text).unwrap();
}
