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

#[test]
fn signing_distribution_blockers_map_to_template_fields() {
    let template = std::fs::read_to_string("../docs/release-notes-template.md").unwrap();
    let blockers_text = std::fs::read_to_string("../docs/release-blockers.md").unwrap();
    for blocker in [
        "Signed DMG",
        "Notarized and stapled DMG",
        "Gatekeeper clean-machine open",
        "Published checksum",
        "Homebrew cask install",
    ] {
        assert!(blockers_text.contains(blocker), "{blocker}");
        let (_, fields) = blockers::MAPPING
            .iter()
            .find(|(mapped, _)| *mapped == blocker)
            .expect(blocker);
        for field in *fields {
            assert!(
                template.contains(&format!("- {field}:")),
                "{blocker}: {field}"
            );
        }
    }
}

fn checked_field(field: &str) -> bool {
    URL.iter().any(|(label, _)| *label == field)
        || EVIDENCE.contains(&field)
        || matches!(
            field,
            "Benchmark sample set" | "Benchmark regression threshold"
        )
}
