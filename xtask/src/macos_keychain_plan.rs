use std::path::PathBuf;

pub fn run(args: Vec<String>) -> Result<(), String> {
    let request = Request::parse(args)?;
    for line in commands(&request)? {
        println!("{line}");
    }
    Ok(())
}

fn commands(request: &Request) -> Result<Vec<String>, String> {
    validate_work_dir(&request.work_dir)?;
    let cert = request.work_dir.join("DropSquash-signing.p12");
    let keychain = request.work_dir.join("DropSquash-signing.keychain-db");
    let cert = quote_path(&cert);
    let keychain = quote_path(&keychain);
    Ok(vec![
        format!("mkdir -p {}", quote_path(&request.work_dir)),
        format!("printf %s \"$APPLE_CERTIFICATE\" | base64 --decode > {cert}"),
        format!("security create-keychain -p \"$APPLE_KEYCHAIN_PASSWORD\" {keychain}"),
        format!("security set-keychain-settings -lut 21600 {keychain}"),
        format!("security unlock-keychain -p \"$APPLE_KEYCHAIN_PASSWORD\" {keychain}"),
        format!("security list-keychains -d user -s {keychain}"),
        format!(
            "security import {cert} -P \"$APPLE_CERTIFICATE_PASSWORD\" -A -t cert -f pkcs12 -k {keychain}"
        ),
        format!(
            "security set-key-partition-list -S apple-tool:,apple: -s -k \"$APPLE_KEYCHAIN_PASSWORD\" {keychain}"
        ),
    ])
}

fn validate_work_dir(path: &std::path::Path) -> Result<(), String> {
    if !path.is_absolute() {
        return Err("macos-keychain-plan requires an absolute work directory".to_string());
    }
    if path.starts_with("/nix/store") {
        return Err("macos-keychain-plan work directory must not be in /nix/store".to_string());
    }
    Ok(())
}

fn quote_path(path: &std::path::Path) -> String {
    let value = path.display().to_string();
    if value
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || "/._-".contains(ch))
    {
        return value;
    }
    format!("'{}'", value.replace('\'', "'\\''"))
}

struct Request {
    work_dir: PathBuf,
}

impl Request {
    fn parse(args: Vec<String>) -> Result<Self, String> {
        match args.as_slice() {
            [work_dir] => Ok(Self {
                work_dir: PathBuf::from(work_dir),
            }),
            _ => Err("macos-keychain-plan requires <absolute-work-dir>".to_string()),
        }
    }
}

#[cfg(test)]
mod tests;
