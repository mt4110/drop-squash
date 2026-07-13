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
The macOS job maps signing and notarization secrets into `macos-signing-check`,
writes the App Store Connect `.p8` key only into the runner temporary directory,
and fails deterministically before signed packaging is enabled when CI
credentials are missing.

Build public QA and release artifacts from a clean git worktree. If Tauri or
Git reports a dirty tree, either commit or intentionally remove the unrelated
local change, then rebuild the `.app` and `.dmg` before recording manual QA or
release evidence.

Use the Nix development shell for local release builds when the host Node or
pnpm version differs from `apps/desktop/package.json`; for example:

```sh
nix develop --command pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci
```

Use this order for local QA and release preparation:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p xtask -- file-size-check
cargo run -p xtask -- website-check
cargo run -p xtask -- release-check
pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci
cargo run -p xtask -- normalize-dmg target/release/bundle/dmg
cargo run -p xtask -- artifact-check target/release/bundle/dmg/DropSquash.dmg
cargo run -p xtask -- checksum target/release/bundle/dmg/DropSquash.dmg --output SHA256SUMS
cargo run -p xtask -- macos-signing-plan target/release/bundle/dmg/DropSquash.dmg /tmp/dropsquash-signed
cargo run -p xtask -- macos-keychain-plan /tmp/dropsquash-signed/keychain
cargo run -p xtask -- signed-dmg-prepare target/release/bundle/dmg/DropSquash.dmg /tmp/dropsquash-signed
cargo run -p xtask -- signed-dmg-copy target/release/bundle/dmg/DropSquash.dmg /tmp/dropsquash-signed
cargo run -p xtask -- macos-codesign-plan /tmp/dropsquash-signed/DropSquash.dmg "Developer ID Application: ..."
cargo run -p xtask -- macos-codesign-verify-plan /tmp/dropsquash-signed/DropSquash.dmg
cargo run -p xtask -- macos-notary-plan /tmp/dropsquash-signed/DropSquash.dmg --api-key
cargo run -p xtask -- macos-stapler-plan /tmp/dropsquash-signed/DropSquash.dmg
cargo run -p xtask -- macos-spctl-plan /tmp/dropsquash-signed/DropSquash.dmg
cargo run -p xtask -- signed-dmg-check /tmp/dropsquash-signed/DropSquash.dmg target/release/bundle/dmg/DropSquash.dmg
cargo run -p xtask -- macos-keychain-cleanup-plan /tmp/dropsquash-signed/keychain
cargo run -p xtask -- manual-qa-prepare --app-artifact target/release/bundle/dmg/DropSquash.dmg
cargo run -p xtask -- manual-qa-check
```

These gates keep production files within the repository size rules, verify the
static sales site links, local resources, approved external links, and required
pages, reject secret-like files, require manual QA evidence, require the
external media process security gate, verify that production CSP does not open
remote network origins, keep desktop capabilities minimal, keep the unsigned
release workflow blocked, and verify that the updater is not enabled before
signing keys are ready.

Tauri updater remains disabled for the first paid beta. Before enabling it,
embed only the updater public key in `tauri.conf.json`; never commit the updater private key. Store the private key in a password manager with a tested backup,
provide it to CI only through GitHub Secrets, and treat private-key loss as a
breaking update event because installed users cannot receive trusted updates
from a replacement key.
Do not commit `latest.json` or update manifest files while the updater is
disabled.

The `manual-qa-prepare` step preserves local DropSquash app state and creates a
dedicated output folder before packaged-app QA evidence is recorded.
It fails when the selected app artifact is older than `HEAD`; rebuild the
`.app` or `.dmg` before recording manual QA in that case.
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
Before implementing the command runner, use `macos-signing-plan` to keep the
macOS signing wrapper order deterministic: prepare the signed target, copy the
unsigned DMG to that target, prepare the temporary signing keychain, apply the
Developer ID `codesign` signature, verify the signed target, submit with
`notarytool`, validate stapling, assess Gatekeeper, run `signed-dmg-check`, then
plan temporary signing keychain cleanup.
The plan does not execute signing commands and must not print secret values.
Use `signed-dmg-copy` only to create the isolated signing target before
`codesign`; it refuses existing targets and still checks the unsigned input.
Use `macos-codesign-plan` to generate the exact `codesign --force --options runtime --timestamp --sign` argv for the checked signing target and a Developer ID Application identity. It validates the target artifact and identity but does not execute `codesign` or import signing credentials.
Use `macos-keychain-cleanup-plan` to generate `security delete-keychain` and `rm -f` cleanup argv for the temporary signing keychain and decoded certificate. It validates the work directory and does not print signing secret values.
Use `macos-codesign-verify-plan` to generate the `codesign --verify --deep --strict --verbose=4` and `codesign -dv --verbose=4` argv for the checked signing target. It validates the target artifact and does not execute codesign verification.
Use `macos-notary-plan` to generate the `xcrun notarytool submit --wait` argv for either the App Store Connect API key path or the Apple ID credential path. It validates the target artifact and prints environment variable references only, not notarization secret values.
Use `macos-stapler-plan` to generate the `xcrun stapler staple` and `xcrun stapler validate` argv for the checked signing target. It validates the target artifact and does not execute stapler.
Use `macos-spctl-plan` to generate the `spctl --assess --type open --verbose=4` argv for the checked signing target. It validates the target artifact and does not execute Gatekeeper assessment.
Before the signing implementation writes a public artifact, run
`signed-dmg-prepare` against the checked unsigned `DropSquash.dmg` and a separate
output directory. The command rejects non-canonical or `/nix/store`-tainted
inputs, refuses to overwrite an existing signed target, and refuses to use the
unsigned artifact directory as the signed output directory.
After the signing implementation writes the candidate, run `signed-dmg-check`
against the signed candidate and the unsigned input. This artifact guard rejects
non-canonical or `/nix/store`-tainted signed candidates, rejects the unsigned
input path, and rejects byte-identical output. It does not replace `codesign`, notary, stapler, or Gatekeeper evidence.

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

Before live checkout is enabled, publish the final pricing page and refund
policy. Update the `Pricing finalized` row in `docs/release-blockers.md` with
the production `/pricing` URL, fill the release notes Pricing URL field with
the same URL, and update the `Refund policy finalized` row with the production
`/refund` URL. Do not treat draft pricing or the draft refund page as paid-beta
evidence.

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
`APPLE_CERTIFICATE_PASSWORD`, plus `APPLE_KEYCHAIN_PASSWORD` for the temporary
keychain import; a local keychain identity name is not enough for a fresh
runner. Use `macos-keychain-plan` to generate the `security create-keychain`,
certificate decode/import, and key partition list argv without printing secret
values. Pair it with `macos-keychain-cleanup-plan` so the temporary keychain and
decoded certificate cleanup argv are reviewed before signing execution is added.
GitHub Actions should store the App Store Connect private key as
`APPLE_API_KEY_P8`, write it to `$RUNNER_TEMP`, and export the generated
`APPLE_API_KEY_PATH`; do not store `APPLE_API_KEY_PATH` as a repository secret.
The certificate value must be base64-encoded certificate data, not a placeholder
such as `base64`. The App Store Connect key id must be a
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
evidence must name the Lemon Squeezy sandbox activation request, show submit is
disabled while Activating, and include checked local-cache fingerprint,
`instance_id`, and raw-key absence evidence. Local-forget evidence must show the
action is disabled while Forgetting. Expired license refresh evidence must show
the expired offline grace cache, reconnect prompt, conversion blocked before
starting, checked local cache, and raw-key absence in the local cache.
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
The command rejects development-only `/nix/store` references and can be attached
directly to the public release. Keep the release notes and Homebrew cask
checksum values as lowercase SHA-256 hex so later publish checks compare them
to the same digest string generated from the DMG.
Generate the release notes artifact fields from the same DMG and public
artifact URL to avoid hand-copying the version, checksum, git commit,
macOS verification commands, signing evidence drafts,
Homebrew cask command inputs, Homebrew tap PR evidence draft, and Homebrew
install result evidence draft:

```sh
cargo run -p xtask -- release-notes-prepare path/to/DropSquash.dmg https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg
```

Pass `--markdown-output /tmp/dropsquash-release-notes-prepared.md` to write
the prepared fields to a new absolute `.md` path outside the repository; the
command refuses to overwrite an existing file.
The prepared fields also include the matching `SHA256SUMS` output command for
the checked DMG, so the checksum file can be regenerated without changing
artifact paths by hand.
The macOS verification command drafts also use the same checked DMG path, so
codesign, `spctl`, and stapler evidence are recorded against the same artifact.
The generated checksum line is a pre-upload aid and the generated
`GitHub Release checksum` field is intentionally marked pending. Replace it
with concrete public evidence only after the matching `SHA256SUMS` file is
attached to the GitHub Release. `release-notes-prepare` rejects artifacts older
than `HEAD` and refuses to run from a dirty git worktree; rebuild the DMG after
any final release commit before generating public release notes.
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
the exact Artifact URL in the signing, notarization, checksum, Gatekeeper, and
Homebrew evidence; Gatekeeper evidence must mention signed, notarized, stapled,
the public `DropSquash.dmg` matching the Artifact URL, and no warning;
benchmark evidence must name the Benchmark sample set, same-machine release candidate baseline, and 20% regression threshold result;
Homebrew evidence must mention the Homebrew tap PR URL, artifact URL, matching
lowercase SHA-256 digest, `brew install --cask`, `brew uninstall --cask`,
`auto_updates false`, and `zap` cleanup:

```sh
cargo run -p xtask -- release-notes-check path/to/release-notes.md
```
Run `cargo run -p xtask -- publish-check path/to/release-notes.md` only after
`docs/manual-qa.md` records the tested public `DropSquash.dmg` matching the release notes Artifact URL
and every row in `docs/release-blockers.md` is `Verified` with concrete Completion evidence
and a traceable Evidence reference.
Before preparing public release notes, update `CHANGELOG.md` and bump the
`apps/desktop/src-tauri/tauri.conf.json` version for the release candidate.
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
URLs for the public website, pricing page, refund policy, live checkout, GitHub
Release, and Homebrew tap PR. Public publish references must point to the same
release notes URLs, not to a different release or tap PR.
The release notes must include the required URL fields before publish:
Artifact URL, Public website URL, Pricing URL, Refund policy URL,
Live checkout URL, GitHub Release URL, and Homebrew tap PR URL.
The Public web proof must show the production `dropsquash.app` release-status
pricing, and refund URLs serving release-status, privacy, pricing, support,
download, checkout, and refund links before the public beta.
The Signing and distribution proof must show the same public `DropSquash.dmg`
from the release notes Artifact URL is signed, notarized, stapled, checksummed,
installable through the Homebrew tap PR, and opens without warning.
