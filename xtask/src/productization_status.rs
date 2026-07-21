mod model;
mod parse;
mod preflight;
mod render;
mod scope;

use std::path::PathBuf;

pub fn run(args: Vec<String>) -> Result<(), String> {
    let (path, track) = parse_args(args)?;
    let text = std::fs::read_to_string(&path)
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    let report = parse::report(&text)?;
    let scope = scope::load()?;
    let track = normalize_track(track);
    let mut output = render::text(&report, track.as_deref(), &scope)?;
    let preflight = preflight::lines_for(&report, track.as_deref(), &scope)?;
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
            "usage: cargo run -p xtask -- productization-status [release-blockers.md] [--track <order|name|Paid beta>]".into(),
        ),
    }
}

fn normalize_track(track: Option<String>) -> Option<String> {
    track.filter(|value| !is_paid_beta_alias(value))
}

fn is_paid_beta_alias(value: &str) -> bool {
    let normalized = value.trim().to_ascii_lowercase();
    matches!(normalized.as_str(), "paid beta" | "paid-beta" | "paid_beta")
}

#[cfg(test)]
mod tests;
