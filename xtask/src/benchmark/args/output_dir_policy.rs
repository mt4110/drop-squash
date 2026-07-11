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

fn message() -> String {
    "benchmark --release-set requires an absolute --output-dir outside the repository".to_string()
}
