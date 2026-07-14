pub(super) fn rows(text: &str) -> impl Iterator<Item = Vec<String>> + '_ {
    text.lines()
        .filter(|line| line.starts_with('|'))
        .map(|line| line.trim_matches('|').split('|').map(cell).collect())
}

fn cell(value: &str) -> String {
    value.trim().trim_matches('`').to_string()
}
