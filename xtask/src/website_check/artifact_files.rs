use std::path::Path;

pub(super) fn check(root: &Path, errors: &mut Vec<String>) -> Result<(), String> {
    check_directory(root, errors)
}

fn check_directory(directory: &Path, errors: &mut Vec<String>) -> Result<(), String> {
    for entry in std::fs::read_dir(directory).map_err(|error| error.to_string())? {
        let path = entry.map_err(|error| error.to_string())?.path();
        if path.is_dir() {
            check_directory(&path, errors)?;
        } else if is_release_artifact(&path) {
            errors.push(format!("{} is a pre-release artifact file", path.display()));
        }
    }
    Ok(())
}

fn is_release_artifact(path: &Path) -> bool {
    let Some(extension) = path.extension().and_then(|value| value.to_str()) else {
        return false;
    };
    matches!(
        extension.to_ascii_lowercase().as_str(),
        "dmg" | "zip" | "pkg" | "app"
    )
}
