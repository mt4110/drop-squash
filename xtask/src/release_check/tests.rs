use super::secret_files::{is_secret_file, reject_secret_files, require_local_agent_ignore};
use super::workflow::{
    missing_ci_workflow_gates, missing_desktop_workflow_gates, missing_release_workflow_gates,
    missing_security_workflow_gates,
};

#[test]
fn accepts_release_workflow_with_required_gates() {
    let missing = missing_release_workflow_gates(
        r#"
tags:
  - "v*.*.*"
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
run: cargo run -p xtask -- checksum target/release/bundle/dmg/DropSquash.dmg > SHA256SUMS
uses: actions/upload-artifact@v4
name: dropsquash-unsigned-dmg-checksum
run: cargo run -p xtask -- macos-signing-check
name: Block unsigned Phase 0 release
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
            "cargo run -p xtask -- checksum target/release/bundle/dmg/DropSquash.dmg > SHA256SUMS",
            "actions/upload-artifact@v4",
            "dropsquash-unsigned-dmg-checksum",
            "cargo run -p xtask -- macos-signing-check",
            "Block unsigned Phase 0 release"
        ]
    );
}

#[test]
fn release_workflow_file_has_required_gates() {
    let text = std::fs::read_to_string("../.github/workflows/release.yml").unwrap();

    assert!(missing_release_workflow_gates(&text).is_empty());
}

#[test]
fn accepts_ci_workflow_with_required_gates() {
    let missing = missing_ci_workflow_gates(
        r#"
run: cargo fmt --all -- --check
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
run: pnpm --dir apps/desktop/web lint
run: pnpm --dir apps/desktop/web build
uses: dtolnay/rust-toolchain@1.95.0
run: cargo test -p dropsquash-desktop
"#,
    );

    assert!(missing.is_empty());
}

#[test]
fn reports_missing_desktop_workflow_gates() {
    let missing = missing_desktop_workflow_gates("run: pnpm --dir apps/desktop/web lint");

    assert_eq!(
        missing,
        vec![
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

fn write(root: &std::path::Path, name: &str, text: &str) {
    let path = root.join(name);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, text).unwrap();
}
