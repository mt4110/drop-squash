pub(super) fn validate_app_build(value: &str, missing: &mut Vec<String>) {
    let lower = value.to_ascii_lowercase();
    if !(has_version(value) && lower.contains("git ") && has_hex_run(value, 7)) {
        missing.push("manual QA App build must include version and git commit".to_string());
        return;
    }
    if current_head().is_ok_and(|head| value.contains(&format!("git {head}"))) {
        return;
    }
    missing.push("manual QA App build must match current HEAD".to_string());
}

pub(super) fn validate_macos_version(value: &str, missing: &mut Vec<String>) {
    if value.starts_with("macOS ") && has_numeric_version(value.trim_start_matches("macOS ")) {
        return;
    }
    missing.push("manual QA macOS version must look like macOS 15.5".to_string());
}

pub(super) fn validate_machine(value: &str, missing: &mut Vec<String>) {
    if value.contains("arm64") || value.contains("x86_64") {
        return;
    }
    missing.push("manual QA Machine must include CPU architecture".to_string());
}

pub(super) fn validate_input_sample_set(value: &str, missing: &mut Vec<String>) {
    let lower = value.to_ascii_lowercase();
    let has_sizes = ["short", "medium", "large"]
        .iter()
        .all(|needle| lower.contains(needle));
    let has_media = lower.contains("recording") || lower.contains("sample");
    if has_sizes && has_media {
        return;
    }
    missing.push(
        "manual QA Input sample set must mention short, medium, and large recordings".to_string(),
    );
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

fn has_numeric_version(value: &str) -> bool {
    let parts = value.split('.').collect::<Vec<_>>();
    (2..=3).contains(&parts.len())
        && parts
            .iter()
            .all(|part| !part.is_empty() && part.chars().all(|value| value.is_ascii_digit()))
}

fn current_head() -> Result<String, String> {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--short=7", "HEAD"])
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err("git rev-parse failed".to_string());
    }
    String::from_utf8(output.stdout)
        .map_err(|error| error.to_string())
        .map(|value| value.trim().to_string())
}
