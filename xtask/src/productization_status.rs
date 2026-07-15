mod model;
mod parse;
mod preflight;
mod render;

use std::path::PathBuf;

pub fn run(args: Vec<String>) -> Result<(), String> {
    let (path, track) = parse_args(args)?;
    let text = std::fs::read_to_string(&path)
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    let report = parse::report(&text)?;
    let mut output = render::text(&report, track.as_deref())?;
    let preflight = preflight::lines_for(&report, track.as_deref())?;
    if !preflight.is_empty() {
        output.push('\n');
        output.push_str(&preflight.join("\n"));
    }
    println!("{output}");
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<(PathBuf, Option<String>), String> {
    match args.as_slice() {
        [] => Ok((PathBuf::from("docs/release-blockers.md"), None)),
        [path] if path != "--track" => Ok((PathBuf::from(path), None)),
        [flag, track] if *flag == "--track" => {
            Ok((PathBuf::from("docs/release-blockers.md"), Some(track.clone())))
        }
        [path, flag, track] if *flag == "--track" => {
            Ok((PathBuf::from(path), Some(track.clone())))
        }
        _ => Err(
            "usage: cargo run -p xtask -- productization-status [release-blockers.md] [--track <order|name>]".into(),
        ),
    }
}

#[cfg(test)]
mod tests;
