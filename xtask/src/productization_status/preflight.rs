use std::process::Command;

use super::model::Report;

mod commands;
#[cfg(test)]
mod tests;

pub(super) fn lines(report: &Report) -> Result<Vec<String>, String> {
    let Some(track) = report
        .tracks
        .iter()
        .find(|track| !track.remaining.is_empty())
    else {
        return Ok(Vec::new());
    };
    match track.name.as_str() {
        "Local packaged-app proof" => Ok(manual_qa_lines(&git_status()?)),
        "License sandbox proof" => Ok(license_sandbox_lines()),
        "Public web proof" => Ok(public_web_lines()),
        "Signing and distribution proof" => Ok(distribution_lines()),
        _ => Ok(Vec::new()),
    }
}

pub(super) fn summary(track_name: &str) -> Option<String> {
    match track_name {
        "Local packaged-app proof" => Some(
            "primary command: cargo run -p xtask -- manual-qa-pending /tmp/dropsquash-manual-qa-prepared.md --section local-proof".to_string(),
        ),
        "License sandbox proof" => Some(
            "primary command: cargo run -p xtask -- manual-qa-pending /tmp/dropsquash-manual-qa-prepared.md --section license".to_string(),
        ),
        "Public web proof" => Some(
            "primary command: cargo run -p xtask -- website-check".to_string(),
        ),
        "Signing and distribution proof" => Some(
            "primary command: cargo run -p xtask -- manual-qa-pending /tmp/dropsquash-manual-qa-prepared.md --section distribution".to_string(),
        ),
        _ => None,
    }
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
        lines.extend(next_commands());
        return lines;
    }
    let mut lines = vec![
        "preflight: current worktree is dirty; use a clean detached QA worktree or clean these changes before rebuilding the app artifact".to_string(),
        crate::git_status::dirty_paths(status),
        "preflight current-worktree option: commit, stash, or intentionally remove these changes, then rebuild the app artifact".to_string(),
        "preflight clean worktree option: git worktree add --detach /tmp/dropsquash-qa-$(git rev-parse --short HEAD) HEAD".to_string(),
    ];
    lines.extend(next_commands());
    lines
}

fn next_commands() -> Vec<String> {
    commands::local_packaged_app()
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
