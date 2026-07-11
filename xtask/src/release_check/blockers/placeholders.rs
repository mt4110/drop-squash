pub(super) fn has_token(value: &str) -> bool {
    value
        .split(|character: char| !character.is_ascii_alphanumeric())
        .any(|token| matches!(token.to_ascii_lowercase().as_str(), "tbd" | "todo"))
}
