use super::missing_manual_verified_evidence;

#[test]
fn accepts_verified_manual_blocker_with_manual_result() {
    let blockers = "| Lemon Squeezy sandbox purchase | Verified | Sandbox checkout completes | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual =
        "| Sandbox purchase | Checkout completes | Completed with test buyer order abc123 |\n";

    assert!(missing_manual_verified_evidence(blockers, manual).is_empty());
}

#[test]
fn reports_verified_manual_blocker_without_manual_result() {
    let blockers = "| Valid sandbox activation | Verified | raw key is absent | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| Valid sandbox activation | Pro state; raw key absent from cache |  |\n";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"Valid sandbox activation"));
}

#[test]
fn reports_incomplete_packaged_macos_manual_qa() {
    let blockers = "| Packaged macOS manual QA | Verified | Filled manual QA table | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| Choose recording conversion | Small recording | Creates smaller MP4 | Concrete file output |\n";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"Packaged macOS manual QA"));
}

#[test]
fn ignores_blocked_manual_blockers() {
    let blockers = "| Gatekeeper clean-machine open | Blocked | Fresh macOS account opens app | TBD | `docs/manual-qa.md` |\n";
    let manual = "| Gatekeeper open test | Signed app opens cleanly |  |\n";

    assert!(missing_manual_verified_evidence(blockers, manual).is_empty());
}
