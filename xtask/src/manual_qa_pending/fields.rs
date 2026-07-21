pub(super) fn field_value<'a>(text: &'a str, label: &str) -> Option<&'a str> {
    text.lines().find_map(|line| {
        let cells = line
            .trim_matches('|')
            .split('|')
            .map(str::trim)
            .collect::<Vec<_>>();
        match cells.as_slice() {
            [found, value] if *found == label => Some(*value),
            _ => None,
        }
    })
}

pub(super) fn table_result<'a>(text: &'a str, label: &str) -> Option<&'a str> {
    text.lines().find_map(|line| {
        let cells = line
            .trim_matches('|')
            .split('|')
            .map(str::trim)
            .collect::<Vec<_>>();
        match cells.as_slice() {
            [row, _, result] | [row, _, _, result] if *row == label => Some(*result),
            _ => None,
        }
    })
}
