use std::path::Path;
use std::process::Command;
use std::time::UNIX_EPOCH;

pub(super) fn require_not_older_than_head(path: &Path) -> Result<(), String> {
    let artifact_epoch = modified_epoch(path)?;
    let head_epoch = head_commit_epoch()?;
    if is_older_than_head(artifact_epoch, head_epoch) {
        return Err("artifact is older than HEAD; rebuild the app before manual QA".to_string());
    }
    Ok(())
}

fn modified_epoch(path: &Path) -> Result<u64, String> {
    let modified = path
        .metadata()
        .map_err(|error| error.to_string())?
        .modified()
        .map_err(|error| error.to_string())?;
    modified
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|error| error.to_string())
}

fn head_commit_epoch() -> Result<u64, String> {
    let output = Command::new("git")
        .args(["show", "-s", "--format=%ct", "HEAD"])
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err("git show HEAD commit time failed".to_string());
    }
    String::from_utf8(output.stdout)
        .map_err(|error| error.to_string())?
        .trim()
        .parse()
        .map_err(|error| format!("git HEAD commit time is invalid: {error}"))
}

fn is_older_than_head(artifact_epoch: u64, head_epoch: u64) -> bool {
    artifact_epoch < head_epoch
}

#[cfg(test)]
mod tests;
