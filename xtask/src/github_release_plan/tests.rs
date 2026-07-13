use std::io::Write;

use super::{command, Request};

const SHA256: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

#[test]
fn prints_gh_release_create_command() {
    let directory = tempfile::tempdir().unwrap();
    let request = request(directory.path(), "v1.2.3");

    let text = command(&request).unwrap();

    assert!(text.starts_with("gh release create v1.2.3 "));
    assert!(text.contains("DropSquash.dmg"));
    assert!(text.contains("SHA256SUMS"));
    assert!(text.contains("--title v1.2.3"));
    assert!(text.contains("--notes-file"));
}

#[test]
fn rejects_non_release_tag() {
    let directory = tempfile::tempdir().unwrap();
    let error = command(&request(directory.path(), "1.2.3")).unwrap_err();

    assert!(error.contains("v1.2.3"));
}

#[test]
fn rejects_malformed_release_tag() {
    let directory = tempfile::tempdir().unwrap();
    let error = command(&request(directory.path(), "v1.two.3")).unwrap_err();

    assert!(error.contains("v1.2.3"));
}

#[test]
fn rejects_wrong_checksum_name() {
    let directory = tempfile::tempdir().unwrap();
    let mut request = request(directory.path(), "v1.2.3");
    request.checksum = directory.path().join("checksums.txt");
    write_file(&request.checksum, "abc  DropSquash.dmg");

    let error = command(&request).unwrap_err();

    assert!(error.contains("SHA256SUMS"));
}

#[test]
fn rejects_checksum_without_dmg_line() {
    let directory = tempfile::tempdir().unwrap();
    let request = request(directory.path(), "v1.2.3");
    write_file(&request.checksum, "abc  Other.dmg");

    let error = command(&request).unwrap_err();

    assert!(error.contains("DropSquash.dmg"));
}

#[test]
fn rejects_uppercase_checksum_digest() {
    let directory = tempfile::tempdir().unwrap();
    let request = request(directory.path(), "v1.2.3");
    write_file(
        &request.checksum,
        "ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789  DropSquash.dmg",
    );

    let error = command(&request).unwrap_err();

    assert!(error.contains("lowercase SHA-256"));
}

#[test]
fn rejects_short_checksum_digest() {
    let directory = tempfile::tempdir().unwrap();
    let request = request(directory.path(), "v1.2.3");
    write_file(&request.checksum, "abc  DropSquash.dmg");

    let error = command(&request).unwrap_err();

    assert!(error.contains("lowercase SHA-256"));
}

#[test]
fn rejects_checksum_with_nix_store_reference() {
    let directory = tempfile::tempdir().unwrap();
    let request = request(directory.path(), "v1.2.3");
    write_file(
        &request.checksum,
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef  DropSquash.dmg\n/nix/store/abc",
    );

    let error = command(&request).unwrap_err();

    assert!(error.contains("/nix/store"));
}

#[test]
fn rejects_missing_notes_file() {
    let directory = tempfile::tempdir().unwrap();
    let mut request = request(directory.path(), "v1.2.3");
    request.notes = directory.path().join("missing.md");

    let error = command(&request).unwrap_err();

    assert!(error.contains(".md"));
}

#[test]
fn rejects_incomplete_release_notes() {
    let directory = tempfile::tempdir().unwrap();
    let request = request(directory.path(), "v1.2.3");
    write_file(&request.notes, "# Release\n");

    let error = command(&request).unwrap_err();

    assert!(error.contains("release-notes-check"));
}

fn request(directory: &std::path::Path, tag: &str) -> Request {
    let dmg = directory.join("DropSquash.dmg");
    let checksum = directory.join("SHA256SUMS");
    let notes = directory.join("release-notes.md");
    write_dmg(&dmg);
    write_file(&checksum, &format!("{SHA256}  DropSquash.dmg"));
    write_file(&notes, &release_notes(directory));
    Request {
        tag: tag.to_string(),
        dmg,
        checksum,
        notes,
    }
}

fn release_notes(directory: &std::path::Path) -> String {
    let csv = directory.join("benchmark.csv");
    write_file(
        &csv,
        "sample,backend,saved_percent,duration_seconds,speed_ratio\n",
    );
    format!(
        r#"
- Artifact URL: https://github.com/mt4110/drop-squash/releases/download/v1.2.3/DropSquash.dmg
- Version: v1.2.3
- Artifact: DropSquash.dmg
- SHA-256: {SHA256}
- Git commit: abc1234
- `codesign`: codesign --verify passed and codesign -dv showed Developer ID Application signature for public https://github.com/mt4110/drop-squash/releases/download/v1.2.3/DropSquash.dmg
- `spctl`: spctl --assess --type open accepted Developer ID source for public https://github.com/mt4110/drop-squash/releases/download/v1.2.3/DropSquash.dmg
- `stapler`: stapler validate showed ticket stapled successfully for public https://github.com/mt4110/drop-squash/releases/download/v1.2.3/DropSquash.dmg
- Apple notary log: notarytool accepted log request abc123 for public https://github.com/mt4110/drop-squash/releases/download/v1.2.3/DropSquash.dmg
- Gatekeeper clean-machine open: Gatekeeper opened signed, notarized, stapled app from public https://github.com/mt4110/drop-squash/releases/download/v1.2.3/DropSquash.dmg cleanly in fresh account without Gatekeeper warning
- `docs/release-blockers.md` status: docs/release-blockers.md has all rows Verified
- Manual QA record: docs/manual-qa.md tested public https://github.com/mt4110/drop-squash/releases/download/v1.2.3/DropSquash.dmg and manual-qa-check passed
- Conversion safety evidence: cancellation returned ready after temp cleanup; failed conversion and larger output not smaller failure preserved original with trial count unchanged; history showed no new success
- Queue evidence: multi-file queue, queued row cancellation, and batch summary showed trial lock blocked pending jobs with finished count 2, saved bytes 123456, failed 0, cancelled 1, and blocked 0
- Trash source policy: Moving original state disabled action; original moved to Trash only after verified smaller output
- Benchmark sample set: three short medium large local recordings produced smaller outputs with backend apple-native, saved percent, duration, and speed ratio on MacBookPro18,4 macOS 26.5.2 with CSV saved outside repo at {}
- Benchmark regression threshold: no sample exceeded 20 percent regression against the same-machine release candidate baseline
- Lemon Squeezy product setup: DropSquash sandbox intended product has license keys enabled and private store IDs not recorded
- Lemon Squeezy sandbox purchase: sandbox checkout completed for intended product test buyer order abc123
- Valid sandbox activation: Lemon Squeezy sandbox activation request entered Activating state, disabled submit, reached Pro state, and checked cache kept fingerprint 1111111111111111111111111111111111111111111111111111111111111111 plus instance_id field with raw key absent
- Empty key activation: Activate disabled for empty input and checked cache showed raw key absent with no fingerprint and no instance
- Invalid license key handling: Activating state disabled submit; friendly error shown and inspected cache showed raw key absent with no fingerprint and no instance
- License network failure: friendly network error shown, checked existing valid cache preserved fingerprint 1111111111111111111111111111111111111111111111111111111111111111 plus instance_id field with raw key absent
- Expired license refresh: attempted conversion with expired offline grace license cache showed reconnect prompt, blocked conversion before starting, and checked cache confirmed raw key absent
- Local license forget: Forgetting state disabled action; confirmed license cache removed and observed trial state restored
- Public website URL: https://dropsquash.app/release-status
- Pricing URL: https://dropsquash.app/pricing
- Refund policy URL: https://dropsquash.app/refund
- Live checkout URL: https://store.lemonsqueezy.com/checkout/buy/abc123
- GitHub Release URL: https://github.com/mt4110/drop-squash/releases/tag/v1.2.3
- GitHub Release checksum: SHA256SUMS attached to public https://github.com/mt4110/drop-squash/releases/tag/v1.2.3 for https://github.com/mt4110/drop-squash/releases/download/v1.2.3/DropSquash.dmg with lowercase SHA-256 {SHA256}
- Homebrew tap PR: cask update reviewed in public PR at https://github.com/mt4110/homebrew-tap/pull/1 for versioned DropSquash.dmg using https://github.com/mt4110/drop-squash/releases/download/v1.2.3/DropSquash.dmg with lowercase SHA-256 {SHA256}, auto_updates false, and zap cleanup path
- Homebrew tap PR URL: https://github.com/mt4110/homebrew-tap/pull/1
- Homebrew install result: brew install --cask mt4110/tap/dropsquash completed from https://github.com/mt4110/homebrew-tap/pull/1 for versioned DropSquash.dmg artifact from https://github.com/mt4110/drop-squash/releases/download/v1.2.3/DropSquash.dmg with lowercase SHA-256 {SHA256} and brew uninstall --cask mt4110/tap/dropsquash removed it cleanly
- Known limitations: macOS MVP only; Windows and Linux platform builds remain unreleased
- Support contact: support handled through GitHub Issues until paid support opens
"#,
        csv.display()
    )
}

fn write_dmg(path: &std::path::Path) {
    let mut bytes = b"signed".to_vec();
    let mut trailer = vec![0; 512];
    trailer[..4].copy_from_slice(b"koly");
    bytes.extend(trailer);
    write_file(path, &String::from_utf8_lossy(&bytes));
}

fn write_file(path: &std::path::Path, text: &str) {
    std::fs::File::create(path)
        .unwrap()
        .write_all(text.as_bytes())
        .unwrap();
}
