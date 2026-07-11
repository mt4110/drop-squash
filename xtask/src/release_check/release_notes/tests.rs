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
Lemon Squeezy sandbox purchase
Lemon Squeezy sandbox activation
Public website URL
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
"#;

    assert!(missing_text(text).is_empty());
}

#[test]
fn reports_missing_release_note_evidence_fields() {
    let missing = missing_text("");

    assert!(missing.contains(&"codesign"));
    assert!(missing.contains(&"SHA-256"));
    assert!(missing.contains(&"Artifact URL"));
    assert!(missing.contains(&"Benchmark sample set"));
    assert!(missing.contains(&"GitHub Release URL"));
    assert!(missing.contains(&"Live checkout URL"));
    assert!(missing.contains(&"Support contact"));
}
