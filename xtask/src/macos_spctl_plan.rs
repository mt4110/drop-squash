use std::path::PathBuf;

use crate::artifact_check;

pub fn run(args: Vec<String>) -> Result<(), String> {
    let request = Request::parse(args)?;
    println!("{}", command(&request)?);
    Ok(())
}

fn command(request: &Request) -> Result<String, String> {
    artifact_check::read_checked(&request.target, "spctl target")?;
    let target = request.target.display().to_string();
    Ok(shell_command(&[
        "spctl",
        "--assess",
        "--type",
        "open",
        "--verbose=4",
        &target,
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
        .all(|ch| ch.is_ascii_alphanumeric() || "/._-=".contains(ch))
    {
        return value.to_string();
    }
    format!("'{}'", value.replace('\'', "'\\''"))
}

struct Request {
    target: PathBuf,
}

impl Request {
    fn parse(args: Vec<String>) -> Result<Self, String> {
        match args.as_slice() {
            [target] => Ok(Self {
                target: PathBuf::from(target),
            }),
            _ => Err("macos-spctl-plan requires <signed DropSquash.dmg>".to_string()),
        }
    }
}

#[cfg(test)]
mod tests;
