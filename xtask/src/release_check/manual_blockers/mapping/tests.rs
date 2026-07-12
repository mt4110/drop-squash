use super::MANUAL_BLOCKERS;

#[test]
fn mapped_manual_blocker_checks_are_required_manual_qa_labels() {
    let missing = MANUAL_BLOCKERS
        .iter()
        .flat_map(|(_, checks)| checks.iter())
        .filter(|check| !is_required_manual_qa_label(check))
        .collect::<Vec<_>>();

    assert!(missing.is_empty());
}

#[test]
fn manual_qa_release_blockers_are_mapped() {
    let missing = manual_qa_release_blockers()
        .iter()
        .filter(|blocker| !MANUAL_BLOCKERS.iter().any(|(mapped, _)| mapped == *blocker))
        .collect::<Vec<_>>();

    assert!(missing.is_empty(), "{missing:?}");
}

fn is_required_manual_qa_label(label: &str) -> bool {
    crate::manual_qa_check::requirements::REQUIRED_FIELDS.contains(&label)
        || crate::manual_qa_check::requirements::REQUIRED_CHECKS.contains(&label)
}

fn manual_qa_release_blockers() -> &'static [&'static str] {
    &[
        "Packaged macOS manual QA",
        "Lemon Squeezy product setup",
        "Lemon Squeezy sandbox purchase",
        "Empty key activation",
        "Valid sandbox activation",
        "Invalid license key handling",
        "License network failure",
        "Local license forget",
        "Gatekeeper clean-machine open",
        "Benchmark release set",
    ]
}
