use super::{model::Report, preflight};

pub(super) fn text(report: &Report, track: Option<&str>) -> Result<String, String> {
    let tracks = filtered_tracks(report, track)?;
    let mut lines = vec![
        "Productization status".to_string(),
        format!("release blockers: {}", counts(report)),
    ];
    lines.extend(tracks.iter().map(summary_line));
    lines.extend(tracks.iter().filter_map(remaining_line));
    lines.extend(tracks.iter().filter_map(record_target_line));
    lines.extend(tracks.iter().filter_map(|entry| action_line(report, entry)));
    lines.extend(tracks.iter().filter_map(command_line));
    lines.extend(next_track(report, &tracks, track.is_some()));
    Ok(lines.join("\n"))
}

fn counts(report: &Report) -> String {
    format!("{} total, {} verified, {} blocked", report.total, report.verified, report.blocked)
}

fn filtered_tracks<'a>(
    report: &'a Report,
    track: Option<&str>,
) -> Result<Vec<&'a super::model::TrackStatus>, String> {
    let Some(track) = track else {
        return Ok(report.tracks.iter().collect());
    };
    let tracks = report
        .tracks
        .iter()
        .filter(|entry| entry.order.to_string() == track || entry.name.eq_ignore_ascii_case(track))
        .collect::<Vec<_>>();
    (!tracks.is_empty())
        .then_some(tracks)
        .ok_or_else(|| format!("unknown track: {track}"))
}

fn summary_line(track: &&super::model::TrackStatus) -> String {
    format!(
        "{}. {}: {}/{} remaining",
        track.order,
        track.name,
        track.remaining.len(),
        track.blockers.len()
    )
}

fn remaining_line(track: &&super::model::TrackStatus) -> Option<String> {
    (!track.remaining.is_empty()).then(|| format!("   remaining blockers: {}", track.remaining.join(", ")))
}

fn record_target_line(track: &&super::model::TrackStatus) -> Option<String> {
    (!track.remaining.is_empty()).then(|| format!("   record target: {}", track.record_target))
}

fn action_line(report: &Report, track: &&super::model::TrackStatus) -> Option<String> {
    let actions = track
        .remaining
        .iter()
        .filter_map(|blocker| report.actions.iter().find(|action| action.blocker == *blocker))
        .map(|action| format!("{} -> {}", action.blocker, action.owner))
        .collect::<Vec<_>>();
    (!actions.is_empty()).then(|| format!("   next actions: {}", actions.join(" | ")))
}

fn command_line(track: &&super::model::TrackStatus) -> Option<String> {
    (!track.remaining.is_empty())
        .then(|| preflight::summary(&track.name))
        .flatten()
        .map(|line| format!("   {line}"))
}

fn next_track(
    report: &Report,
    tracks: &[&super::model::TrackStatus],
    filtered: bool,
) -> Vec<String> {
    let track = if filtered {
        tracks.first().copied()
    } else {
        report.tracks.iter().find(|track| !track.remaining.is_empty())
    };
    let Some(track) = track else {
        return vec!["next track: complete".to_string()];
    };
    let mut lines = vec![format!("next track: {}. {}", track.order, track.name)];
    lines.extend(track.remaining.iter().map(|blocker| next_action_line(report, blocker)));
    lines
}

fn next_action_line(report: &Report, blocker: &str) -> String {
    report
        .actions
        .iter()
        .find(|action| action.blocker == blocker)
        .map(|action| format!("- {blocker}: {} ({})", action.next_action, action.owner))
        .unwrap_or_else(|| format!("- {blocker}"))
}
