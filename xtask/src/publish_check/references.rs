const PAIRS: [(&str, &str); 5] = [
    ("Public website deployment", "Public website URL"),
    ("Refund policy finalized", "Refund policy URL"),
    ("Live checkout link", "Live checkout URL"),
    ("Published checksum", "GitHub Release URL"),
    ("Homebrew cask install", "Homebrew tap PR URL"),
];

pub(super) fn mismatched(blockers: &str, notes: &str) -> Vec<&'static str> {
    PAIRS
        .iter()
        .filter_map(|(blocker, field)| mismatch(blockers, notes, blocker, field))
        .collect()
}

fn mismatch(
    blockers: &str,
    notes: &str,
    blocker: &'static str,
    field: &str,
) -> Option<&'static str> {
    let reference = verified_reference(blockers, blocker)?;
    let expected = field_value(notes, field)?;
    (!reference.contains(expected)).then_some(blocker)
}

fn verified_reference<'a>(text: &'a str, blocker: &str) -> Option<&'a str> {
    text.lines().find_map(|line| {
        let cells = cells(line)?;
        (cells.first() == Some(&blocker) && cells.get(1) == Some(&"Verified"))
            .then(|| cells.get(3).copied())
            .flatten()
    })
}

fn field_value<'a>(text: &'a str, label: &str) -> Option<&'a str> {
    let prefix = format!("- {label}:");
    text.lines()
        .find_map(|line| line.trim().strip_prefix(&prefix).map(str::trim))
}

fn cells(line: &str) -> Option<Vec<&str>> {
    if !line.starts_with('|') {
        return None;
    }
    let cells = line
        .trim_matches('|')
        .split('|')
        .map(str::trim)
        .collect::<Vec<_>>();
    (cells.len() == 5).then_some(cells)
}

#[cfg(test)]
mod tests;
