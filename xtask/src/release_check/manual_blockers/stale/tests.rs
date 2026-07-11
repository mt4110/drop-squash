use super::manual_evidence_left_blocked;

#[test]
fn reports_blocked_manual_blocker_with_complete_manual_evidence() {
    let blockers =
        "| Valid sandbox activation | Blocked | raw key is absent | TBD | `docs/manual-qa.md` |\n";
    let manual = "| Valid sandbox activation | Pro state; raw key absent from cache | Activating state disabled submit; Pro reached; license.json cache checked; raw key absent |\n";

    let stale = manual_evidence_left_blocked(blockers, manual);

    assert!(stale.contains(&"Valid sandbox activation"));
}

#[test]
fn ignores_blocked_manual_blocker_without_complete_manual_evidence() {
    let blockers =
        "| Valid sandbox activation | Blocked | raw key is absent | TBD | `docs/manual-qa.md` |\n";
    let manual =
        "| Valid sandbox activation | Pro state; raw key absent from cache | Pro reached |\n";

    assert!(manual_evidence_left_blocked(blockers, manual).is_empty());
}

#[test]
fn ignores_verified_manual_blocker() {
    let blockers =
        "| Valid sandbox activation | Verified | raw key is absent | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| Valid sandbox activation | Pro state; raw key absent from cache | Activating state disabled submit; Pro reached; license.json cache checked; raw key absent |\n";

    assert!(manual_evidence_left_blocked(blockers, manual).is_empty());
}
