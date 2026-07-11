# Release

Signed macOS, Windows, and Linux builds are planned after the native backend and desktop MVPs.

- macOS: signed and notarized DMG.
- Windows: signed installer.
- Linux: Flatpak with a fixed runtime and media-plugin allowlist.

Unsigned payload creation is kept separate from signing and timestamping. CI must test each native backend on its own operating system and reject release artifacts containing `/nix/store` references.

No signing secrets belong in the repository.
The public paid beta remains blocked until every item in
`docs/release-blockers.md` has concrete evidence in the named location.

The desktop bundle configuration produces both the macOS `.app` and `.dmg`
artifacts. Use unsigned local builds only for QA; public release artifacts must
be signed, notarized, stapled, checked, and checksummed before publication.
The tag release workflow also builds an unsigned macOS DMG, runs the artifact
check, writes `SHA256SUMS`, and uploads that checksum file before blocking
publication until signed release packaging is implemented. The macOS job maps
signing and notarization secrets
into `macos-signing-check` so missing CI credentials fail deterministically
before signed packaging is enabled.

Run the local readiness gate before preparing any release artifact:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p xtask -- file-size-check
cargo run -p xtask -- website-check
cargo run -p xtask -- manual-qa-check
cargo run -p xtask -- release-check
pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci
```

These gates keep production files within the repository size rules, verify the
static sales site links and required pages, reject secret-like files, require
manual QA evidence, require the external media process security gate, verify
that production CSP does not open remote network origins, keep desktop
capabilities minimal, keep the unsigned release workflow blocked, and verify
that the updater is not enabled before signing keys are ready.
The unsigned Tauri build is only a packaging and QA input; public release still
requires signing, notarization, stapling, artifact checks, and checksums.

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
values. In GitHub Actions, signing requires `APPLE_CERTIFICATE` and
`APPLE_CERTIFICATE_PASSWORD`; a local keychain identity name is not enough for a
fresh runner. The certificate value must be base64-encoded certificate data, not
a placeholder such as `base64`.

After packaged-app testing, require the manual QA record to be complete:

```sh
cargo run -p xtask -- manual-qa-check
```

Before a paid beta, complete the Lemon Squeezy sandbox activation row in
`docs/manual-qa.md` and keep the matching release blocker in
`docs/release-blockers.md` blocked until the sandbox purchase and activation
evidence exists.

Generate SHA-256 checksum lines for release artifacts with:

```sh
cargo run -p xtask -- checksum path/to/DropSquash.dmg > SHA256SUMS
```

The generated `SHA256SUMS` line uses the artifact file name, not the local
build directory path, so it can be attached directly to the public release.

Reject release artifacts that accidentally capture development-only Nix store
references:

```sh
cargo run -p xtask -- artifact-check path/to/DropSquash.dmg
```

Generate the Homebrew cask after the public release URL and checksum are known:

```sh
cargo run -p xtask -- homebrew-cask 0.1.0 \
  "$DROPSQUASH_DMG_URL" \
  "$DROPSQUASH_SHA256" \
  https://github.com/mt4110/drop-squash > packaging/homebrew/Casks/dropsquash.rb
```

Publish the checksum file with the GitHub Release after notarization succeeds.
Use `docs/release-notes-template.md` for the public release notes so the
codesign, spctl, stapler, notary, checksum, Gatekeeper, and Homebrew evidence is
recorded in one place.
