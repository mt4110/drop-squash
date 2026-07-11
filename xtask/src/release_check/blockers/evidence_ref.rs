pub(super) fn is_evidence_reference(value: &str) -> bool {
    !value.is_empty()
        && value != "TBD"
        && (value.starts_with("`docs/")
            || value.starts_with("https://")
            || matches!(
                value,
                "Release notes" | "GitHub Release" | "Homebrew tap PR"
            ))
}
