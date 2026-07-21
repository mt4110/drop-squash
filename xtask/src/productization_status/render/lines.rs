use super::tracks;
use crate::productization_status::{model::Report, preflight, scope::Scope};

pub(super) fn summary(
    track: &&crate::productization_status::model::TrackStatus,
    scope: &Scope,
    scoped: bool,
) -> String {
    let blockers = tracks::blockers(track, scope, scoped);
    let remaining = tracks::remaining(track, scope, scoped);
    format!(
        "{}. {}: {}/{} remaining",
        track.order,
        track.name,
        remaining.len(),
        blockers.len()
    )
}

pub(super) fn remaining(
    track: &&crate::productization_status::model::TrackStatus,
    scope: &Scope,
    scoped: bool,
) -> Option<String> {
    let remaining = tracks::remaining(track, scope, scoped);
    (!remaining.is_empty()).then(|| format!("   remaining blockers: {}", remaining.join(", ")))
}

pub(super) fn record_target(
    track: &&crate::productization_status::model::TrackStatus,
) -> Option<String> {
    (!track.remaining.is_empty()).then(|| format!("   record target: {}", track.record_target))
}

pub(super) fn actions(
    report: &Report,
    track: &&crate::productization_status::model::TrackStatus,
    scope: &Scope,
    scoped: bool,
) -> Option<String> {
    let actions = tracks::remaining(track, scope, scoped)
        .iter()
        .filter_map(|blocker| {
            report
                .actions
                .iter()
                .find(|action| action.blocker == *blocker)
        })
        .map(|action| format!("{} -> {}", action.blocker, action.owner))
        .collect::<Vec<_>>();
    (!actions.is_empty()).then(|| format!("   next actions: {}", actions.join(" | ")))
}

pub(super) fn command(
    track: &&crate::productization_status::model::TrackStatus,
    scope: &Scope,
    scoped: bool,
) -> Option<String> {
    (!tracks::remaining(track, scope, scoped).is_empty())
        .then(|| preflight::summary(track, scope, scoped))
        .flatten()
        .map(|line| format!("   {line}"))
}

pub(super) fn next_track(
    report: &Report,
    tracks: &[&crate::productization_status::model::TrackStatus],
    filtered: bool,
    scope: &Scope,
    scoped: bool,
) -> Vec<String> {
    let track = if filtered {
        tracks.first().copied()
    } else {
        report
            .tracks
            .iter()
            .find(|track| !super::tracks::remaining(track, scope, scoped).is_empty())
    };
    let Some(track) = track else {
        return vec!["next track: complete".to_string()];
    };
    if filtered && tracks::remaining(track, scope, scoped).is_empty() {
        return vec!["next track: complete".to_string()];
    }
    let mut lines = vec![format!("next track: {}. {}", track.order, track.name)];
    lines.extend(
        tracks::remaining(track, scope, scoped)
            .into_iter()
            .map(|blocker| next_action(report, &blocker)),
    );
    lines
}

fn next_action(report: &Report, blocker: &str) -> String {
    report
        .actions
        .iter()
        .find(|action| action.blocker == blocker)
        .map(|action| action::render(blocker, action.next_action.as_str(), action.owner.as_str()))
        .unwrap_or_else(|| format!("- {blocker}"))
}
mod action;
