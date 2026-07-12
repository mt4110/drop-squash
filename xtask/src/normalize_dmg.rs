use std::fs;
use std::path::{Path, PathBuf};

const CANONICAL_NAME: &str = "DropSquash.dmg";

pub fn run(args: Vec<String>) -> Result<(), String> {
    let directory = match args.as_slice() {
        [directory] => PathBuf::from(directory),
        _ => return Err("normalize-dmg requires exactly one bundle/dmg directory".to_string()),
    };
    let path = normalize(&directory)?;
    println!("normalized DMG: {}", path.display());
    Ok(())
}

fn normalize(directory: &Path) -> Result<PathBuf, String> {
    let artifacts = dmg_artifacts(directory)?;
    let source = single_artifact(directory, &artifacts)?;
    let target = directory.join(CANONICAL_NAME);
    if source == target {
        return Ok(target);
    }
    fs::rename(&source, &target).map_err(|error| {
        format!(
            "failed to rename {} to {}: {error}",
            source.display(),
            target.display()
        )
    })?;
    Ok(target)
}

fn dmg_artifacts(directory: &Path) -> Result<Vec<PathBuf>, String> {
    let entries = fs::read_dir(directory)
        .map_err(|error| format!("failed to read {}: {error}", directory.display()))?;
    let mut artifacts = Vec::new();
    for entry in entries {
        let path = entry.map_err(|error| error.to_string())?.path();
        if path.extension().and_then(|value| value.to_str()) == Some("dmg") {
            artifacts.push(path);
        }
    }
    artifacts.sort();
    Ok(artifacts)
}

fn single_artifact(directory: &Path, artifacts: &[PathBuf]) -> Result<PathBuf, String> {
    match artifacts {
        [path] => require_dropsquash_name(path),
        [] => Err(format!("no DMG artifacts found in {}", directory.display())),
        _ => Err(format!(
            "expected exactly one DMG artifact in {}, found {}",
            directory.display(),
            artifacts.len()
        )),
    }
}

fn require_dropsquash_name(path: &Path) -> Result<PathBuf, String> {
    let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
        return Err(format!("DMG artifact has no file name: {}", path.display()));
    };
    if name == CANONICAL_NAME || name.starts_with("DropSquash_") {
        return Ok(path.to_path_buf());
    }
    Err(format!(
        "DMG artifact must be {CANONICAL_NAME} or a DropSquash build artifact: {name}"
    ))
}

#[cfg(test)]
mod tests;
