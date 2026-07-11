use super::check_text;

#[test]
fn accepts_concrete_production_urls() {
    let errors = check_text(
        r#"
- Artifact URL: https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg
- Public website URL: https://dropsquash.app
- Live checkout URL: https://store.lemonsqueezy.com/checkout/buy/abc123
- GitHub Release URL: https://github.com/mt4110/drop-squash/releases/tag/v0.1.0
- Homebrew tap PR URL: https://github.com/mt4110/homebrew-tap/pull/1
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
