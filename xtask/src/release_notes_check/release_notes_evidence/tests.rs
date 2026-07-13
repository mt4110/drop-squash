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
    let directory = tempfile::tempdir().unwrap();
    let csv = csv_file(directory.path(), "results.csv");
    let errors = check_text(&format!(
        r#"
- Artifact URL: https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg
- Version: v0.1.0
- Artifact: DropSquash.dmg
- SHA-256: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
- Git commit: abc1234
- `codesign`: codesign verified Developer ID Application signature for public DropSquash.dmg
- `spctl`: spctl accepted Developer ID source for public DropSquash.dmg
- `stapler`: stapler validate showed ticket stapled successfully for public DropSquash.dmg
- Apple notary log: notarytool accepted request abc123 for public DropSquash.dmg
- Gatekeeper clean-machine open: Gatekeeper opened signed, notarized, stapled app from public DropSquash.dmg cleanly in fresh account without Gatekeeper warning
- `docs/release-blockers.md` status: docs/release-blockers.md has all rows Verified
- Manual QA record: docs/manual-qa.md tested public DropSquash.dmg and manual-qa-check passed
- Conversion safety evidence: cancellation, failed conversion, and larger output not smaller failure preserved original with trial count unchanged
- Queue evidence: multi-file queue, queued cancellation, and batch summary showed trial lock blocked pending jobs with finished count 2, saved bytes 123456, failed 0, cancelled 1, and blocked 0
- Trash source policy: Moving original state disabled action; original moved to Trash only after verified smaller output
- Benchmark sample set: short medium large local recordings produced smaller outputs with backend apple-native, saved percent, duration, and speed ratio on MacBookPro18,4 macOS 26.5.2 with CSV saved outside repo at {}
- Benchmark regression threshold: no sample exceeded 20 percent regression against the same-machine release candidate baseline
- Lemon Squeezy product setup: DropSquash sandbox intended product has license keys enabled and private store IDs not recorded
- Lemon Squeezy sandbox purchase: sandbox checkout completed for intended product test buyer order abc123
- Valid sandbox activation: Lemon Squeezy sandbox activation request entered Activating state, disabled submit, reached Pro state, and checked cache kept fingerprint 1111111111111111111111111111111111111111111111111111111111111111 plus instance_id field with raw key absent
- Empty key activation: Activate disabled for empty input and raw key absent from cache with no fingerprint and no instance
- Invalid license key handling: Activating state disabled submit; friendly error shown and raw key absent from cache with no fingerprint and no instance
- License network failure: friendly network error shown, existing valid cache preserved fingerprint 1111111111111111111111111111111111111111111111111111111111111111 plus instance_id field with raw key absent
- Expired license refresh: expired offline grace license cache showed reconnect prompt, blocked conversion before starting, and raw key absent from cache
- Local license forget: Forgetting state disabled action; license cache removed and trial state restored
- Public website URL: HTTPS://dropsquash.app/release-status
- Refund policy URL: https://dropsquash.app/refund
- Live checkout URL: https://store.lemonsqueezy.com/checkout/buy/abc123
- GitHub Release URL: https://github.com/mt4110/drop-squash/releases/tag/v0.1.0
- GitHub Release checksum: SHA256SUMS attached to public https://github.com/mt4110/drop-squash/releases/tag/v0.1.0 for https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg with lowercase SHA-256 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
- Homebrew tap PR: cask update reviewed in public PR at https://github.com/mt4110/homebrew-tap/pull/1 for versioned DropSquash.dmg using https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg with lowercase SHA-256 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef, auto_updates false, and zap cleanup path
- Homebrew tap PR URL: https://github.com/mt4110/homebrew-tap/pull/1
- Homebrew install result: brew install --cask mt4110/tap/dropsquash completed for versioned DropSquash.dmg artifact with lowercase SHA-256 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
- Known limitations: macOS MVP only; Windows and Linux platform builds remain unreleased
- Support contact: support handled through GitHub Issues until paid support opens
"#,
        csv.display()
    ));

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
- Conversion safety evidence: conversions safe
- Queue evidence: queue worked
- Trash source policy: original trashed
- Lemon Squeezy product setup: product ready
- Lemon Squeezy sandbox purchase: purchase completed
- Valid sandbox activation: activated
- Empty key activation: empty key handled
- Invalid license key handling: invalid key handled
- License network failure: network failed
- Expired license refresh: refresh required
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
        .any(|error| error.contains("Conversion safety evidence")));
    assert!(errors.iter().any(|error| error.contains("Queue evidence")));
    assert!(errors
        .iter()
        .any(|error| error.contains("Trash source policy")));
    assert!(errors
        .iter()
        .any(|error| error.contains("Known limitations")));
    assert!(errors.iter().any(|error| error.contains("Support contact")));
}

#[test]
fn rejects_conversion_safety_without_original_remained_evidence() {
    let errors = check_text(
        r#"
- Conversion safety evidence: cancellation, failed conversion, and larger output not smaller failure checked original with trial count unchanged
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Conversion safety evidence")));
}

#[test]
fn rejects_queue_evidence_without_numeric_counts() {
    let errors = check_text(
        r#"
- Queue evidence: multi-file queue, queued cancellation, and batch summary showed finished count, saved bytes, failed count, cancelled count, and blocked count
"#,
    );

    assert!(errors.iter().any(|error| error.contains("Queue evidence")));
}

#[test]
fn rejects_queue_evidence_without_lock_blocking_context() {
    let errors = check_text(
        r#"
- Queue evidence: multi-file queue, queued cancellation, and batch summary showed finished count 2, saved bytes 123456, failed 0, cancelled 1, and blocked 0
"#,
    );

    assert!(errors.iter().any(|error| error.contains("Queue evidence")));
}

#[test]
fn rejects_sandbox_purchase_without_checkout_context() {
    let errors = check_text(
        r#"
- Lemon Squeezy sandbox purchase: sandbox completed for intended product by test buyer order abc123
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Lemon Squeezy sandbox purchase")));
}

#[test]
fn rejects_product_setup_without_private_store_id_absence() {
    let errors = check_text(
        r#"
- Lemon Squeezy product setup: DropSquash sandbox intended product has license keys enabled
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Lemon Squeezy product setup")));
}

#[test]
fn rejects_signing_evidence_without_public_artifact_name() {
    let errors = check_text(
        r#"
- `codesign`: codesign verified Developer ID Application signature
- `spctl`: spctl accepted Developer ID source
- `stapler`: stapler validate showed ticket stapled successfully
- Apple notary log: notarytool accepted request abc123
"#,
    );

    assert!(errors.iter().any(|error| error.contains("`codesign`")));
    assert!(errors.iter().any(|error| error.contains("`spctl`")));
    assert!(errors.iter().any(|error| error.contains("`stapler`")));
    assert!(errors
        .iter()
        .any(|error| error.contains("Apple notary log")));
}

#[test]
fn rejects_signing_evidence_without_public_context() {
    let errors = check_text(
        r#"
- `codesign`: codesign verified Developer ID Application signature for DropSquash.dmg
- `spctl`: spctl accepted Developer ID source for DropSquash.dmg
- `stapler`: stapler validate showed ticket stapled successfully for DropSquash.dmg
- Apple notary log: notarytool accepted request abc123 for DropSquash.dmg
"#,
    );

    assert!(errors.iter().any(|error| error.contains("`codesign`")));
    assert!(errors.iter().any(|error| error.contains("`spctl`")));
    assert!(errors.iter().any(|error| error.contains("`stapler`")));
    assert!(errors
        .iter()
        .any(|error| error.contains("Apple notary log")));
}

#[test]
fn rejects_gatekeeper_evidence_without_signed_notarized_context() {
    let errors = check_text(
        r#"
- Gatekeeper clean-machine open: Gatekeeper opened app cleanly in fresh account
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Gatekeeper clean-machine open")));
}

#[test]
fn rejects_gatekeeper_evidence_without_warning_context() {
    let errors = check_text(
        r#"
- Gatekeeper clean-machine open: Gatekeeper opened signed, notarized, stapled app cleanly in fresh account
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Gatekeeper clean-machine open")));
}

#[test]
fn rejects_gatekeeper_evidence_without_staple_context() {
    let errors = check_text(
        r#"
- Gatekeeper clean-machine open: Gatekeeper opened signed and notarized app cleanly in fresh account without Gatekeeper warning
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Gatekeeper clean-machine open")));
}

#[test]
fn rejects_gatekeeper_evidence_without_public_dmg_context() {
    let errors = check_text(
        r#"
- Gatekeeper clean-machine open: Gatekeeper opened signed, notarized, stapled app cleanly in fresh account without Gatekeeper warning
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Gatekeeper clean-machine open")));
}

#[test]
fn rejects_homebrew_evidence_without_cask_policy_context() {
    let errors = check_text(
        r#"
- Homebrew tap PR: cask update reviewed in tap PR with zap cleanup path
- Homebrew install result: brew install --cask mt4110/tap/dropsquash completed
"#,
    );

    assert!(errors.iter().any(|error| error.contains("Homebrew tap PR")));
    assert!(errors
        .iter()
        .any(|error| error.contains("Homebrew install result")));
}

#[test]
fn rejects_homebrew_tap_pr_without_artifact_url() {
    let errors = check_text(
        r#"
- Artifact URL: https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg
- Homebrew tap PR: cask update reviewed in tap PR for DropSquash.dmg with auto_updates false and zap cleanup path
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Homebrew tap PR must include")));
}

#[test]
fn rejects_homebrew_tap_pr_without_sha256_digest() {
    let errors = check_text(
        r#"
- Artifact URL: https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg
- SHA-256: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
- Homebrew tap PR: cask update reviewed in tap PR for DropSquash.dmg using https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg with auto_updates false and zap cleanup path
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("lowercase SHA-256 digest")));
}

#[test]
fn rejects_homebrew_tap_pr_without_matching_pr_url() {
    let errors = check_text(
        r#"
- Homebrew tap PR URL: https://github.com/mt4110/homebrew-tap/pull/1
- Homebrew tap PR: cask update reviewed at https://github.com/mt4110/homebrew-tap/pull/2 for versioned DropSquash.dmg using https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg with lowercase SHA-256 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef, auto_updates false, and zap cleanup path
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Homebrew tap PR URL")));
}

#[test]
fn rejects_github_release_checksum_without_release_url() {
    let errors = check_text(
        r#"
- Artifact URL: https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg
- SHA-256: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
- GitHub Release URL: https://github.com/mt4110/drop-squash/releases/tag/v0.1.0
- GitHub Release checksum: SHA256SUMS attached for https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg with lowercase SHA-256 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("GitHub Release checksum")));
}

#[test]
fn rejects_github_release_checksum_without_public_context() {
    let errors = check_text(
        r#"
- Artifact URL: https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg
- SHA-256: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
- GitHub Release URL: https://github.com/mt4110/drop-squash/releases/tag/v0.1.0
- GitHub Release checksum: SHA256SUMS attached to https://github.com/mt4110/drop-squash/releases/tag/v0.1.0 for https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg with lowercase SHA-256 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("GitHub Release checksum")));
}

#[test]
fn rejects_github_release_checksum_without_artifact_url() {
    let errors = check_text(
        r#"
- Artifact URL: https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg
- SHA-256: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
- GitHub Release URL: https://github.com/mt4110/drop-squash/releases/tag/v0.1.0
- GitHub Release checksum: SHA256SUMS attached to https://github.com/mt4110/drop-squash/releases/tag/v0.1.0 with lowercase SHA-256 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("GitHub Release checksum")));
}

#[test]
fn rejects_github_release_checksum_without_sha256_digest() {
    let errors = check_text(
        r#"
- Artifact URL: https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg
- SHA-256: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
- GitHub Release URL: https://github.com/mt4110/drop-squash/releases/tag/v0.1.0
- GitHub Release checksum: SHA256SUMS attached to https://github.com/mt4110/drop-squash/releases/tag/v0.1.0 for https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("GitHub Release checksum")));
}

#[test]
fn rejects_homebrew_tap_pr_without_versioned_artifact_context() {
    let errors = check_text(
        r#"
- Artifact URL: https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg
- SHA-256: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
- Homebrew tap PR: cask update reviewed in tap PR for DropSquash.dmg using https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg with lowercase SHA-256 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef, auto_updates false, and zap cleanup path
"#,
    );

    assert!(errors.iter().any(|error| error.contains("Homebrew tap PR")));
}

#[test]
fn rejects_homebrew_tap_pr_without_public_context() {
    let errors = check_text(
        r#"
- Artifact URL: https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg
- Homebrew tap PR URL: https://github.com/mt4110/homebrew-tap/pull/1
- SHA-256: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
- Homebrew tap PR: cask update reviewed in PR at https://github.com/mt4110/homebrew-tap/pull/1 for versioned DropSquash.dmg using https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg with lowercase SHA-256 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef, auto_updates false, and zap cleanup path
"#,
    );

    assert!(errors.iter().any(|error| error.contains("Homebrew tap PR")));
}

#[test]
fn rejects_homebrew_install_without_versioned_artifact_context() {
    let errors = check_text(
        r#"
- Homebrew install result: brew install --cask mt4110/tap/dropsquash completed for DropSquash.dmg artifact
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Homebrew install result")));
}

#[test]
fn rejects_homebrew_install_without_sha256_digest() {
    let errors = check_text(
        r#"
- SHA-256: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
- Homebrew install result: brew install --cask mt4110/tap/dropsquash completed for versioned DropSquash.dmg artifact
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Homebrew install result")));
}

#[test]
fn rejects_prepared_homebrew_install_draft() {
    let errors = check_text(
        r#"
- SHA-256: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
- Homebrew install result: after `brew install --cask mt4110/tap/dropsquash` installs the versioned DropSquash.dmg artifact with lowercase SHA-256 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef, replace this line with observed install evidence
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Homebrew install result")));
}

#[test]
fn rejects_prepared_homebrew_tap_pr_draft() {
    let errors = check_text(
        r#"
- Artifact URL: https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg
- SHA-256: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
- Homebrew tap PR: public cask PR for versioned DropSquash.dmg uses public https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg with lowercase SHA-256 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef, auto_updates false, and zap cleanup path; replace this line with reviewed public PR evidence
"#,
    );

    assert!(errors.iter().any(|error| error.contains("Homebrew tap PR")));
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
fn rejects_network_failure_without_fingerprint_instance_evidence() {
    let errors = check_text(
        r#"
- License network failure: friendly network error shown, existing valid cache preserved, raw key absent from cache
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("License network failure")));
}

#[test]
fn rejects_expired_refresh_without_reconnect_prompt() {
    let errors = check_text(
        r#"
- Expired license refresh: expired offline grace cache blocked conversion before starting and raw key absent from cache
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Expired license refresh")));
}

#[test]
fn rejects_expired_refresh_without_blocked_conversion() {
    let errors = check_text(
        r#"
- Expired license refresh: expired offline grace cache showed reconnect prompt and raw key absent from cache
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Expired license refresh")));
}

#[test]
fn rejects_activation_with_persisted_raw_key() {
    let errors = check_text(
        r#"
- Valid sandbox activation: Activating state disabled submit; Pro state reached and raw key persisted in cache
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Valid sandbox activation")));
}

#[test]
fn rejects_valid_activation_without_sandbox_request_context() {
    let errors = check_text(
        r#"
- Valid sandbox activation: Activating state disabled submit; Pro state reached and raw key absent from cache
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Valid sandbox activation")));
}

#[test]
fn rejects_valid_activation_without_fingerprint_instance_evidence() {
    let errors = check_text(
        r#"
- Valid sandbox activation: Lemon Squeezy sandbox activation request entered Activating state, disabled submit, reached Pro state, and raw key absent from cache
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Valid sandbox activation")));
}

#[test]
fn rejects_valid_activation_without_cache_observation() {
    let errors = check_text(
        r#"
- Valid sandbox activation: Lemon Squeezy sandbox activation request entered Activating state, disabled submit, reached Pro state, and cache kept fingerprint 1111111111111111111111111111111111111111111111111111111111111111 plus instance_id field with raw key absent
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Valid sandbox activation")));
}

#[test]
fn rejects_activation_without_submit_context() {
    let errors = check_text(
        r#"
- Valid sandbox activation: Lemon Squeezy sandbox activation request entered Activating state, disabled, reached Pro state, and cache kept fingerprint 1111111111111111111111111111111111111111111111111111111111111111 plus instance_id field with raw key absent
- Invalid license key handling: Activating state disabled; friendly error shown and raw key absent from cache with no fingerprint and no instance
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Valid sandbox activation")));
    assert!(errors
        .iter()
        .any(|error| error.contains("Invalid license key handling")));
}

#[test]
fn rejects_valid_activation_without_hex_fingerprint_evidence() {
    let errors = check_text(
        r#"
- Valid sandbox activation: Lemon Squeezy sandbox activation request entered Activating state, disabled submit, reached Pro state, and cache kept fingerprint and instance fields with raw key absent
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Valid sandbox activation")));
}

#[test]
fn rejects_valid_activation_without_instance_id_evidence() {
    let errors = check_text(
        r#"
- Valid sandbox activation: Lemon Squeezy sandbox activation request entered Activating state, disabled submit, reached Pro state, and cache kept fingerprint 1111111111111111111111111111111111111111111111111111111111111111 plus instance fields with raw key absent
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Valid sandbox activation")));
}

#[test]
fn rejects_network_failure_without_hex_fingerprint_evidence() {
    let errors = check_text(
        r#"
- License network failure: friendly network error shown, existing valid cache preserved fingerprint and instance fields with raw key absent
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("License network failure")));
}

#[test]
fn rejects_network_failure_without_instance_id_evidence() {
    let errors = check_text(
        r#"
- License network failure: friendly network error shown, existing valid cache preserved fingerprint 1111111111111111111111111111111111111111111111111111111111111111 plus instance fields with raw key absent
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("License network failure")));
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
fn rejects_ellipsis_placeholder_urls() {
    let errors = check_text(
        r#"
- Public website URL: https://.../release-status
- GitHub Release URL: GitHub Release https://...
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Public website URL")));
    assert!(errors
        .iter()
        .any(|error| error.contains("GitHub Release URL")));
}

#[test]
fn rejects_release_urls_with_imposter_hosts() {
    let errors = check_text(
        r#"
- Artifact URL: https://github.com.evil/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg
- Live checkout URL: https://lemonsqueezy.com.evil/checkout/buy/abc123
- GitHub Release URL: https://github.com.evil/mt4110/drop-squash/releases/tag/v0.1.0
- Homebrew tap PR URL: https://github.com.evil/mt4110/homebrew-tap/pull/1
"#,
    );

    assert!(errors.iter().any(|error| error.contains("Artifact URL")));
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
fn rejects_release_urls_with_query_or_fragment() {
    let errors = check_text(
        r#"
- Artifact URL: https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg?download=1
- Public website URL: https://dropsquash.app/release-status?source=release
- Refund policy URL: https://dropsquash.app/refund#terms
- Live checkout URL: https://store.lemonsqueezy.com/checkout/buy/abc123?utm=release
- GitHub Release URL: https://github.com/mt4110/drop-squash/releases/tag/v0.1.0#assets
- Homebrew tap PR URL: https://github.com/mt4110/homebrew-tap/pull/1?plain=1
"#,
    );

    assert!(errors.iter().any(|error| error.contains("Artifact URL")));
    assert!(errors
        .iter()
        .any(|error| error.contains("Public website URL")));
    assert!(errors
        .iter()
        .any(|error| error.contains("Refund policy URL")));
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
fn rejects_nested_github_release_tag_url() {
    let errors = check_text(
        r#"
- Version: v0.1.0
- GitHub Release URL: https://github.com/mt4110/drop-squash/releases/tag/v0.1.0/notes
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("GitHub Release URL")));
}

#[test]
fn rejects_github_release_url_without_semver_tag() {
    let errors = check_text(
        r#"
- Version: v0.1.0
- GitHub Release URL: https://github.com/mt4110/drop-squash/releases/tag/v0.1
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("GitHub Release URL")));
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
fn rejects_public_website_urls_outside_canonical_host() {
    let errors = check_text(
        r#"
- Public website URL: https://other.example/release-status
- Refund policy URL: https://other.example/refund
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Public website URL")));
    assert!(errors
        .iter()
        .any(|error| error.contains("Refund policy URL")));
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
fn rejects_checkout_url_with_extra_path_after_buy_id() {
    let errors = check_text(
        r#"
- Live checkout URL: https://store.lemonsqueezy.com/checkout/buy/abc123/extra
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Live checkout URL")));
}

#[test]
fn rejects_checkout_url_with_prefixed_checkout_path() {
    let errors = check_text(
        r#"
- Live checkout URL: https://store.lemonsqueezy.com/store/checkout/buy/abc123
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
fn accepts_refund_policy_url_on_same_origin_with_host_case_difference() {
    let errors = check_text(
        r#"
- Public website URL: https://DropSquash.app/release-status
- Refund policy URL: https://dropsquash.app/refund
"#,
    );

    assert!(!errors
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
fn rejects_placeholder_version() {
    let errors = check_text(
        r#"
- Version: v0.0.0
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Version must be a concrete semver version")));
}

#[test]
fn rejects_short_success_release_evidence() {
    let errors = check_text(
        r#"
- Known limitations: Looks good
- Support contact: Success
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Known limitations")));
    assert!(errors.iter().any(|error| error.contains("Support contact")));
}

#[test]
fn rejects_release_notes_manual_qa_record_without_public_dmg() {
    let errors = check_text(
        r#"
- Manual QA record: docs/manual-qa.md filled for DropSquash.app
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Manual QA record")));
}

#[test]
fn rejects_release_notes_manual_qa_record_without_manual_check() {
    let errors = check_text(
        r#"
- Manual QA record: docs/manual-qa.md filled for DropSquash.dmg
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Manual QA record")));
}

#[test]
fn rejects_release_notes_manual_qa_record_without_public_context() {
    let errors = check_text(
        r#"
- Manual QA record: docs/manual-qa.md filled for DropSquash.dmg and manual-qa-check passed
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Manual QA record")));
}

#[test]
fn rejects_release_notes_manual_qa_record_without_tested_context() {
    let errors = check_text(
        r#"
- Manual QA record: docs/manual-qa.md filled for public DropSquash.dmg and manual-qa-check passed
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Manual QA record")));
}

#[test]
fn rejects_release_evidence_with_embedded_placeholders() {
    let errors = check_text(
        r#"
- Known limitations: macOS MVP only; Windows and Linux platform builds TODO
- Support contact: support through GitHub Issues; email TBD
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Known limitations")));
    assert!(errors.iter().any(|error| error.contains("Support contact")));
}

#[test]
fn rejects_release_evidence_with_local_only_urls() {
    let errors = check_text(
        r#"
- Known limitations: macOS MVP only; Windows and Linux platform builds remain unreleased; status is tracked at https://192.168.0.10/release-status
- Support contact: support through GitHub Issues and https://dropsquash.local/support
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Known limitations")));
    assert!(errors.iter().any(|error| error.contains("Support contact")));
}

#[test]
fn rejects_release_evidence_with_non_https_urls() {
    let errors = check_text(
        r#"
- Known limitations: macOS MVP only; Windows and Linux platform builds remain unreleased; status is tracked at http://dropsquash.app/release-status
- Support contact: support through GitHub Issues and http://dropsquash.app/support
"#,
    );

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
fn rejects_sandbox_purchase_without_sandbox_context() {
    let errors = check_text(
        r#"
- Lemon Squeezy sandbox purchase: intended product checkout completed for test buyer order abc123
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
        .any(|error| error.contains("CSV path outside repo")));
    assert!(errors.iter().any(|error| error.contains("baseline")));
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
- Benchmark sample set: short medium large local recordings produced smaller outputs with CSV saved outside repo at /tmp/dropsquash-bench/results.csv
- Benchmark regression threshold: no sample exceeded 20% regression against the same-machine release candidate baseline
- Public website URL: https://dropsquash.app/release-status
- Live checkout URL: https://store.lemonsqueezy.com/checkout/buy/abc123
- GitHub Release URL: https://github.com/mt4110/drop-squash/releases/tag/v0.1.0
- Homebrew tap PR URL: https://github.com/mt4110/homebrew-tap/pull/1
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("CSV path outside repo, machine, and OS context")));
}

#[test]
fn rejects_benchmark_sample_set_without_smaller_outputs() {
    let errors = check_text(
        r#"
- Benchmark sample set: short medium large local recordings recorded on MacBookPro18,4 macOS 26.5.2 with CSV saved outside repo at /tmp/dropsquash-bench/results.csv
- Benchmark regression threshold: no sample exceeded 20% regression against the same-machine release candidate baseline
"#,
    );

    assert!(errors.iter().any(|error| error.contains("smaller outputs")));
}

#[test]
fn rejects_benchmark_sample_set_without_backend() {
    let errors = check_text(
        r#"
- Benchmark sample set: short medium large local recordings produced smaller outputs on MacBookPro18,4 macOS 26.5.2 with CSV saved outside repo at /tmp/dropsquash-bench/results.csv
- Benchmark regression threshold: no sample exceeded 20% regression against the same-machine release candidate baseline
"#,
    );

    assert!(errors.iter().any(|error| error.contains("backend")));
}

#[test]
fn rejects_benchmark_evidence_with_placeholder_notes() {
    let errors = check_text(
        r#"
- Benchmark sample set: short medium large smaller machine macOS TODO
- Benchmark regression threshold: no sample exceeded 20% TBD
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Benchmark sample set")));
    assert!(errors
        .iter()
        .any(|error| error.contains("Benchmark regression threshold")));
}

#[test]
fn rejects_benchmark_sample_set_without_csv_path_context() {
    let errors = check_text(
        r#"
- Benchmark sample set: short medium large local recordings produced smaller outputs on MacBookPro18,4 macOS 26.5.2
- Benchmark regression threshold: no sample exceeded 20% regression against the same-machine release candidate baseline
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("CSV path outside repo")));
}

#[test]
fn rejects_benchmark_sample_set_with_repo_local_csv_path() {
    let csv = std::env::current_dir()
        .unwrap()
        .join("target/dropsquash-bench/results.csv");
    let errors = check_text(&format!(
        "\
- Benchmark sample set: short medium large local recordings produced smaller outputs with backend apple-native, saved percent, duration, and speed ratio on MacBookPro18,4 macOS 26.5.2 with CSV saved outside repo at {}
- Benchmark regression threshold: no sample exceeded 20% regression against the same-machine release candidate baseline
",
        csv.display()
    ));

    assert!(errors
        .iter()
        .any(|error| error.contains("CSV path outside repo")));
}

#[test]
fn rejects_benchmark_sample_set_with_normalized_repo_local_csv_path() {
    let csv = normalized_repo_path("target/dropsquash-bench/results.csv");
    let errors = check_text(&format!(
        "\
- Benchmark sample set: short medium large local recordings produced smaller outputs with backend apple-native, saved percent, duration, and speed ratio on MacBookPro18,4 macOS 26.5.2 with CSV saved outside repo at {}
- Benchmark regression threshold: no sample exceeded 20% regression against the same-machine release candidate baseline
",
        csv.display()
    ));

    assert!(errors
        .iter()
        .any(|error| error.contains("CSV path outside repo")));
}

#[test]
fn rejects_benchmark_sample_set_with_missing_csv_file() {
    let directory = tempfile::tempdir().unwrap();
    let csv = directory.path().join("missing.csv");
    let errors = check_text(&format!(
        "\
- Benchmark sample set: short medium large local recordings produced smaller outputs with backend apple-native, saved percent, duration, and speed ratio on MacBookPro18,4 macOS 26.5.2 with CSV saved outside repo at {}
- Benchmark regression threshold: no sample exceeded 20% regression against the same-machine release candidate baseline
",
        csv.display()
    ));

    assert!(errors
        .iter()
        .any(|error| error.contains("CSV path outside repo")));
}

#[test]
fn accepts_benchmark_sample_set_with_labeled_csv_path() {
    let directory = tempfile::tempdir().unwrap();
    let csv = csv_file(directory.path(), "results.csv");
    let errors = check_text(&format!(
        "\
- Benchmark sample set: short medium large local recordings produced smaller outputs with backend apple-native, saved percent, duration, and speed ratio on MacBookPro18,4 macOS 26.5.2 with CSV saved outside repo at csv={}
- Benchmark regression threshold: no sample exceeded 20% regression against the same-machine release candidate baseline
",
        csv.display()
    ));

    assert!(!errors
        .iter()
        .any(|error| error.contains("Benchmark sample set")));
}

#[test]
fn rejects_benchmark_threshold_without_baseline_context() {
    let errors = check_text(
        r#"
- Benchmark sample set: short medium large local recordings produced smaller outputs with backend apple-native, saved percent, duration, and speed ratio on MacBookPro18,4 macOS 26.5.2 with CSV saved outside repo at /tmp/dropsquash-bench/results.csv
- Benchmark regression threshold: no sample exceeded 20% regression
"#,
    );

    assert!(errors.iter().any(|error| error.contains("baseline")));
}

#[test]
fn rejects_benchmark_threshold_without_release_candidate_context() {
    let errors = check_text(
        r#"
- Benchmark sample set: short medium large local recordings produced smaller outputs with backend apple-native, saved percent, duration, and speed ratio on MacBookPro18,4 macOS 26.5.2 with CSV saved outside repo at /tmp/dropsquash-bench/results.csv
- Benchmark regression threshold: no sample exceeded 20% regression against the same-machine baseline
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("release candidate baseline")));
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
fn rejects_uppercase_git_commit() {
    let errors = check_text(
        r#"
- Git commit: ABC1234
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("lowercase commit hash")));
}

#[test]
fn rejects_placeholder_git_commit() {
    let errors = check_text(
        r#"
- Git commit: 0000000
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("lowercase commit hash")));
}

#[test]
fn rejects_uppercase_sha256() {
    let errors = check_text(
        r#"
- Artifact URL: https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg
- Version: v0.1.0
- Artifact: DropSquash.dmg
- SHA-256: ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789
- Git commit: abc1234
- Public website URL: https://dropsquash.app/release-status
- Live checkout URL: https://store.lemonsqueezy.com/checkout/buy/abc123
- GitHub Release URL: https://github.com/mt4110/drop-squash/releases/tag/v0.1.0
- Homebrew tap PR URL: https://github.com/mt4110/homebrew-tap/pull/1
"#,
    );

    assert!(errors.iter().any(|error| error.contains("lowercase")));
}

#[test]
fn rejects_placeholder_sha256() {
    let errors = check_text(
        r#"
- Artifact URL: https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg
- Version: v0.1.0
- Artifact: DropSquash.dmg
- SHA-256: 0000000000000000000000000000000000000000000000000000000000000000
- Git commit: abc1234
- Public website URL: https://dropsquash.app/release-status
- Live checkout URL: https://store.lemonsqueezy.com/checkout/buy/abc123
- GitHub Release URL: https://github.com/mt4110/drop-squash/releases/tag/v0.1.0
- Homebrew tap PR URL: https://github.com/mt4110/homebrew-tap/pull/1
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("lowercase 64-character hex checksum")));
}

#[test]
fn rejects_secret_like_release_note_values() {
    let errors = check_text(
        r#"
- Apple notary log: accepted with APPLE_PASSWORD=not-for-release
- Lemon Squeezy product setup: DropSquash intended product has license keys enabled; product_id=123 and license key: raw-test-key
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("secret-like value")));
}

#[test]
fn rejects_release_checksum_evidence_without_matching_digest() {
    let errors = check_text(
        r#"
- SHA-256: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
- GitHub Release checksum: SHA256SUMS attached to release for DropSquash.dmg
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("GitHub Release checksum must include")));
}

#[test]
fn rejects_release_checksum_evidence_without_artifact_url() {
    let errors = check_text(
        r#"
- Artifact URL: https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg
- SHA-256: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
- GitHub Release checksum: SHA256SUMS attached to release for DropSquash.dmg with lowercase SHA-256 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("GitHub Release checksum must include the Artifact URL")));
}

#[test]
fn rejects_release_checksum_evidence_without_github_release_url() {
    let errors = check_text(
        r#"
- Artifact URL: https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg
- GitHub Release URL: https://github.com/mt4110/drop-squash/releases/tag/v0.1.0
- SHA-256: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
- GitHub Release checksum: SHA256SUMS attached to release for https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg with lowercase SHA-256 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
"#,
    );

    assert!(errors.iter().any(|error| {
        error.contains("GitHub Release checksum must include the GitHub Release URL")
    }));
}

#[test]
fn rejects_prepared_pending_release_checksum() {
    let errors = check_text(
        r#"
- Artifact URL: https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg
- SHA-256: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
- GitHub Release checksum: pending upload; after attaching SHA256SUMS to public https://github.com/mt4110/drop-squash/releases/tag/v0.1.0 for public https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg with lowercase SHA-256 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef, replace this line with public release evidence
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("GitHub Release checksum")));
}

#[test]
fn rejects_prepared_draft_marker() {
    let errors = check_text(
        r#"
Prepared draft only. Replace every pending line before public release.
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("prepared draft markers")));
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

#[test]
fn rejects_artifact_and_github_release_url_version_mismatch() {
    let errors = check_text(
        r#"
- Artifact URL: https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg
- GitHub Release URL: https://github.com/mt4110/drop-squash/releases/tag/v0.2.0
"#,
    );

    assert!(errors
        .iter()
        .any(|error| error.contains("Artifact URL must match GitHub Release URL version")));
}

#[test]
fn rejects_noncanonical_artifact_name() {
    let errors = check_text(
        r#"
- Artifact URL: https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash-beta.dmg
- Version: v0.1.0
- Artifact: DropSquash-beta.dmg
- SHA-256: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
- Git commit: abc1234
- Public website URL: https://dropsquash.app/release-status
- Live checkout URL: https://store.lemonsqueezy.com/checkout/buy/abc123
- GitHub Release URL: https://github.com/mt4110/drop-squash/releases/tag/v0.1.0
- Homebrew tap PR URL: https://github.com/mt4110/homebrew-tap/pull/1
"#,
    );

    assert!(errors.iter().any(|error| error.contains("Artifact")));
}

fn csv_file(directory: &std::path::Path, name: &str) -> std::path::PathBuf {
    let path = directory.join(name);
    std::fs::write(&path, "sample,duration_ms\nshort,100\n").unwrap();
    path
}

fn normalized_repo_path(child: &str) -> std::path::PathBuf {
    let cwd = std::env::current_dir().unwrap();
    cwd.parent()
        .unwrap()
        .join("outside")
        .join("..")
        .join(cwd.file_name().unwrap())
        .join(child)
}
