use std::process::Command;

use super::model::Report;

mod commands;

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
        let mut lines =
            vec!["preflight: manual QA can start from a clean git worktree".to_string()];
        lines.extend(next_commands());
        return lines;
    }
    let mut lines = vec![
        "preflight: manual QA is blocked by dirty git worktree".to_string(),
        crate::git_status::dirty_paths(status),
        "preflight next: commit, stash, or intentionally remove these changes, then rebuild the app artifact".to_string(),
    ];
    lines.extend(next_commands());
    lines
}

fn next_commands() -> Vec<String> {
    commands::local_packaged_app()
}

#[cfg(test)]
mod tests {
    use super::manual_qa_lines;

    #[test]
    fn reports_clean_manual_qa_preflight() {
        let lines = manual_qa_lines("");

        assert!(lines[0].contains("can start"));
        assert!(lines[1].contains("tauri build"));
        assert!(lines[2].contains("normalize-dmg"));
        assert!(lines[3].contains("artifact-check"));
        assert!(lines[4].contains("manual-qa-prepare"));
        assert!(lines[4].contains("--app-artifact"));
        assert!(lines[4].contains("--markdown-output"));
        assert!(lines[5].contains("benchmark --release-set"));
        assert!(lines[6].contains("benchmark-csv-check"));
        assert!(lines[7].contains("manual-qa-check"));
    }

    #[test]
    fn reports_dirty_manual_qa_preflight() {
        let lines = manual_qa_lines(" M docs/manual-qa.md\n");

        assert!(lines[0].contains("blocked"));
        assert_eq!(lines[1], " M docs/manual-qa.md");
        assert!(lines[2].contains("rebuild the app artifact"));
        assert!(lines[3].contains("tauri build"));
        assert!(lines[4].contains("normalize-dmg"));
        assert!(lines[5].contains("artifact-check"));
        assert!(lines[6].contains("manual-qa-prepare"));
        assert!(lines[6].contains("/tmp/dropsquash-manual-qa-prepared.md"));
        assert!(lines[7].contains("benchmark-results.csv"));
        assert!(lines[8].contains("benchmark-csv-check"));
        assert!(lines[9].contains("manual-qa-check"));
    }
}
