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
After the narrower private/manual paid beta technical proof is done, use
[docs/public-beta-operator-checklist.md](/Users/masakitakemura/_workspace/drop-squash/docs/public-beta-operator-checklist.md)
to finish the deferred production-onboarding proof before enabling public
checkout or a public paid beta.

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
For local macOS proof, that preflight currently requires either
`APPLE_SIGNING_IDENTITY` or `APPLE_CERTIFICATE` with
`APPLE_CERTIFICATE_PASSWORD`, plus either
`APPLE_API_KEY` / `APPLE_API_ISSUER` / `APPLE_API_KEY_PATH` or
`APPLE_ID` / `APPLE_PASSWORD` / `APPLE_TEAM_ID` for notarization.
It now runs `Import macOS signing certificate`, `Sign macOS DMG`,
`Verify macOS codesign`, `Notarize macOS DMG`, `Staple macOS DMG`,
`Assess macOS Gatekeeper`, `Check signed DMG artifact`,
`Write signed DMG checksum`, `Upload signed DMG artifact`,
`Upload signed DMG checksum`, and `Cleanup macOS signing keychain` against the
isolated signing target with `APPLE_CODESIGN_IDENTITY`. The signed uploads are
private CI artifacts for review; public GitHub Release publication remains
blocked until release notes, manual QA evidence, and distribution evidence are
complete.

Build public QA and release artifacts from a clean git worktree. If Tauri or
Git reports a dirty tree, either commit or intentionally remove the unrelated
local change, then rebuild the `.app` and `.dmg` before recording manual QA or
release evidence.
If the QA candidate currently lives only in a dirty tree and includes untracked
files, snapshot that exact tree into a detached QA worktree before rebuilding:

```sh
scripts/manual-qa-snapshot-worktree.sh
```

For the shortest snapshot handoff into the signing/notarization pass, use:

```sh
scripts/manual-qa-distribution-handoff.sh /tmp/dropsquash-qa-snapshot-$(git rev-parse --short HEAD)
```

That helper prints the snapshot path plus the next
`productization-status --track "Paid beta"`, desktop/web install,
`file-size-check`, `release-check`, unsigned build,
`manual-qa-prepare --reset-trial`, `macos-signing-check`,
`manual-qa-distribution-rerun`, and `paid-beta-check` commands for that exact
snapshot.

Run the QA build and `manual-qa-prepare` commands from that clean snapshot
worktree so the App build, DMG, and manual QA draft all point at the same exact
source state.

Use the Nix development shell for local QA and release builds so Node, pnpm,
Rust, and desktop build inputs match the pinned development environment:

```sh
nix develop --command pnpm --dir apps/desktop install --frozen-lockfile
nix develop --command pnpm --dir apps/desktop/web install --frozen-lockfile
nix develop --command pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci
```

For the signed macOS distribution pass, use
`docs/signed-dmg-runbook.md` as the shorter operator checklist. Keep
`docs/release.md` as the broader policy and gate reference.

Use this order for local QA and release preparation:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p xtask -- file-size-check
cargo run -p xtask -- public-web-ready
cargo run -p xtask -- release-check
nix develop --command pnpm --dir apps/desktop install --frozen-lockfile
nix develop --command pnpm --dir apps/desktop/web install --frozen-lockfile
nix develop --command pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci
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
cargo run -p xtask -- checksum /tmp/dropsquash-signed/DropSquash.dmg --output /tmp/dropsquash-signed/SHA256SUMS
cargo run -p xtask -- macos-keychain-cleanup-plan /tmp/dropsquash-signed/keychain
cargo run -p xtask -- manual-qa-prepare --reset-trial --app-artifact target/release/bundle/dmg/DropSquash.dmg --input-sample-set "short, medium, and large local recordings" --markdown-output /tmp/dropsquash-manual-qa-prepared-<app-build>.md
cargo run -p xtask -- benchmark --release-set --input /absolute/path/to/short.mov --input /absolute/path/to/medium.mov --input /absolute/path/to/large.mov --output-dir /tmp/dropsquash-manual-qa-output --csv-output /tmp/dropsquash-manual-qa-output/benchmark-results-<app-build>.csv
cargo run -p xtask -- benchmark-csv-check /tmp/dropsquash-manual-qa-output/benchmark-results-<app-build>.csv
cargo run -p xtask -- manual-qa-ready-all /tmp/dropsquash-manual-qa-prepared-<app-build>.md /tmp/dropsquash-manual-qa-output/benchmark-results-<app-build>.csv
# Optional packaged-only alternative instead of the full paid-beta pass above:
# cargo run -p xtask -- manual-qa-ready-local-proof /tmp/dropsquash-manual-qa-prepared-<app-build>.md /tmp/dropsquash-manual-qa-output/benchmark-results-<app-build>.csv
cargo run -p xtask -- manual-qa-pending /tmp/dropsquash-manual-qa-prepared-<app-build>.md --section local-proof
cargo run -p xtask -- manual-qa-license-rerun /tmp/dropsquash-manual-qa-prepared-<app-build>.md
cargo run -p xtask -- manual-qa-pending /tmp/dropsquash-manual-qa-prepared-<app-build>.md --section license
cargo run -p xtask -- manual-qa-distribution-rerun /tmp/dropsquash-manual-qa-prepared-<app-build>.md
cargo run -p xtask -- manual-qa-pending /tmp/dropsquash-manual-qa-prepared-<app-build>.md --section distribution
cargo run -p xtask -- manual-qa-check
cargo run -p xtask -- manual-qa-prepare --restore-state
```

These gates keep production files within the repository size rules, verify the
static sales site links, local resources, approved external links, and required
pages, reject secret-like files, require manual QA evidence, require the
external media process security gate, verify that production CSP does not open
remote network origins, keep desktop capabilities minimal, keep the unsigned
release workflow blocked, and verify that the updater is not enabled before
signing keys are ready.
The final public publish gate is stricter: `publish-check` also requires the
deployable `apps/site` surface to pass `npm --prefix apps/site run
verify:site`, so the public release path cannot rely on the static `website/`
draft alone.
As of Saturday, July 18, 2026, use `cargo run -p xtask -- public-web-ready` as
the first public-web entrypoint, use `cargo run -p xtask -- public-web-probe`
as the authoritative current DNS / canonical-host observation, and use
`cargo run -p xtask -- public-web-rerun` as the operator memo when you need
the current owner-only Sites URL, DNS visibility, and canonical-host
failure-state checks before claiming any `dropsquash.app` blocker evidence.
If `public-web-probe` still reports NXDOMAIN, `Could not resolve host:
dropsquash.app`, or Sites custom-domain state `pending` /
`pending_validation`, keep the public-web blockers open.

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
If the default `target/` tree is stale, or a local packaged-app rebuild hangs
while reusing old artifacts, prefer a fresh isolated build such as
`CARGO_TARGET_DIR=/tmp/dsq-build-target pnpm --dir apps/desktop tauri build`
and use `/tmp/dsq-build-target/release/bundle/macos/DropSquash.app` for the
packaged-app manual QA rerun instead of pretending the stale default bundle was
retested.
It prints the benchmark command, benchmark CSV check command, and benchmark
manual-QA rows. Use the exact benchmark CSV path emitted by
`manual-qa-prepare`, which currently includes the app-build suffix, so the
prepared file and the benchmark evidence stay aligned outside the repository.
If the release-set benchmark stops on `output is not smaller`, do not weaken the
benchmark gate. Reuse that clip for the packaged-app `Larger output` row, then
rerun the release set with another realistic recording or the exact shipping
size setting under test.
After the benchmark CSV is checked, prefer
`manual-qa-ready-all` for one deterministic pass across local proof, license,
and distribution. If you want only the packaged-app path, use
`manual-qa-ready-local-proof` to fill deterministic local-proof rows and remove
the prepared-draft marker in one step. `manual-qa-ready-all` now also prints
the next operator checklist plus focused license, distribution, and final
`manual-qa-check` gates. If the benchmark CSV path passed into
`manual-qa-ready-all` no longer points to an existing file, it also prints the isolated
`manual-qa-prepare --reset-trial` recovery command plus the matching
`benchmark --release-set` and `benchmark-csv-check` commands for that exact
CSV path; rerun those first instead of reusing stale `/tmp` evidence. Then use
`manual-qa-pending --section local-proof` to focus only on the remaining
packaged-app observations before running `manual-qa-check`. Keep
`manual-qa-fill-benchmark` and `manual-qa-fill-benchmark-threshold` for
intentional reruns or baseline-comparison troubleshooting rather than the
default packaged-app QA flow. `manual-qa-ready-local-proof` also echoes the
small, duplicate-output, queue, and large sample paths from the checked
benchmark CSV so the remaining packaged-app observations can stay on the same
sample set. After the benchmark sample-set row is recorded, the same hints are
repeated by `manual-qa-pending --section local-proof`, and rows that reuse the
checked benchmark samples print a row-specific `sample:` hint. Rows that need a
mounted DMG or intentionally bad input print a `note:` line instead. The same
pending output also groups packaged-app rows into practical manual-QA phases
and prints a per-phase remaining-count summary. When the checked sample set
includes the current kept-original candidate, `manual-qa-ready-local-proof`
also prints `fresh packaged-app not-smaller command:` and the packaged-app
pending output mirrors it with `fresh open-file not-smaller command:` so the
`Larger output` rerun can stay on `/tmp/dsq-build-target/.../DropSquash.app`
instead of a stale default `target/` bundle.
If `docs/manual-qa.md` already contains the checked benchmark CSV and you want
the shortest public-`DropSquash.dmg` packaged-app rerun path, use
`cargo run -p xtask -- manual-qa-packaged-rerun`. That wrapper prints the
current rerun helper, sample-link helper, packaged-app pending helper,
local-proof gate, final gate, and installed-app commands together.
If you want one short reminder of the packaged, license, and distribution
rerun entrypoints together before picking a section, use
`cargo run -p xtask -- manual-qa-paid-beta-rerun`.
That helper now also prints `next license sandbox runbook`,
`next signed DMG runbook`, and `paid beta license browser sign-in checkpoint`
so the paid-beta handoff can stop at the Lemon Squeezy sign-in step before
sandbox product setup, purchase, or activation is recorded.
When those helpers target a prepared draft outside the repository, the
deterministic release-gate fill intentionally leaves the `release-check` row
empty. `release-check` depends on the real `docs/manual-qa.md` blocker
evidence, so fill that row only after the prepared results are merged back into
the main manual-QA record.
After the release-gate commands pass, prefer `manual-qa-license-rerun` before
the license sandbox pass so the prepared file refreshes deterministic gate rows
and prints the next `manual-qa-pending --section license`,
`cargo run -p dropsquash -- license status`, and `manual-qa-check` commands.
The same `manual-qa-pending` phase summary now applies to `--section license`,
so sandbox setup, activation safety, valid activation, failure recovery, and
local diagnostics can be worked in order.
After the same release-gate commands pass, prefer
`manual-qa-distribution-rerun` before the distribution/signing pass so the
prepared file refreshes deterministic gate rows and prints the next
`manual-qa-pending --section distribution` plus `manual-qa-check` commands.
The same phase summary now also applies to `--section distribution`, so final
manual QA, Homebrew, signing environment, signature verification, and
Gatekeeper work can be staged in order.
Any custom `--app-state-dir`, `--state-dir`, or `--output-dir` must stay
outside the repository so private app state and generated QA media cannot be
committed or deleted by accident.
Custom `--app-state-dir` values must still point at an
`Application Support/DropSquash` directory, because `manual-qa-check` requires
the generated `config.json`, `history.jsonl`, and `license.json` paths to match
that app-support shape.
If the QA run used a reset trial state, restore the backed up local state after
recording evidence:

```sh
cargo run -p xtask -- manual-qa-prepare --restore-state
```

When prepare used custom state paths, use the printed
`trial state restore command` so restore uses the same `--app-state-dir` and
`--state-dir` values as the reset.

The unsigned Tauri build is only a packaging and QA input. Normalize the
generated Tauri DMG to `DropSquash.dmg` before artifact checks, checksums,
manual QA evidence, or public release notes refer to it. Public release still
requires signing, notarization, stapling, artifact checks, checksums, and
Gatekeeper no-warning evidence for the signed app.
Do not promise automatic deletion of the downloaded `.dmg` after Finder copy or
drag-to-Applications install: that copy path does not execute DropSquash code,
and the installed app cannot reliably know the original downloaded DMG path.
The same limitation applies to copy-and-paste installs from the mounted disk
image. Treat those as Finder-owned installs with no app-controlled completion
hook.
A first-launch helper detects when the app is running from `/Volumes` and offers
an explicit in-app install action. That action copies DropSquash to
`/Applications` without replacing an existing app. The post-copy notice can
open the installed app through native macOS APIs and quit the disk image copy
after the installed app opens. It can also request mounted-volume eject through
native macOS APIs and then quit the disk image copy when the install copy result
proves the source volume.
It does not delete the downloaded `.dmg` yet. Future
downloaded-installer cleanup must require an explicit user action, and may only
be offered when the backing `.dmg` path is known through a deterministic macOS
API and verified as the DropSquash distribution image. If that path cannot be
proven, keep the mounted-volume eject only. The UI must say that it moves the
downloaded DMG to Trash; it must not present the action as automatic deletion.
Schedule downloaded-DMG Trash cleanup after signed/notarized DMG manual QA,
because the proof must use the same distribution image shape that users receive.
Acceptance requires a reproducible packaged-app check showing the mounted volume
maps to the exact downloaded `DropSquash.dmg`, the image name and bundle identity
match DropSquash, copy-and-paste installs still do not expose cleanup, and the
Trash action is shown only after the user-triggered Applications copy succeeds.
The desktop command `load_install_location` currently provides the read-only
preflight for that helper: it reports the current `.app` path, whether launch is
from `/Volumes`, whether the app is already under `/Applications`, and whether
the app may offer an Applications move. The desktop UI uses that preflight to
warn when DropSquash is running from the disk image, and `copy_to_applications`
performs the user-triggered copy. The copy result reports whether mounted-volume
eject may be offered, but keeps downloaded `.dmg` Trash cleanup disabled until a
deterministic backing `.dmg` path is proven. The desktop command
`open_installed_application` opens only a validated direct `/Applications/*.app`
bundle with native `NSWorkspace`, `quit_current_app` exits only after explicit
user action, and `quit_after_installer_volume_eject` first validates and
requests native `NSWorkspace` eject on a direct `/Volumes` mount before exiting;
they do not shell out and do not delete the downloaded `.dmg`. Downloaded `.dmg`
cleanup remains future work.
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

Keep `Block unsigned Phase 0 release` in the release workflow until the public
release notes, manual QA, release blockers, website URLs, checkout URL, GitHub
Release URL, Homebrew tap PR URL, and `publish-check` evidence are complete.
The runner must run temporary keychain cleanup even when a signing,
notarization, or verification step fails. It must upload or publish only the
checked signed `DropSquash.dmg`; the unsigned QA artifact must remain clearly
named unsigned and must not be used as the release Artifact URL.

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
Before the canonical host is ready, the owner-only Sites URL may still answer
unauthenticated `curl -I` requests with HTTP 401. Treat that as "owner-only
deploy alive" only. If `curl` or the browser still reports `Could not resolve
host: dropsquash.app`, the canonical host is not ready and the public-web
blockers must stay open.

Check the local macOS signing and notarization credentials before a signed build:

```sh
CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- macos-signing-check
```

If the everyday `target/` tree is noisy or stale, keep using that isolated
`CARGO_TARGET_DIR` for signing-related `xtask` verification instead of mixing
release proof with the default cache.

The preflight accepts either `APPLE_SIGNING_IDENTITY` or
`APPLE_CERTIFICATE` with `APPLE_CERTIFICATE_PASSWORD` for signing. For
notarization it accepts either the App Store Connect API variables
`APPLE_API_KEY`, `APPLE_API_ISSUER`, and `APPLE_API_KEY_PATH`, or the Apple ID
variables `APPLE_ID`, `APPLE_PASSWORD`, and `APPLE_TEAM_ID`. Do not commit these
values. In GitHub Actions, signing requires `APPLE_CERTIFICATE` and
`APPLE_CERTIFICATE_PASSWORD`, plus `APPLE_KEYCHAIN_PASSWORD` for the temporary
keychain import; a local keychain identity name is not enough for a fresh
runner. The signing identity must be a Developer ID Application identity before
public release. Use `macos-keychain-plan` to generate the `security create-keychain`,
temporary keychain selection, certificate decode/import, and key partition list
argv without printing secret values. Pair it with `macos-keychain-cleanup-plan`
so the temporary keychain and decoded certificate cleanup argv are reviewed
before signing execution is added.
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

For the narrower paid-beta decision, check only the blockers required by
`docs/paid-beta-readiness.md`:

```sh
cargo run -p xtask -- paid-beta-check
```

Before a paid beta, complete the license sandbox rows in `docs/manual-qa.md`.
They cover the Lemon Squeezy product setup, license keys enabled, sandbox test purchase,
Live checkout URL, refund policy, cache safety, and UI action state.
Empty-key evidence must show
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
They also include a `github-release-plan` command so the tag, signed DMG,
`SHA256SUMS`, and prepared release notes file can be reviewed together before
publication.
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
Before publishing, generate the reviewed GitHub Release command from the same
tag, signed DMG, `SHA256SUMS`, and release notes file:

```sh
cargo run -p xtask -- github-release-plan v0.1.0 /tmp/dropsquash-signed/DropSquash.dmg /tmp/dropsquash-signed/SHA256SUMS /tmp/dropsquash-release-notes.md
```

The plan validates the signed `DropSquash.dmg`, rejects an artifact older than
`HEAD`, requires a `SHA256SUMS` file containing the `DropSquash.dmg` checksum
line, requires an existing `.md` notes file, and prints a
`gh release create --draft` command. It does not publish the release.

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

Check the generated cask against the public release notes before opening the
Homebrew tap PR:

```sh
cargo run -p xtask -- homebrew-cask-check packaging/homebrew/Casks/dropsquash.rb path/to/release-notes.md
```

The check verifies the cask version, release artifact URL, and lowercase
SHA-256 match the release notes `Version`, `Artifact URL`, and `SHA-256`
fields, and confirms the cask still declares `auto_updates false` and the
DropSquash app-state `zap` path.

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
That publish gate also reruns the deployable public site check through
`apps/site`, so a stale or non-deployable Sites surface blocks publication
even if `website/` still passes the static draft checks.
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
The owner-only Sites URL is still useful as deployment preflight, but an
access-controlled HTTP response there is not public-web completion evidence.
The Signing and distribution proof must show the same public `DropSquash.dmg`
from the release notes Artifact URL is signed, notarized, stapled, checksummed,
installable through the Homebrew tap PR, and opens without warning.
