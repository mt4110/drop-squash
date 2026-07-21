use std::process::Command;

pub(crate) fn print_quickstart() {
    for line in quickstart_lines(git_status().ok().as_deref()) {
        println!("{line}");
    }
}

pub(crate) fn quickstart_lines(git_status: Option<&str>) -> Vec<String> {
    if git_status.map_or(true, crate::git_status::is_clean) {
        return Vec::new();
    }
    vec![
        "dirty worktree quickstart: snapshot this exact tree with `scripts/manual-qa-snapshot-worktree.sh /tmp/dropsquash-qa-snapshot-$(git rev-parse --short HEAD)` before packaged, sandbox, or signing proof".to_string(),
        "dirty worktree handoff: `snapshot=$(scripts/manual-qa-snapshot-worktree.sh /tmp/dropsquash-qa-snapshot-$(git rev-parse --short HEAD) | sed -n 's/^snapshot worktree: //p') && cd \"$snapshot\" && CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- productization-status --track 'Paid beta'`".to_string(),
        "dirty worktree sandbox handoff: `scripts/manual-qa-license-sandbox-handoff.sh /tmp/dropsquash-qa-snapshot-$(git rev-parse --short HEAD)` prints the snapshot-specific sandbox proof commands through `manual-qa-license-rerun` and `paid-beta-check`".to_string(),
        "dirty worktree distribution handoff: `scripts/manual-qa-distribution-handoff.sh /tmp/dropsquash-qa-snapshot-$(git rev-parse --short HEAD)` prints the snapshot-specific signing proof commands through `manual-qa-distribution-rerun` and `paid-beta-check`".to_string(),
        "dirty worktree deterministic pass: in the snapshot run Nix build, `manual-qa-prepare --reset-trial`, `benchmark --release-set`, `benchmark-csv-check`, then `manual-qa-ready-all` before `manual-qa-license-rerun` or `manual-qa-distribution-rerun`".to_string(),
    ]
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

#[cfg(test)]
mod tests {
    use super::quickstart_lines;

    #[test]
    fn omits_copy_for_clean_status() {
        assert!(quickstart_lines(Some("")).is_empty());
    }

    #[test]
    fn keeps_dirty_worktree_copy_stable() {
        let lines = quickstart_lines(Some(" M docs/manual-qa.md\n"));

        assert!(lines
            .iter()
            .any(|line| line.contains("dirty worktree quickstart")));
        assert!(lines
            .iter()
            .any(|line| line.contains("manual-qa-license-sandbox-handoff.sh")));
        assert!(lines
            .iter()
            .any(|line| line.contains("manual-qa-distribution-handoff.sh")));
        assert!(lines
            .iter()
            .any(|line| line.contains("manual-qa-ready-all")));
        assert!(lines
            .iter()
            .any(|line| line.contains("manual-qa-license-rerun")));
        assert!(lines
            .iter()
            .any(|line| line.contains("manual-qa-distribution-rerun")));
    }
}
