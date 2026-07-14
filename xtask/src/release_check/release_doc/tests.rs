use super::missing_requirements;

#[test]
fn accepts_release_doc_covering_checklist_categories() {
    let text = r#"
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
nix develop --command pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci
pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci
cargo run -p xtask -- manual-qa-prepare
--input-sample-set
cargo run -p xtask -- manual-qa-prepare --restore-state
cargo run -p xtask -- manual-qa-check
cargo run -p xtask -- benchmark-csv-check
/tmp/dropsquash-manual-qa-output/benchmark-results.csv
cargo run -p xtask -- normalize-dmg target/release/bundle/dmg
cargo run -p xtask -- checksum path/to/DropSquash.dmg --output SHA256SUMS
unsigned DMG as a QA artifact
No signing secrets belong in the repository
`ffmpeg`, `ffprobe`, shell
do not upload media files
do not enable telemetry by default
cargo run -p xtask -- media-policy-check
cargo run -p xtask -- privacy-policy-check
cargo run -p xtask -- artifact-check path/to/DropSquash.dmg
signed, notarized, stapled, checked, and checksummed
Gatekeeper no-warning evidence
first public beta target
Developer ID Application
notarization credentials
cargo run -p xtask -- macos-signing-check
docs/release-blockers.md
Lemon Squeezy product setup
license keys enabled
test purchase
Live checkout URL
refund policy
Lemon Squeezy sandbox purchase
Valid sandbox activation
Empty key activation
Invalid key activation
License network failure
Expired license refresh
Local license forget
cargo run -p xtask -- homebrew-cask 0.1.0
cargo run -p xtask -- homebrew-cask-check
brew install --cask
brew uninstall --cask
auto_updates false
zap
cargo run -p xtask -- release-notes-check
cargo run -p xtask -- github-release-plan
cargo run -p xtask -- publish-check
Benchmark sample set
20% regression threshold
Expired license refresh
Homebrew evidence
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
    assert!(missing.iter().any(|item| item.contains("release notes:")));
}
