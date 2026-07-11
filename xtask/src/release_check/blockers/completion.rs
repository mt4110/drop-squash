use super::{row, REQUIRED_BLOCKERS};

const REQUIRED_PHRASES: [(&str, &str); 10] = [
    ("Packaged macOS manual QA", ".app` or `.dmg"),
    ("Lemon Squeezy sandbox purchase", "Sandbox checkout"),
    ("Valid sandbox activation", "raw key is absent"),
    ("Public website deployment", "Production website serves"),
    ("Live checkout link", "Public pricing page opens"),
    ("Signed DMG", "`codesign`"),
    ("Notarized and stapled DMG", "`spctl`"),
    ("Gatekeeper clean-machine open", "Fresh macOS account"),
    ("Published checksum", "SHA-256"),
    ("Homebrew cask install", "brew install --cask"),
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
        || required_phrase(blocker).is_some_and(|phrase| !value.contains(phrase))
}

fn required_phrase(blocker: &str) -> Option<&'static str> {
    REQUIRED_PHRASES
        .iter()
        .find(|(candidate, _)| *candidate == blocker)
        .map(|(_, phrase)| *phrase)
}

#[cfg(test)]
mod tests;
