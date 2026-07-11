use super::row;

const EXPECTED_RECORD_TARGETS: [(&str, &str); 10] = [
    ("Packaged macOS manual QA", "`docs/manual-qa.md`"),
    ("Lemon Squeezy sandbox purchase", "`docs/manual-qa.md`"),
    ("Valid sandbox activation", "`docs/manual-qa.md`"),
    ("Public website deployment", "`https://...`"),
    ("Live checkout link", "`https://...`"),
    ("Signed DMG", "Release notes"),
    ("Notarized and stapled DMG", "Release notes"),
    ("Gatekeeper clean-machine open", "`docs/manual-qa.md`"),
    ("Published checksum", "GitHub Release"),
    ("Homebrew cask install", "Homebrew tap PR"),
];

pub(super) fn misplaced_record_targets(text: &str) -> Vec<&'static str> {
    EXPECTED_RECORD_TARGETS
        .iter()
        .copied()
        .filter(|(blocker, expected)| {
            row::find(text, blocker)
                .and_then(row::record_in)
                .is_some_and(|actual| actual != *expected)
        })
        .map(|(blocker, _)| blocker)
        .collect()
}

#[cfg(test)]
mod tests;
