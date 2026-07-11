use std::path::{Path, PathBuf};

mod requirements;
use requirements::{require_labels, REQUIRED_CHECKS, REQUIRED_FIELDS};

pub fn run(args: Vec<String>) -> Result<(), String> {
    let path = PathBuf::from(
        args.first()
            .map(String::as_str)
            .unwrap_or("docs/manual-qa.md"),
    );
    let missing = check_file(&path)?;
    if missing.is_empty() {
        println!("manual QA checks passed");
        return Ok(());
    }
    Err(missing.join("\n"))
}

fn check_file(path: &Path) -> Result<Vec<String>, String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    let mut missing = Vec::new();
    let mut labels = Vec::new();
    for line in text.lines() {
        check_line(line, &mut missing, &mut labels);
    }
    require_labels(
        "manual QA field is missing",
        &REQUIRED_FIELDS,
        &labels,
        &mut missing,
    );
    require_labels(
        "manual QA check is missing",
        &REQUIRED_CHECKS,
        &labels,
        &mut missing,
    );
    Ok(missing)
}

fn check_line(line: &str, missing: &mut Vec<String>, labels: &mut Vec<String>) {
    if !line.starts_with('|') || line.contains("---") {
        return;
    }
    let cells = cells(line);
    if let Some(label) = cells.first().map(|value| value.trim()) {
        labels.push(label.to_string());
    }
    if cells.len() == 2 && cells[1].trim().is_empty() {
        missing.push(format!("manual QA field is empty: {}", cells[0].trim()));
    }
    if cells.len() == 4 && cells[3].trim().is_empty() {
        missing.push(format!("manual QA result is empty: {}", cells[0].trim()));
    }
    if cells.len() == 4 && has_vague_manual_result(cells[0], cells[3]) {
        missing.push(format!(
            "manual QA result needs evidence: {}",
            cells[0].trim()
        ));
    }
    if cells.len() == 3 && cells[2].trim().is_empty() {
        missing.push(format!("manual QA result is empty: {}", cells[0].trim()));
    }
    if cells.len() == 3 && has_vague_manual_result(cells[0], cells[2]) {
        missing.push(format!(
            "manual QA result needs evidence: {}",
            cells[0].trim()
        ));
    }
}

fn has_vague_manual_result(label: &str, result: &str) -> bool {
    let label = label.trim();
    if label.starts_with('`') {
        return false;
    }
    matches!(
        result.trim().to_ascii_lowercase().as_str(),
        "pass" | "ok" | "done"
    )
}

fn cells(line: &str) -> Vec<&str> {
    line.trim_matches('|').split('|').collect()
}

#[cfg(test)]
mod tests;
