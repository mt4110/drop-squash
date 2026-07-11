use std::path::Path;

const REQUIRED_TEXT: [&str; 9] = [
    "Artifact",
    "SHA-256",
    "codesign",
    "spctl",
    "stapler",
    "notary",
    "Gatekeeper",
    "Homebrew",
    "docs/release-blockers.md",
];

pub(super) fn check(path: &Path) -> Result<(), String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    let missing = missing_text(&text);
    if missing.is_empty() {
        return Ok(());
    }
    Err(format!(
        "{} is missing release note evidence fields: {}",
        path.display(),
        missing.join(", ")
    ))
}

fn missing_text(text: &str) -> Vec<&'static str> {
    REQUIRED_TEXT
        .iter()
        .copied()
        .filter(|needle| !text.contains(needle))
        .collect()
}

#[cfg(test)]
mod tests;
