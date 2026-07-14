pub(super) fn validate(text: &str) -> Vec<String> {
    crate::dmg_cleanup_claims::matches(text)
        .into_iter()
        .map(|phrase| format!("release notes contain unsupported DMG cleanup claim: {phrase}"))
        .collect()
}
