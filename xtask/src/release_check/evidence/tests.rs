use super::{manual_pairs, missing_manual_only_coverage};

#[test]
fn accepts_matching_evidence_and_manual_checks() {
    let evidence = manual_pairs::ALL
        .iter()
        .map(|(area, _)| format!("| {area} | Requires manual proof |\n"))
        .collect::<String>();
    let manual = manual_pairs::ALL
        .iter()
        .map(|(_, check)| format!("| {check} | Input | Expected | Evidence |\n"))
        .collect::<String>();

    assert!(missing_manual_only_coverage(&evidence, &manual).is_empty());
}

#[test]
fn reports_missing_manual_only_area() {
    let missing = missing_manual_only_coverage("", "");

    assert!(missing
        .iter()
        .any(|error| error.contains("Choose recording conversion")));
}

#[test]
fn reports_missing_manual_qa_check() {
    let evidence = manual_pairs::ALL
        .iter()
        .map(|(area, _)| format!("| {area} | Requires manual proof |\n"))
        .collect::<String>();
    let missing = missing_manual_only_coverage(&evidence, "");

    assert!(missing
        .iter()
        .any(|error| error.contains("manual-qa missing")));
}

const EXTERNAL_ONLY_AREAS: &[&str] = &[
    "Public website deployment",
    "Pricing finalized",
    "Refund policy finalized",
    "Live checkout link",
    "Published checksum",
    "Homebrew cask install",
];

#[test]
fn manual_only_qa_rows_are_mapped_or_classified() {
    let evidence = std::fs::read_to_string("../docs/qa-evidence.md").unwrap();
    let missing = manual_only_areas(&evidence)
        .into_iter()
        .filter(|area| {
            !manual_pairs::ALL.iter().any(|(mapped, _)| mapped == area)
                && !EXTERNAL_ONLY_AREAS.contains(&area.as_str())
        })
        .collect::<Vec<_>>();

    assert!(missing.is_empty(), "{missing:?}");
}

#[test]
fn external_manual_only_areas_are_release_url_fields() {
    let missing = EXTERNAL_ONLY_AREAS
        .iter()
        .filter(|area| {
            !crate::release_url_fields::PAIRS
                .iter()
                .any(|(blocker, _)| blocker == *area)
        })
        .collect::<Vec<_>>();

    assert!(missing.is_empty(), "{missing:?}");
}

#[test]
fn rejects_partial_manual_qa_label_match() {
    let evidence = manual_pairs::ALL
        .iter()
        .map(|(area, _)| format!("| {area} | Requires manual proof |\n"))
        .collect::<String>();
    let manual = manual_pairs::ALL
        .iter()
        .map(|(_, check)| format!("| Prefix {check} | Input | Expected | Evidence |\n"))
        .collect::<String>();

    assert!(missing_manual_only_coverage(&evidence, &manual)
        .iter()
        .any(|error| error.contains("manual-qa missing")));
}

#[test]
fn rejects_malformed_manual_qa_rows() {
    let evidence = manual_pairs::ALL
        .iter()
        .map(|(area, _)| format!("| {area} | Requires manual proof |\n"))
        .collect::<String>();
    let manual = manual_pairs::ALL
        .iter()
        .map(|(_, check)| format!("| {check} | Incomplete |\n"))
        .collect::<String>();

    assert!(missing_manual_only_coverage(&evidence, &manual)
        .iter()
        .any(|error| error.contains("manual-qa missing")));
}

#[test]
fn rejects_partial_evidence_area_match() {
    let evidence = manual_pairs::ALL
        .iter()
        .map(|(area, _)| format!("| Prefix {area} | Requires manual proof |\n"))
        .collect::<String>();
    let manual = manual_pairs::ALL
        .iter()
        .map(|(_, check)| format!("| {check} | Input | Expected | Evidence |\n"))
        .collect::<String>();

    assert!(missing_manual_only_coverage(&evidence, &manual)
        .iter()
        .any(|error| error.contains("qa-evidence missing")));
}

fn manual_only_areas(markdown: &str) -> Vec<String> {
    let mut in_section = false;
    let mut in_table = false;
    let mut areas = Vec::new();

    for line in markdown.lines() {
        if line.trim() == "## Manual-Only Evidence" {
            in_section = true;
            continue;
        }
        if in_section && line.starts_with("## ") {
            break;
        }
        if !in_section || !line.starts_with('|') {
            continue;
        }
        if line.starts_with("| Area |") {
            in_table = true;
            continue;
        }
        if in_table && !line.contains("---") {
            if let Some(area) = line.trim_matches('|').split('|').next().map(str::trim) {
                areas.push(area.to_string());
            }
        }
    }

    areas
}
