mod groups;

pub(super) fn lacks_required_evidence(label: &str, value: &str) -> bool {
    let Some(groups) = groups::for_label(label) else {
        return super::homebrew::lacks_required_evidence(label, value).unwrap_or(false);
    };
    let lower = value.to_ascii_lowercase();
    !groups
        .iter()
        .all(|group| group.iter().any(|needle| lower.contains(needle)))
}
