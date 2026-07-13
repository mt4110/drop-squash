use super::Entry;

pub(super) const ENTRIES: &[Entry] = &[
    ("docs/qa-evidence.md", "release-notes-check"),
    ("docs/qa-evidence.md", "release-notes-prepare"),
    (
        "docs/qa-evidence.md",
        "release notes template field-label synchronization",
    ),
    ("docs/qa-evidence.md", "GitHub Release URL"),
    ("docs/qa-evidence.md", "SHA256SUMS line"),
    ("docs/qa-evidence.md", "SHA256SUMS output command"),
    ("docs/qa-evidence.md", "`--output` must be named `SHA256SUMS`"),
    ("docs/qa-evidence.md", "pending checksum upload placeholder"),
    (
        "docs/qa-evidence.md",
        "stale artifacts older than `HEAD` are rejected",
    ),
    ("docs/qa-evidence.md", "dirty git worktrees are rejected"),
    ("docs/qa-evidence.md", "matching Homebrew cask command"),
    ("docs/qa-evidence.md", "Homebrew tap PR evidence draft"),
    (
        "docs/release-notes-template.md",
        "Homebrew tap PR URL above",
    ),
    (
        "docs/release-notes-template.md",
        "cancellation returning ready after temp cleanup",
    ),
    (
        "docs/release-notes-template.md",
        "history, and no new success",
    ),
    (
        "docs/release-notes-template.md",
        "waiting or queued row cancellation",
    ),
    (
        "docs/qa-evidence.md",
        "new absolute `.md` path outside the repository",
    ),
    ("docs/qa-evidence.md", "version-matched public Artifact URL"),
    ("docs/qa-evidence.md", "non-HTTPS URLs"),
    ("docs/qa-evidence.md", "URL query or fragment parts"),
    (
        "docs/qa-evidence.md",
        "version-matched public Artifact URL with host/path boundaries",
    ),
    (
        "docs/qa-evidence.md",
        "Artifact URL/GitHub Release URL version mismatch",
    ),
    ("docs/qa-evidence.md", "imposter public URL hosts"),
    (
        "docs/qa-evidence.md",
        "non-canonical checkout hosts outside `store.lemonsqueezy.com`",
    ),
    (
        "docs/qa-evidence.md",
        "missing existing benchmark CSV path outside repo",
    ),
    ("docs/qa-evidence.md", "imposter GitHub release hosts"),
    (
        "docs/qa-evidence.md",
        "checksum evidence without public context, the Artifact URL, GitHub Release URL, or exact lowercase SHA-256 digest",
    ),
    (
        "docs/qa-evidence.md",
        "prepared draft markers/placeholders",
    ),
    (
        "docs/qa-evidence.md",
        "Homebrew install evidence without the Artifact URL, lowercase SHA-256 digest, or uninstall result",
    ),
    ("docs/qa-evidence.md", "non-lowercase Git commits"),
    (
        "docs/qa-evidence.md",
        "signing/notary/stapler/`spctl`, Manual QA record, and Gatekeeper evidence without the exact Artifact URL",
    ),
];
