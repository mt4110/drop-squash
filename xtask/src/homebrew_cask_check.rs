use std::path::{Path, PathBuf};

mod parse;

pub fn run(args: Vec<String>) -> Result<(), String> {
    let request = Request::parse(args)?;
    check(&request.cask, &request.notes)?;
    println!("Homebrew cask checks passed");
    Ok(())
}

fn check(cask_path: &Path, notes_path: &Path) -> Result<(), String> {
    let cask = parse::cask(&read(cask_path, "Homebrew cask")?)?;
    let notes = parse::notes(&read(notes_path, "release notes")?)?;
    if cask.version != notes.version {
        return Err("Homebrew cask version must match release notes Version".to_string());
    }
    if cask.url != notes.artifact_url {
        return Err("Homebrew cask url must match release notes Artifact URL".to_string());
    }
    if cask.sha256 != notes.sha256 {
        return Err("Homebrew cask sha256 must match release notes SHA-256".to_string());
    }
    Ok(())
}

fn read(path: &Path, label: &str) -> Result<String, String> {
    std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read {label} {}: {error}", path.display()))
}

struct Request {
    cask: PathBuf,
    notes: PathBuf,
}

impl Request {
    fn parse(args: Vec<String>) -> Result<Self, String> {
        match args.as_slice() {
            [cask, notes] => {
                let request = Self {
                    cask: PathBuf::from(cask),
                    notes: PathBuf::from(notes),
                };
                request.validate()?;
                Ok(request)
            }
            _ => Err("homebrew-cask-check requires <dropsquash.rb> <release-notes.md>".into()),
        }
    }

    fn validate(&self) -> Result<(), String> {
        if self.cask.file_name().and_then(|value| value.to_str()) != Some("dropsquash.rb") {
            return Err("homebrew-cask-check cask must be named dropsquash.rb".to_string());
        }
        if self.notes.extension().and_then(|value| value.to_str()) != Some("md") {
            return Err("homebrew-cask-check release notes must be a .md file".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
