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

fn validate_artifact(value: &str, missing: &mut Vec<String>) {
    let path = Path::new(value);
    if !path.exists() {
        missing.push(format!("manual QA artifact does not exist: {value}"));
    }
    let extension = path.extension().and_then(|value| value.to_str());
    if !matches!(extension, Some("app" | "dmg")) {
        missing.push("manual QA App artifact must be a .app or .dmg".to_string());
    }
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
    let month = value[5..7].parse::<u8>().unwrap_or(0);
    let day = value[8..].parse::<u8>().unwrap_or(0);
    (1..=12).contains(&month) && (1..=31).contains(&day)
}
