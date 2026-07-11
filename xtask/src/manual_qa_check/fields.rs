use crate::dmg;
use std::path::Path;

pub(super) fn validate(label: &str, value: &str, missing: &mut Vec<String>) {
    let label = label.trim();
    let value = value.trim();
    if value.is_empty() {
        return;
    }
    match label {
        "App artifact" => validate_artifact(value, missing),
        "App build" => validate_app_build(value, missing),
        "Date" if !is_iso_date(value) => {
            missing.push("manual QA Date must use YYYY-MM-DD".to_string());
        }
        _ => {}
    }
}

fn validate_app_build(value: &str, missing: &mut Vec<String>) {
    let lower = value.to_ascii_lowercase();
    if has_version(value) && lower.contains("git ") && has_hex_run(value, 7) {
        return;
    }
    missing.push("manual QA App build must include version and git commit".to_string());
}

fn has_version(value: &str) -> bool {
    value.split_whitespace().any(|part| {
        let cleaned =
            part.trim_matches(|value: char| !value.is_ascii_alphanumeric() && value != '.');
        let segments = cleaned.split('.').collect::<Vec<_>>();
        segments.len() == 3
            && segments.iter().all(|segment| {
                !segment.is_empty() && segment.chars().all(|value| value.is_ascii_digit())
            })
    })
}

fn has_hex_run(value: &str, minimum: usize) -> bool {
    value
        .split(|value: char| !value.is_ascii_hexdigit())
        .any(|part| part.len() >= minimum)
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

fn is_iso_date(value: &str) -> bool {
    let bytes = value.as_bytes();
    if !has_iso_shape(bytes) {
        return false;
    }
    let year = value[..4].parse::<u16>().unwrap_or(0);
    let month = value[5..7].parse::<u8>().unwrap_or(0);
    let day = value[8..].parse::<u8>().unwrap_or(0);
    year >= 2000 && (1..=days_in_month(year, month)).contains(&day)
}

fn has_iso_shape(bytes: &[u8]) -> bool {
    bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes[..4].iter().all(u8::is_ascii_digit)
        && bytes[5..7].iter().all(u8::is_ascii_digit)
        && bytes[8..].iter().all(u8::is_ascii_digit)
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
