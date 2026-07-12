use crate::dmg;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::process::Command;

mod output;
mod url;

const TAURI_CONFIG: &str = "apps/desktop/src-tauri/tauri.conf.json";

pub fn run(args: Vec<String>) -> Result<(), String> {
    let input = Input::parse(args)?;
    let notes = PreparedNotes::current(&input)?;
    for line in notes.lines() {
        println!("{line}");
    }
    Ok(())
}

#[derive(Debug)]
struct Input {
    artifact: PathBuf,
    artifact_url: String,
}

impl Input {
    fn parse(args: Vec<String>) -> Result<Self, String> {
        if args.len() != 2 {
            return Err("release-notes-prepare requires <DropSquash.dmg> <Artifact URL>".into());
        }
        Ok(Self {
            artifact: PathBuf::from(&args[0]),
            artifact_url: args[1].clone(),
        })
    }
}

struct PreparedNotes {
    version: String,
    artifact_url: String,
    sha256: String,
    commit: String,
}

impl PreparedNotes {
    fn current(input: &Input) -> Result<Self, String> {
        let bytes = dmg::read(&input.artifact, "release notes artifact")?;
        require_dmg_name(&input.artifact)?;
        let version = read_version(Path::new(TAURI_CONFIG))?;
        url::validate(&input.artifact_url, &version)?;
        Ok(Self {
            version,
            artifact_url: input.artifact_url.clone(),
            sha256: sha256_hex(&bytes),
            commit: git_commit()?,
        })
    }

    fn lines(&self) -> Vec<String> {
        output::lines(output::Fields {
            version: &self.version,
            artifact_url: &self.artifact_url,
            sha256: &self.sha256,
            commit: &self.commit,
        })
    }
}

fn require_dmg_name(path: &Path) -> Result<(), String> {
    if path.file_name().and_then(|value| value.to_str()) == Some("DropSquash.dmg") {
        return Ok(());
    }
    Err("release notes artifact must be named DropSquash.dmg".into())
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
        return Err("git rev-parse failed".into());
    }
    String::from_utf8(output.stdout)
        .map_err(|error| error.to_string())
        .map(|value| value.trim().to_string())
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[cfg(test)]
mod tests;
