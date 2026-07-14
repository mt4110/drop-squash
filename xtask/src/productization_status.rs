mod model;
mod parse;
mod preflight;
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
    let mut output = render::text(&report);
    let preflight = preflight::lines(&report)?;
    if !preflight.is_empty() {
        output.push('\n');
        output.push_str(&preflight.join("\n"));
    }
    println!("{output}");
    Ok(())
}

#[cfg(test)]
mod tests;
