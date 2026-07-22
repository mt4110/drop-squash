use super::dev_environment::{
    reject_parallel_version_manager, require_nix_systems, require_web_toolchain_contract,
};
use super::secret_files::{is_secret_file, reject_secret_files, require_local_agent_ignore};
use super::workflow::{
    forbidden_release_workflow_values, missing_ci_workflow_gates, missing_desktop_workflow_gates,
    missing_release_workflow_gates, missing_security_workflow_gates,
};

#[test]
fn accepts_release_workflow_with_required_gates() {
    let text = std::fs::read_to_string("../.github/workflows/release.yml").unwrap();

    assert!(missing_release_workflow_gates(&text).is_empty());
}

#[test]
fn reports_missing_release_workflow_gates() {
    let missing = missing_release_workflow_gates("run: cargo run -p xtask -- release-check");

    assert!(missing.contains(&"tags:"));
    assert!(missing.contains(&"Build signed and notarized macOS DMG"));
    assert!(missing.contains(&"Verify signed app inside notarized DMG"));
    assert!(missing.contains(&"Block unsigned Phase 0 release"));
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
name: Install Linux desktop dependencies
libwebkit2gtk-4.1-dev
run: cargo run -p xtask -- file-size-check
run: cargo run -p xtask -- website-check
run: cargo run -p xtask -- release-check
name: clippy macOS
run: cargo clippy --workspace --exclude dropsquash-desktop --all-targets -- -D warnings
name: test macOS
run: cargo test --workspace --exclude dropsquash-desktop
name: clippy portable crates
name: test portable crates
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
            "Install Linux desktop dependencies",
            "libwebkit2gtk-4.1-dev",
            "cargo run -p xtask -- file-size-check",
            "cargo run -p xtask -- website-check",
            "cargo run -p xtask -- release-check",
            "clippy macOS",
            "cargo clippy --workspace --exclude dropsquash-desktop --all-targets -- -D warnings",
            "test macOS",
            "cargo test --workspace --exclude dropsquash-desktop",
            "clippy portable crates",
            "test portable crates",
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
name: Install Linux desktop dependencies
libwebkit2gtk-4.1-dev
run: pnpm --dir apps/desktop/web test
run: pnpm --dir apps/desktop/web build
uses: dtolnay/rust-toolchain@1.95.0
name: desktop-rust-macos
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
            "desktop-rust-macos",
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
name: Install Linux desktop dependencies
libwebkit2gtk-4.1-dev
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
            "Install Linux desktop dependencies",
            "libwebkit2gtk-4.1-dev",
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
fn rejects_committed_release_artifact_directories() {
    let directory = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(directory.path().join("release/DropSquash.app")).unwrap();

    let error = reject_secret_files(directory.path()).unwrap_err();

    assert!(error.contains("local evidence file"));
    assert!(error.contains("DropSquash.app"));
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
fn accepts_required_nix_dev_shell_systems() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("flake.nix");
    std::fs::write(
        &path,
        r#""aarch64-darwin" "x86_64-darwin" "aarch64-linux" "x86_64-linux""#,
    )
    .unwrap();

    require_nix_systems(&path).unwrap();
}

#[test]
fn reports_missing_intel_macos_nix_dev_shell_system() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("flake.nix");
    std::fs::write(&path, r#""aarch64-darwin" "aarch64-linux" "x86_64-linux""#).unwrap();

    let error = require_nix_systems(&path).unwrap_err();

    assert!(error.contains("x86_64-darwin"));
}

#[test]
fn accepts_aligned_nix_and_web_toolchain_contract() {
    let directory = tempfile::tempdir().unwrap();
    let flake = directory.path().join("flake.nix");
    let package = directory.path().join("package.json");
    std::fs::write(&flake, "nodejs_24 pnpm_10").unwrap();
    std::fs::write(
        &package,
        r#"{ "packageManager": "pnpm@10.34.0", "engines": { "node": ">=24 <25" } }"#,
    )
    .unwrap();

    require_web_toolchain_contract(&flake, &package).unwrap();
}

#[test]
fn reports_when_web_toolchain_contract_drifts() {
    let directory = tempfile::tempdir().unwrap();
    let flake = directory.path().join("flake.nix");
    let package = directory.path().join("package.json");
    std::fs::write(&flake, "nodejs_24").unwrap();
    std::fs::write(
        &package,
        r#"{ "packageManager": "pnpm@9.0.0", "engines": { "node": ">=25 <26" } }"#,
    )
    .unwrap();

    let error = require_web_toolchain_contract(&flake, &package).unwrap_err();

    assert!(error.contains("missing markers"));
    assert!(error.contains("pnpm_10") || error.contains("\"node\": \">=24 <25\""));
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
