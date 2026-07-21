use super::Entry;

pub(super) const ENTRIES: &[Entry] = &[
    ("docs/release-blockers.md", "Signed DMG"),
    ("docs/release-blockers.md", "Notarized and stapled DMG"),
    ("docs/release-blockers.md", "stapler evidence"),
    ("docs/release-blockers.md", "Signing/notarization"),
    ("docs/release-blockers.md", "Signing and distribution proof"),
    (
        "docs/release-blockers.md",
        "signed, notarized, stapled, checksummed, installable, and opens without warning",
    ),
    ("docs/productization.md", "macOS signing command plan"),
    ("docs/productization.md", "macOS keychain argv plan"),
    ("docs/productization.md", "macOS keychain cleanup argv plan"),
    ("docs/productization.md", "macOS codesign argv plan"),
    (
        "docs/productization.md",
        "macOS codesign verification argv plan",
    ),
    ("docs/productization.md", "macOS notarytool argv plan"),
    ("docs/productization.md", "macOS stapler argv plan"),
    ("docs/productization.md", "macOS spctl argv plan"),
    (
        "docs/productization.md",
        "signing runner acceptance criteria",
    ),
    ("docs/productization.md", "CI codesign execution runner"),
    ("docs/productization.md", "CI notarization execution runner"),
    ("docs/productization.md", "CI stapler execution runner"),
    ("docs/productization.md", "CI Gatekeeper assessment runner"),
    ("docs/productization.md", "CI signed DMG artifact check"),
    ("docs/productization.md", "CI signed checksum generation"),
    (
        "docs/productization.md",
        "private CI signed DMG artifact upload",
    ),
    (
        "docs/productization.md",
        "private CI signed checksum upload",
    ),
    ("docs/productization.md", "GitHub release command plan"),
    ("docs/productization.md", "signed DMG target preparation"),
    ("docs/productization.md", "signed DMG copy isolation"),
    ("docs/productization.md", "signed DMG artifact guard"),
    ("docs/signed-dmg-runbook.md", "## Short Execution Memo"),
    (
        "docs/signed-dmg-runbook.md",
        "scripts/manual-qa-distribution-handoff.sh",
    ),
    (
        "docs/signed-dmg-runbook.md",
        "productization-status --track \"Paid beta\"",
    ),
    ("docs/signed-dmg-runbook.md", "distribution quickstart 7"),
    (
        "docs/public-beta-operator-checklist.md",
        "## Short Execution Memo",
    ),
    (
        "docs/public-beta-operator-checklist.md",
        "Published checksum",
    ),
    (
        "docs/public-beta-operator-checklist.md",
        "release notes Artifact URL",
    ),
    (
        "docs/public-beta-operator-checklist.md",
        "lowercase SHA-256 line",
    ),
    (
        "docs/public-beta-operator-checklist.md",
        "versioned `DropSquash.dmg`",
    ),
    (
        "docs/public-beta-operator-checklist.md",
        "`brew uninstall --cask`",
    ),
    (
        "docs/public-beta-operator-checklist.md",
        "Homebrew tap PR URL",
    ),
    (
        "docs/public-beta-operator-checklist.md",
        "GitHub Release URL",
    ),
];
