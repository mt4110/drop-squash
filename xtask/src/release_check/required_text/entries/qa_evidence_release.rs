use super::Entry;

pub(super) const ENTRIES: &[Entry] = &[
    ("docs/qa-evidence.md", "publish-check"),
    (
        "docs/qa-evidence.md",
        "release blocker URLs matching the release notes URLs",
    ),
    (
        "docs/qa-evidence.md",
        "public publish references that point to the release notes URLs",
    ),
    (
        "docs/qa-evidence.md",
        "complete manual QA evidence for a UDIF public `DropSquash.dmg`",
    ),
    ("docs/qa-evidence.md", "`App build` matching current `HEAD`"),
    (
        "docs/qa-evidence.md",
        "release notes `SHA-256` matching the manual QA `App artifact`",
    ),
    (
        "docs/qa-evidence.md",
        "release notes benchmark CSV path matching manual QA",
    ),
    ("docs/qa-evidence.md", "concrete Completion evidence"),
    ("docs/qa-evidence.md", "traceable Evidence reference"),
    (
        "docs/qa-evidence.md",
        "public URL release blocker references",
    ),
    ("docs/qa-evidence.md", "release-notes-check"),
    ("docs/qa-evidence.md", "release-notes-prepare"),
    ("docs/qa-evidence.md", "GitHub Release URL"),
    ("docs/qa-evidence.md", "SHA256SUMS line"),
    ("docs/qa-evidence.md", "SHA256SUMS output command"),
    ("docs/qa-evidence.md", "pending checksum upload placeholder"),
    (
        "docs/qa-evidence.md",
        "stale artifacts older than `HEAD` are rejected",
    ),
    ("docs/qa-evidence.md", "matching Homebrew cask command"),
    ("docs/qa-evidence.md", "Homebrew tap PR evidence draft"),
    (
        "docs/qa-evidence.md",
        "new absolute `.md` path outside the repository",
    ),
    ("docs/qa-evidence.md", "version-matched public Artifact URL"),
    (
        "docs/qa-evidence.md",
        "version-matched public Artifact URL with host/path boundaries",
    ),
    ("docs/qa-evidence.md", "imposter public URL hosts"),
    (
        "docs/qa-evidence.md",
        "host-boundary checked public publish references",
    ),
    ("docs/qa-evidence.md", "imposter GitHub release hosts"),
    (
        "docs/qa-evidence.md",
        "checksum evidence without the Artifact URL",
    ),
    (
        "docs/qa-evidence.md",
        "Homebrew install evidence without the SHA-256 digest",
    ),
    ("docs/qa-evidence.md", "non-lowercase Git commits"),
    (
        "docs/qa-evidence.md",
        "Gatekeeper evidence without the public `DropSquash.dmg`",
    ),
    (
        "docs/qa-evidence.md",
        "Gatekeeper completion without the public `DropSquash.dmg`",
    ),
];
