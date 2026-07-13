use super::REQUIRED_BLOCKERS;

const TRACKS: &[(&str, &[&str])] = &[
    (
        "Local packaged-app proof",
        &["Packaged macOS manual QA", "Benchmark release set"],
    ),
    (
        "License sandbox proof",
        &[
            "Lemon Squeezy product setup",
            "Lemon Squeezy sandbox purchase",
            "Empty key activation",
            "Valid sandbox activation",
            "Invalid license key handling",
            "License network failure",
            "Expired license refresh",
            "Local license forget",
        ],
    ),
    (
        "Public web proof",
        &[
            "Public website deployment",
            "Refund policy finalized",
            "Live checkout link",
        ],
    ),
    (
        "Signing and distribution proof",
        &[
            "Signed DMG",
            "Notarized and stapled DMG",
            "Gatekeeper clean-machine open",
            "Published checksum",
            "Homebrew cask install",
        ],
    ),
];

pub(super) fn unplanned_blockers(text: &str) -> Vec<&'static str> {
    REQUIRED_BLOCKERS
        .iter()
        .copied()
        .filter(|blocker| {
            expected_track(blocker).is_some_and(|track| !row_contains_blocker(text, track, blocker))
        })
        .collect()
}

pub(super) fn unknown_blockers(text: &str) -> Vec<&str> {
    text.lines()
        .filter_map(execution_row)
        .flat_map(|(_, blockers)| split_blockers(blockers))
        .filter(|blocker| !REQUIRED_BLOCKERS.contains(blocker))
        .collect()
}

fn row_contains_blocker(text: &str, track: &str, blocker: &str) -> bool {
    text.lines()
        .filter_map(execution_row)
        .any(|(row_track, blockers)| blockers_track(row_track, blockers, track, blocker))
}

fn blockers_track(row_track: &str, blockers: &str, track: &str, blocker: &str) -> bool {
    row_track == track && split_blockers(blockers).any(|value| value == blocker)
}

fn expected_track(blocker: &str) -> Option<&'static str> {
    TRACKS
        .iter()
        .find_map(|(track, blockers)| blockers.contains(&blocker).then_some(*track))
}

fn split_blockers(value: &str) -> impl Iterator<Item = &str> {
    value
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn execution_row(line: &str) -> Option<(&str, &str)> {
    let cells = cells(line)?;
    is_order(cells[0]).then_some((cells[1], cells[2]))
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

fn is_order(value: &str) -> bool {
    value.chars().all(|character| character.is_ascii_digit()) && !value.is_empty()
}

#[cfg(test)]
mod tests;
