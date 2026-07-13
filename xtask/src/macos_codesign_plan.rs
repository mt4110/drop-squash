use std::path::PathBuf;

use crate::artifact_check;

pub fn run(args: Vec<String>) -> Result<(), String> {
    let request = Request::parse(args)?;
    println!("{}", command(&request)?);
    Ok(())
}

fn command(request: &Request) -> Result<String, String> {
    artifact_check::read_checked(&request.target, "codesign target")?;
    validate_identity(&request.identity)?;
    Ok(shell_command(&[
        "codesign",
        "--force",
        "--options",
        "runtime",
        "--timestamp",
        "--sign",
        &request.identity,
        &request.target.display().to_string(),
    ]))
}

fn validate_identity(identity: &str) -> Result<(), String> {
    if identity.contains("Developer ID Application") {
        return Ok(());
    }
    Err("codesign identity must be a Developer ID Application identity".to_string())
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
        .all(|ch| ch.is_ascii_alphanumeric() || "/._-".contains(ch))
    {
        return value.to_string();
    }
    format!("'{}'", value.replace('\'', "'\\''"))
}

struct Request {
    target: PathBuf,
    identity: String,
}

impl Request {
    fn parse(args: Vec<String>) -> Result<Self, String> {
        match args.as_slice() {
            [target, identity] => Ok(Self {
                target: PathBuf::from(target),
                identity: identity.to_string(),
            }),
            _ => Err(
                "macos-codesign-plan requires <signed DropSquash.dmg> <Developer ID identity>"
                    .to_string(),
            ),
        }
    }
}

#[cfg(test)]
mod tests;
