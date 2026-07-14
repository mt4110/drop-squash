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
    lines.extend(track.remaining.iter().map(|blocker| format!("- {blocker}")));
    lines
}
