use std::path::{Component, Path, PathBuf};

pub(super) fn require_outside_repo(label: &str, path: &Path) -> Result<(), String> {
    if !path.is_absolute() {
        return Err(format!(
            "manual QA {label} must be an absolute path outside the repository: {}",
            path.display()
        ));
    }
    let repo = std::env::current_dir().map_err(|error| error.to_string())?;
    if normalize(path).starts_with(normalize(&repo)) {
        return Err(format!(
            "manual QA {label} must stay outside the repository: {}",
            path.display()
        ));
    }
    Ok(())
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
