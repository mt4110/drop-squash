use super::model::Report;

pub(super) fn text(report: &Report) -> String {
    let mut lines = vec![
        "Productization status".to_string(),
        format!("release blockers: {}", counts(report)),
    ];
    lines.extend(report.tracks.iter().map(|track| {
        format!(
            "{}. {}: {}/{} remaining",
            track.order,
            track.name,
            track.remaining.len(),
            track.blockers.len()
        )
    }));
    lines.extend(report.tracks.iter().filter_map(track_remaining_line));
    lines.extend(
        report
            .tracks
            .iter()
            .filter_map(|track| track_action_line(report, track)),
    );
    lines.extend(
        report
            .tracks
            .iter()
            .filter_map(track_command_line),
    );
    lines.extend(next_track(report));
    lines.join("\n")
}

fn counts(report: &Report) -> String {
    format!(
        "{} total, {} verified, {} blocked",
        report.total, report.verified, report.blocked
    )
}

fn next_track(report: &Report) -> Vec<String> {
    let Some(track) = report
        .tracks
        .iter()
        .find(|track| !track.remaining.is_empty())
    else {
        return vec!["next track: complete".to_string()];
    };
    let mut lines = vec![format!("next track: {}. {}", track.order, track.name)];
    lines.extend(
        track
            .remaining
            .iter()
            .map(|blocker| next_action_line(report, blocker)),
    );
    lines
}

fn track_remaining_line(track: &super::model::TrackStatus) -> Option<String> {
    (!track.remaining.is_empty()).then(|| {
        format!("   remaining blockers: {}", track.remaining.join(", "))
    })
}

fn track_action_line(
    report: &Report,
    track: &super::model::TrackStatus,
) -> Option<String> {
    let actions = track
        .remaining
        .iter()
        .filter_map(|blocker| short_action(report, blocker))
        .collect::<Vec<_>>();
    (!actions.is_empty()).then(|| format!("   next actions: {}", actions.join(" | ")))
}

fn short_action(report: &Report, blocker: &str) -> Option<String> {
    report
        .actions
        .iter()
        .find(|action| action.blocker == blocker)
        .map(|action| format!("{blocker} -> {}", action.owner))
}

fn track_command_line(track: &super::model::TrackStatus) -> Option<String> {
    (!track.remaining.is_empty())
        .then(|| super::preflight::summary(&track.name))
        .flatten()
        .map(|line| format!("   {line}"))
}

fn next_action_line(report: &Report, blocker: &str) -> String {
    let Some(action) = report
        .actions
        .iter()
        .find(|action| action.blocker == blocker)
    else {
        return format!("- {blocker}");
    };
    format!("- {blocker}: {} ({})", action.next_action, action.owner)
}
