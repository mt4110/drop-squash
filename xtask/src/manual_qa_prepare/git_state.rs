use std::process::Command;

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
    Err("manual QA preparation requires a clean git worktree".to_string())
}

pub(super) fn clean_status(status: &str) -> bool {
    status.trim().is_empty()
}

#[cfg(test)]
mod tests {
    use super::clean_status;

    #[test]
    fn accepts_empty_git_status() {
        assert!(clean_status(""));
    }

    #[test]
    fn rejects_dirty_git_status() {
        assert!(!clean_status(" M docs/manual-qa.md\n"));
    }
}
