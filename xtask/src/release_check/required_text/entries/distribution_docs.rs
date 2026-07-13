use super::Entry;

pub(super) const ENTRIES: &[Entry] = &[
    ("docs/licensing.md", "Lemon Squeezy sandbox purchase"),
    (
        "docs/licensing.md",
        "64-character lowercase hex license fingerprint",
    ),
    (
        "docs/licensing.md",
        "available during Trial and Locked states",
    ),
    ("docs/release.md", "Empty key activation"),
    ("docs/release.md", "Activate is disabled"),
    (
        "docs/release.md",
        "no raw key, fingerprint, or instance was written",
    ),
    ("docs/release.md", "Invalid-key evidence"),
    ("docs/release.md", "friendly"),
    ("docs/release.md", "disabled while Activating"),
    ("docs/release.md", "disabled while Forgetting"),
    ("docs/release.md", "private store IDs"),
    ("docs/release.md", "License network failure"),
    ("docs/release.md", "Expired license refresh"),
    ("docs/release.md", "expired offline grace cache"),
    ("docs/release.md", "conversion blocked before"),
    (
        "docs/release.md",
        "Lemon Squeezy sandbox activation request",
    ),
    (
        "docs/manual-qa.md",
        "Lemon Squeezy sandbox activation request",
    ),
    ("docs/release.md", "release blocker URLs to match"),
    ("docs/release-blockers.md", "GitHub Release https://..."),
    ("docs/release-blockers.md", "Homebrew tap PR https://..."),
    ("docs/release-blockers.md", "## Execution Order"),
    ("docs/release-blockers.md", "Local packaged-app proof"),
    ("docs/release-blockers.md", "License sandbox proof"),
    ("docs/release-blockers.md", "Public web proof"),
    (
        "docs/release-blockers.md",
        "Signing and distribution proof",
    ),
    (
        "docs/release-blockers.md",
        "same public `DropSquash.dmg`",
    ),
    (
        "docs/release.md",
        "Gatekeeper evidence must mention signed, notarized, stapled",
    ),
    ("docs/release.md", "public `DropSquash.dmg`, and no warning"),
    (
        "docs/manual-qa.md",
        "and Gatekeeper result rows must name the",
    ),
    ("docs/manual-qa.md", "notary, stapler, `spctl` assessment"),
    ("docs/manual-qa.md", "notary/stapler/`spctl`"),
    ("docs/manual-qa.md", "same public `DropSquash.dmg` file"),
    ("docs/manual-qa.md", "64-character lowercase SHA-256 digest"),
    (
        "docs/manual-qa.md",
        "app from public `DropSquash.dmg` opens cleanly",
    ),
    ("docs/manual-qa.md", "license cache has no raw key"),
    (
        "docs/manual-qa.md",
        "existing valid cache with 64-character lowercase hex fingerprint and `instance_id` fields remains intact and has no raw key",
    ),
    (
        "docs/manual-qa.md",
        "Forgetting state disables action; license cache removed",
    ),
    ("docs/release.md", "10-character `APPLE_API_KEY`"),
    ("docs/release.md", "APPLE_API_ISSUER"),
    ("docs/release.md", "all-zero value"),
    ("docs/release.md", "placeholder secret"),
];
