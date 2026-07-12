use super::url;

pub(super) const URL: [(&str, url::Kind); 6] = [
    ("Artifact URL", url::Kind::Artifact),
    ("Public website URL", url::Kind::Website),
    ("Refund policy URL", url::Kind::Refund),
    ("Live checkout URL", url::Kind::Checkout),
    ("GitHub Release URL", url::Kind::GitHubRelease),
    ("Homebrew tap PR URL", url::Kind::HomebrewPullRequest),
];

pub(super) const EVIDENCE: [&str; 22] = [
    "`codesign`",
    "`spctl`",
    "`stapler`",
    "Apple notary log",
    "Gatekeeper clean-machine open",
    "`docs/release-blockers.md` status",
    "Manual QA record",
    "Conversion safety evidence",
    "Queue evidence",
    "Trash source policy",
    "Lemon Squeezy product setup",
    "Lemon Squeezy sandbox purchase",
    "Valid sandbox activation",
    "Empty key activation",
    "Invalid license key handling",
    "License network failure",
    "Local license forget",
    "GitHub Release checksum",
    "Homebrew tap PR",
    "Homebrew install result",
    "Known limitations",
    "Support contact",
];

#[cfg(test)]
mod tests {
    use super::URL;

    #[test]
    fn release_url_pairs_point_to_required_release_note_url_fields() {
        let missing = crate::release_url_fields::PAIRS
            .iter()
            .filter(|(_, field)| !URL.iter().any(|(label, _)| label == field))
            .collect::<Vec<_>>();

        assert!(missing.is_empty());
    }

    #[test]
    fn release_url_pairs_point_to_required_blockers() {
        let missing = crate::release_url_fields::PAIRS
            .iter()
            .filter(|(blocker, _)| !crate::release_check::required_blockers().contains(blocker))
            .collect::<Vec<_>>();

        assert!(missing.is_empty());
    }
}
