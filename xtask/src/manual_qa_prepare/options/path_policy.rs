use std::path::{Component, Path, PathBuf};

pub(super) fn require_outside_repo(label: &str, path: &Path) -> Result<(), String> {
    let repo = std::env::current_dir().map_err(|error| error.to_string())?;
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        repo.join(path)
    };
    if normalize(&absolute).starts_with(normalize(&repo)) {
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
