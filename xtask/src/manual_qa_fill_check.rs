use std::path::PathBuf;

const LABEL: &str = "`cargo run -p xtask -- manual-qa-check`";
const RESULT: &str = "manual-qa-check passed";
const USAGE: &str = "usage: cargo run -p xtask -- manual-qa-fill-check <manual-qa.md>";

#[cfg(test)]
mod tests;

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let path = parse_args(args)?;
    let text = std::fs::read_to_string(&path)
        .map_err(|error| format!("failed to read manual QA file: {error}"))?;
    let filled = fill_row(&text)?;
    let missing = crate::manual_qa_check::check_text(&filled);
    if !missing.is_empty() {
        return Err(missing.join("\n"));
    }
    std::fs::write(&path, filled)
        .map_err(|error| format!("failed to write manual QA file: {error}"))?;
    println!("filled manual QA check row: {}", path.display());
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<PathBuf, String> {
    match args.as_slice() {
        [path] if path != "--help" && path != "-h" => Ok(PathBuf::from(path)),
        _ => Err(USAGE.to_string()),
    }
}

fn fill_row(text: &str) -> Result<String, String> {
    let mut found = false;
    let lines = text
        .lines()
        .map(|line| {
            let cells = line
                .trim_matches('|')
                .split('|')
                .map(str::trim)
                .collect::<Vec<_>>();
            if cells.first().copied() == Some(LABEL) {
                found = true;
                return format!("| {} | {} | {} |", cells[0], cells[1], RESULT);
            }
            line.to_string()
        })
        .collect::<Vec<_>>();
    found
        .then(|| lines.join("\n"))
        .ok_or_else(|| "manual QA file is missing the manual-qa-check row".to_string())
}
