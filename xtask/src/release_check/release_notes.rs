use std::path::Path;

const REQUIRED_TEXT: [&str; 45] = [
    "Version",
    "Artifact",
    "Artifact: DropSquash.dmg",
    "Artifact URL",
    "SHA-256",
    "Git commit",
    "codesign",
    "spctl",
    "stapler",
    "Apple notary log",
    "Gatekeeper",
    "docs/release-blockers.md",
    "Manual QA record",
    "tested public `DropSquash.dmg`",
    "Conversion safety evidence",
    "Queue evidence",
    "Trash source policy",
    "Benchmark sample set",
    "Benchmark regression threshold",
    "Lemon Squeezy product setup",
    "Lemon Squeezy sandbox purchase",
    "Valid sandbox activation",
    "Empty key activation: mention",
    "Empty key activation",
    "Invalid license key handling: mention",
    "Invalid license key handling",
    "License network failure",
    "Local license forget",
    "Public website URL",
    "Refund policy URL",
    "Live checkout URL",
    "GitHub Release checksum",
    "the Artifact URL above",
    "GitHub Release URL",
    "Homebrew tap PR",
    "Homebrew tap PR URL",
    "Homebrew install result",
    "Known limitations",
    "Support contact",
    "Do not paste signing secrets",
    "license keys",
    "Homebrew",
    "Evidence Wording Checklist",
    "all rows Verified",
    "SHA256SUMS",
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
