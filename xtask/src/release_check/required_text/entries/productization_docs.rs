use super::Entry;

mod release;

pub(super) const RELEASE: &[Entry] = release::ENTRIES;

pub(super) const CORE: &[Entry] = &[
    ("docs/productization.md", "docs/release-blockers.md"),
    ("docs/productization.md", "final publish gate"),
    (
        "docs/productization.md",
        "macOS single-file conversion is verified",
    ),
    ("docs/productization.md", "cancel does not count trial"),
    (
        "docs/productization.md",
        "failed conversion does not count trial",
    ),
    (
        "docs/productization.md",
        "larger output is treated as failure",
    ),
    (
        "docs/productization.md",
        "original is never moved without verified success",
    ),
    (
        "docs/productization.md",
        "privacy claims match implementation",
    ),
    (
        "docs/productization.md",
        "license secrets are not in the repository",
    ),
    (
        "docs/productization.md",
        "refund/support contact copy gates",
    ),
    (
        "docs/productization.md",
        "conversion/queue/Trash/license action-state release notes evidence",
    ),
];
