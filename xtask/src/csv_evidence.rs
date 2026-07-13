use std::path::{Path, PathBuf};

pub(crate) fn existing_outside_repo_path(value: &str) -> Option<PathBuf> {
    value
        .split_whitespace()
        .filter_map(absolute_path)
        .find(|path| path.is_file() && outside_repo(path))
}

pub(crate) fn canonical_existing_path(value: &str) -> Option<PathBuf> {
    value
        .split_whitespace()
        .filter_map(absolute_path)
        .find(|path| path.is_file())
        .and_then(|path| path.canonicalize().ok())
}

pub(crate) fn absolute_path(token: &str) -> Option<PathBuf> {
    absolute_csv(csv_token(token))
}

pub(crate) fn outside_repo_path(path: &Path) -> bool {
    outside_repo(path)
}

fn absolute_csv(token: &str) -> Option<PathBuf> {
    let path = PathBuf::from(token);
    (path.is_absolute() && path.extension().and_then(|value| value.to_str()) == Some("csv"))
        .then_some(path)
}

fn csv_token(token: &str) -> &str {
    let token = token
        .trim_matches(|character: char| matches!(character, ',' | '.' | ';' | ')' | '(' | '`'));
    token
        .strip_prefix("csv=")
        .or_else(|| token.strip_prefix("CSV="))
        .or_else(|| token.strip_prefix("csv:"))
        .or_else(|| token.strip_prefix("CSV:"))
        .unwrap_or(token)
}

fn outside_repo(path: &Path) -> bool {
    let Ok(repo) = std::env::current_dir() else {
        return false;
    };
    !normalize(path).starts_with(normalize(&repo))
}

fn normalize(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                normalized.pop();
            }
            other => normalized.push(other.as_os_str()),
        }
    }
    normalized
}
