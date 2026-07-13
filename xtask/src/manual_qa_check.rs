use std::path::{Path, PathBuf};

mod app;
mod artifact_consistency;
mod benchmark;
mod fields;
mod license;
pub(crate) mod requirements;
mod rows;
mod secrets;
mod validation;
use requirements::{require_labels, REQUIRED_CHECKS, REQUIRED_FIELDS};

pub fn run(args: Vec<String>) -> Result<(), String> {
    let path = match args.as_slice() {
        [] => PathBuf::from("docs/manual-qa.md"),
        [path] => PathBuf::from(path),
        _ => return Err("manual-qa-check accepts at most one <manual-qa.md>".to_string()),
    };
    let missing = check_file(&path)?;
    if missing.is_empty() {
        println!("manual QA checks passed");
        return Ok(());
    }
    Err(missing.join("\n"))
}

pub(crate) fn check_file(path: &Path) -> Result<Vec<String>, String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    let mut missing = secrets::validate(&text);
    if has_prepared_draft_marker(&text) {
        missing.push("manual QA must not contain prepared draft markers".to_string());
    }
    let mut labels = Vec::new();
    let mut rows = Vec::new();
    for line in text.lines() {
        rows::check_line(line, &mut missing, &mut labels, &mut rows);
    }
    artifact_consistency::validate(&rows, &mut missing);
    benchmark::validate_rows(&rows, &mut missing);
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
    requirements::reject_duplicate_labels(&labels, &mut missing);
    Ok(missing)
}

fn has_prepared_draft_marker(text: &str) -> bool {
    text.to_ascii_lowercase()
        .contains("prepared manual qa draft only")
}

#[cfg(test)]
mod tests;
