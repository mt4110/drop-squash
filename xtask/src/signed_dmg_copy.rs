use std::path::PathBuf;

use crate::signed_dmg_prepare;

pub fn run(args: Vec<String>) -> Result<(), String> {
    let request = Request::parse(args)?;
    let target = copy(&request)?;
    println!(
        "copied unsigned DMG to signing target: {}",
        target.display()
    );
    Ok(())
}

fn copy(request: &Request) -> Result<PathBuf, String> {
    let target = signed_dmg_prepare::prepare_paths(&request.unsigned, &request.output_dir)?;
    std::fs::copy(&request.unsigned, &target).map_err(|error| {
        format!(
            "failed to copy unsigned DMG {} to {}: {error}",
            request.unsigned.display(),
            target.display()
        )
    })?;
    Ok(target)
}

struct Request {
    unsigned: PathBuf,
    output_dir: PathBuf,
}

impl Request {
    fn parse(args: Vec<String>) -> Result<Self, String> {
        match args.as_slice() {
            [unsigned, output_dir] => Ok(Self {
                unsigned: PathBuf::from(unsigned),
                output_dir: PathBuf::from(output_dir),
            }),
            _ => Err(
                "signed-dmg-copy requires <unsigned DropSquash.dmg> <signed-output-dir>"
                    .to_string(),
            ),
        }
    }
}

#[cfg(test)]
mod tests;
