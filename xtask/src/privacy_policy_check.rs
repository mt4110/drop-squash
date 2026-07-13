use std::path::{Path, PathBuf};

mod markers;

const LICENSE_API_BASE: &str = "https://api.lemonsqueezy.com/v1/licenses";

pub fn run() -> Result<(), String> {
    check_default_roots()?;
    println!("privacy policy checks passed");
    Ok(())
}

pub fn check_default_roots() -> Result<(), String> {
    check_roots(&[
        PathBuf::from("apps/desktop/web/src"),
        PathBuf::from("apps/desktop/src-tauri/src"),
        PathBuf::from("crates"),
        PathBuf::from("website"),
    ])
}

fn check_roots(roots: &[PathBuf]) -> Result<(), String> {
    let mut violations = Vec::new();
    for root in roots {
        collect_violations(root, &mut violations)?;
    }
    if violations.is_empty() {
        return Ok(());
    }
    Err(violations.join("\n"))
}

fn collect_violations(path: &Path, violations: &mut Vec<String>) -> Result<(), String> {
    if should_skip(path) {
        return Ok(());
    }
    if path.is_dir() {
        for entry in std::fs::read_dir(path).map_err(|error| error.to_string())? {
            collect_violations(
                &entry.map_err(|error| error.to_string())?.path(),
                violations,
            )?;
        }
        return Ok(());
    }
    if is_text_source(path) {
        scan_file(path, violations)?;
    }
    Ok(())
}

fn scan_file(path: &Path, violations: &mut Vec<String>) -> Result<(), String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    let lower = text.to_lowercase();
    for needle in markers::DISALLOWED {
        if lower.contains(needle) {
            violations.push(format!("{} contains disallowed {needle}", path.display()));
        }
    }
    if !is_license_network_source(path) {
        for marker in markers::NETWORK {
            if lower.contains(marker) {
                violations.push(format!(
                    "{} contains disallowed network marker {marker}",
                    path.display()
                ));
            }
        }
    } else {
        validate_license_network_source(path, &text, violations);
    }
    Ok(())
}

fn validate_license_network_source(path: &Path, text: &str, violations: &mut Vec<String>) {
    if !text.contains(LICENSE_API_BASE) {
        violations.push(format!(
            "{} must pin license networking to {LICENSE_API_BASE}",
            path.display()
        ));
    }
    for url in https_literals(text) {
        if url != LICENSE_API_BASE {
            violations.push(format!(
                "{} contains disallowed license network URL {url}",
                path.display()
            ));
        }
    }
}

fn https_literals(text: &str) -> Vec<&str> {
    text.split(|character: char| {
        character.is_whitespace() || matches!(character, '"' | '\'' | '<' | '>' | ')' | ']')
    })
    .filter(|part| part.starts_with("https://"))
    .collect()
}

fn is_license_network_source(path: &Path) -> bool {
    path.ends_with("crates/dropsquash-license/src/lemonsqueezy/transport.rs")
}

fn is_text_source(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|value| value.to_str()),
        Some("html" | "js" | "json" | "rs" | "ts" | "tsx")
    )
}

fn should_skip(path: &Path) -> bool {
    path.file_name()
        .and_then(|value| value.to_str())
        .is_some_and(|name| matches!(name, "target" | "node_modules" | "dist"))
}

#[cfg(test)]
mod tests;
