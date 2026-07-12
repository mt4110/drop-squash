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

pub(super) fn unknown_classification_rows(text: &str) -> Vec<&str> {
    text.lines()
        .filter_map(classification_cells)
        .filter_map(|cells| unknown_classification_name(&cells))
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
        && has_required_action_detail(blocker, cells[2])
        && is_named_owner(cells[3])
        && has_required_owner_detail(blocker, cells[3])
}

fn unknown_classification_name<'a>(cells: &[&'a str]) -> Option<&'a str> {
    let name = cells.first().copied()?;
    let known = name == "Blocker" || name.starts_with("---") || REQUIRED_BLOCKERS.contains(&name);
    (!known).then_some(name)
}

fn is_actionable(value: &str) -> bool {
    let value = value.trim();
    value.len() >= 16 && !has_placeholder(value)
}

fn is_named_owner(value: &str) -> bool {
    let value = value.trim();
    value.len() >= 10 && !has_placeholder(value)
}

fn has_placeholder(value: &str) -> bool {
    value.contains("...") || super::placeholders::has_token(value)
}

fn has_required_action_detail(blocker: &str, action: &str) -> bool {
    match blocker {
        "Packaged macOS manual QA" => {
            action.contains("DropSquash.app") && action.contains("DropSquash.dmg")
        }
        "Benchmark release set" => action.contains("absolute CSV path outside repo"),
        _ => true,
    }
}

fn has_required_owner_detail(blocker: &str, owner: &str) -> bool {
    crate::release_url_fields::PAIRS
        .iter()
        .find(|(candidate, _)| *candidate == blocker)
        .map_or(true, |(_, field)| owner == *field)
}

#[cfg(test)]
mod tests;
