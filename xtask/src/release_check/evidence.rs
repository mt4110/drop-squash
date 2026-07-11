use std::path::Path;

const REQUIRED_MANUAL_EVIDENCE: [(&str, &str); 21] = [
    ("Choose recording conversion", "Choose recording conversion"),
    ("Drag-and-drop conversion", "Drag-and-drop conversion"),
    ("Privacy receipt sidecar", "Privacy receipt sidecar"),
    ("Privacy receipt Finder reveal", "Reveal privacy receipt"),
    ("Duplicate output naming", "Duplicate output naming"),
    ("Finder reveal", "Reveal output"),
    ("Ask source policy", "Ask source policy"),
    ("Trash source policy", "Trash source policy"),
    ("Failed conversion", "Failed conversion"),
    ("Larger output", "Larger output"),
    ("Large-recording cancellation", "Cancellation"),
    ("Benchmark sample set", "Benchmark sample set"),
    (
        "Benchmark regression threshold",
        "Benchmark regression threshold",
    ),
    ("Multi-file queue", "Multi-file queue"),
    ("Queued job cancellation", "Queued job cancellation"),
    ("Batch summary", "Batch summary"),
    ("Lemon Squeezy sandbox purchase", "Sandbox purchase"),
    (
        "Lemon Squeezy sandbox activation",
        "Valid sandbox activation",
    ),
    ("Signed DMG verification", "Codesign verification"),
    (
        "Notarized/stapled DMG verification",
        "Notarization staple verification",
    ),
    ("Signed/notarized Gatekeeper open", "Gatekeeper open test"),
];

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
    REQUIRED_MANUAL_EVIDENCE
        .iter()
        .filter_map(|(area, check)| missing_pair(evidence, manual, area, check))
        .collect()
}

fn missing_pair(evidence: &str, manual: &str, area: &str, check: &str) -> Option<String> {
    if !evidence.contains(&format!("| {area} |")) {
        return Some(format!("qa-evidence missing {area}"));
    }
    if !manual.contains(&format!("| {check} |")) {
        return Some(format!("manual-qa missing {check}"));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{missing_manual_only_coverage, REQUIRED_MANUAL_EVIDENCE};

    #[test]
    fn accepts_matching_evidence_and_manual_checks() {
        let evidence = REQUIRED_MANUAL_EVIDENCE
            .iter()
            .map(|(area, _)| format!("| {area} | Requires manual proof |\n"))
            .collect::<String>();
        let manual = REQUIRED_MANUAL_EVIDENCE
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
        let evidence = REQUIRED_MANUAL_EVIDENCE
            .iter()
            .map(|(area, _)| format!("| {area} | Requires manual proof |\n"))
            .collect::<String>();
        let missing = missing_manual_only_coverage(&evidence, "");

        assert!(missing
            .iter()
            .any(|error| error.contains("manual-qa missing")));
    }
}
