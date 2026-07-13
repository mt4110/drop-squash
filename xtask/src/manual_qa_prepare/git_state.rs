use std::process::Command;

use crate::git_status;

pub(super) fn require_clean_worktree() -> Result<(), String> {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err("git status failed".to_string());
    }
    let status = String::from_utf8(output.stdout).map_err(|error| error.to_string())?;
    if clean_status(&status) {
        return Ok(());
    }
    Err(format!(
        "manual QA preparation requires a clean git worktree; dirty paths:\n{}",
        dirty_paths(&status)
    ))
}

pub(super) fn clean_status(status: &str) -> bool {
    git_status::is_clean(status)
}

fn dirty_paths(status: &str) -> String {
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
        assert!(!clean_status(" M docs/manual-qa.md\n"));
    }

    #[test]
    fn summarizes_dirty_paths() {
        let summary = dirty_paths(" M docs/manual-qa.md\n D old.md\n");

        assert_eq!(summary, " M docs/manual-qa.md\n D old.md");
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
