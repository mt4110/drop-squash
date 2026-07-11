use super::REQUIRED_BLOCKERS;

const ALLOWED_CLASSES: [&str; 6] = [
    "Manual packaged-app",
    "License sandbox",
    "Public web",
    "Signing/notarization",
    "Benchmark",
    "Distribution",
];

pub(super) fn unclassified_blockers(text: &str) -> Vec<&'static str> {
    REQUIRED_BLOCKERS
        .iter()
        .copied()
        .filter(|blocker| !has_complete_classification(text, blocker))
        .collect()
}

fn has_complete_classification(text: &str, blocker: &str) -> bool {
    text.lines()
        .filter_map(classification_cells)
        .any(|cells| matches_classification(&cells, blocker))
}

fn classification_cells(line: &str) -> Option<Vec<&str>> {
    if !line.starts_with('|') {
        return None;
    }
    let cells = line
        .trim_matches('|')
        .split('|')
        .map(str::trim)
        .collect::<Vec<_>>();
    (cells.len() == 4).then_some(cells)
}

fn matches_classification(cells: &[&str], blocker: &str) -> bool {
    cells.first() == Some(&blocker)
        && ALLOWED_CLASSES.contains(&cells[1])
        && is_actionable(cells[2])
        && is_named_owner(cells[3])
}

fn is_actionable(value: &str) -> bool {
    let value = value.trim();
    value.len() >= 16 && value != "TBD" && !value.contains("...")
}

fn is_named_owner(value: &str) -> bool {
    let value = value.trim();
    value.len() >= 10 && value != "TBD" && !value.contains("...")
}

#[cfg(test)]
mod tests;
