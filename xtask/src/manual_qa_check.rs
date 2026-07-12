use std::path::{Path, PathBuf};

mod app;
mod artifact_consistency;
mod benchmark;
mod fields;
mod license;
mod requirements;
mod rows;
mod validation;
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

pub(crate) fn check_file(path: &Path) -> Result<Vec<String>, String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    let mut missing = Vec::new();
    let mut labels = Vec::new();
    let mut rows = Vec::new();
    for line in text.lines() {
        rows::check_line(line, &mut missing, &mut labels, &mut rows);
    }
    artifact_consistency::validate(&rows, &mut missing);
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

#[cfg(test)]
mod tests;
