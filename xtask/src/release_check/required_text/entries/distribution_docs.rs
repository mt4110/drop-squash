use super::Entry;

mod license;

pub(super) const LICENSE: &[Entry] = license::ENTRIES;

pub(super) const CORE: &[Entry] = &[
    ("docs/release.md", "release blocker URLs to match"),
    ("docs/release-blockers.md", "GitHub Release https://..."),
    ("docs/release-blockers.md", "Homebrew tap PR https://..."),
    ("docs/release-blockers.md", "dropsquash.app"),
    (
        "docs/release-blockers.md",
        "Public website, pricing, and refund references",
    ),
    (
        "docs/release-blockers.md",
        "store.lemonsqueezy.com/checkout/buy/<id>",
    ),
    (
        "docs/release-blockers.md",
        "must not include query or fragment parts",
    ),
    ("docs/release-blockers.md", "## Execution Order"),
    ("docs/release-blockers.md", "Local packaged-app proof"),
    ("docs/release-blockers.md", "License sandbox proof"),
    ("docs/release-blockers.md", "Public web proof"),
    ("docs/release-blockers.md", "Signing and distribution proof"),
    (
        "docs/release-blockers.md",
        "docs/license-sandbox-runbook.md",
    ),
    ("docs/release-blockers.md", "docs/signed-dmg-runbook.md"),
    (
        "docs/release-blockers.md",
        "docs/public-beta-operator-checklist.md",
    ),
    ("docs/release-blockers.md", "Short Execution Memo"),
    ("docs/release-blockers.md", "public-web-ready"),
    ("docs/release-blockers.md", "manual-qa-packaged-rerun"),
    (
        "docs/release-blockers.md",
        "manual-qa-pending docs/manual-qa.md --section license",
    ),
    (
        "docs/release-blockers.md",
        "disk-image notice, license field",
    ),
    ("docs/release-blockers.md", "benchmark --release-set"),
    ("docs/release-blockers.md", "benchmark-csv-check"),
    (
        "docs/release-blockers.md",
        "matching the release notes Artifact URL",
    ),
    ("docs/release-blockers.md", "brew uninstall --cask"),
    ("docs/release-notes-template.md", "brew uninstall --cask"),
    ("docs/release.md", "brew uninstall --cask"),
    (
        "docs/release.md",
        "Gatekeeper evidence must mention signed, notarized, stapled",
    ),
    (
        "docs/release.md",
        "the public `DropSquash.dmg` matching the Artifact URL, and no warning",
    ),
    (
        "docs/manual-qa.md",
        "and Gatekeeper result rows must name the",
    ),
    ("docs/manual-qa.md", "notary, stapler, `spctl` assessment"),
    ("docs/manual-qa.md", "notary/stapler/`spctl`"),
    (
        "docs/manual-qa.md",
        "exact Artifact URL for the same public `DropSquash.dmg`",
    ),
    ("docs/manual-qa.md", "64-character lowercase SHA-256 digest"),
    (
        "docs/manual-qa.md",
        "app from public `DropSquash.dmg` matching the release notes Artifact URL opens cleanly",
    ),
    ("docs/release.md", "10-character `APPLE_API_KEY`"),
    (
        "docs/release.md",
        "In GitHub Actions, signing requires `APPLE_CERTIFICATE`",
    ),
    ("docs/release.md", "APPLE_API_KEY_P8"),
    (
        "docs/release.md",
        "do not store `APPLE_API_KEY_PATH` as a repository secret",
    ),
    (
        "docs/release.md",
        "local keychain identity name is not enough",
    ),
    ("docs/release.md", "APPLE_API_ISSUER"),
    ("docs/release.md", "all-zero value"),
    ("docs/release.md", "placeholder secret"),
    (
        "docs/release.md",
        "The command rejects development-only `/nix/store` references",
    ),
    (
        "docs/reproducible-builds.md",
        "Both commands reject development-only `/nix/store` references",
    ),
];
