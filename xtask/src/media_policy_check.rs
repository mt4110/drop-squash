use std::path::{Path, PathBuf};

mod markers;

pub fn run() -> Result<(), String> {
    check_default_roots()?;
    println!("media policy checks passed");
    Ok(())
}

pub fn check_default_roots() -> Result<(), String> {
    check_roots(&[PathBuf::from("apps"), PathBuf::from("crates")])
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
    Ok(())
}

fn is_text_source(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|value| value.to_str()),
        Some("rs" | "ts" | "tsx" | "toml" | "json")
    )
}

fn should_skip(path: &Path) -> bool {
    path.file_name()
        .and_then(|value| value.to_str())
        .is_some_and(|name| matches!(name, "target" | "node_modules" | "gen"))
}

#[cfg(test)]
mod tests;
