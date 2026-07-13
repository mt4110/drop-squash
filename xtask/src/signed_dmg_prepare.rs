use std::path::{Path, PathBuf};

use crate::artifact_check;

const CANONICAL_NAME: &str = "DropSquash.dmg";

pub fn run(args: Vec<String>) -> Result<(), String> {
    let request = Request::parse(args)?;
    let target = prepare(&request)?;
    println!("signed DMG target: {}", target.display());
    Ok(())
}

fn prepare(request: &Request) -> Result<PathBuf, String> {
    artifact_check::read_checked(&request.source, "unsigned signing input")?;
    std::fs::create_dir_all(&request.output_dir).map_err(|error| {
        format!(
            "failed to create signed DMG output directory {}: {error}",
            request.output_dir.display()
        )
    })?;
    let target = request.output_dir.join(CANONICAL_NAME);
    reject_same_path(&request.source, &target)?;
    if target.exists() {
        return Err(format!(
            "signed DMG target already exists: {}",
            target.display()
        ));
    }
    Ok(target)
}

fn reject_same_path(source: &Path, target: &Path) -> Result<(), String> {
    let source = source.canonicalize().map_err(|error| error.to_string())?;
    let parent = target
        .parent()
        .ok_or_else(|| "signed DMG target has no parent directory".to_string())?
        .canonicalize()
        .map_err(|error| error.to_string())?;
    if source == parent.join(CANONICAL_NAME) {
        return Err("signed DMG output must not overwrite the unsigned input".to_string());
    }
    Ok(())
}

struct Request {
    source: PathBuf,
    output_dir: PathBuf,
}

impl Request {
    fn parse(args: Vec<String>) -> Result<Self, String> {
        match args.as_slice() {
            [source, output_dir] => Ok(Self {
                source: PathBuf::from(source),
                output_dir: PathBuf::from(output_dir),
            }),
            _ => Err(
                "signed-dmg-prepare requires <unsigned DropSquash.dmg> <signed-output-dir>"
                    .to_string(),
            ),
        }
    }
}

#[cfg(test)]
mod tests;
