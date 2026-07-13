pub(super) struct Row<'a> {
    pub order: &'a str,
    pub track: &'a str,
    pub blockers: &'a str,
    pub exit: &'a str,
    pub target: &'a str,
}

pub(super) fn all(text: &str) -> impl Iterator<Item = Row<'_>> {
    text.lines().filter_map(execution_row)
}

pub(super) fn split_blockers(value: &str) -> impl Iterator<Item = &str> {
    value
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn execution_row(line: &str) -> Option<Row<'_>> {
    let cells = cells(line)?;
    is_order(cells[0]).then_some(Row {
        order: cells[0],
        track: cells[1],
        blockers: cells[2],
        exit: cells[3],
        target: cells[4],
    })
}

fn cells(line: &str) -> Option<Vec<&str>> {
    if !line.starts_with('|') {
        return None;
    }
    let cells = line
        .trim_matches('|')
        .split('|')
        .map(str::trim)
        .collect::<Vec<_>>();
    (cells.len() == 5).then_some(cells)
}

fn is_order(value: &str) -> bool {
    value.chars().all(|character| character.is_ascii_digit()) && !value.is_empty()
}
