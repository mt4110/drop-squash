# Release

Signed macOS, Windows, and Linux builds are planned after the native backend and desktop MVPs.

- macOS: signed and notarized DMG.
- Windows: signed installer.
- Linux: Flatpak with a fixed runtime and media-plugin allowlist.

Unsigned payload creation is kept separate from signing and timestamping. CI must test each native backend on its own operating system and reject release artifacts containing `/nix/store` references.

No signing secrets belong in the repository.

Run the local readiness gate before preparing any release artifact:

```sh
cargo run -p xtask -- file-size-check
cargo run -p xtask -- website-check
cargo run -p xtask -- release-check
```

These gates keep production files within the repository size rules, verify the
static sales site links and required pages, reject secret-like files, require
the external media process security gate, verify that production CSP does not
open remote network origins, keep the unsigned release workflow blocked, and
verify that the updater is not enabled before signing keys are ready.

Run the media policy gate directly after touching native backends or desktop
commands:

```sh
cargo run -p xtask -- media-policy-check
```

Run the privacy policy gate after touching the desktop app, website, or
privacy-sensitive code:

```sh
cargo run -p xtask -- privacy-policy-check
```

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

After packaged-app testing, require the manual QA record to be complete:

```sh
cargo run -p xtask -- manual-qa-check
```

Generate SHA-256 checksum lines for release artifacts with:

```sh
cargo run -p xtask -- checksum path/to/DropSquash.dmg > SHA256SUMS
```

Reject release artifacts that accidentally capture development-only Nix store
references:

```sh
cargo run -p xtask -- artifact-check path/to/DropSquash.dmg
```

Generate the Homebrew cask after the public release URL and checksum are known:

```sh
cargo run -p xtask -- homebrew-cask 0.1.0 \
  https://example.com/DropSquash.dmg \
  SHA256_HEX \
  https://example.com/dropsquash > packaging/homebrew/Casks/dropsquash.rb
```

Publish the checksum file with the GitHub Release after notarization succeeds.
