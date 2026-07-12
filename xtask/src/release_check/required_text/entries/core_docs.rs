use super::Entry;

pub(super) const ENTRIES: &[Entry] = &[
    ("docs/release.md", "docs/release-blockers.md"),
    ("docs/release.md", "traceable Evidence"),
    ("docs/release-blockers.md", "must include the public URL"),
    ("docs/productization.md", "docs/release-blockers.md"),
    ("docs/productization.md", "final publish gate"),
    (
        "docs/productization.md",
        "refund/support contact copy gates",
    ),
    (
        "docs/productization.md",
        "conversion/queue/Trash/license action-state release notes evidence",
    ),
    (
        "docs/productization.md",
        "same-DMG manual QA signing evidence",
    ),
    ("docs/productization.md", "Gatekeeper no-warning evidence"),
    (
        "docs/productization.md",
        "release-notes URL reference matching",
    ),
    ("docs/release.md", "tested public `DropSquash.dmg`"),
    ("README.md", "macOS today"),
    ("README.md", "Windows and Linux support is planned"),
    (
        "Cargo.toml",
        "repository = \"https://github.com/mt4110/drop-squash\"",
    ),
    ("docs/product.md", "macOS today"),
    ("docs/product.md", "Windows and Linux support is planned"),
    ("docs/privacy.md", "dropsquash receipt <output.mp4>"),
    (
        "docs/benchmarking.md",
        "at least three private local samples",
    ),
    ("docs/benchmarking.md", "--release-set"),
    ("docs/benchmarking.md", "absolute `--output-dir`"),
    ("docs/benchmarking.md", "20%"),
    ("website/README.md", "docs/release-blockers.md"),
    ("apps/desktop/src-tauri/tauri.conf.json", "!\"updater\""),
];
