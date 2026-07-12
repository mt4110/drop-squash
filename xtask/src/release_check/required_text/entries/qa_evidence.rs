use super::Entry;

pub(super) const ENTRIES: &[Entry] = &[
    ("docs/qa-evidence.md", "docs/release-blockers.md"),
    (
        "docs/qa-evidence.md",
        "license, refund, support contact copy",
    ),
    ("docs/qa-evidence.md", ".codex`/Nix local ignore policy"),
    ("docs/qa-evidence.md", "file names"),
    ("docs/qa-evidence.md", "UDIF `.dmg` artifacts"),
    ("docs/qa-evidence.md", "concrete identity fields"),
    ("docs/qa-evidence.md", "same `.dmg` file evidence"),
    ("docs/qa-evidence.md", "Gatekeeper no-warning evidence"),
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
    ("docs/qa-evidence.md", "publish-check"),
    (
        "docs/qa-evidence.md",
        "release blocker URLs matching the release notes URLs",
    ),
    (
        "docs/qa-evidence.md",
        "public publish references that point to the release notes URLs",
    ),
    ("docs/qa-evidence.md", "traceable Evidence reference"),
    ("docs/qa-evidence.md", "release-notes-check"),
    (
        "docs/qa-evidence.md",
        "conversion/queue/Trash/license action-state evidence",
    ),
    (
        "docs/qa-evidence.md",
        "post-encode postprocess/history guard",
    ),
    ("docs/qa-evidence.md", "UDIF trailer"),
    ("docs/qa-evidence.md", "wrong DMG names"),
    ("docs/qa-evidence.md", "non-canonical homepages"),
    ("docs/qa-evidence.md", "Public website deployment"),
    ("docs/qa-evidence.md", "Live checkout link"),
    ("docs/qa-evidence.md", "Published checksum"),
    ("docs/qa-evidence.md", "Homebrew cask install"),
    ("docs/qa-evidence.md", "versioned `DropSquash.dmg` artifact"),
    ("docs/qa-evidence.md", "`auto_updates false`"),
    ("docs/qa-evidence.md", "declares no in-app auto-update"),
    ("docs/qa-evidence.md", "malformed App Store Connect key ids"),
    ("docs/qa-evidence.md", "malformed issuer UUIDs"),
];
