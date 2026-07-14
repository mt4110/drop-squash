mod model;
mod parse;
mod render;

use std::path::PathBuf;

pub fn run(args: Vec<String>) -> Result<(), String> {
    let path = match args.as_slice() {
        [] => PathBuf::from("docs/release-blockers.md"),
        [path] => PathBuf::from(path),
        _ => {
            return Err(
                "usage: cargo run -p xtask -- productization-status [release-blockers.md]".into(),
            )
        }
    };
    let text = std::fs::read_to_string(&path)
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    let report = parse::report(&text)?;
    println!("{}", render::text(&report));
    Ok(())
}

#[cfg(test)]
mod tests;
