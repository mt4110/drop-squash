use super::REQUIRED_BLOCKERS;

mod action_detail;
mod classes;

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
        && classes::matches(blocker, cells[1])
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
    action_detail::has_required_detail(blocker, action)
}

fn has_required_owner_detail(blocker: &str, owner: &str) -> bool {
    if let Some((_, field)) = crate::release_url_fields::PAIRS
        .iter()
        .find(|(candidate, _)| *candidate == blocker)
    {
        return owner == *field;
    }
    match blocker {
        "Packaged macOS manual QA"
        | "Lemon Squeezy product setup"
        | "Lemon Squeezy sandbox purchase"
        | "Empty key activation"
        | "Valid sandbox activation"
        | "Invalid license key handling"
        | "License network failure"
        | "Local license forget"
        | "Gatekeeper clean-machine open"
        | "Benchmark release set" => owner == "`docs/manual-qa.md`",
        "Signed DMG" | "Notarized and stapled DMG" => owner == "Release notes",
        _ => false,
    }
}

#[cfg(test)]
mod tests;
