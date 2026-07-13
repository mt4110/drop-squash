use std::path::Path;

const INCOMPLETE_MARKERS: &[&str] = &[
    "evidence remains",
    "validation remains",
    "publication remains",
    "signing remains",
    "remain unimplemented",
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
        .filter(|line| line.starts_with('|') && has_status(line))
        .filter(|line| has_incomplete_marker(line))
        .map(row_name)
        .collect()
}

fn has_status(line: &str) -> bool {
    cells(line).contains(&"Done")
}

fn has_incomplete_marker(line: &str) -> bool {
    let lower = line.to_ascii_lowercase();
    INCOMPLETE_MARKERS
        .iter()
        .any(|marker| lower.contains(marker))
}

fn row_name(line: &str) -> String {
    let cells = cells(line);
    let index = if cells
        .first()
        .is_some_and(|value| value.chars().all(|ch| ch.is_ascii_digit()))
    {
        1
    } else {
        0
    };
    cells
        .get(index)
        .map_or("<unknown row>", |value| value)
        .to_string()
}

fn cells(line: &str) -> Vec<&str> {
    line.split('|')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .collect()
}

#[cfg(test)]
mod tests;
