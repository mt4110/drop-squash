use crate::productization_status::{model::Report, scope::Scope};

pub(super) fn filtered<'a>(
    report: &'a Report,
    track: Option<&str>,
    scope: &Scope,
    scoped: bool,
) -> Result<Vec<&'a crate::productization_status::model::TrackStatus>, String> {
    let Some(track) = track else {
        return Ok(report
            .tracks
            .iter()
            .filter(|entry| !scoped || !visible_blockers(entry, scope).is_empty())
            .collect());
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

pub(super) fn blockers<'a>(
    track: &'a crate::productization_status::model::TrackStatus,
    scope: &Scope,
    scoped: bool,
) -> Vec<&'a String> {
    if !scoped {
        return track.blockers.iter().collect();
    }
    visible_blockers(track, scope)
}

pub(super) fn remaining(
    track: &crate::productization_status::model::TrackStatus,
    scope: &Scope,
    scoped: bool,
) -> Vec<String> {
    if !scoped {
        return track.remaining.clone();
    }
    track
        .remaining
        .iter()
        .filter(|blocker| !scope.is_deferred(blocker))
        .cloned()
        .collect()
}

fn visible_blockers<'a>(
    track: &'a crate::productization_status::model::TrackStatus,
    scope: &Scope,
) -> Vec<&'a String> {
    track
        .blockers
        .iter()
        .filter(|blocker| !scope.is_deferred(blocker))
        .collect()
}
