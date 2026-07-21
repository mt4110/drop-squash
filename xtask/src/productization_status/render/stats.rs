use crate::productization_status::{model::Report, scope::Scope};

pub(super) fn all(report: &Report) -> String {
    format!(
        "{} total, {} verified, {} blocked",
        report.total, report.verified, report.blocked
    )
}

pub(super) fn scoped(report: &Report, scope: &Scope) -> String {
    let blockers = report
        .tracks
        .iter()
        .flat_map(|track| track.blockers.iter())
        .filter(|blocker| !scope.is_deferred(blocker))
        .collect::<Vec<_>>();
    let verified = blockers
        .iter()
        .filter(|blocker| is_verified(report, blocker))
        .count();
    let blocked = blockers.len().saturating_sub(verified);
    format!(
        "{} total, {} verified, {} blocked",
        blockers.len(),
        verified,
        blocked
    )
}

fn is_verified(report: &Report, blocker: &str) -> bool {
    report.tracks.iter().any(|track| {
        track.blockers.iter().any(|entry| entry == blocker)
            && !track.remaining.iter().any(|entry| entry == blocker)
    })
}
