use std::process::Command;

use super::{model::Report, scope::Scope};

mod commands;
mod summary;
#[cfg(test)]
mod tests;

pub(crate) use summary::summary;

pub(super) fn lines_for(
    report: &Report,
    filter: Option<&str>,
    scope: &Scope,
) -> Result<Vec<String>, String> {
    let visible = report
        .tracks
        .iter()
        .filter(|track| has_visible_remaining(track, scope, filter.is_none()))
        .collect::<Vec<_>>();
    let all_tracks = report.tracks.iter().collect::<Vec<_>>();
    let selected = if filter.is_some() {
        &all_tracks
    } else {
        &visible
    };
    let Some(track) = select_track(selected, filter)? else {
        return Ok(Vec::new());
    };
    if !has_visible_remaining(track, scope, false) {
        return Ok(Vec::new());
    }
    match track.name.as_str() {
        "Local packaged-app proof" => Ok(manual_qa_lines(&git_status()?)),
        "License sandbox proof" => Ok(license_sandbox_lines()),
        "Public web proof" => Ok(public_web_lines()),
        "Signing and distribution proof" => Ok(distribution_lines()),
        _ => Ok(Vec::new()),
    }
}

fn has_visible_remaining(track: &super::model::TrackStatus, scope: &Scope, scoped: bool) -> bool {
    if !scoped {
        return !track.remaining.is_empty();
    }
    track
        .remaining
        .iter()
        .any(|blocker| !scope.is_deferred(blocker))
}

fn git_status() -> Result<String, String> {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err("git status failed".to_string());
    }
    String::from_utf8(output.stdout).map_err(|error| error.to_string())
}

fn manual_qa_lines(status: &str) -> Vec<String> {
    if crate::git_status::is_clean(status) {
        let mut lines =
            vec!["preflight: manual QA can start from a clean git worktree".to_string()];
        lines.extend(commands::local_packaged_app());
        return lines;
    }
    let mut lines = vec![
        "preflight: current worktree is dirty; use a clean detached QA worktree or clean these changes before rebuilding the app artifact".to_string(),
        crate::git_status::dirty_paths(status),
        "preflight current-worktree option: commit, stash, or intentionally remove these changes, then rebuild the app artifact".to_string(),
        "preflight clean worktree option: git worktree add --detach /tmp/dropsquash-qa-$(git rev-parse --short HEAD) HEAD, or snapshot this exact dirty tree with `scripts/manual-qa-snapshot-worktree.sh /tmp/dropsquash-qa-snapshot-$(git rev-parse --short HEAD)`".to_string(),
    ];
    lines.extend(commands::local_packaged_app());
    lines
}

fn license_sandbox_lines() -> Vec<String> {
    let mut lines = vec![
        "preflight: license sandbox proof needs sandbox credentials and local cache inspection without recording keys".to_string(),
    ];
    lines.extend(commands::license_sandbox());
    lines
}

fn public_web_lines() -> Vec<String> {
    let mut lines = vec![
        "preflight: public web proof needs production dropsquash.app URLs and a live checkout URL before blockers can move".to_string(),
    ];
    lines.extend(commands::public_web());
    lines
}

fn distribution_lines() -> Vec<String> {
    let mut lines = vec![
        "preflight: signing and distribution proof needs release environment secrets plus one public DropSquash.dmg carried through verification".to_string(),
    ];
    lines.extend(commands::distribution_signing());
    lines
}

fn select_track<'a>(
    tracks: &[&'a super::model::TrackStatus],
    filter: Option<&str>,
) -> Result<Option<&'a super::model::TrackStatus>, String> {
    let Some(filter) = filter else {
        return Ok(tracks.first().copied());
    };
    tracks
        .iter()
        .find(|track| track.order.to_string() == filter || track.name.eq_ignore_ascii_case(filter))
        .copied()
        .map(Some)
        .ok_or_else(|| format!("unknown track: {filter}"))
}
