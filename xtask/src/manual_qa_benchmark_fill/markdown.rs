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

pub(super) fn field<'a>(manual: &'a str, name: &str) -> Option<&'a str> {
    manual.lines().find_map(|line| {
        let cells = line
            .trim_matches('|')
            .split('|')
            .map(str::trim)
            .collect::<Vec<_>>();
        if cells.len() != 2 || cells[0] != name {
            return None;
        }
        Some(cells[1])
    })
}
