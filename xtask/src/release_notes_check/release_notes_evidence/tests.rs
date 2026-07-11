use super::check_text;

#[test]
fn accepts_concrete_production_urls() {
    let errors = check_text(
        r#"
- Artifact URL: https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg
- Version: v0.1.0
- Artifact: DropSquash.dmg
- SHA-256: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
- Git commit: abc1234
- `codesign`: valid on DropSquash.app
- `spctl`: accepted source Developer ID
- `stapler`: ticket stapled successfully
- Apple notary log: notarytool accepted request abc123
- Gatekeeper clean-machine open: fresh account opened app
- `docs/release-blockers.md` status: all rows Verified
- Manual QA record: docs/manual-qa.md filled for DropSquash.dmg
- Benchmark sample set: short medium large local recordings recorded
- Benchmark regression threshold: no sample exceeded 20 percent regression
- Lemon Squeezy sandbox purchase: test buyer order abc123 completed
- Lemon Squeezy sandbox activation: Pro state reached without raw key cache
- Public website URL: https://dropsquash.app
- Live checkout URL: https://store.lemonsqueezy.com/checkout/buy/abc123
- GitHub Release checksum: SHA256SUMS attached to release
- GitHub Release URL: https://github.com/mt4110/drop-squash/releases/tag/v0.1.0
- Homebrew tap PR: cask update reviewed in tap PR
- Homebrew tap PR URL: https://github.com/mt4110/homebrew-tap/pull/1
- Homebrew install result: brew install completed
"#,
    );

    assert!(errors.is_empty());
}

#[test]
fn rejects_placeholders_and_wrong_url_kinds() {
    let errors = check_text(
        r#"
- Artifact URL: https://example.com/DropSquash.dmg
- Public website URL: https://store.lemonsqueezy.com/checkout/buy/abc123
- Live checkout URL: https://dropsquash.app/pricing
- GitHub Release URL: https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg
- Homebrew tap PR URL: TBD
"#,
    );

    assert!(errors.iter().any(|error| error.contains("Artifact URL")));
    assert!(errors
        .iter()
        .any(|error| error.contains("Public website URL")));
    assert!(errors
        .iter()
        .any(|error| error.contains("Live checkout URL")));
    assert!(errors
        .iter()
        .any(|error| error.contains("GitHub Release URL")));
    assert!(errors
        .iter()
        .any(|error| error.contains("Homebrew tap PR URL")));
}

#[test]
fn rejects_missing_or_generic_release_evidence() {
    let errors = check_text(
        r#"
- Artifact URL: https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg
- Version: soon
- Artifact: DropSquash.zip
- SHA-256:
- Git commit: release source commit
- `codesign`: OK
- Public website URL: https://dropsquash.app
- Live checkout URL: https://store.lemonsqueezy.com/checkout/buy/abc123
- GitHub Release URL: https://github.com/mt4110/drop-squash/releases/tag/v0.1.0
- Homebrew tap PR URL: https://github.com/mt4110/homebrew-tap/pull/1
"#,
    );

    assert!(errors.iter().any(|error| error.contains("SHA-256")));
    assert!(errors.iter().any(|error| error.contains("Git commit")));
    assert!(errors.iter().any(|error| error.contains("Version")));
    assert!(errors.iter().any(|error| error.contains("Artifact")));
    assert!(errors.iter().any(|error| error.contains("`codesign`")));
    assert!(errors.iter().any(|error| error.contains("`spctl`")));
    assert!(errors
        .iter()
        .any(|error| error.contains("GitHub Release checksum")));
    assert!(errors
        .iter()
        .any(|error| error.contains("Manual QA record")));
    assert!(errors
        .iter()
        .any(|error| error.contains("Lemon Squeezy sandbox purchase")));
}

#[test]
fn rejects_non_hex_sha256() {
    let errors = check_text(
        r#"
- Artifact URL: https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg
- Version: v0.1.0
- Artifact: DropSquash.dmg
- SHA-256: 64 hex chars recorded in SHA256SUMS
- Git commit: abc1234
- Public website URL: https://dropsquash.app
- Live checkout URL: https://store.lemonsqueezy.com/checkout/buy/abc123
- GitHub Release URL: https://github.com/mt4110/drop-squash/releases/tag/v0.1.0
- Homebrew tap PR URL: https://github.com/mt4110/homebrew-tap/pull/1
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("64-character hex checksum")));
}

#[test]
fn rejects_mismatched_release_identity_values() {
    let errors = check_text(
        r#"
- Artifact URL: https://github.com/mt4110/drop-squash/releases/download/v9.9.9/Other.dmg
- Version: v0.1.0
- Artifact: DropSquash.dmg
- SHA-256: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
- Git commit: abc1234
- Public website URL: https://dropsquash.app
- Live checkout URL: https://store.lemonsqueezy.com/checkout/buy/abc123
- GitHub Release URL: https://github.com/mt4110/drop-squash/releases/tag/v9.9.9
- Homebrew tap PR URL: https://github.com/mt4110/homebrew-tap/pull/1
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Artifact URL must match Version")));
    assert!(errors
        .iter()
        .any(|error| error.contains("GitHub Release URL must match Version")));
    assert!(errors
        .iter()
        .any(|error| error.contains("Artifact must match Artifact URL")));
}
