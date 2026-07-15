use super::model::{Blocker, EvidenceAction, Report, Track, TrackStatus};

mod table;

pub(super) fn report(text: &str) -> Result<Report, String> {
    let blockers = blockers(text);
    if blockers.is_empty() {
        return Err("release blocker table has no blocker rows".into());
    }
    let tracks = tracks(text);
    if tracks.is_empty() {
        return Err("execution order table has no track rows".into());
    }
    Ok(build_report(blockers, evidence_actions(text), tracks))
}

fn build_report(
    blockers: Vec<Blocker>,
    actions: Vec<EvidenceAction>,
    tracks: Vec<Track>,
) -> Report {
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
        actions,
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
        record_target: track.record_target,
    }
}

fn status_for<'a>(blockers: &'a [Blocker], name: &str) -> Option<&'a str> {
    blockers
        .iter()
        .find(|blocker| blocker.name == name)
        .map(|blocker| blocker.status.as_str())
}

fn blockers(text: &str) -> Vec<Blocker> {
    table::rows(text)
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

fn evidence_actions(text: &str) -> Vec<EvidenceAction> {
    text.split("## Evidence Classes")
        .nth(1)
        .and_then(|text| text.split("## Execution Order").next())
        .map(action_rows)
        .unwrap_or_default()
}

fn action_rows(text: &str) -> Vec<EvidenceAction> {
    table::rows(text)
        .filter(|cells| cells.len() == 4)
        .filter(|cells| cells[0] != "Blocker" && !cells[0].starts_with("---"))
        .map(|cells| EvidenceAction {
            blocker: cells[0].clone(),
            next_action: cells[2].clone(),
            owner: cells[3].clone(),
        })
        .collect()
}

fn execution_rows(text: &str) -> Vec<Track> {
    table::rows(text)
        .filter(|cells| cells.len() == 5)
        .filter_map(|cells| {
            let order = cells[0].parse::<usize>().ok()?;
            Some(Track {
                order,
                name: cells[1].clone(),
                blockers: cells[2].split(", ").map(str::to_string).collect(),
                record_target: cells[4].clone(),
            })
        })
        .collect()
}
