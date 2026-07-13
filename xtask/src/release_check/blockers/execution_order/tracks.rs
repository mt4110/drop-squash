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

fn has_track_order(text: &str, order: &str, track: &str) -> bool {
    rows::all(text).any(|row| row.order == order && row.track == track)
}

fn has_track(text: &str, track: &str) -> bool {
    rows::all(text).any(|row| row.track == track)
}

fn has_track_target(text: &str, track: &str, target: &str) -> bool {
    rows::all(text).any(|row| row.track == track && row.target == target)
}
