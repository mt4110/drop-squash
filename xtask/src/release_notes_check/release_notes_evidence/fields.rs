#[cfg(test)]
mod blockers;

use super::url;

pub(super) const URL: [(&str, url::Kind); 7] = [
    ("Artifact URL", url::Kind::Artifact),
    ("Public website URL", url::Kind::Website),
    ("Pricing URL", url::Kind::Pricing),
    ("Refund policy URL", url::Kind::Refund),
    ("Live checkout URL", url::Kind::Checkout),
    ("GitHub Release URL", url::Kind::GitHubRelease),
    ("Homebrew tap PR URL", url::Kind::HomebrewPullRequest),
];

pub(super) const EVIDENCE: [&str; 23] = [
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
    "Expired license refresh",
    "Local license forget",
    "GitHub Release checksum",
    "Homebrew tap PR",
    "Homebrew install result",
    "Known limitations",
    "Support contact",
];

#[cfg(test)]
mod tests {
    use super::{blockers, EVIDENCE, URL};

    #[test]
    fn release_notes_template_contains_checked_fields() {
        let template = std::fs::read_to_string("../docs/release-notes-template.md").unwrap();
        let missing = URL
            .iter()
            .map(|(label, _)| *label)
            .chain(EVIDENCE)
            .filter(|label| !template.contains(&format!("- {label}:")))
            .collect::<Vec<_>>();

        assert!(missing.is_empty());
    }

    #[test]
    fn release_url_pairs_point_to_required_release_note_url_fields() {
        let missing = crate::release_url_fields::PAIRS
            .iter()
            .filter(|(_, field)| !URL.iter().any(|(label, _)| label == field))
            .collect::<Vec<_>>();

        assert!(missing.is_empty());
    }

    #[test]
    fn checked_release_note_urls_are_publish_checked() {
        let missing = URL
            .iter()
            .map(|(label, _)| *label)
            .filter(|label| *label != "Artifact URL")
            .filter(|label| {
                !crate::release_url_fields::PAIRS
                    .iter()
                    .any(|(_, field)| field == label)
            })
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

    #[test]
    fn release_blockers_have_release_note_evidence_fields() {
        let missing = crate::release_check::required_blockers()
            .iter()
            .filter(|blocker| !blockers::has_mapping(blocker))
            .collect::<Vec<_>>();

        assert!(missing.is_empty());
    }

    #[test]
    fn mapped_release_note_fields_are_checked() {
        let missing = blockers::MAPPING
            .iter()
            .flat_map(|(_, fields)| fields.iter())
            .filter(|field| !checked_field(field))
            .collect::<Vec<_>>();

        assert!(missing.is_empty());
    }

    fn checked_field(field: &str) -> bool {
        URL.iter().any(|(label, _)| *label == field)
            || EVIDENCE.contains(&field)
            || matches!(
                field,
                "Benchmark sample set" | "Benchmark regression threshold"
            )
    }
}
