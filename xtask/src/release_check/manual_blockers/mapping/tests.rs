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

fn is_required_manual_qa_label(label: &str) -> bool {
    crate::manual_qa_check::requirements::REQUIRED_FIELDS.contains(&label)
        || crate::manual_qa_check::requirements::REQUIRED_CHECKS.contains(&label)
}
