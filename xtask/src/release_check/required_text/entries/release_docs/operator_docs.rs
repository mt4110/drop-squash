use super::Entry;

pub(super) const ENTRIES: &[Entry] = &[
    ("docs/paid-beta-readiness.md", "Quick routes:"),
    ("docs/paid-beta-readiness.md", "deferred public proof 6 行を回収する"),
    ("docs/paid-beta-readiness.md", "public-web-ready"),
    (
        "docs/paid-beta-readiness.md",
        "cargo run -p xtask -- productization-status --track \"Paid beta\"",
    ),
    (
        "docs/manual-beta-license-issuance.md",
        "This document is for private/manual beta operations only.",
    ),
    (
        "docs/manual-beta-license-issuance.md",
        "## Short Execution Memo",
    ),
    (
        "docs/manual-beta-license-issuance.md",
        "docs/public-beta-operator-checklist.md` -> `Short Execution Memo`",
    ),
    (
        "docs/manual-beta-license-issuance.md",
        "cargo run -p xtask -- public-web-ready",
    ),
    (
        "docs/manual-beta-license-issuance.md",
        "Sign in to Lemon Squeezy",
    ),
    (
        "docs/manual-beta-license-issuance.md",
        "auth.lemonsqueezy.com/login",
    ),
    (
        "xtask/src/manual_qa_ready_license/steps.rs",
        "docs/public-beta-operator-checklist.md` (`Short Execution Memo` for the fast path)",
    ),
    (
        "xtask/src/manual_qa_ready_distribution.rs",
        "docs/public-beta-operator-checklist.md` (`Short Execution Memo` for the fast path)",
    ),
    (
        "xtask/src/manual_qa_ready_all.rs",
        "scripts/manual-qa-distribution-handoff.sh",
    ),
    (
        "xtask/src/manual_qa_ready_local_proof.rs",
        "scripts/manual-qa-distribution-handoff.sh",
    ),
    (
        "xtask/src/manual_qa_packaged_rerun.rs",
        "scripts/manual-qa-distribution-handoff.sh",
    ),
    (
        "xtask/src/manual_qa_ready_license/steps.rs",
        "scripts/manual-qa-distribution-handoff.sh",
    ),
    (
        "xtask/src/manual_qa_dirty_worktree.rs",
        "cargo run -p xtask -- productization-status --track 'Paid beta'",
    ),
    (
        "docs/paid-beta-operator-checklist.md",
        "## Dirty Worktree Quickstart",
    ),
    (
        "docs/paid-beta-operator-checklist.md",
        "snapshot=$(scripts/manual-qa-snapshot-worktree.sh",
    ),
    (
        "docs/paid-beta-operator-checklist.md",
        "cargo run -p xtask -- productization-status --track \"Paid beta\"",
    ),
    (
        "docs/paid-beta-operator-checklist.md",
        "scripts/manual-qa-license-sandbox-handoff.sh",
    ),
    (
        "docs/paid-beta-operator-checklist.md",
        "scripts/manual-qa-distribution-handoff.sh",
    ),
    (
        "docs/paid-beta-operator-checklist.md",
        "manual-qa-ready-all /tmp/dropsquash-manual-qa-prepared-<app-build>.md /tmp/dropsquash-manual-qa-output/benchmark-results-<app-build>.csv",
    ),
    ("docs/public-beta-operator-checklist.md", "public-web-ready"),
];
