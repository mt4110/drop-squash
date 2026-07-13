use super::Entry;

pub(super) const ENTRIES: &[Entry] = &[
    ("docs/qa-evidence.md", "publish-check"),
    (
        "docs/qa-evidence.md",
        "release blocker URLs matching the release notes URLs",
    ),
    (
        "docs/qa-evidence.md",
        "public publish references without query or fragment parts that point to the release notes URLs",
    ),
    (
        "docs/qa-evidence.md",
        "exact `GitHub Release https://...` and `Homebrew tap PR https://...`",
    ),
    (
        "docs/qa-evidence.md",
        "complete manual QA evidence for a UDIF public `DropSquash.dmg` matching the release notes Artifact URL",
    ),
    ("docs/qa-evidence.md", "`App build` matching current `HEAD`"),
    (
        "docs/qa-evidence.md",
        "release notes `SHA-256` matching the manual QA `App artifact`",
    ),
    (
        "docs/qa-evidence.md",
        "release notes canonical benchmark CSV path matching manual QA",
    ),
    (
        "docs/qa-evidence.md",
        "filled release notes evidence without prepared draft placeholders",
    ),
    ("docs/qa-evidence.md", "staying outside the repository"),
    ("docs/qa-evidence.md", "concrete Completion evidence"),
    ("docs/qa-evidence.md", "traceable Evidence reference"),
    (
        "docs/qa-evidence.md",
        "public URL release blocker references",
    ),
    (
        "docs/qa-evidence.md",
        "host-boundary checked public publish references without query or fragment parts",
    ),
    (
        "docs/qa-evidence.md",
        "distribution completion without the release notes Artifact URL",
    ),
    (
        "docs/qa-evidence.md",
        "artifact-check/checksum/signing/Gatekeeper path evidence",
    ),
    (
        "docs/qa-evidence.md",
        "concrete Completion evidence that names the release notes Artifact URL",
    ),
];
