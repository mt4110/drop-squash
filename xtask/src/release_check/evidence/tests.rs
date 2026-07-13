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
