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
    let (path, section) = parse_args(args)?;
    let missing = check_file_section(&path, section.as_deref())?;
    if missing.is_empty() {
        println!("manual QA checks passed");
        return Ok(());
    }
    Err(missing.join("\n"))
}

pub(crate) fn check_file(path: &Path) -> Result<Vec<String>, String> {
    check_file_section(path, None)
}

pub(crate) fn check_file_section(
    path: &Path,
    section: Option<&str>,
) -> Result<Vec<String>, String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    Ok(check_text_section(&text, section))
}

pub(crate) fn check_text(text: &str) -> Vec<String> {
    check_text_section(text, None)
}

pub(crate) fn check_text_section(text: &str, section: Option<&str>) -> Vec<String> {
    let mut missing = secrets::validate(text);
    if has_prepared_draft_marker(text) {
        missing.push("manual QA must not contain prepared draft markers".to_string());
    }
    let mut labels = Vec::new();
    let mut rows = Vec::new();
    for line in text.lines() {
        rows::check_line(line, &mut missing, &mut labels, &mut rows, section);
    }
    artifact_consistency::validate(&rows, &mut missing);
    benchmark::validate_rows(&rows, &mut missing);
    require_labels(
        "manual QA field is missing",
        &REQUIRED_FIELDS,
        &labels,
        &mut missing,
    );
    let required_checks = required_checks(section);
    require_labels(
        "manual QA check is missing",
        &required_checks,
        &labels,
        &mut missing,
    );
    requirements::reject_duplicate_labels(&labels, &mut missing);
    missing
}

fn parse_args(args: Vec<String>) -> Result<(PathBuf, Option<String>), String> {
    match args.as_slice() {
        [] => Ok((PathBuf::from("docs/manual-qa.md"), None)),
        [path] => Ok((PathBuf::from(path), None)),
        [path, flag, section] if *flag == "--section" => {
            Ok((PathBuf::from(path), Some(section.clone())))
        }
        _ => Err(
            "manual-qa-check accepts <manual-qa.md> [--section packaged-app|license|benchmark|distribution|local-proof]"
                .to_string(),
        ),
    }
}

fn required_checks(section: Option<&str>) -> Vec<&'static str> {
    match section {
        None => REQUIRED_CHECKS.to_vec(),
        Some(filter) => REQUIRED_CHECKS
            .iter()
            .copied()
            .filter(|label| {
                crate::manual_qa_pending::section::matches_requested_filter(label, filter)
            })
            .collect(),
    }
}

fn has_prepared_draft_marker(text: &str) -> bool {
    text.to_ascii_lowercase()
        .contains("prepared manual qa draft only")
}

#[cfg(test)]
mod tests;
