use super::missing_text;

#[test]
fn accepts_required_release_note_evidence_fields() {
    let text = r#"
Version
Artifact
Artifact URL
SHA-256
Git commit
codesign
spctl
stapler
Apple notary log
Gatekeeper
docs/release-blockers.md
Manual QA record
Benchmark sample set
Benchmark regression threshold
Lemon Squeezy product setup
Lemon Squeezy sandbox purchase
Lemon Squeezy sandbox activation
Empty key activation
Invalid license key handling
License network failure
Local license forget
Public website URL
Refund policy URL
Live checkout URL
GitHub Release checksum
GitHub Release URL
Homebrew tap PR
Homebrew tap PR URL
Homebrew install result
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
    assert!(missing.contains(&"SHA-256"));
    assert!(missing.contains(&"Artifact URL"));
    assert!(missing.contains(&"Benchmark sample set"));
    assert!(missing.contains(&"Lemon Squeezy product setup"));
    assert!(missing.contains(&"GitHub Release URL"));
    assert!(missing.contains(&"Refund policy URL"));
    assert!(missing.contains(&"Live checkout URL"));
    assert!(missing.contains(&"Support contact"));
}
