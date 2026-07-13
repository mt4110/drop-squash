use std::path::{Component, Path, PathBuf};

#[derive(Debug)]
pub(super) struct Input {
    pub(super) artifact: PathBuf,
    pub(super) artifact_url: String,
    pub(super) markdown_output: Option<PathBuf>,
}

impl Input {
    pub(super) fn parse(args: Vec<String>) -> Result<Self, String> {
        let mut positional = Vec::new();
        let mut markdown_output = None;
        let mut args = args.into_iter();
        while let Some(arg) = args.next() {
            if arg == "--markdown-output" {
                if markdown_output.is_some() {
                    return Err("--markdown-output must be specified at most once".to_string());
                }
                let value = args
                    .next()
                    .ok_or_else(|| "--markdown-output requires a value".to_string())?;
                markdown_output = Some(PathBuf::from(value));
                continue;
            }
            positional.push(arg);
        }
        if positional.len() != 2 {
            return Err(usage());
        }
        if let Some(path) = &markdown_output {
            require_markdown_output(path)?;
        }
        Ok(Self {
            artifact: PathBuf::from(&positional[0]),
            artifact_url: positional[1].clone(),
            markdown_output,
        })
    }
}

fn usage() -> String {
    "release-notes-prepare requires <DropSquash.dmg> <Artifact URL> [--markdown-output <path>]"
        .to_string()
}

fn require_markdown_output(path: &Path) -> Result<(), String> {
    if !path.is_absolute() {
        return Err("--markdown-output must be an absolute .md path outside the repository".into());
    }
    if path.extension().and_then(|value| value.to_str()) != Some("md") {
        return Err("--markdown-output must point to a .md file".into());
    }
    let repo = std::env::current_dir().map_err(|error| error.to_string())?;
    if normalize(path).starts_with(normalize(&repo)) {
        return Err("--markdown-output must stay outside the repository".into());
    }
    Ok(())
}

fn normalize(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            other => normalized.push(other.as_os_str()),
        }
    }
    normalized
}
