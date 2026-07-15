use std::{collections::HashMap, path::PathBuf};

#[cfg(test)]
mod tests;

const USAGE: &str =
    "usage: cargo run -p xtask -- manual-qa-merge-prepared <prepared.md> [manual-qa.md]";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let (source, target) = parse_args(args)?;
    let source_text = std::fs::read_to_string(&source)
        .map_err(|error| format!("failed to read prepared manual QA: {error}"))?;
    let target_text = std::fs::read_to_string(&target)
        .map_err(|error| format!("failed to read target manual QA: {error}"))?;
    let merged = merge(&target_text, &source_text)?;
    std::fs::write(&target, merged)
        .map_err(|error| format!("failed to write target manual QA: {error}"))?;
    println!(
        "merged prepared manual QA rows from {} into {}",
        source.display(),
        target.display()
    );
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<(PathBuf, PathBuf), String> {
    match args.as_slice() {
        [source] if source != "--help" && source != "-h" => {
            Ok((PathBuf::from(source), PathBuf::from("docs/manual-qa.md")))
        }
        [source, target] if source != "--help" && source != "-h" => {
            Ok((PathBuf::from(source), PathBuf::from(target)))
        }
        _ => Err(USAGE.to_string()),
    }
}

fn merge(target: &str, source: &str) -> Result<String, String> {
    let updates = source
        .lines()
        .filter_map(update_row)
        .collect::<HashMap<_, _>>();
    if updates.is_empty() {
        return Err("prepared manual QA contains no filled rows to merge".to_string());
    }
    let mut merged = 0usize;
    let lines = target
        .lines()
        .map(|line| match row_label(line).and_then(|label| updates.get(label)) {
            Some(update) => {
                merged += 1;
                update.clone()
            }
            None => line.to_string(),
        })
        .collect::<Vec<_>>();
    if merged == 0 {
        return Err("prepared manual QA did not match any target rows".to_string());
    }
    Ok(format!("{}\n", lines.join("\n")))
}

fn update_row(line: &str) -> Option<(String, String)> {
    let cells = row_cells(line)?;
    has_merge_value(&cells).then(|| (cells[0].to_string(), line.to_string()))
}

fn row_label(line: &str) -> Option<&str> {
    let cells = row_cells(line)?;
    Some(cells[0])
}

fn row_cells(line: &str) -> Option<Vec<&str>> {
    let trimmed = line.trim();
    if !trimmed.starts_with('|') || !trimmed.ends_with('|') {
        return None;
    }
    let cells = trimmed[1..trimmed.len() - 1]
        .split('|')
        .map(str::trim)
        .collect::<Vec<_>>();
    let label = *cells.first()?;
    (!label.is_empty() && label != "Field" && label != "Check" && !label.starts_with("---"))
        .then_some(cells)
}

fn has_merge_value(cells: &[&str]) -> bool {
    match cells {
        [] => false,
        [_, value] => !value.is_empty(),
        [.., result] => !result.is_empty(),
    }
}
