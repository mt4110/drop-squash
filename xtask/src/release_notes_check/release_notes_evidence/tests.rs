use super::{check, check_text};

#[test]
fn reports_missing_release_notes_path() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("missing-release-notes.md");

    let error = check(&path).unwrap_err();

    assert!(error.contains("failed to read release notes"));
    assert!(error.contains("missing-release-notes.md"));
}

#[test]
fn accepts_concrete_production_urls() {
    let errors = check_text(
        r#"
- Artifact URL: https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg
- Version: v0.1.0
- Artifact: DropSquash.dmg
- SHA-256: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
- Git commit: abc1234
- `codesign`: codesign verified Developer ID Application signature
- `spctl`: spctl accepted Developer ID source
- `stapler`: stapler validate showed ticket stapled successfully
- Apple notary log: notarytool accepted request abc123
- Gatekeeper clean-machine open: Gatekeeper opened app cleanly in fresh account
- `docs/release-blockers.md` status: docs/release-blockers.md has all rows Verified
- Manual QA record: docs/manual-qa.md filled for DropSquash.dmg
- Benchmark sample set: short medium large local recordings recorded on MacBookPro18,4 macOS 26.5.2
- Benchmark regression threshold: no sample exceeded 20 percent regression
- Lemon Squeezy product setup: DropSquash intended product has license keys enabled
- Lemon Squeezy sandbox purchase: intended product checkout completed for test buyer order abc123
- Lemon Squeezy sandbox activation: Pro state reached and raw key absent from cache
- Empty key activation: friendly validation shown and raw key absent from cache
- Invalid license key handling: friendly error shown and raw key absent from cache
- License network failure: friendly network error shown, existing valid cache preserved, raw key absent from cache
- Local license forget: license cache removed and trial state restored
- Public website URL: https://dropsquash.app/release-status
- Refund policy URL: https://dropsquash.app/refund
- Live checkout URL: https://store.lemonsqueezy.com/checkout/buy/abc123
- GitHub Release checksum: SHA256SUMS attached to release for DropSquash.dmg
- GitHub Release URL: https://github.com/mt4110/drop-squash/releases/tag/v0.1.0
- Homebrew tap PR: cask update reviewed in tap PR with zap cleanup path
- Homebrew tap PR URL: https://github.com/mt4110/homebrew-tap/pull/1
- Homebrew install result: brew install --cask mt4110/tap/dropsquash completed
- Known limitations: macOS MVP only; Windows and Linux platform builds remain unreleased
- Support contact: support handled through GitHub Issues until paid support opens
"#,
    );

    assert!(errors.is_empty());
}

#[test]
fn rejects_weak_distribution_evidence() {
    let errors = check_text(
        r#"
- Artifact URL: https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg
- Version: v0.1.0
- Artifact: DropSquash.dmg
- SHA-256: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
- Git commit: abc1234
- `codesign`: signature ok
- `spctl`: accepted
- `stapler`: ticket ok
- Apple notary log: request completed
- Gatekeeper clean-machine open: opened app
- `docs/release-blockers.md` status: verified
- Manual QA record: manual QA filled
- Lemon Squeezy product setup: product ready
- Lemon Squeezy sandbox purchase: purchase completed
- Lemon Squeezy sandbox activation: activated
- Empty key activation: empty key handled
- Invalid license key handling: invalid key handled
- License network failure: network failed
- Local license forget: forgot license
- Public website URL: https://dropsquash.app/release-status
- Refund policy URL: https://dropsquash.app/pricing
- Live checkout URL: https://store.lemonsqueezy.com/checkout/buy/abc123
- GitHub Release checksum: SHA256SUMS attached
- GitHub Release URL: https://github.com/mt4110/drop-squash/releases/tag/v0.1.0
- Homebrew tap PR: reviewed
- Homebrew tap PR URL: https://github.com/mt4110/homebrew-tap/pull/1
- Homebrew install result: installed
- Known limitations: none
- Support contact: support soon
"#,
    );

    assert!(errors.iter().any(|error| error.contains("`codesign`")));
    assert!(errors.iter().any(|error| error.contains("`stapler`")));
    assert!(errors
        .iter()
        .any(|error| error.contains("Gatekeeper clean-machine open")));
    assert!(errors
        .iter()
        .any(|error| error.contains("Refund policy URL")));
    assert!(errors
        .iter()
        .any(|error| error.contains("Homebrew install result")));
    assert!(errors.iter().any(|error| error.contains("Homebrew tap PR")));
    assert!(errors
        .iter()
        .any(|error| error.contains("GitHub Release checksum")));
    assert!(errors
        .iter()
        .any(|error| error.contains("docs/release-blockers.md")));
    assert!(errors
        .iter()
        .any(|error| error.contains("Lemon Squeezy product setup")));
    assert!(errors
        .iter()
        .any(|error| error.contains("Known limitations")));
    assert!(errors.iter().any(|error| error.contains("Support contact")));
}

#[test]
fn rejects_local_license_forget_without_cache_removal() {
    let errors = check_text(
        r#"
- Local license forget: license cache checked and trial state restored
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Local license forget")));
}

#[test]
fn rejects_network_failure_without_existing_valid_cache() {
    let errors = check_text(
        r#"
- License network failure: friendly network error shown, license cache preserved, raw key absent from cache
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("License network failure")));
}

#[test]
fn rejects_activation_with_persisted_raw_key() {
    let errors = check_text(
        r#"
- Lemon Squeezy sandbox activation: Pro state reached and raw key persisted in cache
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Lemon Squeezy sandbox activation")));
}

#[test]
fn rejects_duplicate_release_note_fields() {
    let errors = check_text(
        r#"
- Version: v0.1.0
- Version: v0.1.1
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Version must appear only once")));
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
fn rejects_nested_artifact_download_url() {
    let errors = check_text(
        r#"
- Artifact URL: https://github.com/mt4110/drop-squash/releases/download/v0.1.0/nested/DropSquash.dmg
"#,
    );

    assert!(errors.iter().any(|error| error.contains("Artifact URL")));
}

#[test]
fn rejects_release_notes_without_specific_public_urls() {
    let errors = check_text(
        r#"
- Public website URL: https://dropsquash.app
- Live checkout URL: https://store.lemonsqueezy.com/checkout
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Public website URL")));
    assert!(errors
        .iter()
        .any(|error| error.contains("Live checkout URL")));
}

#[test]
fn rejects_checkout_url_without_buy_id() {
    let errors = check_text(
        r#"
- Live checkout URL: https://store.lemonsqueezy.com/checkout/buy/
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Live checkout URL")));
}

#[test]
fn rejects_url_fields_with_inline_notes() {
    let errors = check_text(
        r#"
- Live checkout URL: https://store.lemonsqueezy.com/checkout/buy/abc123 TBD
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Live checkout URL")));
}

#[test]
fn rejects_refund_policy_url_on_different_origin() {
    let errors = check_text(
        r#"
- Public website URL: https://dropsquash.app/release-status
- Refund policy URL: https://support.dropsquash.app/refund
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Refund policy URL must use the same origin")));
}

#[test]
fn rejects_html_release_status_url() {
    let errors = check_text(
        r#"
- Public website URL: https://dropsquash.app/release-status.html
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Public website URL")));
}

#[test]
fn rejects_homebrew_pr_outside_expected_tap() {
    let errors = check_text(
        r#"
- Artifact URL: https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg
- Version: v0.1.0
- Artifact: DropSquash.dmg
- SHA-256: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
- Git commit: abc1234
- Public website URL: https://dropsquash.app/release-status
- Live checkout URL: https://store.lemonsqueezy.com/checkout/buy/abc123
- GitHub Release URL: https://github.com/mt4110/drop-squash/releases/tag/v0.1.0
- Homebrew tap PR URL: https://github.com/someone/other-tap/pull/1
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Homebrew tap PR URL")));
}

#[test]
fn rejects_homebrew_pr_without_numeric_pull_request() {
    let errors = check_text(
        r#"
- Artifact URL: https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg
- Version: v0.1.0
- Artifact: DropSquash.dmg
- SHA-256: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
- Git commit: abc1234
- Public website URL: https://dropsquash.app/release-status
- Live checkout URL: https://store.lemonsqueezy.com/checkout/buy/abc123
- GitHub Release URL: https://github.com/mt4110/drop-squash/releases/tag/v0.1.0
- Homebrew tap PR URL: https://github.com/mt4110/homebrew-tap/pull/not-a-number
"#,
    );

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
- Public website URL: https://dropsquash.app/release-status
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
        .any(|error| error.contains("Lemon Squeezy product setup")));
    assert!(errors
        .iter()
        .any(|error| error.contains("Lemon Squeezy sandbox purchase")));
    assert!(errors
        .iter()
        .any(|error| error.contains("Empty key activation")));
    assert!(errors
        .iter()
        .any(|error| error.contains("Invalid license key handling")));
    assert!(errors
        .iter()
        .any(|error| error.contains("Local license forget")));
    assert!(errors
        .iter()
        .any(|error| error.contains("Known limitations")));
    assert!(errors.iter().any(|error| error.contains("Support contact")));
}

#[test]
fn rejects_sandbox_purchase_without_intended_product() {
    let errors = check_text(
        r#"
- Lemon Squeezy sandbox purchase: test buyer order abc123 completed
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Lemon Squeezy sandbox purchase")));
}

#[test]
fn rejects_incomplete_benchmark_evidence() {
    let errors = check_text(
        r#"
- Artifact URL: https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg
- Version: v0.1.0
- Artifact: DropSquash.dmg
- SHA-256: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
- Git commit: abc1234
- Benchmark sample set: local recordings recorded
- Benchmark regression threshold: no meaningful regression
- Public website URL: https://dropsquash.app/release-status
- Live checkout URL: https://store.lemonsqueezy.com/checkout/buy/abc123
- GitHub Release URL: https://github.com/mt4110/drop-squash/releases/tag/v0.1.0
- Homebrew tap PR URL: https://github.com/mt4110/homebrew-tap/pull/1
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("machine, and OS context")));
    assert!(errors.iter().any(|error| error.contains("20%")));
}

#[test]
fn rejects_benchmark_sample_set_without_machine_context() {
    let errors = check_text(
        r#"
- Artifact URL: https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg
- Version: v0.1.0
- Artifact: DropSquash.dmg
- SHA-256: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
- Git commit: abc1234
- Benchmark sample set: short medium large local recordings recorded
- Benchmark regression threshold: no sample exceeded 20% regression
- Public website URL: https://dropsquash.app/release-status
- Live checkout URL: https://store.lemonsqueezy.com/checkout/buy/abc123
- GitHub Release URL: https://github.com/mt4110/drop-squash/releases/tag/v0.1.0
- Homebrew tap PR URL: https://github.com/mt4110/homebrew-tap/pull/1
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("machine, and OS context")));
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
- Public website URL: https://dropsquash.app/release-status
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
- Public website URL: https://dropsquash.app/release-status
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
