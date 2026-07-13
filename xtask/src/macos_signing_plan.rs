use std::path::PathBuf;

const SIGNED_NAME: &str = "DropSquash.dmg";

pub fn run(args: Vec<String>) -> Result<(), String> {
    let request = Request::parse(args)?;
    for line in plan(&request)? {
        println!("{line}");
    }
    Ok(())
}

fn plan(request: &Request) -> Result<Vec<String>, String> {
    let target = request.output_dir.join(SIGNED_NAME);
    if request.unsigned == target {
        return Err("macos-signing-plan output must not overwrite the unsigned input".to_string());
    }
    Ok(vec![
        format!(
            "cargo run -p xtask -- signed-dmg-prepare {} {}",
            display(&request.unsigned),
            display(&request.output_dir)
        ),
        format!(
            "cargo run -p xtask -- signed-dmg-copy {} {}",
            display(&request.unsigned),
            display(&request.output_dir)
        ),
        format!(
            "cargo run -p xtask -- macos-codesign-plan {} 'Developer ID Application: ...'",
            display(&target)
        ),
        format!(
            "cargo run -p xtask -- macos-notary-plan {} --api-key",
            display(&target)
        ),
        format!("xcrun stapler validate {}", display(&target)),
        format!(
            "cargo run -p xtask -- signed-dmg-check {} {}",
            display(&target),
            display(&request.unsigned)
        ),
    ])
}

fn display(path: &std::path::Path) -> String {
    path.display().to_string()
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
                "macos-signing-plan requires <unsigned DropSquash.dmg> <signed-output-dir>"
                    .to_string(),
            ),
        }
    }
}

#[cfg(test)]
mod tests;
