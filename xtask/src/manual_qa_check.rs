use std::path::{Path, PathBuf};

const REQUIRED_FIELDS: [&str; 8] = [
    "App build",
    "App artifact",
    "macOS version",
    "Machine",
    "Input sample set",
    "Output folder",
    "Tester",
    "Date",
];
const REQUIRED_CHECKS: [&str; 24] = [
    "Choose recording conversion",
    "Drag-and-drop conversion",
    "Duplicate output naming",
    "Cancellation",
    "Multi-file queue",
    "Ask source policy",
    "Trash source policy",
    "Failed conversion",
    "Larger output",
    "Reveal output",
    "Empty key activation",
    "Invalid key activation",
    "Valid sandbox activation",
    "Forget license on this Mac",
    "`cargo run -p xtask -- release-check`",
    "`cargo run -p xtask -- file-size-check`",
    "`cargo run -p xtask -- media-policy-check`",
    "`cargo run -p xtask -- privacy-policy-check`",
    "`cargo run -p xtask -- website-check`",
    "`cargo run -p xtask -- manual-qa-check`",
    "`cargo run -p xtask -- artifact-check path/to/DropSquash.dmg`",
    "`cargo run -p xtask -- checksum path/to/DropSquash.dmg`",
    "`cargo run -p xtask -- macos-signing-check`",
    "Gatekeeper open test",
];

pub fn run(args: Vec<String>) -> Result<(), String> {
    let path = PathBuf::from(
        args.first()
            .map(String::as_str)
            .unwrap_or("docs/manual-qa.md"),
    );
    let missing = check_file(&path)?;
    if missing.is_empty() {
        println!("manual QA checks passed");
        return Ok(());
    }
    Err(missing.join("\n"))
}

fn check_file(path: &Path) -> Result<Vec<String>, String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    let mut missing = Vec::new();
    let mut labels = Vec::new();
    for line in text.lines() {
        check_line(line, &mut missing, &mut labels);
    }
    require_labels(
        "manual QA field is missing",
        &REQUIRED_FIELDS,
        &labels,
        &mut missing,
    );
    require_labels(
        "manual QA check is missing",
        &REQUIRED_CHECKS,
        &labels,
        &mut missing,
    );
    Ok(missing)
}

fn check_line(line: &str, missing: &mut Vec<String>, labels: &mut Vec<String>) {
    if !line.starts_with('|') || line.contains("---") {
        return;
    }
    let cells = cells(line);
    if let Some(label) = cells.first().map(|value| value.trim()) {
        labels.push(label.to_string());
    }
    if cells.len() == 2 && cells[1].trim().is_empty() {
        missing.push(format!("manual QA field is empty: {}", cells[0].trim()));
    }
    if cells.len() == 4 && cells[3].trim().is_empty() {
        missing.push(format!("manual QA result is empty: {}", cells[0].trim()));
    }
    if cells.len() == 3 && cells[2].trim().is_empty() {
        missing.push(format!("manual QA result is empty: {}", cells[0].trim()));
    }
}

fn require_labels(prefix: &str, required: &[&str], labels: &[String], missing: &mut Vec<String>) {
    for label in required {
        if !labels.iter().any(|value| value == label) {
            missing.push(format!("{prefix}: {label}"));
        }
    }
}

fn cells(line: &str) -> Vec<&str> {
    line.trim_matches('|').split('|').collect()
}

#[cfg(test)]
mod tests;
