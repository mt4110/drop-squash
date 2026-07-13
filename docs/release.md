# Release

The first public beta target is a signed and notarized macOS build. Signed
Windows and Linux builds are planned after their native backends and desktop
MVPs are implemented and verified.

- macOS: signed and notarized DMG.
- Windows: signed installer.
- Linux: Flatpak with a fixed runtime and media-plugin allowlist.

Unsigned payload creation is kept separate from signing and timestamping. CI must test each native backend on its own operating system and reject release artifacts containing `/nix/store` references.

No signing secrets belong in the repository.
Media conversion remains local-first: do not add `ffmpeg`, `ffprobe`, shell
execution, or `PATH` lookup to the media path, do not upload media files to a
cloud service, and do not enable telemetry by default.
The public paid beta remains blocked until every item in
`docs/release-blockers.md` has concrete evidence in the named location.

The desktop bundle configuration produces both the macOS `.app` and `.dmg`
artifacts. Use unsigned local builds only for QA; public release artifacts must
be signed, notarized, stapled, checked, and checksummed before publication.
The tag release workflow builds an unsigned macOS DMG, checks it, uploads the
unsigned DMG as a QA artifact, writes and uploads `SHA256SUMS`, then blocks
publication until signed packaging exists.
The macOS job maps signing and notarization secrets into `macos-signing-check`
so missing CI credentials fail deterministically before signed packaging is
enabled.

Run the local readiness gate before preparing any release artifact:

Use the Nix development shell for local release builds when the host Node or
pnpm version differs from `apps/desktop/package.json`; for example:

```sh
nix develop --command pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci
```

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
cargo run -p xtask -- normalize-dmg target/release/bundle/dmg
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
It also refuses to print an `App build` value when the selected app artifact is
older than `HEAD`; rebuild the `.app` or `.dmg` before recording manual QA in
that case.
Any custom `--app-state-dir`, `--state-dir`, or `--output-dir` must stay
outside the repository so private app state and generated QA media cannot be
committed or deleted by accident.
If the QA run used a reset trial state, restore the backed up local state after
recording evidence:

```sh
cargo run -p xtask -- manual-qa-prepare --restore-state
```

The unsigned Tauri build is only a packaging and QA input. Normalize the
generated Tauri DMG to `DropSquash.dmg` before artifact checks, checksums,
manual QA evidence, or public release notes refer to it. Public release still
requires signing, notarization, stapling, artifact checks, checksums, and
Gatekeeper no-warning evidence for the signed app.

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

Before live checkout is enabled, publish the final refund policy and update the
`Refund policy finalized` row in `docs/release-blockers.md` with the production
`/refund` URL. Do not treat the draft refund page as paid-beta evidence.

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
a placeholder such as `base64`. The App Store Connect key id must be a
10-character `APPLE_API_KEY`, and `APPLE_API_ISSUER` must be the issuer UUID,
not placeholder text or an all-zero value.
`APPLE_API_KEY_PATH` must point to a non-empty `.p8` file containing App Store
Connect private key data, not an empty placeholder file.
Signing and notarization secret values such as `APPLE_CERTIFICATE_PASSWORD` and
`APPLE_PASSWORD` must be real secrets. A placeholder secret such as `password`
or `app-password` is rejected.

After packaged-app testing, require the manual QA record to be complete:

```sh
cargo run -p xtask -- manual-qa-check
```

Before a paid beta, complete the license sandbox rows in `docs/manual-qa.md`.
They cover both cache safety and UI action state. Empty-key evidence must show
Activate is disabled and that no raw key, fingerprint, or instance was written.
Invalid-key evidence must show submit is disabled while Activating, the error is
friendly, and no raw key, fingerprint, or instance was written. Valid activation
evidence must name the Lemon Squeezy sandbox activation request and show submit
is disabled while Activating, and local-forget evidence must show the action is
disabled while Forgetting. Expired license refresh evidence must show the
expired offline grace cache, reconnect prompt, conversion blocked before
starting, and raw-key absence in the local cache.
Do not record sandbox keys or private store IDs in the manual QA table.

- Lemon Squeezy product setup
- Lemon Squeezy sandbox purchase
- Valid sandbox activation
- Empty key activation
- Invalid key activation
- License network failure
- Expired license refresh
- Local license forget

Record these rows:

- Sandbox product setup
- Sandbox purchase
- Empty key activation
- Invalid key activation
- Valid sandbox activation
- License network failure
- Expired license refresh
- Forget license on this Mac

Keep the matching release blockers in `docs/release-blockers.md` blocked until
the sandbox purchase, activation, empty-key, invalid-key, network-failure,
expired-refresh, and local-forget evidence exist.

Generate SHA-256 checksum lines for release artifacts with:

```sh
cargo run -p xtask -- checksum path/to/DropSquash.dmg --output SHA256SUMS
```

The generated `SHA256SUMS` line uses the artifact file name, not the local
build directory path. The `--output` target must be named `SHA256SUMS`, and
the command refuses to overwrite an existing file.
It can be attached directly to the public release. Keep the release notes and
Homebrew cask checksum values as lowercase SHA-256 hex so later publish checks
compare them to the same digest string generated from the DMG.
Generate the release notes artifact fields from the same DMG and public
artifact URL to avoid hand-copying the version, checksum, git commit,
Homebrew cask command inputs, and Homebrew tap PR evidence draft:

```sh
cargo run -p xtask -- release-notes-prepare path/to/DropSquash.dmg https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg
```

Pass `--markdown-output /tmp/dropsquash-release-notes-prepared.md` to write
the prepared fields to a new absolute `.md` path outside the repository; the
command refuses to overwrite an existing file.
The prepared fields also include the matching `SHA256SUMS` output command for
the checked DMG, so the checksum file can be regenerated without changing
artifact paths by hand.
The generated checksum line is a pre-upload aid and the generated
`GitHub Release checksum` field is intentionally marked pending. Replace it
with concrete public evidence only after the matching `SHA256SUMS` file is
attached to the GitHub Release. `release-notes-prepare` rejects artifacts older
than `HEAD`; rebuild the DMG after any final release commit before generating
public release notes.
Prepared draft placeholders, including pending upload notes and Homebrew
evidence drafts, are not public release evidence. Replace every prepared draft
line with observed public evidence before running `publish-check`;
`release-notes-check` and `publish-check` reject those draft placeholders.

Reject release artifacts that accidentally capture development-only Nix store
references:

```sh
cargo run -p xtask -- artifact-check path/to/DropSquash.dmg
```

Generate the Homebrew cask after the public `DropSquash.dmg` release URL and
checksum are known:

```sh
cargo run -p xtask -- homebrew-cask 0.1.0 \
  "$DROPSQUASH_DMG_URL" \
  "$DROPSQUASH_SHA256" \
  https://github.com/mt4110/drop-squash > packaging/homebrew/Casks/dropsquash.rb
```

After notarization succeeds, publish the checksum with the GitHub Release and
fill `docs/release-notes-template.md` with codesign, spctl, stapler, notary,
checksum, Gatekeeper, and Homebrew evidence. The public release notes must name
`DropSquash.dmg` in the signing, notarization, checksum, Gatekeeper, and
Homebrew evidence; Gatekeeper evidence must mention signed, notarized, stapled,
public `DropSquash.dmg`, and no warning;
benchmark evidence must name the Benchmark sample set, same-machine release candidate baseline, and 20% regression threshold result;
Homebrew evidence must mention the Homebrew tap PR URL, artifact URL, matching
lowercase SHA-256 digest, `auto_updates false`, and `zap` cleanup:

```sh
cargo run -p xtask -- release-notes-check path/to/release-notes.md
```
Run `cargo run -p xtask -- publish-check path/to/release-notes.md` only after
`docs/manual-qa.md` records the tested public `DropSquash.dmg` and every row in
`docs/release-blockers.md` is `Verified` with concrete Completion evidence and
a traceable Evidence reference.
The publish check also requires the release notes `Git commit` field to exactly
match the current short or full `HEAD`, so rebuild and recheck the artifact
after any final commit.
The manual QA `App build` field must also name the current `HEAD`, so rerun
packaged-app QA whenever the release commit changes.
The release notes `SHA-256` field must match the `App artifact` recorded in
manual QA, so do not reuse checksums from a different DMG.
The release notes Benchmark sample set CSV path must also match an existing
manual QA Benchmark sample set CSV path, so benchmark evidence cannot drift
between private QA and the public release notes.
The publish check also requires release blocker URLs to match the release notes
URLs for the public website, refund policy, live checkout, GitHub Release, and
Homebrew tap PR. Public publish references must point to the same release notes
URLs, not to a different release or tap PR.
