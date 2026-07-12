use super::{fields, validation};

pub(super) fn check_line(
    line: &str,
    missing: &mut Vec<String>,
    labels: &mut Vec<String>,
    rows: &mut Vec<(String, String)>,
) {
    if !line.starts_with('|') || line.contains("---") {
        return;
    }
    let cells = cells(line);
    if let Some(label) = cells.first().map(|value| value.trim()) {
        labels.push(label.to_string());
    }
    if !matches!(cells.len(), 2..=4) {
        missing.push(format!(
            "manual QA row has unexpected column count: {}",
            cells[0].trim()
        ));
        return;
    }
    check_field_row(&cells, missing, rows);
    check_result_row(&cells, missing, rows);
}

fn check_field_row(cells: &[&str], missing: &mut Vec<String>, rows: &mut Vec<(String, String)>) {
    if cells.len() != 2 {
        return;
    }
    if cells[1].trim().is_empty() {
        missing.push(format!("manual QA field is empty: {}", cells[0].trim()));
    }
    if validation::has_placeholder_evidence(cells[1]) {
        missing.push(format!(
            "manual QA field needs evidence: {}",
            cells[0].trim()
        ));
    }
    fields::validate(cells[0], cells[1], missing);
    rows.push((cells[0].trim().to_string(), cells[1].trim().to_string()));
}

fn check_result_row(cells: &[&str], missing: &mut Vec<String>, rows: &mut Vec<(String, String)>) {
    let value = match cells.len() {
        3 => cells[2],
        4 => cells[3],
        _ => return,
    };
    validation::validate_result(cells[0], value, missing);
    rows.push((cells[0].trim().to_string(), value.trim().to_string()));
}

pub(super) fn cells(line: &str) -> Vec<&str> {
    line.trim_matches('|').split('|').collect()
}
