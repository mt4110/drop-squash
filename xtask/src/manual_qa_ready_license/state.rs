use std::path::Path;

use super::steps;

pub(super) fn license_cache_path(text: &str) -> Option<String> {
    text.lines().find_map(|line| {
        line.strip_prefix("| License cache path | ")
            .and_then(|value| value.strip_suffix(" |"))
            .map(str::to_string)
    })
}

pub(super) fn license_pending_command(path: &Path) -> String {
    format!(
        "cargo run -p xtask -- manual-qa-pending '{}' --section license",
        shell_single_quote(path)
    )
}

pub(super) fn manual_check_command(path: &Path) -> String {
    format!(
        "cargo run -p xtask -- manual-qa-check '{}'",
        shell_single_quote(path)
    )
}

pub(super) fn sandbox_rows_command(path: &Path) -> String {
    format!(
        "{} | rg '^license (product setup|sandbox purchase|valid activation) markdown row:'",
        license_pending_command(path)
    )
}

pub(super) fn fresh_launch_command(text: &str, api_base: Option<&str>) -> Option<String> {
    let config = text.lines().find_map(|line| {
        line.strip_prefix("| Config path | ")
            .and_then(|value| value.strip_suffix(" |"))
    })?;
    let mut parts = vec!["cargo run -p xtask -- manual-qa-launch-app".to_string()];
    if let Some(url) = api_base {
        parts.push("--license-api-base-url".to_string());
        parts.push(single_quote(url));
    }
    parts.push(single_quote(steps::FRESH_APP));
    parts.push(single_quote(config));
    Some(parts.join(" "))
}

pub(super) fn sandbox_quickstart_lines(text: &str) -> Vec<String> {
    let mut lines = Vec::new();
    if let Some(command) = fresh_launch_command(text, None) {
        lines.push(format!("sandbox quickstart 1: {command}"));
    }
    lines.push(format!("sandbox quickstart 2: {}", status_command(text)));
    if let Some(path) = license_cache_path(text) {
        lines.push(format!("sandbox quickstart 3: sed -n '1,160p' \"{path}\""));
    }
    lines.push(format!("sandbox quickstart 4: {}", status_command(text)));
    lines
}

pub(super) fn is_tmp_path(path: &str) -> bool {
    path.starts_with("/tmp/")
}

pub(super) fn isolation_recovery_line(path: &str) -> Option<String> {
    (!is_tmp_path(path)).then(|| {
        format!(
            "license isolation recovery command: {}",
            steps::ISOLATED_PREPARE
        )
    })
}

pub(super) fn uses_dmg_artifact(text: &str) -> bool {
    text.lines()
        .any(|line| line.starts_with("| App artifact | ") && line.ends_with(".dmg |"))
}

fn shell_single_quote(path: &Path) -> String {
    path.display().to_string().replace('\'', "'\\''")
}

fn single_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

fn status_command(text: &str) -> String {
    let mut parts = vec!["cargo run -p dropsquash -- license status".to_string()];
    if let Some(path) = text.lines().find_map(|line| {
        line.strip_prefix("| History path | ")
            .and_then(|value| value.strip_suffix(" |"))
    }) {
        parts.push(format!("--history {}", single_quote(path)));
    }
    if let Some(path) = license_cache_path(text) {
        parts.push(format!("--cache-path {}", single_quote(&path)));
    }
    parts.join(" ")
}
