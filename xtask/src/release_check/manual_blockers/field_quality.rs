pub(super) fn lacks_required_evidence(check: &str, result: &str) -> bool {
    let Some(groups) = groups_for(check) else {
        return false;
    };
    let lower = result.to_ascii_lowercase();
    !groups
        .iter()
        .all(|group| group.iter().any(|needle| lower.contains(needle)))
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
