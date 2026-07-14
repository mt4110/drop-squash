use std::path::PathBuf;

#[cfg(test)]
mod tests;

const MARKER: &str = "Prepared manual QA draft only.";
const USAGE: &str = "usage: cargo run -p xtask -- manual-qa-clean-draft <manual-qa.md>";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let path = parse_args(args)?;
    let text = std::fs::read_to_string(&path)
        .map_err(|error| format!("failed to read manual QA file: {error}"))?;
    let cleaned = clean(&text)?;
    std::fs::write(&path, cleaned)
        .map_err(|error| format!("failed to write manual QA file: {error}"))?;
    println!("cleaned prepared manual QA draft: {}", path.display());
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<PathBuf, String> {
    match args.as_slice() {
        [path] if path != "--help" && path != "-h" => Ok(PathBuf::from(path)),
        _ => Err(USAGE.to_string()),
    }
}

fn clean(text: &str) -> Result<String, String> {
    if !text.contains(MARKER) {
        return Err("manual QA file does not look like a prepared draft".to_string());
    }
    let lines = text
        .lines()
        .filter(|line| line.starts_with('|'))
        .collect::<Vec<_>>();
    if lines.is_empty() {
        return Err("prepared manual QA draft contains no table rows".to_string());
    }
    Ok(format!("{}\n", lines.join("\n")))
}
