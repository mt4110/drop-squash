use std::path::Path;

mod manual_pairs;
mod table;

pub(super) fn check_manual_only_coverage(
    qa_evidence: &Path,
    manual_qa: &Path,
) -> Result<(), String> {
    let evidence = std::fs::read_to_string(qa_evidence).map_err(|error| error.to_string())?;
    let manual = std::fs::read_to_string(manual_qa).map_err(|error| error.to_string())?;
    let missing = missing_manual_only_coverage(&evidence, &manual);
    if missing.is_empty() {
        return Ok(());
    }
    Err(format!(
        "manual-only QA evidence is incomplete: {}",
        missing.join(", ")
    ))
}

fn missing_manual_only_coverage(evidence: &str, manual: &str) -> Vec<String> {
    manual_pairs::ALL
        .iter()
        .filter_map(|(area, check)| missing_pair(evidence, manual, area, check))
        .collect()
}

fn missing_pair(evidence: &str, manual: &str, area: &str, check: &str) -> Option<String> {
    if !table::has_row_label(evidence, area, 2) {
        return Some(format!("qa-evidence missing {area}"));
    }
    if !table::has_row_label(manual, check, 3) {
        return Some(format!("manual-qa missing {check}"));
    }
    None
}

#[cfg(test)]
mod tests {
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
}
