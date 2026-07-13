use std::path::{Path, PathBuf};

use crate::artifact_check;

pub fn run(args: Vec<String>) -> Result<(), String> {
    let request = Request::parse(args)?;
    println!("{}", command(&request)?);
    Ok(())
}

fn command(request: &Request) -> Result<String, String> {
    validate_tag(&request.tag)?;
    artifact_check::read_checked(&request.dmg, "GitHub Release DMG")?;
    validate_checksum(&request.checksum)?;
    validate_notes(&request.notes)?;
    Ok(shell_command(&[
        "gh",
        "release",
        "create",
        &request.tag,
        &request.dmg.display().to_string(),
        &request.checksum.display().to_string(),
        "--title",
        &request.tag,
        "--notes-file",
        &request.notes.display().to_string(),
    ]))
}

fn validate_tag(tag: &str) -> Result<(), String> {
    if tag.starts_with('v') && tag.matches('.').count() == 2 {
        return Ok(());
    }
    Err("GitHub Release tag must look like v1.2.3".to_string())
}

fn validate_checksum(path: &Path) -> Result<(), String> {
    if path.file_name().and_then(|value| value.to_str()) != Some("SHA256SUMS") {
        return Err("GitHub Release checksum must be named SHA256SUMS".to_string());
    }
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read SHA256SUMS: {error}"))?;
    if text.contains("  DropSquash.dmg") && !text.contains("/nix/store") {
        return Ok(());
    }
    Err("SHA256SUMS must contain the DropSquash.dmg checksum line".to_string())
}

fn validate_notes(path: &Path) -> Result<(), String> {
    if path.extension().and_then(|value| value.to_str()) == Some("md") && path.is_file() {
        return Ok(());
    }
    Err("GitHub Release notes must be an existing .md file".to_string())
}

fn shell_command(args: &[&str]) -> String {
    args.iter()
        .map(|arg| shell_arg(arg))
        .collect::<Vec<_>>()
        .join(" ")
}

fn shell_arg(value: &str) -> String {
    if value
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || "/._-$".contains(ch))
    {
        return value.to_string();
    }
    format!("'{}'", value.replace('\'', "'\\''"))
}

struct Request {
    tag: String,
    dmg: PathBuf,
    checksum: PathBuf,
    notes: PathBuf,
}

impl Request {
    fn parse(args: Vec<String>) -> Result<Self, String> {
        match args.as_slice() {
            [tag, dmg, checksum, notes] => Ok(Self {
                tag: tag.to_string(),
                dmg: PathBuf::from(dmg),
                checksum: PathBuf::from(checksum),
                notes: PathBuf::from(notes),
            }),
            _ => Err("github-release-plan requires <tag> <DropSquash.dmg> <SHA256SUMS> <release-notes.md>".to_string()),
        }
    }
}

#[cfg(test)]
mod tests;
