use std::process::Command;

pub(super) fn require_current(notes: &str) -> Result<(), String> {
    let head = current_head()?;
    if matches_head(notes, &head.short, &head.full) {
        return Ok(());
    }
    Err("release notes Git commit must match current HEAD before publish".to_string())
}

struct Head {
    short: String,
    full: String,
}

fn current_head() -> Result<Head, String> {
    Ok(Head {
        short: git_head(["rev-parse", "--short=7", "HEAD"])?,
        full: git_head(["rev-parse", "HEAD"])?,
    })
}

fn git_head<const N: usize>(args: [&str; N]) -> Result<String, String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err("git rev-parse failed".to_string());
    }
    String::from_utf8(output.stdout)
        .map_err(|error| error.to_string())
        .map(|value| value.trim().to_string())
}

fn matches_head(notes: &str, short: &str, full: &str) -> bool {
    field_value(notes, "Git commit").is_some_and(|commit| commit == short || commit == full)
}

fn field_value<'a>(text: &'a str, label: &str) -> Option<&'a str> {
    let prefix = format!("- {label}:");
    text.lines()
        .find_map(|line| line.trim().strip_prefix(&prefix).map(str::trim))
}

#[cfg(test)]
mod tests {
    use super::matches_head;

    #[test]
    fn accepts_current_short_commit() {
        assert!(matches_head(
            "- Git commit: abc1234\n",
            "abc1234",
            "abc1234ffff"
        ));
    }

    #[test]
    fn accepts_current_full_commit() {
        assert!(matches_head(
            "- Git commit: abc1234ffff\n",
            "abc1234",
            "abc1234ffff"
        ));
    }

    #[test]
    fn rejects_fake_full_commit_with_matching_prefix() {
        assert!(!matches_head(
            "- Git commit: abc1234eeee\n",
            "abc1234",
            "abc1234ffff"
        ));
    }

    #[test]
    fn rejects_different_commit() {
        assert!(!matches_head(
            "- Git commit: def5678\n",
            "abc1234",
            "abc1234ffff"
        ));
    }

    #[test]
    fn rejects_missing_commit() {
        assert!(!matches_head(
            "- Version: v0.1.0\n",
            "abc1234",
            "abc1234ffff"
        ));
    }
}
