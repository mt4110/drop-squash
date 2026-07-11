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
        Some("app") if path.is_dir() => Ok(()),
        Some("app") => Err("manual QA .app artifact must be a directory".to_string()),
        Some("dmg") if path.is_file() => dmg::read(path, "manual QA artifact").map(|_| ()),
        Some("dmg") => Err("manual QA .dmg artifact must be a file".to_string()),
        _ => Err("manual QA App artifact must be a .app or .dmg".to_string()),
    }
}

#[cfg(test)]
mod tests;
