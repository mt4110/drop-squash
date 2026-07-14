use super::Entry;

mod site;

pub(super) const SITE: &[Entry] = site::ENTRIES;

pub(super) const CORE: &[Entry] = &[
    ("docs/qa-evidence.md", "docs/release-blockers.md"),
    (
        "docs/qa-evidence.md",
        "license, refund, support contact copy",
    ),
    (
        "docs/qa-evidence.md",
        "parallel version manager config rejection",
    ),
    (
        "docs/qa-evidence.md",
        "productization Done overclaim rejection",
    ),
    (
        "docs/qa-evidence.md",
        "productization Release Gate text coverage",
    ),
    (
        "docs/qa-evidence.md",
        "read-only `GITHUB_TOKEN` permissions",
    ),
    ("docs/qa-evidence.md", "unsigned release failure gate"),
    ("docs/qa-evidence.md", ".codex`/Nix local ignore policy"),
    ("docs/qa-evidence.md", "required Nix dev shell systems"),
    ("docs/qa-evidence.md", "file names"),
    ("docs/qa-evidence.md", "raw-key field-name rejection"),
    (
        "docs/qa-evidence.md",
        "raw-license-key field-name rejection",
    ),
    (
        "docs/qa-evidence.md",
        "expired refresh priority over trial completion",
    ),
    (
        "docs/qa-evidence.md",
        "license networking outside the pinned Lemon Squeezy API endpoint",
    ),
    ("docs/qa-evidence.md", "benchmark CSV path evidence"),
    (
        "docs/qa-evidence.md",
        "release blocker to release notes evidence-field mapping",
    ),
    (
        "docs/qa-evidence.md",
        "release notes URL field publish-check synchronization",
    ),
    (
        "docs/qa-evidence.md",
        "manual-only evidence mapping or external classification",
    ),
    (
        "docs/qa-evidence.md",
        "Manual QA required field/check label synchronization",
    ),
    (
        "docs/qa-evidence.md",
        "Execution Order to Evidence Classes synchronization",
    ),
    ("docs/qa-evidence.md", "placeholder checkout buy IDs"),
    (
        "docs/qa-evidence.md",
        "backend/saved-percent/duration/speed-ratio CSV fields",
    ),
    (
        "docs/qa-evidence.md",
        "release-set duration and speed-ratio evidence rejection",
    ),
    ("docs/qa-evidence.md", "queued waiting-row cancellation"),
    (
        "docs/qa-evidence.md",
        "refresh-lock queued message preservation",
    ),
    ("docs/qa-evidence.md", "updater enablement"),
    ("docs/qa-evidence.md", "Tauri updater config absence"),
    (
        "docs/qa-evidence.md",
        "missing update manifests while updater is disabled",
    ),
    ("docs/qa-evidence.md", "clean git worktree preflight"),
    ("docs/qa-evidence.md", "Nix QA build preflight"),
    (
        "docs/qa-evidence.md",
        "cargo test -p xtask productization_status",
    ),
    (
        "docs/qa-evidence.md",
        "repository-local and relative app-state/state/output/markdown-output path rejection",
    ),
    ("docs/qa-evidence.md", "`.md` markdown-output rejection"),
    (
        "docs/qa-evidence.md",
        "existing markdown-output no-clobber rejection",
    ),
    ("docs/qa-evidence.md", "--restore-state"),
    (
        "docs/qa-evidence.md",
        "conversion/queue/Trash/license action-state evidence",
    ),
    (
        "docs/qa-evidence.md",
        "post-encode postprocess/history guard",
    ),
    ("docs/qa-evidence.md", "temp cleanup evidence"),
    ("docs/qa-evidence.md", "native-encoder-unavailable message"),
    ("docs/qa-evidence.md", "non-lowercase checksums"),
    ("docs/qa-evidence.md", "contradictory raw-key evidence"),
];
