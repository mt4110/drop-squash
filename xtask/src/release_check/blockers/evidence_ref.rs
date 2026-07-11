pub(super) fn is_evidence_reference(value: &str) -> bool {
    !value.is_empty()
        && value != "TBD"
        && !super::placeholders::has_token(value)
        && !has_placeholder_url(value)
        && (value.starts_with("`docs/")
            || value.starts_with("https://")
            || matches!(
                value,
                "Release notes" | "GitHub Release" | "Homebrew tap PR"
            ))
}

fn has_placeholder_url(value: &str) -> bool {
    let value = value.to_ascii_lowercase();
    value.contains("example.")
        || value.contains(".example/")
        || value.contains("localhost")
        || value.contains(".test/")
        || value.ends_with(".test")
}
