use super::missing_requirements;

#[test]
fn accepts_release_doc_covering_checklist_categories() {
    let text = r#"
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci
cargo run -p xtask -- manual-qa-prepare
cargo run -p xtask -- manual-qa-prepare --restore-state
cargo run -p xtask -- manual-qa-check
cargo run -p xtask -- normalize-dmg target/release/bundle/dmg
unsigned DMG as a QA artifact
No signing secrets belong in the repository
cargo run -p xtask -- media-policy-check
cargo run -p xtask -- privacy-policy-check
cargo run -p xtask -- artifact-check path/to/DropSquash.dmg
signed, notarized, stapled, checked, and checksummed
Gatekeeper no-warning evidence
first public beta target
cargo run -p xtask -- macos-signing-check
docs/release-blockers.md
Lemon Squeezy sandbox purchase
Valid sandbox activation
cargo run -p xtask -- homebrew-cask 0.1.0
Benchmark sample set
20% regression threshold
auto_updates false
zap
"#;

    assert!(missing_requirements(text).is_empty());
}

#[test]
fn release_doc_covers_checklist_categories() {
    let text = std::fs::read_to_string("../docs/release.md").unwrap();

    assert!(missing_requirements(&text).is_empty());
}

#[test]
fn reports_missing_release_doc_checklist_coverage() {
    let missing = missing_requirements("");

    assert!(missing
        .iter()
        .any(|item| item.contains("build: cargo test")));
    assert!(missing.iter().any(|item| item.contains("security:")));
    assert!(missing.iter().any(|item| item.contains("macos:")));
    assert!(missing.iter().any(|item| item.contains("store:")));
    assert!(missing.iter().any(|item| item.contains("homebrew:")));
}
