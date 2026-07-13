use std::path::PathBuf;

use crate::artifact_check;

mod validation;

pub fn run(args: Vec<String>) -> Result<(), String> {
    let request = Request::parse(args)?;
    println!("{}", command(&request)?);
    Ok(())
}

fn command(request: &Request) -> Result<String, String> {
    validation::tag(&request.tag)?;
    artifact_check::read_checked(&request.dmg, "GitHub Release DMG")?;
    crate::artifact_age::require_not_older_than_head(&request.dmg, "GitHub Release")?;
    let digest = validation::checksum(&request.checksum)?;
    validation::notes(&request.notes, &request.tag, &digest)?;
    Ok(shell_command(&[
        "gh",
        "release",
        "create",
        &request.tag,
        &request.dmg.display().to_string(),
        &request.checksum.display().to_string(),
        "--title",
        &request.tag,
        "--draft",
        "--notes-file",
        &request.notes.display().to_string(),
    ]))
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
