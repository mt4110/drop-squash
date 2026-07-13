use super::Entry;

pub(super) const ENTRIES: &[Entry] = &[
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
];
