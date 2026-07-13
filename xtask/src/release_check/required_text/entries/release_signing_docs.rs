use super::Entry;

pub(super) const ENTRIES: &[Entry] = &[
    ("docs/release.md", "macos-signing-plan"),
    (
        "docs/release.md",
        "The plan does not execute signing commands and must not print secret values",
    ),
    ("docs/release.md", "macos-keychain-plan"),
    ("docs/release.md", "macos-keychain-cleanup-plan"),
    ("docs/release.md", "APPLE_KEYCHAIN_PASSWORD"),
    (
        "docs/release.md",
        "certificate decode/import, and key partition list argv without printing secret",
    ),
    (
        "docs/release.md",
        "security delete-keychain` and `rm -f` cleanup argv",
    ),
    ("docs/release.md", "signed-dmg-copy"),
    (
        "docs/release.md",
        "refuses existing targets and still checks the unsigned input",
    ),
    ("docs/release.md", "macos-codesign-plan"),
    (
        "docs/release.md",
        "does not execute `codesign` or import signing credentials",
    ),
    ("docs/release.md", "macos-codesign-verify-plan"),
    (
        "docs/release.md",
        "It validates the target artifact and does not execute codesign verification",
    ),
    ("docs/release.md", "macos-notary-plan"),
    (
        "docs/release.md",
        "prints environment variable references only, not notarization secret values",
    ),
    ("docs/release.md", "macos-stapler-plan"),
    (
        "docs/release.md",
        "It validates the target artifact and does not execute stapler",
    ),
    ("docs/release.md", "macos-spctl-plan"),
    (
        "docs/release.md",
        "It validates the target artifact and does not execute Gatekeeper assessment",
    ),
    ("docs/release.md", "signed-dmg-prepare"),
    (
        "docs/release.md",
        "refuses to overwrite an existing signed target",
    ),
    ("docs/release.md", "signed-dmg-check"),
    (
        "docs/release.md",
        "It does not replace `codesign`, notary, stapler, or Gatekeeper evidence",
    ),
    ("docs/release.md", "Block unsigned Phase 0 release"),
    (
        "docs/release.md",
        "runner executes the reviewed keychain, codesign, codesign verification",
    ),
    (
        "docs/release.md",
        "The runner must run temporary keychain cleanup even when",
    ),
    (
        "docs/release.md",
        "signing, notarization, or verification step fails",
    ),
    (
        "docs/release.md",
        "only the checked signed `DropSquash.dmg`",
    ),
    ("docs/release.md", "clearly named unsigned"),
];
