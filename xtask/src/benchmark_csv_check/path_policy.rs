use std::path::{Component, Path, PathBuf};

pub(super) fn validate(path: &Path) -> Result<(), String> {
    if !path.is_absolute() || !has_csv_extension(path) {
        return Err(message(path));
    }
    if is_inside_repo(path)? {
        return Err(message(path));
    }
    Ok(())
}

fn has_csv_extension(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("csv"))
}

fn is_inside_repo(path: &Path) -> Result<bool, String> {
    let cwd = std::env::current_dir().map_err(|error| error.to_string())?;
    Ok(normalize(path).starts_with(normalize(&cwd)))
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

fn message(path: &Path) -> String {
    format!(
        "benchmark CSV evidence must be an absolute .csv path outside the repository: {}",
        path.display()
    )
}
