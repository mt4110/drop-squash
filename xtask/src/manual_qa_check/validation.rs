use crate::dmg;
use std::path::Path;

pub(super) fn validate_field(label: &str, value: &str, missing: &mut Vec<String>) {
    let label = label.trim();
    let value = value.trim();
    if value.is_empty() {
        return;
    }
    if label == "App artifact" {
        validate_artifact(value, missing);
    }
    if label == "Date" && !is_iso_date(value) {
        missing.push("manual QA Date must use YYYY-MM-DD".to_string());
    }
}

pub(super) fn validate_result(label: &str, result: &str, missing: &mut Vec<String>) {
    let label = label.trim();
    let result = result.trim();
    if result.is_empty() {
        missing.push(format!("manual QA result is empty: {label}"));
        return;
    }
    if has_placeholder_evidence(result) || has_vague_manual_result(label, result) {
        missing.push(format!("manual QA result needs evidence: {label}"));
    }
    super::benchmark::validate_result(label, result, missing);
}

fn validate_artifact(value: &str, missing: &mut Vec<String>) {
    let path = Path::new(value);
    if !path.exists() {
        missing.push(format!("manual QA artifact does not exist: {value}"));
    }
    let extension = path.extension().and_then(|value| value.to_str());
    if !matches!(extension, Some("app" | "dmg")) {
        missing.push("manual QA App artifact must be a .app or .dmg".to_string());
    }
    if extension == Some("app") && !path.is_dir() {
        missing.push("manual QA .app artifact must be a directory".to_string());
    }
    if extension == Some("dmg") {
        validate_dmg_artifact(path, missing);
    }
}

fn validate_dmg_artifact(path: &Path, missing: &mut Vec<String>) {
    if !path.is_file() {
        missing.push("manual QA .dmg artifact must be a file".to_string());
        return;
    }
    if let Err(error) = dmg::read(path, "manual QA artifact") {
        missing.push(error);
    }
}

fn has_vague_manual_result(label: &str, result: &str) -> bool {
    if label.starts_with('`') {
        return false;
    }
    matches!(
        result.trim().to_ascii_lowercase().as_str(),
        "pass" | "ok" | "done" | "works" | "verified" | "observed expected behavior"
    )
}

pub(super) fn has_placeholder_evidence(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "tbd" | "todo" | "n/a" | "na" | "none" | "blocked" | "skipped"
    )
}

fn is_iso_date(value: &str) -> bool {
    let bytes = value.as_bytes();
    if !(bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes[..4].iter().all(u8::is_ascii_digit)
        && bytes[5..7].iter().all(u8::is_ascii_digit)
        && bytes[8..].iter().all(u8::is_ascii_digit))
    {
        return false;
    }
    let year = value[..4].parse::<u16>().unwrap_or(0);
    if year < 2000 {
        return false;
    }
    let month = value[5..7].parse::<u8>().unwrap_or(0);
    let day = value[8..].parse::<u8>().unwrap_or(0);
    (1..=days_in_month(year, month)).contains(&day)
}

fn days_in_month(year: u16, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn is_leap_year(year: u16) -> bool {
    year % 4 == 0 && year % 100 != 0 || year % 400 == 0
}
