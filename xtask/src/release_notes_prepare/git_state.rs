use std::process::Command;

use crate::git_status;

pub(super) fn commit() -> Result<String, String> {
    let output = Command::new("git")
        .args(["rev-parse", "--short=7", "HEAD"])
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err("git rev-parse failed".into());
    }
    String::from_utf8(output.stdout)
        .map_err(|error| error.to_string())
        .map(|value| value.trim().to_string())
}

pub(super) fn require_clean_worktree() -> Result<(), String> {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err("git status failed".into());
    }
    let status = String::from_utf8(output.stdout).map_err(|error| error.to_string())?;
    if clean_status(&status) {
        return Ok(());
    }
    Err(format!(
        "release notes require a clean git worktree; dirty paths:\n{}",
        dirty_paths(&status)
    ))
}

pub(super) fn clean_status(status: &str) -> bool {
    git_status::is_clean(status)
}

pub(super) fn dirty_paths(status: &str) -> String {
    git_status::dirty_paths(status)
}

#[cfg(test)]
mod tests {
    use super::{clean_status, dirty_paths};

    #[test]
    fn accepts_empty_git_status() {
        assert!(clean_status(""));
    }

    #[test]
    fn rejects_dirty_git_status() {
        assert!(!clean_status(" D dropsquash_rust_repo_blueprint.md\n"));
    }

    #[test]
    fn summarizes_dirty_paths() {
        let summary = dirty_paths(" D old.md\n M docs/release.md\n");

        assert_eq!(summary, " D old.md\n M docs/release.md");
    }

    #[test]
    fn reports_truncated_dirty_paths() {
        let status = (0..12)
            .map(|index| format!(" M file-{index}.md"))
            .collect::<Vec<_>>()
            .join("\n");
        let summary = dirty_paths(&status);

        assert!(summary.ends_with("... and 2 more"));
    }
}
