pub(super) fn pending_rows(text: &str) -> Vec<(String, String)> {
    text.lines()
        .filter_map(pending_row)
        .map(|(label, expected)| (label.to_string(), expected.to_string()))
        .collect()
}

pub(super) fn includes_packaged_app(groups: &[(&str, Vec<(String, String)>)]) -> bool {
    groups.iter().any(|(name, _)| *name == "Packaged App")
}

fn pending_row(line: &str) -> Option<(&str, &str)> {
    if !line.starts_with('|') || line.contains("---") {
        return None;
    }
    let cells = line.trim_matches('|').split('|').collect::<Vec<_>>();
    match cells.as_slice() {
        [label, expected, result] if result.trim().is_empty() => {
            Some((label.trim(), expected.trim()))
        }
        [label, expected, _, result] if result.trim().is_empty() => {
            Some((label.trim(), expected.trim()))
        }
        _ => None,
    }
}
