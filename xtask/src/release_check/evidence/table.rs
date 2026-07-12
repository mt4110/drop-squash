pub(super) fn has_row_label(markdown: &str, label: &str, min_cells: usize) -> bool {
    markdown
        .lines()
        .filter_map(cells)
        .any(|cells| cells.len() >= min_cells && cells.first().is_some_and(|cell| cell == label))
}

fn cells(line: &str) -> Option<Vec<String>> {
    let trimmed = line.trim();
    if !trimmed.starts_with('|') || !trimmed.ends_with('|') {
        return None;
    }
    let cells = trimmed
        .trim_matches('|')
        .split('|')
        .map(|cell| cell.trim().to_string())
        .collect::<Vec<_>>();
    if cells.iter().all(|cell| is_separator(cell)) {
        return None;
    }
    Some(cells)
}

fn is_separator(cell: &str) -> bool {
    let trimmed = cell.trim_matches(':');
    !trimmed.is_empty() && trimmed.chars().all(|character| character == '-')
}

#[cfg(test)]
mod tests {
    use super::has_row_label;

    #[test]
    fn finds_exact_first_cell_label() {
        let markdown =
            "| Check | Expected | Result |\n| Reveal output | Opens Finder | selected |\n";

        assert!(has_row_label(markdown, "Reveal output", 3));
    }

    #[test]
    fn ignores_partial_labels() {
        let markdown =
            "| Check | Expected | Result |\n| Prefix Reveal output | Opens Finder | selected |\n";

        assert!(!has_row_label(markdown, "Reveal output", 3));
    }

    #[test]
    fn ignores_rows_with_too_few_cells() {
        let markdown = "| Check | Expected | Result |\n| Reveal output | Incomplete |\n";

        assert!(!has_row_label(markdown, "Reveal output", 3));
    }
}
