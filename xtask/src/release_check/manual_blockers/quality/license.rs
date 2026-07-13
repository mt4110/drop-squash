pub(super) fn has_order_id(result: &str) -> bool {
    result
        .to_ascii_lowercase()
        .split(|value: char| !value.is_ascii_alphanumeric())
        .collect::<Vec<_>>()
        .windows(2)
        .any(|parts| parts[0] == "order" && parts[1].chars().any(|value| value.is_ascii_digit()))
}

pub(super) fn has_raw_key_contradiction(result: &str) -> bool {
    let lower = result.to_ascii_lowercase();
    [
        "raw key persisted",
        "raw key present",
        "raw key stored",
        "raw key written",
        "raw key saved",
        "persisted raw key",
        "stored raw key",
        "saved raw key",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
}
