use std::path::PathBuf;

const USAGE: &str = "usage: cargo run -p xtask -- manual-qa-pending <manual-qa.md>";

#[cfg(test)]
mod tests;

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let path = parse_args(args)?;
    let text = std::fs::read_to_string(&path)
        .map_err(|error| format!("failed to read manual QA file: {error}"))?;
    let pending = pending_rows(&text);
    if pending.is_empty() {
        println!("manual QA has no pending result rows");
        return Ok(());
    }
    for (label, expected) in pending {
        println!("- {label}: {expected}");
    }
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<PathBuf, String> {
    match args.as_slice() {
        [path] if path != "--help" && path != "-h" => Ok(PathBuf::from(path)),
        _ => Err(USAGE.to_string()),
    }
}

fn pending_rows(text: &str) -> Vec<(String, String)> {
    text.lines()
        .filter_map(pending_row)
        .map(|cells| (cells[0].trim().to_string(), cells[1].trim().to_string()))
        .collect()
}

fn pending_row(line: &str) -> Option<Vec<&str>> {
    if !line.starts_with('|') || line.contains("---") {
        return None;
    }
    let cells = line.trim_matches('|').split('|').collect::<Vec<_>>();
    if cells.len() != 4 || !cells[3].trim().is_empty() {
        return None;
    }
    Some(cells)
}
