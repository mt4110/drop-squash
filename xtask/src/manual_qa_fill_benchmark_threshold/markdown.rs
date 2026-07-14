pub(super) fn replace(line: &str, result: &str) -> String {
    let cells = line
        .trim_matches('|')
        .split('|')
        .map(str::trim)
        .collect::<Vec<_>>();
    format!("| {} | {} | {} |", cells[0], cells[1], result)
}

pub(super) fn label(line: &str) -> Option<&str> {
    line.starts_with('|')
        .then(|| line.trim_matches('|').split('|').next().map(str::trim))
        .flatten()
}
