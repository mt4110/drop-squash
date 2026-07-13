use super::rows;

const TRACK_ROWS: &[(&str, &str, &str)] = &[
    ("1", "Local packaged-app proof", "`docs/manual-qa.md`"),
    ("2", "License sandbox proof", "`docs/manual-qa.md`"),
    ("3", "Public web proof", "Production website URLs"),
    (
        "4",
        "Signing and distribution proof",
        "Release notes and public distribution URLs",
    ),
];

const EXIT_PHRASES: &[(&str, &[&str])] = &[
    (
        "Local packaged-app proof",
        &[
            "DropSquash.dmg",
            "manual QA",
            "CSV outside the repo",
            "manual-qa-check",
        ],
    ),
    (
        "License sandbox proof",
        &["Sandbox purchase", "friendly failures", "raw-key absence"],
    ),
    (
        "Public web proof",
        &["dropsquash.app", "checkout", "refund"],
    ),
    (
        "Signing and distribution proof",
        &[
            "Release notes",
            "GitHub Release",
            "Homebrew tap PR",
            "signed",
            "notarized",
        ],
    ),
];

pub(super) fn misordered_tracks(text: &str) -> Vec<&'static str> {
    TRACK_ROWS
        .iter()
        .filter(|(order, track, _)| !has_track_order(text, order, track))
        .map(|(_, track, _)| *track)
        .collect()
}

pub(super) fn misplaced_record_targets(text: &str) -> Vec<&'static str> {
    TRACK_ROWS
        .iter()
        .filter(|(_, track, target)| {
            has_track(text, track) && !has_track_target(text, track, target)
        })
        .map(|(_, track, _)| *track)
        .collect()
}

pub(super) fn duplicate_tracks(text: &str) -> Vec<&'static str> {
    TRACK_ROWS
        .iter()
        .filter(|(_, track, _)| track_count(text, track) > 1)
        .map(|(_, track, _)| *track)
        .collect()
}

pub(super) fn weak_exit_conditions(text: &str) -> Vec<&'static str> {
    EXIT_PHRASES
        .iter()
        .filter(|(track, phrases)| !has_exit_phrases(text, track, phrases))
        .map(|(track, _)| *track)
        .collect()
}

fn has_exit_phrases(text: &str, track: &str, phrases: &[&str]) -> bool {
    rows::all(text).any(|row| {
        row.track == track
            && !has_placeholder(row.exit)
            && phrases.iter().all(|phrase| row.exit.contains(phrase))
    })
}

fn has_placeholder(value: &str) -> bool {
    value.contains("...") || super::super::placeholders::has_token(value)
}

fn has_track_order(text: &str, order: &str, track: &str) -> bool {
    rows::all(text).any(|row| row.order == order && row.track == track)
}

fn has_track(text: &str, track: &str) -> bool {
    rows::all(text).any(|row| row.track == track)
}

fn track_count(text: &str, track: &str) -> usize {
    rows::all(text).filter(|row| row.track == track).count()
}

fn has_track_target(text: &str, track: &str, target: &str) -> bool {
    rows::all(text).any(|row| row.track == track && row.target == target)
}
