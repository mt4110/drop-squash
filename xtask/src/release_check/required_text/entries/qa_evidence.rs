use super::Entry;

pub(super) const ENTRIES: &[Entry] = &[
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
        "read-only `GITHUB_TOKEN` permissions",
    ),
    ("docs/qa-evidence.md", "unsigned release failure gate"),
    ("docs/qa-evidence.md", ".codex`/Nix local ignore policy"),
    ("docs/qa-evidence.md", "file names"),
    ("docs/qa-evidence.md", "benchmark CSV path evidence"),
    (
        "docs/qa-evidence.md",
        "approved external links with host/path boundaries",
    ),
    (
        "docs/qa-evidence.md",
        "quoted and unquoted HTML links/resources/form actions",
    ),
    ("docs/qa-evidence.md", "UDIF `.dmg` artifacts"),
    ("docs/qa-evidence.md", "concrete identity fields"),
    (
        "docs/qa-evidence.md",
        "same `.dmg` file evidence for artifact/checksum/signing/Gatekeeper rows",
    ),
    (
        "docs/qa-evidence.md",
        "public signing/notary/stapler/`spctl` evidence",
    ),
    ("docs/qa-evidence.md", "Gatekeeper no-warning evidence"),
    (
        "docs/qa-evidence.md",
        "same-machine release candidate benchmark baseline evidence",
    ),
    (
        "docs/qa-evidence.md",
        "intended product license setup evidence",
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
    (
        "docs/qa-evidence.md",
        "prepared Markdown label synchronization",
    ),
    ("docs/qa-evidence.md", "--restore-state"),
    ("docs/qa-evidence.md", "Tauri DMG output is normalized"),
    ("docs/qa-evidence.md", "cargo test -p xtask normalize_dmg"),
    ("docs/qa-evidence.md", "non-DMG targets are rejected"),
    (
        "docs/qa-evidence.md",
        "canonical `DropSquash.dmg` file name",
    ),
    ("docs/qa-evidence.md", "wrong-name targets are rejected"),
    (
        "docs/qa-evidence.md",
        "artifact file name instead of local parent directories",
    ),
    (
        "docs/qa-evidence.md",
        "conversion/queue/Trash/license action-state evidence",
    ),
    (
        "docs/qa-evidence.md",
        "post-encode postprocess/history guard",
    ),
    (
        "docs/qa-evidence.md",
        "Verified manual QA blockers reject weak evidence",
    ),
    ("docs/qa-evidence.md", "privacy receipt file-name handling"),
    (
        "docs/qa-evidence.md",
        "64-character hex fingerprint requirements",
    ),
    (
        "docs/qa-evidence.md",
        "failed activation raw-key/fingerprint/instance absence",
    ),
    ("docs/qa-evidence.md", "plain and encoded error redaction"),
    ("docs/qa-evidence.md", "non-lowercase checksums"),
    ("docs/qa-evidence.md", "no-success trial/history"),
    ("docs/qa-evidence.md", "numeric queue counts"),
    ("docs/qa-evidence.md", "verified smaller Trash output"),
    (
        "docs/qa-evidence.md",
        "Lemon Squeezy sandbox activation with 64-character hex fingerprint and `instance_id`",
    ),
    ("docs/qa-evidence.md", "unchanged trial counts"),
];
