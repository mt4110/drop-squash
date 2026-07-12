use std::path::{Component, Path, PathBuf};

pub(super) fn validate(release_set: bool, output_dir: &Path) -> Result<(), String> {
    if !release_set {
        return Ok(());
    }
    if !output_dir.is_absolute() {
        return Err(message());
    }
    if is_inside_repo(output_dir)? {
        return Err(message());
    }
    Ok(())
}

pub(super) fn validate_csv(release_set: bool, csv_output: Option<&Path>) -> Result<(), String> {
    if !release_set {
        return Ok(());
    }
    let csv_output = csv_output.ok_or_else(csv_message)?;
    if !csv_output.is_absolute() {
        return Err(csv_message());
    }
    if is_inside_repo(csv_output)? {
        return Err(csv_message());
    }
    if !has_csv_extension(csv_output) {
        return Err(csv_message());
    }
    Ok(())
}

fn message() -> String {
    "benchmark --release-set requires an absolute --output-dir outside the repository".to_string()
}

fn csv_message() -> String {
    "benchmark --release-set requires an absolute .csv --csv-output outside the repository"
        .to_string()
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
