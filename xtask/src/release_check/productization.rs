use std::path::Path;

const INCOMPLETE_MARKERS: &[&str] = &[
    "remain",
    "remains",
    "blocked",
    "not implemented",
    "external evidence",
    "manual validation remains",
];

pub(super) fn check(path: &Path) -> Result<(), String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    let rows = done_rows_with_incomplete_markers(&text);
    if rows.is_empty() {
        return Ok(());
    }
    Err(format!(
        "{} has Done rows with incomplete markers: {}",
        path.display(),
        rows.join(", ")
    ))
}

fn done_rows_with_incomplete_markers(text: &str) -> Vec<String> {
    text.lines()
        .filter(|line| line.starts_with('|') && has_status(line, "Done"))
        .filter(|line| has_incomplete_marker(line))
        .map(row_name)
        .collect()
}

fn has_status(line: &str, status: &str) -> bool {
    line.split('|')
        .map(str::trim)
        .nth(2)
        .is_some_and(|value| value == status)
}

fn has_incomplete_marker(line: &str) -> bool {
    let lower = line.to_ascii_lowercase();
    INCOMPLETE_MARKERS
        .iter()
        .any(|marker| lower.contains(marker))
}

fn row_name(line: &str) -> String {
    line.split('|')
        .map(str::trim)
        .nth(1)
        .filter(|value| !value.is_empty())
        .unwrap_or("<unknown row>")
        .to_string()
}

#[cfg(test)]
mod tests;
