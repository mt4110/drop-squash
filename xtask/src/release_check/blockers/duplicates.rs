use super::REQUIRED_BLOCKERS;

pub(super) fn release_blocker_rows(text: &str) -> Vec<&'static str> {
    duplicate_rows(text, 5)
}

pub(super) fn classification_rows(text: &str) -> Vec<&'static str> {
    duplicate_rows(text, 4)
}

fn duplicate_rows(text: &str, columns: usize) -> Vec<&'static str> {
    REQUIRED_BLOCKERS
        .iter()
        .copied()
        .filter(|blocker| row_count(text, columns, blocker) > 1)
        .collect()
}

fn row_count(text: &str, columns: usize, blocker: &str) -> usize {
    text.lines()
        .filter_map(|line| row_name(line, columns))
        .filter(|name| *name == blocker)
        .count()
}

fn row_name(line: &str, columns: usize) -> Option<&str> {
    if !line.starts_with('|') {
        return None;
    }
    let cells = line
        .trim_matches('|')
        .split('|')
        .map(str::trim)
        .collect::<Vec<_>>();
    (cells.len() == columns).then(|| cells[0])
}

#[cfg(test)]
mod tests;
