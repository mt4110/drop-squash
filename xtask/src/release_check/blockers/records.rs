use super::{row, REQUIRED_BLOCKERS};

mod reference;

pub(super) fn misplaced_record_targets(text: &str) -> Vec<&'static str> {
    REQUIRED_BLOCKERS
        .iter()
        .copied()
        .filter(|blocker| {
            let Some(expected) = expected_target(blocker) else {
                return true;
            };
            row::find(text, blocker)
                .and_then(row::record_in)
                .is_some_and(|actual| actual != expected)
        })
        .collect()
}

pub(super) fn reference_matches_record_target(blocker: &str, reference: &str) -> bool {
    reference::matches_record_target(blocker, reference)
}

pub(super) fn expected_target(blocker: &str) -> Option<&'static str> {
    match blocker {
        "Packaged macOS manual QA"
        | "Lemon Squeezy product setup"
        | "Lemon Squeezy sandbox purchase"
        | "Empty key activation"
        | "Valid sandbox activation"
        | "Invalid license key handling"
        | "License network failure"
        | "Expired license refresh"
        | "Local license forget"
        | "Gatekeeper clean-machine open"
        | "Benchmark release set" => Some("`docs/manual-qa.md`"),
        "Public website deployment" | "Refund policy finalized" | "Live checkout link" => {
            Some("`https://...`")
        }
        "Signed DMG" | "Notarized and stapled DMG" => Some("Release notes"),
        "Published checksum" => Some("GitHub Release"),
        "Homebrew cask install" => Some("Homebrew tap PR"),
        _ => None,
    }
}

#[cfg(test)]
mod tests;
