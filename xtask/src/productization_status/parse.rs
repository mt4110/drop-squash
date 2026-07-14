use super::model::{Blocker, Report, Track, TrackStatus};

pub(super) fn report(text: &str) -> Result<Report, String> {
    let blockers = blockers(text);
    if blockers.is_empty() {
        return Err("release blocker table has no blocker rows".into());
    }
    let tracks = tracks(text);
    if tracks.is_empty() {
        return Err("execution order table has no track rows".into());
    }
    Ok(build_report(blockers, tracks))
}

fn build_report(blockers: Vec<Blocker>, tracks: Vec<Track>) -> Report {
    let verified = blockers
        .iter()
        .filter(|blocker| blocker.status == "Verified")
        .count();
    let blocked = blockers
        .iter()
        .filter(|blocker| blocker.status == "Blocked")
        .count();
    let tracks = tracks
        .into_iter()
        .map(|track| track_status(track, &blockers))
        .collect();
    Report {
        total: blockers.len(),
        verified,
        blocked,
        tracks,
    }
}

fn track_status(track: Track, blockers: &[Blocker]) -> TrackStatus {
    let remaining = track
        .blockers
        .iter()
        .filter(|name| status_for(blockers, name) != Some("Verified"))
        .cloned()
        .collect();
    TrackStatus {
        order: track.order,
        name: track.name,
        blockers: track.blockers,
        remaining,
    }
}

fn status_for<'a>(blockers: &'a [Blocker], name: &str) -> Option<&'a str> {
    blockers
        .iter()
        .find(|blocker| blocker.name == name)
        .map(|blocker| blocker.status.as_str())
}

fn blockers(text: &str) -> Vec<Blocker> {
    rows(text)
        .filter(|cells| cells.len() == 5 && !cells[0].starts_with("---"))
        .filter(|cells| cells[0] != "Blocker")
        .filter(|cells| matches!(cells[1].as_str(), "Blocked" | "Verified"))
        .map(|cells| Blocker {
            name: cells[0].clone(),
            status: cells[1].clone(),
        })
        .collect()
}

fn tracks(text: &str) -> Vec<Track> {
    text.split("## Execution Order")
        .nth(1)
        .map(execution_rows)
        .unwrap_or_default()
}

fn execution_rows(text: &str) -> Vec<Track> {
    rows(text)
        .filter(|cells| cells.len() == 5)
        .filter_map(|cells| {
            let order = cells[0].parse::<usize>().ok()?;
            Some(Track {
                order,
                name: cells[1].clone(),
                blockers: cells[2].split(", ").map(str::to_string).collect(),
            })
        })
        .collect()
}

fn rows(text: &str) -> impl Iterator<Item = Vec<String>> + '_ {
    text.lines()
        .filter(|line| line.starts_with('|'))
        .map(|line| line.trim_matches('|').split('|').map(cell).collect())
}

fn cell(value: &str) -> String {
    value.trim().trim_matches('`').to_string()
}
