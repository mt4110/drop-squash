pub(super) fn distribution_quickstart(text: &str) -> Vec<String> {
    crate::manual_qa_pending::ready::distribution_lines(text)
        .into_iter()
        .filter(|line| prefixes().iter().any(|prefix| line.starts_with(prefix)))
        .enumerate()
        .map(|(index, line)| format!("distribution quickstart {}: {line}", index + 1))
        .collect()
}

pub(super) fn uses_local_unsigned_dmg(text: &str) -> bool {
    text.lines().any(|line| {
        line.starts_with("| App artifact | ") && line.contains("/target/release/bundle/dmg/")
    })
}

fn prefixes() -> [&'static str; 7] {
    [
        "distribution fresh build command:",
        "distribution fresh normalize command:",
        "distribution fresh artifact check:",
        "distribution fresh signing plan:",
        "distribution fresh codesign verify plan:",
        "distribution fresh stapler plan:",
        "distribution fresh spctl plan:",
    ]
}
