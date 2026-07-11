use super::{row, REQUIRED_BLOCKERS};

const REQUIRED_PHRASES: [(&str, &str); 18] = [
    ("Packaged macOS manual QA", ".app` or `.dmg"),
    ("Lemon Squeezy sandbox purchase", "Sandbox checkout"),
    ("Lemon Squeezy sandbox purchase", "test buyer"),
    ("Lemon Squeezy sandbox purchase", "order"),
    ("Valid sandbox activation", "raw key is absent"),
    ("Invalid license key handling", "raw key is absent"),
    ("Local license forget", "Local cache"),
    ("Public website deployment", "Production website serves"),
    ("Live checkout link", "Public pricing page opens"),
    ("Signed DMG", "`codesign`"),
    ("Signed DMG", "Developer ID"),
    ("Notarized and stapled DMG", "`spctl`"),
    ("Notarized and stapled DMG", "notary"),
    ("Notarized and stapled DMG", "stapled"),
    ("Gatekeeper clean-machine open", "Fresh macOS account"),
    ("Published checksum", "SHA-256"),
    ("Homebrew cask install", "brew install --cask"),
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
