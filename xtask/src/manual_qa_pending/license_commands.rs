use super::fields;

pub(super) fn status_command(text: &str) -> String {
    let mut parts = vec!["cargo run -p dropsquash -- license status".to_string()];
    if let Some(path) = fields::field_value(text, "History path") {
        parts.push(format!("--history {}", shell_single_quote(path)));
    }
    if let Some(path) = fields::field_value(text, "License cache path") {
        parts.push(format!("--cache-path {}", shell_single_quote(path)));
    }
    parts.join(" ")
}

pub(super) fn forget_command(text: &str) -> String {
    let mut parts = vec!["cargo run -p dropsquash -- license forget".to_string()];
    if let Some(path) = fields::field_value(text, "License cache path") {
        parts.push(format!("--cache-path {}", shell_single_quote(path)));
    }
    parts.join(" ")
}

pub(super) fn forget_helper_command(text: &str, cache_path: &str) -> String {
    let mut parts = vec![
        "cargo run -p xtask -- manual-qa-forget-license".to_string(),
        shell_single_quote(cache_path),
    ];
    if let Some(path) = fields::field_value(text, "History path") {
        parts.push(shell_single_quote(path));
    }
    parts.join(" ")
}

pub(super) fn seed_cache_command(cache_path: &str, expired: bool) -> String {
    let mut parts = vec![
        "cargo run -p xtask -- manual-qa-seed-license-cache".to_string(),
        shell_single_quote(cache_path),
    ];
    if expired {
        parts.push("--expired".to_string());
    }
    parts.join(" ")
}

pub(super) fn launch_app_command(
    artifact: &str,
    config: &str,
    license_api_base_url: Option<&str>,
) -> String {
    let mut parts = vec!["cargo run -p xtask -- manual-qa-launch-app".to_string()];
    if let Some(url) = license_api_base_url {
        parts.push("--license-api-base-url".to_string());
        parts.push(shell_single_quote(url));
    }
    parts.push(shell_single_quote(artifact));
    parts.push(shell_single_quote(config));
    parts.join(" ")
}

fn shell_single_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}
