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

pub(super) fn reference_matches_record_target(blocker: &str, reference: &str) -> bool {
    let Some((_, target)) = EXPECTED_RECORD_TARGETS
        .iter()
        .find(|(candidate, _)| *candidate == blocker)
    else {
        return false;
    };
    match *target {
        "`docs/manual-qa.md`" => reference.starts_with("`docs/manual-qa.md"),
        "`https://...`" if blocker == "Public website deployment" => is_public_website(reference),
        "`https://...`" if blocker == "Live checkout link" => is_live_checkout(reference),
        "`https://...`" => reference.starts_with("https://"),
        "GitHub Release" => has_expected_url(
            reference,
            "GitHub Release",
            "https://github.com/mt4110/drop-squash/releases/tag/",
        ),
        "Homebrew tap PR" => has_expected_url(
            reference,
            "Homebrew tap PR",
            "https://github.com/mt4110/homebrew-tap/pull/",
        ),
        other => reference == other,
    }
}

fn is_public_website(reference: &str) -> bool {
    let lower = reference.to_ascii_lowercase();
    reference.starts_with("https://")
        && !lower.contains("lemonsqueezy.com")
        && !lower.contains("checkout")
}

fn is_live_checkout(reference: &str) -> bool {
    let lower = reference.to_ascii_lowercase();
    reference.starts_with("https://")
        && lower.contains("lemonsqueezy.com")
        && lower.contains("checkout")
}

fn has_expected_url(reference: &str, label: &str, prefix: &str) -> bool {
    reference.starts_with(label) && reference.contains(prefix)
}

#[cfg(test)]
mod tests;
