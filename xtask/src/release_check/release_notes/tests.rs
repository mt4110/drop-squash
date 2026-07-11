use super::missing_text;

#[test]
fn accepts_required_release_note_evidence_fields() {
    let text = r#"
Artifact
SHA-256
codesign
spctl
stapler
notary
Gatekeeper
Homebrew
docs/release-blockers.md
"#;

    assert!(missing_text(text).is_empty());
}

#[test]
fn reports_missing_release_note_evidence_fields() {
    let missing = missing_text("");

    assert!(missing.contains(&"codesign"));
    assert!(missing.contains(&"SHA-256"));
    assert!(missing.contains(&"Homebrew"));
}
