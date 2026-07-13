use std::path::{Component, Path, PathBuf};

pub(super) fn outside_repo(value: &str) -> bool {
    value.split_whitespace().any(|token| {
        let path = csv_token(token);
        is_absolute_csv_outside_repo(path)
    })
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

fn is_absolute_csv_outside_repo(value: &str) -> bool {
    let path = Path::new(value);
    if !path.is_absolute() || path.extension().and_then(|value| value.to_str()) != Some("csv") {
        return false;
    }
    let Ok(repo) = std::env::current_dir() else {
        return false;
    };
    path.is_file() && !normalize(path).starts_with(normalize(&repo))
}

fn normalize(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            other => normalized.push(other.as_os_str()),
        }
    }
    normalized
}
