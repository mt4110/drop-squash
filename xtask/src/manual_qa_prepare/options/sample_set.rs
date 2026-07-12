pub(super) fn is_valid(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    has_all_sizes(&lower) && mentions_recording_or_sample(&lower)
}

fn has_all_sizes(value: &str) -> bool {
    ["short", "medium", "large"]
        .iter()
        .all(|needle| value.contains(needle))
}

fn mentions_recording_or_sample(value: &str) -> bool {
    value.contains("recording") || value.contains("sample")
}
