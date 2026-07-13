use super::REQUIRED_BLOCKERS;

mod rows;
mod tracks;

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
    rows::all(text)
        .flat_map(|row| split_blockers(row.blockers))
        .filter(|blocker| !REQUIRED_BLOCKERS.contains(blocker))
        .collect()
}

pub(super) fn duplicate_blockers(text: &str) -> Vec<&'static str> {
    REQUIRED_BLOCKERS
        .iter()
        .copied()
        .filter(|blocker| execution_count(text, blocker) > 1)
        .collect()
}

pub(super) fn misordered_tracks(text: &str) -> Vec<&'static str> {
    tracks::misordered_tracks(text)
}

pub(super) fn misplaced_record_targets(text: &str) -> Vec<&'static str> {
    tracks::misplaced_record_targets(text)
}

fn execution_count(text: &str, blocker: &str) -> usize {
    rows::all(text)
        .flat_map(|row| split_blockers(row.blockers))
        .filter(|value| *value == blocker)
        .count()
}

fn row_contains_blocker(text: &str, track: &str, blocker: &str) -> bool {
    rows::all(text).any(|row| blockers_track(row.track, row.blockers, track, blocker))
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
    rows::split_blockers(value)
}

#[cfg(test)]
mod tests;
