use std::path::PathBuf;

use crate::artifact_check;

pub fn run(args: Vec<String>) -> Result<(), String> {
    let request = Request::parse(args)?;
    println!("{}", command(&request)?);
    Ok(())
}

fn command(request: &Request) -> Result<String, String> {
    artifact_check::read_checked(&request.target, "notary target")?;
    let mut args = vec![
        "xcrun".to_string(),
        "notarytool".into(),
        "submit".into(),
        request.target.display().to_string(),
        "--wait".into(),
    ];
    request.credentials.extend_args(&mut args)?;
    Ok(shell_command(&args))
}

enum Credentials {
    ApiKey,
    AppleId,
}

impl Credentials {
    fn extend_args(&self, args: &mut Vec<String>) -> Result<(), String> {
        match self {
            Self::ApiKey => {
                args.extend(["--key".into(), "$APPLE_API_KEY_PATH".into()]);
                args.extend(["--key-id".into(), "$APPLE_API_KEY".into()]);
                args.extend(["--issuer".into(), "$APPLE_API_ISSUER".into()]);
            }
            Self::AppleId => {
                args.extend(["--apple-id".into(), "$APPLE_ID".into()]);
                args.extend(["--password".into(), "$APPLE_PASSWORD".into()]);
                args.extend(["--team-id".into(), "$APPLE_TEAM_ID".into()]);
            }
        }
        Ok(())
    }
}

fn shell_command(args: &[String]) -> String {
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
    target: PathBuf,
    credentials: Credentials,
}

impl Request {
    fn parse(args: Vec<String>) -> Result<Self, String> {
        match args.as_slice() {
            [target, flag] if flag == "--api-key" => Ok(Self {
                target: PathBuf::from(target),
                credentials: Credentials::ApiKey,
            }),
            [target, flag] if flag == "--apple-id" => Ok(Self {
                target: PathBuf::from(target),
                credentials: Credentials::AppleId,
            }),
            _ => Err(
                "macos-notary-plan requires <signed DropSquash.dmg> <--api-key|--apple-id>"
                    .to_string(),
            ),
        }
    }
}

#[cfg(test)]
mod tests;
