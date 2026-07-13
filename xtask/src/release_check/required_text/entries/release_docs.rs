use super::Entry;

pub(super) const ENTRIES: &[Entry] = &[
    ("docs/release.md", "release-notes-prepare"),
    (
        "docs/release.md",
        "--markdown-output /tmp/dropsquash-release-notes-prepared.md",
    ),
    ("docs/release.md", "intentionally marked pending"),
    (
        "docs/release.md",
        "`release-notes-prepare` rejects artifacts older",
    ),
    ("docs/release.md", "than `HEAD`"),
    ("docs/release.md", "dirty git worktree"),
    ("docs/release.md", "Prepared draft placeholders"),
    ("docs/release.md", "not public release evidence"),
    (
        "docs/release.md",
        "`release-notes-check` and `publish-check` reject those draft placeholders",
    ),
    (
        "docs/release.md",
        "publication until signed packaging exists",
    ),
    ("docs/release.md", "The Public web proof must show"),
    ("docs/release.md", "`Pricing finalized`"),
    ("docs/release.md", "draft pricing"),
    ("docs/release.md", "Pricing URL"),
    (
        "docs/release.md",
        "Artifact URL, Public website URL, Pricing URL, Refund policy URL",
    ),
    (
        "docs/release.md",
        "macOS verification commands, signing evidence drafts",
    ),
    (
        "docs/release.md",
        "macOS verification command drafts also use the same checked DMG path",
    ),
    ("docs/release.md", "signed-dmg-prepare"),
    (
        "docs/release.md",
        "refuses to overwrite an existing signed target",
    ),
    (
        "docs/release.md",
        "Live checkout URL, GitHub Release URL, and Homebrew tap PR URL",
    ),
    ("docs/release.md", "checkout, and refund links"),
    (
        "docs/release.md",
        "The Signing and distribution proof must show",
    ),
    (
        "docs/release.md",
        "installable through the Homebrew tap PR",
    ),
    (
        "docs/release-notes-template.md",
        "existing CSV path outside the repository",
    ),
    (
        "docs/release-notes-template.md",
        "exact lowercase SHA-256 digest",
    ),
    (
        "docs/release-notes-template.md",
        "same-machine comparison",
    ),
    (
        "docs/release-notes-template.md",
        "release candidate baseline",
    ),
    (
        "docs/reproducible-builds.md",
        "nix develop --command pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci",
    ),
];
