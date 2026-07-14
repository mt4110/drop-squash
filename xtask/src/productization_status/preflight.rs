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
    vec![
        manual_qa_command(),
        benchmark_command(),
        csv_check_command(),
        manual_check_command(),
    ]
}

fn manual_qa_command() -> String {
    "preflight after clean: cargo run -p xtask -- manual-qa-prepare --app-artifact target/release/bundle/dmg/DropSquash.dmg --input-sample-set \"short, medium, and large local recordings\"".to_string()
}

fn benchmark_command() -> String {
    "preflight benchmark: cargo run -p xtask -- benchmark --release-set --input /absolute/path/to/short.mov --input /absolute/path/to/medium.mov --input /absolute/path/to/large.mov --output-dir /tmp/dropsquash-manual-qa-output --csv-output /tmp/dropsquash-manual-qa-output/benchmark-results.csv".to_string()
}

fn csv_check_command() -> String {
    "preflight benchmark check: cargo run -p xtask -- benchmark-csv-check /tmp/dropsquash-manual-qa-output/benchmark-results.csv".to_string()
}

fn manual_check_command() -> String {
    "preflight final gate: cargo run -p xtask -- manual-qa-check".to_string()
}

#[cfg(test)]
mod tests {
    use super::manual_qa_lines;

    #[test]
    fn reports_clean_manual_qa_preflight() {
        let lines = manual_qa_lines("");

        assert!(lines[0].contains("can start"));
        assert!(lines[1].contains("manual-qa-prepare"));
        assert!(lines[1].contains("--app-artifact"));
        assert!(lines[2].contains("benchmark --release-set"));
        assert!(lines[3].contains("benchmark-csv-check"));
        assert!(lines[4].contains("manual-qa-check"));
    }

    #[test]
    fn reports_dirty_manual_qa_preflight() {
        let lines = manual_qa_lines(" M docs/manual-qa.md\n");

        assert!(lines[0].contains("blocked"));
        assert_eq!(lines[1], " M docs/manual-qa.md");
        assert!(lines[2].contains("rebuild the app artifact"));
        assert!(lines[3].contains("manual-qa-prepare"));
        assert!(lines[4].contains("benchmark-results.csv"));
        assert!(lines[5].contains("benchmark-csv-check"));
        assert!(lines[6].contains("manual-qa-check"));
    }
}
