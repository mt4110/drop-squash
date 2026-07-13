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
    Ok(vec![
        format!("security delete-keychain {}", quote_path(&keychain)),
        format!("rm -f {}", quote_path(&cert)),
        format!("rm -f {}", quote_path(&keychain)),
    ])
}

fn validate_work_dir(path: &std::path::Path) -> Result<(), String> {
    if !path.is_absolute() {
        return Err("macos-keychain-cleanup-plan requires an absolute work directory".to_string());
    }
    if path.starts_with("/nix/store") {
        return Err(
            "macos-keychain-cleanup-plan work directory must not be in /nix/store".to_string(),
        );
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
            _ => Err("macos-keychain-cleanup-plan requires <absolute-work-dir>".to_string()),
        }
    }
}

#[cfg(test)]
mod tests;
