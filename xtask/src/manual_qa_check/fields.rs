use std::path::Path;

mod artifact;
mod date;

pub(super) fn validate(label: &str, value: &str, missing: &mut Vec<String>) {
    let label = label.trim();
    let value = value.trim();
    if value.is_empty() {
        return;
    }
    match label {
        "App artifact" => artifact::validate(value, missing),
        "App build" => validate_app_build(value, missing),
        "macOS version" => validate_macos_version(value, missing),
        "Machine" => validate_machine(value, missing),
        "Output folder" => validate_output_folder(value, missing),
        "Date" if !date::is_iso(value) => {
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

fn validate_macos_version(value: &str, missing: &mut Vec<String>) {
    if value.starts_with("macOS ") && has_numeric_version(value.trim_start_matches("macOS ")) {
        return;
    }
    missing.push("manual QA macOS version must look like macOS 15.5".to_string());
}

fn validate_machine(value: &str, missing: &mut Vec<String>) {
    if value.contains("arm64") || value.contains("x86_64") {
        return;
    }
    missing.push("manual QA Machine must include CPU architecture".to_string());
}

fn validate_output_folder(value: &str, missing: &mut Vec<String>) {
    let path = Path::new(value);
    if path.is_dir() {
        return;
    }
    missing.push(format!("manual QA Output folder must exist: {value}"));
}

fn has_numeric_version(value: &str) -> bool {
    let parts = value.split('.').collect::<Vec<_>>();
    (2..=3).contains(&parts.len())
        && parts
            .iter()
            .all(|part| !part.is_empty() && part.chars().all(|value| value.is_ascii_digit()))
}
