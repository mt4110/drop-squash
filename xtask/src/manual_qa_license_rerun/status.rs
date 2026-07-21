use std::path::Path;

use dropsquash_license::LicenseCache;

const FRESH_APP: &str = "/tmp/dsq-build-target/release/bundle/macos/DropSquash.app";

pub(crate) fn lines(path: &Path) -> Result<Vec<String>, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read manual QA file: {error}"))?;
    let config = value(&text, "Config path");
    let history = value(&text, "History path");
    let cache = value(&text, "License cache path");
    let mut lines = vec![format!(
        "license diagnostics command: {}",
        status_command(&history, &cache)
    )];
    lines.push(fresh_app_status());
    if let Some(line) = fresh_launch_status(config.as_deref()) {
        lines.push(line);
    }
    lines.push(cache_status(cache.as_deref()));
    if let Some(path) = cache.as_deref() {
        lines.push(cache_inspect_status(path));
        lines.push(cache_helper_status(path));
        lines.extend(cache_summary_status(path)?);
    }
    lines.push(history_status(history.as_deref()));
    Ok(lines)
}

fn value(text: &str, label: &str) -> Option<String> {
    let prefix = format!("| {label} | ");
    text.lines()
        .find_map(|line| line.strip_prefix(&prefix)?.strip_suffix(" |"))
        .map(str::to_string)
}

fn status_command(history: &Option<String>, cache: &Option<String>) -> String {
    let mut parts = vec!["cargo run -p dropsquash -- license status".to_string()];
    if let Some(path) = history {
        parts.push(format!("--history '{}'", shell_quote(path)));
    }
    if let Some(path) = cache {
        parts.push(format!("--cache-path '{}'", shell_quote(path)));
    }
    parts.join(" ")
}

fn fresh_app_status() -> String {
    if Path::new(FRESH_APP).exists() {
        return format!("fresh packaged-app status: found {FRESH_APP}");
    }
    format!("fresh packaged-app status: missing {FRESH_APP}; run the printed packaged-app build command before app-side activation")
}

fn fresh_launch_status(config: Option<&str>) -> Option<String> {
    let config = config?;
    Some(format!(
        "fresh packaged-app launch command: cargo run -p xtask -- manual-qa-launch-app '{}' '{}'",
        shell_quote(FRESH_APP),
        shell_quote(config),
    ))
}

fn cache_status(path: Option<&str>) -> String {
    match path {
        Some(value) if Path::new(value).exists() => format!("license cache status: found {value}"),
        Some(value) => format!("license cache status: missing {value}; create it through the activation loop before expecting cache deltas"),
        None => "license cache status: manual QA file has no `License cache path` row yet".to_string(),
    }
}

fn cache_summary_status(path: &str) -> Result<Vec<String>, String> {
    let cache = LicenseCache::load_or_default(Path::new(path))
        .map_err(|error| format!("failed to read license cache: {error}"))?;
    Ok(crate::manual_qa_license_cache::cache_summary_lines(&cache)
        .into_iter()
        .map(|line| format!("license cache summary: {line}"))
        .collect())
}

fn cache_helper_status(path: &str) -> String {
    format!(
        "license cache helper command: cargo run -p xtask -- manual-qa-license-cache '{}'",
        shell_quote(path),
    )
}

fn cache_inspect_status(path: &str) -> String {
    format!(
        "license cache inspect command: sed -n '1,160p' '{}'",
        shell_quote(path)
    )
}

fn history_status(path: Option<&str>) -> String {
    match path {
        Some(value) if value.starts_with("/tmp/") => format!("license history status: isolated path {value}"),
        Some(value) => format!("license history status: non-isolated path {value}; prefer /tmp-backed prepared manual QA state before sandbox proof"),
        None => "license history status: manual QA file has no `History path` row yet".to_string(),
    }
}

fn shell_quote(value: &str) -> String {
    value.replace('\'', "'\\''")
}

#[cfg(test)]
mod tests;
