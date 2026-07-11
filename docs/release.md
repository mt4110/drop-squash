# Release

The first public beta target is a signed and notarized macOS build. Signed
Windows and Linux builds are planned after their native backends and desktop
MVPs are implemented and verified.

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
The tag release workflow builds an unsigned macOS DMG, checks it, writes
`SHA256SUMS`, uploads it, then blocks publication until signed packaging exists.
The macOS job maps signing and notarization secrets into `macos-signing-check`
so missing CI credentials fail deterministically before signed packaging is
enabled.

Run the local readiness gate before preparing any release artifact:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p xtask -- file-size-check
cargo run -p xtask -- website-check
cargo run -p xtask -- manual-qa-prepare
cargo run -p xtask -- manual-qa-check
cargo run -p xtask -- release-check
pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci
```

These gates keep production files within the repository size rules, verify the
static sales site links, local resources, approved external links, and required
pages, reject secret-like files, require manual QA evidence, require the
external media process security gate, verify that production CSP does not open
remote network origins, keep desktop capabilities minimal, keep the unsigned
release workflow blocked, and verify that the updater is not enabled before
signing keys are ready.
The `manual-qa-prepare` step preserves local DropSquash app state and creates a
dedicated output folder before packaged-app QA evidence is recorded.
If the QA run used a reset trial state, restore the backed up local state after
recording evidence:

```sh
cargo run -p xtask -- manual-qa-prepare --restore-state
```

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

Before a paid beta, complete the license sandbox rows in `docs/manual-qa.md`.
They cover:

- Lemon Squeezy sandbox purchase
- Lemon Squeezy sandbox activation

Record these rows:

- Sandbox purchase
- Empty key activation
- Invalid key activation
- Valid sandbox activation
- Forget license on this Mac

Keep the matching release blockers in `docs/release-blockers.md` blocked until
the sandbox purchase, activation, invalid-key, and local-forget evidence exist.

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

After notarization succeeds, publish the checksum with the GitHub Release and
fill `docs/release-notes-template.md` with codesign, spctl, stapler, notary,
checksum, Gatekeeper, and Homebrew evidence:

```sh
cargo run -p xtask -- release-notes-check path/to/release-notes.md
```
Run `cargo run -p xtask -- publish-check path/to/release-notes.md` only after every
row in `docs/release-blockers.md` is `Verified` with concrete evidence.
