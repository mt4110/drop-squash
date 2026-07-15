pub(crate) fn contains_current_short_head_after_git(value: &str) -> Result<bool, String> {
    let head = current_short_head()?;
    if has_git_commit(value, &head) {
        return Ok(true);
    }
    let Some(build) = git_commit_after_token(value) else {
        return Ok(false);
    };
    helper_only_descendant(&build)
}

fn has_git_commit(value: &str, head: &str) -> bool {
    git_commit_after_token(value).as_deref() == Some(head)
}

fn git_commit_after_token(value: &str) -> Option<String> {
    let mut tokens = value.split_whitespace();
    while let Some(token) = tokens.next() {
        if token == "git" {
            return tokens.next().map(ToString::to_string);
        }
    }
    None
}

fn helper_only_descendant(base: &str) -> Result<bool, String> {
    if !git_ok(["merge-base", "--is-ancestor", base, "HEAD"])? {
        return Ok(false);
    }
    let changed = git_stdout(["diff", "--name-only", &format!("{base}..HEAD")])?;
    Ok(!changed.is_empty() && changed.lines().all(is_helper_path))
}

fn is_helper_path(path: &str) -> bool {
    path.starts_with("xtask/") || path.starts_with("docs/")
}

fn git_ok(args: [&str; 4]) -> Result<bool, String> {
    std::process::Command::new("git")
        .args(args)
        .status()
        .map(|status| status.success())
        .map_err(|error| error.to_string())
}

fn git_stdout(args: [&str; 3]) -> Result<String, String> {
    let output = std::process::Command::new("git").args(args).output().map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err("git command failed".to_string());
    }
    String::from_utf8(output.stdout).map_err(|error| error.to_string())
}

fn current_short_head() -> Result<String, String> {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--short=7", "HEAD"])
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err("git rev-parse failed".to_string());
    }
    String::from_utf8(output.stdout)
        .map_err(|error| error.to_string())
        .map(|value| value.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::{git_commit_after_token, has_git_commit, is_helper_path};

    #[test]
    fn accepts_exact_git_token() {
        assert!(has_git_commit("DropSquash 0.1.0 git abc1234", "abc1234"));
    }

    #[test]
    fn rejects_commit_with_matching_prefix() {
        assert!(!has_git_commit(
            "DropSquash 0.1.0 git abc1234ffff",
            "abc1234"
        ));
    }

    #[test]
    fn finds_git_commit_after_token() {
        assert_eq!(
            git_commit_after_token("DropSquash 0.1.0 git abc1234"),
            Some("abc1234".to_string())
        );
    }

    #[test]
    fn helper_path_allowlist_is_narrow() {
        assert!(is_helper_path("xtask/src/main.rs"));
        assert!(is_helper_path("docs/manual-qa.md"));
        assert!(!is_helper_path("apps/desktop/src-tauri/src/main.rs"));
    }
}
