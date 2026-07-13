use super::{duplicate_blockers, unknown_blockers, unplanned_blockers};

#[test]
fn release_blockers_template_plans_required_rows() {
    let text = std::fs::read_to_string("../docs/release-blockers.md").unwrap();

    assert!(unplanned_blockers(&text).is_empty());
    assert!(unknown_blockers(&text).is_empty());
    assert!(duplicate_blockers(&text).is_empty());
}

#[test]
fn reports_missing_execution_order_blocker() {
    let text = "\
| 1 | Local packaged-app proof | Packaged macOS manual QA | Public DMG evidence | `docs/manual-qa.md` |
";

    let unplanned = unplanned_blockers(text);

    assert!(unplanned.contains(&"Benchmark release set"));
}

#[test]
fn reports_blocker_in_wrong_execution_track() {
    let text = "\
| 1 | Local packaged-app proof | Packaged macOS manual QA, Benchmark release set | Public DMG evidence | `docs/manual-qa.md` |
| 2 | License sandbox proof | Public website deployment | License evidence | `docs/manual-qa.md` |
";

    let unplanned = unplanned_blockers(text);

    assert!(unplanned.contains(&"Public website deployment"));
}

#[test]
fn reports_unknown_execution_order_blocker() {
    let text = "\
| 1 | Local packaged-app proof | Packaged macOS manual QA, Extra launch task | Public DMG evidence | `docs/manual-qa.md` |
";

    let unknown = unknown_blockers(text);

    assert_eq!(unknown, vec!["Extra launch task"]);
}

#[test]
fn reports_duplicate_execution_order_blocker() {
    let text = "\
| 1 | Local packaged-app proof | Packaged macOS manual QA, Benchmark release set | Public DMG evidence | `docs/manual-qa.md` |
| 2 | License sandbox proof | Benchmark release set, Valid sandbox activation | License evidence | `docs/manual-qa.md` |
";

    let duplicates = duplicate_blockers(text);

    assert_eq!(duplicates, vec!["Benchmark release set"]);
}

#[test]
fn ignores_execution_order_header_rows() {
    let text =
        "| Order | Track | Blockers | Exit condition | Record target |\n|---:|---|---|---|---|\n";

    assert!(unknown_blockers(text).is_empty());
}
