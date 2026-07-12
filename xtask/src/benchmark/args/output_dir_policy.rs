use std::path::Path;

pub(super) fn validate(release_set: bool, output_dir: &Path) -> Result<(), String> {
    if !release_set {
        return Ok(());
    }
    if !output_dir.is_absolute() {
        return Err(message());
    }
    let cwd = std::env::current_dir().map_err(|error| error.to_string())?;
    if output_dir.starts_with(cwd) {
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
    let cwd = std::env::current_dir().map_err(|error| error.to_string())?;
    if csv_output.starts_with(cwd) {
        return Err(csv_message());
    }
    Ok(())
}

fn message() -> String {
    "benchmark --release-set requires an absolute --output-dir outside the repository".to_string()
}

fn csv_message() -> String {
    "benchmark --release-set requires an absolute --csv-output outside the repository".to_string()
}
