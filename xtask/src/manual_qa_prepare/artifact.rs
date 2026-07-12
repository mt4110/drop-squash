use crate::dmg;
use std::path::{Path, PathBuf};

use super::options::Options;

const DEFAULT_APP: &str = "target/release/bundle/macos/DropSquash.app";

pub(super) fn qa_artifact(options: &Options) -> Result<Option<PathBuf>, String> {
    if let Some(path) = &options.app_artifact {
        validate(path)?;
        return Ok(Some(path.clone()));
    }
    let default = PathBuf::from(DEFAULT_APP);
    if default.exists() {
        validate(&default)?;
        return Ok(Some(default));
    }
    Ok(None)
}

fn validate(path: &Path) -> Result<(), String> {
    let extension = path.extension().and_then(|value| value.to_str());
    match extension {
        Some("app") if path.is_dir() => validate_app(path),
        Some("app") => Err("manual QA DropSquash.app artifact must be a directory".to_string()),
        Some("dmg") if path.is_file() => validate_dmg(path),
        Some("dmg") => Err("manual QA DropSquash.dmg artifact must be a file".to_string()),
        _ => Err("manual QA App artifact must be DropSquash.app or DropSquash.dmg".to_string()),
    }
}

fn validate_app(path: &Path) -> Result<(), String> {
    if path.file_name().and_then(|value| value.to_str()) == Some("DropSquash.app") {
        return Ok(());
    }
    Err("manual QA .app artifact must be named DropSquash.app".to_string())
}

fn validate_dmg(path: &Path) -> Result<(), String> {
    if path.file_name().and_then(|value| value.to_str()) != Some("DropSquash.dmg") {
        return Err("manual QA .dmg artifact must be named DropSquash.dmg".to_string());
    }
    dmg::read(path, "manual QA artifact").map(|_| ())
}

#[cfg(test)]
mod tests;
