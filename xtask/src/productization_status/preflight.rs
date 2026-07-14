use std::process::Command;

use super::model::Report;

pub(super) fn lines(report: &Report) -> Result<Vec<String>, String> {
    let Some(track) = report
        .tracks
        .iter()
        .find(|track| !track.remaining.is_empty())
    else {
        return Ok(Vec::new());
    };
    if track.name != "Local packaged-app proof" {
        return Ok(Vec::new());
    }
    let status = git_status()?;
    Ok(manual_qa_lines(&status))
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
        return vec!["preflight: manual QA can start from a clean git worktree".to_string()];
    }
    vec![
        "preflight: manual QA is blocked by dirty git worktree".to_string(),
        crate::git_status::dirty_paths(status),
        "preflight next: commit, stash, or intentionally remove these changes, then rebuild the app artifact".to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use super::manual_qa_lines;

    #[test]
    fn reports_clean_manual_qa_preflight() {
        let lines = manual_qa_lines("");

        assert_eq!(
            lines,
            vec!["preflight: manual QA can start from a clean git worktree"]
        );
    }

    #[test]
    fn reports_dirty_manual_qa_preflight() {
        let lines = manual_qa_lines(" M docs/manual-qa.md\n");

        assert!(lines[0].contains("blocked"));
        assert_eq!(lines[1], " M docs/manual-qa.md");
        assert!(lines[2].contains("rebuild the app artifact"));
    }
}
