use serde_json::Value;
use std::{path::Path, process::Command};

const TAURI_CONFIG: &str = "apps/desktop/src-tauri/tauri.conf.json";
const PRODUCT_NAME: &str = "DropSquash";

#[derive(Debug)]
pub(super) struct BuildIdentity {
    version: String,
    commit: String,
}

impl BuildIdentity {
    pub(super) fn current() -> Result<Self, String> {
        Self::from_parts(read_version(Path::new(TAURI_CONFIG))?, git_commit()?)
    }

    pub(super) fn app_build(&self) -> String {
        format!("{PRODUCT_NAME} {} git {}", self.version, self.commit)
    }

    fn from_parts(version: String, commit: String) -> Result<Self, String> {
        if !is_semver(&version) {
            return Err("app version must use major.minor.patch digits".to_string());
        }
        if commit.len() < 7 || !commit.chars().all(|value| value.is_ascii_hexdigit()) {
            return Err("git commit must be a short hexadecimal revision".to_string());
        }
        Ok(Self { version, commit })
    }
}

fn read_version(path: &Path) -> Result<String, String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    let config: Value = serde_json::from_str(&text).map_err(|error| error.to_string())?;
    config
        .get("version")
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| format!("{} is missing version", path.display()))
}

fn git_commit() -> Result<String, String> {
    let output = Command::new("git")
        .args(["rev-parse", "--short=7", "HEAD"])
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err("git rev-parse failed".to_string());
    }
    String::from_utf8(output.stdout)
        .map_err(|error| error.to_string())
        .map(|value| value.trim().to_string())
}

fn is_semver(version: &str) -> bool {
    let parts = version.split('.').collect::<Vec<_>>();
    parts.len() == 3
        && parts
            .iter()
            .all(|part| !part.is_empty() && part.chars().all(|value| value.is_ascii_digit()))
}

#[cfg(test)]
mod tests;
