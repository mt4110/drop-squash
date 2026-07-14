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

#[cfg(test)]
mod tests {
    use super::manual_qa_lines;

    #[test]
    fn reports_clean_manual_qa_preflight() {
        let lines = manual_qa_lines("");

        assert!(lines[0].contains("can start"));
        assert!(lines[1].contains("apps/desktop install --frozen-lockfile"));
        assert!(lines[2].contains("apps/desktop/web install --frozen-lockfile"));
        assert!(lines[3].contains("nix develop --command pnpm"));
        assert!(lines[3].contains("tauri build"));
        assert!(lines[4].contains("normalize-dmg"));
        assert!(lines[5].contains("artifact-check"));
        assert!(lines[6].contains("manual-qa-prepare"));
        assert!(lines[6].contains("--reset-trial"));
        assert!(lines[6].contains("--app-artifact"));
        assert!(lines[6].contains("Application Support/DropSquash"));
        assert!(lines[6].contains("--state-dir /tmp/dropsquash-manual-qa-state"));
        assert!(lines[6].contains("--output-dir /tmp/dropsquash-manual-qa-output"));
        assert!(lines[6].contains("--markdown-output"));
        assert!(lines[7].contains("checksum"));
        assert!(lines[7].contains("SHA256SUMS"));
        assert!(lines[8].contains("benchmark --release-set"));
        assert!(lines[9].contains("benchmark-csv-check"));
        assert!(lines[10].contains("manual-qa-check"));
        assert!(lines[11].contains("--restore-state"));
        assert!(lines[11].contains("Application Support/DropSquash"));
        assert!(lines[11].contains("--state-dir /tmp/dropsquash-manual-qa-state"));
    }

    #[test]
    fn reports_dirty_manual_qa_preflight() {
        let lines = manual_qa_lines(" M docs/manual-qa.md\n");

        assert!(lines[0].contains("current worktree is dirty"));
        assert!(lines[0].contains("clean detached QA worktree"));
        assert_eq!(lines[1], " M docs/manual-qa.md");
        assert!(lines[2].contains("rebuild the app artifact"));
        assert!(lines[3].contains("git worktree add --detach"));
        assert!(lines[4].contains("apps/desktop install --frozen-lockfile"));
        assert!(lines[5].contains("apps/desktop/web install --frozen-lockfile"));
        assert!(lines[6].contains("nix develop --command pnpm"));
        assert!(lines[6].contains("tauri build"));
        assert!(lines[7].contains("normalize-dmg"));
        assert!(lines[8].contains("artifact-check"));
        assert!(lines[9].contains("manual-qa-prepare"));
        assert!(lines[9].contains("--reset-trial"));
        assert!(lines[9].contains("Application Support/DropSquash"));
        assert!(lines[9].contains("--state-dir /tmp/dropsquash-manual-qa-state"));
        assert!(lines[9].contains("--output-dir /tmp/dropsquash-manual-qa-output"));
        assert!(lines[9].contains("/tmp/dropsquash-manual-qa-prepared.md"));
        assert!(lines[10].contains("checksum"));
        assert!(lines[10].contains("SHA256SUMS"));
        assert!(lines[11].contains("benchmark-results.csv"));
        assert!(lines[12].contains("benchmark-csv-check"));
        assert!(lines[13].contains("manual-qa-check"));
        assert!(lines[14].contains("--restore-state"));
        assert!(lines[14].contains("Application Support/DropSquash"));
        assert!(lines[14].contains("--state-dir /tmp/dropsquash-manual-qa-state"));
    }
}
