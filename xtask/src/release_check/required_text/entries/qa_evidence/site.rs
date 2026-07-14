use super::Entry;

pub(super) const ENTRIES: &[Entry] = &[
    (
        "docs/qa-evidence.md",
        "approved external links with host/path boundaries and no query/fragment tracking",
    ),
    (
        "docs/qa-evidence.md",
        "quoted and unquoted HTML links/resources/form actions",
    ),
    (
        "docs/qa-evidence.md",
        "website-root boundary rejection for local links/resources",
    ),
    ("docs/qa-evidence.md", "local link fragments"),
    ("docs/qa-evidence.md", "short download CTAs"),
    ("docs/qa-evidence.md", "natural release copy"),
    (
        "docs/qa-evidence.md",
        "unsupported DMG cleanup claim rejection",
    ),
];
