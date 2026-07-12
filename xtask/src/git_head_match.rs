pub(crate) fn contains_current_short_head_after_git(value: &str) -> Result<bool, String> {
    let head = current_short_head()?;
    Ok(has_git_commit(value, &head))
}

fn has_git_commit(value: &str, head: &str) -> bool {
    let mut tokens = value.split_whitespace();
    while let Some(token) = tokens.next() {
        if token == "git" && tokens.next() == Some(head) {
            return true;
        }
    }
    false
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
    use super::has_git_commit;

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
}
