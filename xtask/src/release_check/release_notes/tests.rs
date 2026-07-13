use super::missing_text;

#[test]
fn accepts_required_release_note_evidence_fields() {
    let text = r#"
Version
Artifact
Artifact: DropSquash.dmg
Artifact URL
SHA-256
Git commit
release-notes-prepare
codesign --verify --deep --strict --verbose=2
spctl --assess --type open --verbose=4
xcrun stapler validate
codesign
spctl
stapler
confirm stapled status
Apple notary log
Gatekeeper
docs/release-blockers.md
Manual QA record
tested exact Artifact URL
Conversion safety evidence
Queue evidence
Trash source policy
Benchmark sample set
Benchmark regression threshold
Benchmark regression threshold: mention
20% regression threshold
Lemon Squeezy product setup
Lemon Squeezy sandbox purchase
Valid sandbox activation
Empty key activation: mention
Empty key activation
Invalid license key handling: mention
Invalid license key handling
License network failure
Expired license refresh
Local license forget
Public website URL
Pricing URL
Refund policy URL
Live checkout URL
store.lemonsqueezy.com/checkout/buy
GitHub Release checksum
attached to the GitHub Release
the Artifact URL above
GitHub Release URL
Homebrew tap PR
Homebrew tap PR URL
Homebrew install result
brew uninstall --cask mt4110/tap/dropsquash
removes it cleanly
the versioned `DropSquash.dmg` artifact, the Artifact URL above
Known limitations
Support contact
Do not paste signing secrets
license keys
Homebrew
Evidence Wording Checklist
all rows Verified
SHA256SUMS
"#;

    assert!(missing_text(text).is_empty());
}

#[test]
fn release_notes_template_contains_required_fields() {
    let text = std::fs::read_to_string("../docs/release-notes-template.md").unwrap();

    assert!(missing_text(&text).is_empty());
}

#[test]
fn reports_missing_release_note_evidence_fields() {
    let missing = missing_text("");

    assert!(missing.contains(&"codesign"));
    assert!(missing.contains(&"release-notes-prepare"));
    assert!(missing.contains(&"xcrun stapler validate"));
    assert!(missing.contains(&"SHA-256"));
    assert!(missing.contains(&"Artifact URL"));
    assert!(missing.contains(&"tested exact Artifact URL"));
    assert!(missing.contains(&"Benchmark sample set"));
    assert!(missing.contains(&"Lemon Squeezy product setup"));
    assert!(missing.contains(&"GitHub Release URL"));
    assert!(missing.contains(&"attached to the GitHub Release"));
    assert!(missing.contains(&"Refund policy URL"));
    assert!(missing.contains(&"removes it cleanly"));
    assert!(missing.contains(&"Live checkout URL"));
    assert!(missing.contains(&"Support contact"));
}
