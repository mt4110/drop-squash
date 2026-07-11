use super::missing_release_workflow_gates;
use super::secret_files::{is_secret_file, reject_secret_files};

#[test]
fn accepts_release_workflow_with_required_gates() {
    let missing = missing_release_workflow_gates(
        r#"
run: cargo run -p xtask -- file-size-check
run: cargo run -p xtask -- website-check
run: cargo run -p xtask -- release-check
run: pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci
run: cargo run -p xtask -- artifact-check target/release/bundle/dmg/*.dmg
run: cargo run -p xtask -- checksum target/release/bundle/dmg/*.dmg > SHA256SUMS
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
            "cargo run -p xtask -- file-size-check",
            "cargo run -p xtask -- website-check",
            "pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci",
            "cargo run -p xtask -- artifact-check target/release/bundle/dmg/*.dmg",
            "cargo run -p xtask -- checksum target/release/bundle/dmg/*.dmg > SHA256SUMS",
            "actions/upload-artifact@v4",
            "dropsquash-unsigned-dmg-checksum",
            "cargo run -p xtask -- macos-signing-check",
            "Block unsigned Phase 0 release"
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
    assert!(is_secret_file(".env.local", Some("local")));
    assert!(is_secret_file(".env.production", Some("production")));
}

#[test]
fn rejects_signing_secret_extensions() {
    assert!(is_secret_file("Distribution.p12", Some("p12")));
    assert!(is_secret_file("AuthKey_TEST.p8", Some("p8")));
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

fn write(root: &std::path::Path, name: &str, text: &str) {
    let path = root.join(name);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, text).unwrap();
}
