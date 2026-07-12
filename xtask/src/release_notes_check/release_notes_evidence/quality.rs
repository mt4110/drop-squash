mod groups;

pub(super) fn lacks_required_evidence(label: &str, value: &str) -> bool {
    let Some(groups) = groups::for_label(label) else {
        return super::homebrew::lacks_required_evidence(label, value).unwrap_or(false);
    };
    let lower = value.to_ascii_lowercase();
    !groups
        .iter()
        .all(|group| group.iter().any(|needle| lower.contains(needle)))
        || lacks_special_evidence(label, value)
}

fn lacks_special_evidence(label: &str, value: &str) -> bool {
    match label {
        "Queue evidence" => count_numbers(value) < 5,
        _ => false,
    }
}

fn count_numbers(value: &str) -> usize {
    value
        .split(|character: char| !character.is_ascii_digit())
        .filter(|part| !part.is_empty())
        .count()
}
