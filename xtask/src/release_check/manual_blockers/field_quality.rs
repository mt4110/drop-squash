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
        "App build" => !has_version_and_current_commit(result),
        "App artifact" => !has_canonical_artifact(result),
        "Config path" => !is_absolute_state_path(result, "config.json"),
        "History path" => !is_absolute_state_path(result, "history.jsonl"),
        "License cache path" => !is_absolute_state_path(result, "license.json"),
        "Date" => !is_iso_date(result),
        _ => false,
    }
}

fn has_version_and_current_commit(result: &str) -> bool {
    let parts = result.split_whitespace().collect::<Vec<_>>();
    parts.iter().any(|part| is_semver(part))
        && parts.iter().any(|part| {
            (7..=40).contains(&part.len()) && part.chars().all(|value| value.is_ascii_hexdigit())
        })
        && crate::git_head_match::contains_current_short_head_after_git(result).unwrap_or(false)
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
    if !has_iso_shape(bytes) {
        return false;
    }
    let year = value[..4].parse::<u16>().unwrap_or(0);
    let month = value[5..7].parse::<u8>().unwrap_or(0);
    let day = value[8..].parse::<u8>().unwrap_or(0);
    year >= 2000 && (1..=days_in_month(year, month)).contains(&day)
}

fn has_iso_shape(bytes: &[u8]) -> bool {
    bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes[..4].iter().all(u8::is_ascii_digit)
        && bytes[5..7].iter().all(u8::is_ascii_digit)
        && bytes[8..].iter().all(u8::is_ascii_digit)
}

fn days_in_month(year: u16, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn is_leap_year(year: u16) -> bool {
    year % 4 == 0 && year % 100 != 0 || year % 400 == 0
}

fn has_canonical_artifact(result: &str) -> bool {
    result.contains("DropSquash.app") || result.contains("DropSquash.dmg")
}

fn is_absolute_state_path(result: &str, file_name: &str) -> bool {
    let result = result.trim().to_ascii_lowercase();
    (result.starts_with('/') || result.as_bytes().get(1) == Some(&b':'))
        && (result.contains("application support/dropsquash/")
            || result.contains("application support\\dropsquash\\"))
        && result.ends_with(file_name)
}

fn groups_for(check: &str) -> Option<&'static [&'static [&'static str]]> {
    match check {
        "App build" => Some(&[&["dropsquash"], &["git"]]),
        "App artifact" => Some(&[&["dropsquash"], &[".app", ".dmg"]]),
        "macOS version" => Some(&[&["macos"]]),
        "Machine" => Some(&[&["arm64", "x86_64", "apple", "intel"]]),
        "Input sample set" => Some(&[&["short"], &["medium"], &["large"]]),
        "Output folder" => Some(&[&["/"], &["output"]]),
        "Config path" => Some(&[&["config.json"]]),
        "History path" => Some(&[&["history.jsonl"]]),
        "License cache path" => Some(&[&["license.json"]]),
        "Tester" => Some(&[&["masaki", "tester"]]),
        "Date" => Some(&[&["20"]]),
        _ => None,
    }
}
