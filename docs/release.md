# Release

Signed macOS, Windows, and Linux builds are planned after the native backend and desktop MVPs.

- macOS: signed and notarized DMG.
- Windows: signed installer.
- Linux: Flatpak with a fixed runtime and media-plugin allowlist.

Unsigned payload creation is kept separate from signing and timestamping. CI must test each native backend on its own operating system and reject release artifacts containing `/nix/store` references.

No signing secrets belong in the repository.

Run the local readiness gate before preparing any release artifact:

```sh
cargo run -p xtask -- release-check
```

This gate rejects secret-like files, keeps the unsigned release workflow blocked,
and verifies that the updater is not enabled before signing keys are ready.

Check the local macOS signing and notarization environment before a signed build:

```sh
cargo run -p xtask -- macos-signing-check
```

The preflight accepts either `APPLE_SIGNING_IDENTITY` or
`APPLE_CERTIFICATE` with `APPLE_CERTIFICATE_PASSWORD` for signing. For
notarization it accepts either the App Store Connect API variables
`APPLE_API_KEY`, `APPLE_API_ISSUER`, and `APPLE_API_KEY_PATH`, or the Apple ID
variables `APPLE_ID`, `APPLE_PASSWORD`, and `APPLE_TEAM_ID`. Do not commit these
values.

Generate SHA-256 checksum lines for release artifacts with:

```sh
cargo run -p xtask -- checksum path/to/DropSquash.dmg > SHA256SUMS
```

Publish the checksum file with the GitHub Release after notarization succeeds.
