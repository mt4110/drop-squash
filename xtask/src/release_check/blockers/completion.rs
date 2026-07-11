use super::{row, REQUIRED_BLOCKERS};

const REQUIRED_PHRASES: [(&str, &str); 53] = [
    ("Packaged macOS manual QA", ".app` or `.dmg"),
    ("Lemon Squeezy product setup", "Sandbox product"),
    ("Lemon Squeezy product setup", "DropSquash"),
    ("Lemon Squeezy product setup", "license keys enabled"),
    ("Lemon Squeezy sandbox purchase", "Sandbox checkout"),
    ("Lemon Squeezy sandbox purchase", "intended product"),
    ("Lemon Squeezy sandbox purchase", "test buyer"),
    ("Lemon Squeezy sandbox purchase", "order"),
    ("Empty key activation", "Activate stays disabled"),
    ("Empty key activation", "raw key is absent"),
    ("Empty key activation", "local cache"),
    (
        "Valid sandbox activation",
        "Activating state disables submit",
    ),
    ("Valid sandbox activation", "Pro state"),
    ("Valid sandbox activation", "raw key is absent"),
    ("Valid sandbox activation", "local cache"),
    (
        "Invalid license key handling",
        "Activating state disables submit",
    ),
    ("Invalid license key handling", "friendly error"),
    ("Invalid license key handling", "raw key is absent"),
    ("Invalid license key handling", "local cache"),
    ("License network failure", "Friendly network error"),
    ("License network failure", "existing valid local cache"),
    ("License network failure", "raw key is absent"),
    ("License network failure", "local cache"),
    ("Local license forget", "Forgetting state disables action"),
    ("Local license forget", "local cache"),
    ("Local license forget", "removed"),
    ("Local license forget", "trial or locked"),
    ("Public website deployment", "Production website serves"),
    ("Refund policy finalized", "Production refund policy"),
    ("Refund policy finalized", "final"),
    ("Refund policy finalized", "checkout goes live"),
    ("Live checkout link", "Public pricing page opens"),
    ("Signed DMG", "`codesign`"),
    ("Signed DMG", "Developer ID"),
    ("Signed DMG", "DropSquash.dmg"),
    ("Notarized and stapled DMG", "`spctl`"),
    ("Notarized and stapled DMG", "notary"),
    ("Notarized and stapled DMG", "stapled"),
    ("Notarized and stapled DMG", "DropSquash.dmg"),
    ("Gatekeeper clean-machine open", "Fresh macOS account"),
    (
        "Gatekeeper clean-machine open",
        "without Gatekeeper warning",
    ),
    ("Benchmark release set", "Release-set benchmark CSV"),
    ("Benchmark release set", "short, medium, and large"),
    ("Benchmark release set", "smaller outputs"),
    ("Benchmark release set", "machine/OS context"),
    ("Benchmark release set", "20% regression threshold"),
    ("Published checksum", "SHA-256"),
    ("Published checksum", "SHA256SUMS"),
    ("Published checksum", "DropSquash.dmg"),
    ("Published checksum", "attached"),
    ("Homebrew cask install", "brew install --cask"),
    ("Homebrew cask install", "versioned artifact"),
    ("Homebrew cask install", "zap"),
];

pub(super) fn incomplete_requirements(text: &str) -> Vec<&'static str> {
    REQUIRED_BLOCKERS
        .iter()
        .copied()
        .filter(|blocker| {
            row::find(text, blocker)
                .and_then(row::completion_evidence)
                .is_some_and(|value| is_missing_requirement(blocker, value))
        })
        .collect()
}

fn is_missing_requirement(blocker: &str, value: &str) -> bool {
    let value = value.trim();
    value.is_empty()
        || value == "TBD"
        || required_phrases(blocker)
            .iter()
            .any(|phrase| !value.contains(phrase))
}

fn required_phrases(blocker: &str) -> Vec<&'static str> {
    REQUIRED_PHRASES
        .iter()
        .filter(|(candidate, _)| *candidate == blocker)
        .map(|(_, phrase)| *phrase)
        .collect()
}

#[cfg(test)]
mod tests;
