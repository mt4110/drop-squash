const MAX_DIRTY_LINES: usize = 10;

pub(crate) fn is_clean(status: &str) -> bool {
    status.trim().is_empty()
}

pub(crate) fn dirty_paths(status: &str) -> String {
    let lines = dirty_lines(status);
    let shown = lines
        .iter()
        .take(MAX_DIRTY_LINES)
        .copied()
        .collect::<Vec<_>>()
        .join("\n");
    let remaining = lines.len().saturating_sub(MAX_DIRTY_LINES);
    if remaining == 0 {
        return shown;
    }
    format!("{shown}\n... and {remaining} more")
}

fn dirty_lines(status: &str) -> Vec<&str> {
    status
        .lines()
        .map(str::trim_end)
        .filter(|line| !line.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{dirty_paths, is_clean};

    #[test]
    fn detects_clean_status() {
        assert!(is_clean(""));
        assert!(is_clean("\n"));
        assert!(!is_clean(" M docs/manual-qa.md\n"));
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

        assert!(summary.contains(" M file-9.md"));
        assert!(!summary.contains(" M file-10.md"));
        assert!(summary.ends_with("... and 2 more"));
    }
}
