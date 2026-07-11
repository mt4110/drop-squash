use std::path::Path;

pub(super) fn validate_field(label: &str, value: &str, missing: &mut Vec<String>) {
    let label = label.trim();
    let value = value.trim();
    if value.is_empty() {
        return;
    }
    if label == "App artifact" && !Path::new(value).exists() {
        missing.push(format!("manual QA artifact does not exist: {value}"));
    }
    if label == "Date" && !is_iso_date(value) {
        missing.push("manual QA Date must use YYYY-MM-DD".to_string());
    }
}

fn is_iso_date(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes[..4].iter().all(u8::is_ascii_digit)
        && bytes[5..7].iter().all(u8::is_ascii_digit)
        && bytes[8..].iter().all(u8::is_ascii_digit)
}
