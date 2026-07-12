pub(super) fn lacks_required_evidence(check: &str, result: &str) -> bool {
    let Some(groups) = groups_for(check) else {
        return false;
    };
    let lower = result.to_ascii_lowercase();
    !groups
        .iter()
        .all(|group| group.iter().any(|needle| lower.contains(needle)))
        || lacks_special_evidence(check, result)
}

fn lacks_special_evidence(check: &str, result: &str) -> bool {
    match check {
        "App build" => !has_version_and_commit(result),
        "App artifact" => !has_canonical_artifact(result),
        "Date" => !is_iso_date(result),
        _ => false,
    }
}

fn has_version_and_commit(result: &str) -> bool {
    let parts = result.split_whitespace().collect::<Vec<_>>();
    parts.iter().any(|part| is_semver(part))
        && parts.iter().any(|part| {
            (7..=40).contains(&part.len()) && part.chars().all(|value| value.is_ascii_hexdigit())
        })
}

fn is_semver(value: &str) -> bool {
    let parts = value.split('.').collect::<Vec<_>>();
    parts.len() == 3
        && parts
            .iter()
            .all(|part| !part.is_empty() && part.chars().all(|value| value.is_ascii_digit()))
}

fn is_iso_date(value: &str) -> bool {
    let value = value.trim();
    let bytes = value.as_bytes();
    bytes.len() == 10
        && bytes.get(4) == Some(&b'-')
        && bytes.get(7) == Some(&b'-')
        && bytes
            .iter()
            .enumerate()
            .all(|(index, value)| matches!(index, 4 | 7) || value.is_ascii_digit())
}

fn has_canonical_artifact(result: &str) -> bool {
    result.contains("DropSquash.app") || result.contains("DropSquash.dmg")
}

fn groups_for(check: &str) -> Option<&'static [&'static [&'static str]]> {
    match check {
        "App build" => Some(&[&["dropsquash"], &["git"]]),
        "App artifact" => Some(&[&["dropsquash"], &[".app", ".dmg"]]),
        "macOS version" => Some(&[&["macos"]]),
        "Machine" => Some(&[&["arm64", "x86_64", "apple", "intel"]]),
        "Input sample set" => Some(&[&["short"], &["medium"], &["large"]]),
        "Output folder" => Some(&[&["/"], &["output"]]),
        "Tester" => Some(&[&["masaki", "tester"]]),
        "Date" => Some(&[&["20"]]),
        _ => None,
    }
}
