# Signed DMG Runbook

Use this when moving from the unsigned local `DropSquash.dmg` QA artifact to the
signed, notarized, stapled macOS beta artifact.

Do not paste certificate material, App Store Connect secrets, license keys, or
private store IDs into this file, `docs/manual-qa.md`, or the release notes.

## Recommended Order

迷ったら、この順番を崩しません。

1. `cargo run -p xtask -- paid-beta-check`
2. fresh prepared manual-QA draft を作る
3. checked benchmark CSV を用意する
4. `cargo run -p xtask -- manual-qa-ready-all <manual-qa.md> <results.csv>`
5. `cargo run -p xtask -- manual-qa-distribution-rerun <manual-qa.md>`

`manual-qa-ready-all` は deterministic に埋まる行を先に整えるための入口です。
Codesign / notarization / Gatekeeper の distribution 行だけが残る状態を先に
作ってから、この runbook に入ってください。

## Short Execution Memo

急ぎで distribution 3行だけ埋めるときは、この順だけ守ります。

1. `cargo run -p xtask -- paid-beta-check`
2. `cargo run -p xtask -- manual-qa-distribution-rerun /tmp/dropsquash-manual-qa-prepared-<app-build>.md`
3. `cargo run -p xtask -- manual-qa-pending /tmp/dropsquash-manual-qa-prepared-<app-build>.md --section distribution | rg 'distribution .*markdown row:'`
4. `distribution quickstart 1`
5. `distribution quickstart 2`
6. `distribution quickstart 3`
7. `CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- macos-signing-check`
8. `distribution quickstart 4`
9. emitted signing-plan steps を実行して signed DMG を作る
10. `distribution quickstart 5`
11. `distribution quickstart 6`
12. `distribution quickstart 7`
13. `docs/manual-qa.md` と release notes に distribution evidence を貼る
14. `cargo run -p xtask -- manual-qa-check docs/manual-qa.md --section distribution`
15. `cargo run -p xtask -- paid-beta-check`

最低限の観測ポイント:

- `Codesign verification`: `codesign --verify`, `codesign -dv`, `Developer ID`
- `Notarization staple verification`: `notarytool accepted`, `stapler validate`, `spctl accepted`
- `Gatekeeper open test`: clean or fresh machine/account、signed、notarized、stapled、without warning

## Preconditions

- Build from a clean release candidate worktree.
- Use the Nix shell for the unsigned `.dmg` build.
- Keep the public Artifact URL and the tested `DropSquash.dmg` aligned.
- Confirm signing and notarization credentials before any signing-plan step:

```sh
CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- macos-signing-check
```

- That preflight now names incomplete variable groups directly, for example a
  missing `APPLE_CERTIFICATE_PASSWORD`, `APPLE_API_KEY_PATH`, or
  `APPLE_TEAM_ID`.
- Stop here if preflight fails with:
  `macOS signing requires APPLE_SIGNING_IDENTITY or APPLE_CERTIFICATE with APPLE_CERTIFICATE_PASSWORD`
- On Saturday, July 18, 2026, the current local preflight still fails with:

```text
macOS signing requires APPLE_SIGNING_IDENTITY or APPLE_CERTIFICATE with APPLE_CERTIFICATE_PASSWORD; notarization also needs APPLE_API_KEY/APPLE_API_ISSUER/APPLE_API_KEY_PATH or APPLE_ID/APPLE_PASSWORD/APPLE_TEAM_ID
```

- Treat that exact result as an environment blocker. Do not keep iterating on
  the DMG, manual QA rows, or release notes until one signing group and one
  notarization group are actually present.
- Local macOS signing needs either:
  - `APPLE_SIGNING_IDENTITY`
  - `APPLE_CERTIFICATE` with `APPLE_CERTIFICATE_PASSWORD`
- Local notarization needs either:
  - `APPLE_API_KEY`, `APPLE_API_ISSUER`, `APPLE_API_KEY_PATH`
  - `APPLE_ID`, `APPLE_PASSWORD`, `APPLE_TEAM_ID`
- CI signing also needs `APPLE_KEYCHAIN_PASSWORD` and `APPLE_CODESIGN_IDENTITY`.
- Do not continue into codesign, notarization, stapler, or Gatekeeper steps until
  `macos-signing-check` exits successfully.
- If `macos-signing-check` fails at this point, treat the run as blocked on
  release credentials rather than on product code. Fix the environment first,
  then rerun the preflight before touching any distribution row.
- Prefer a fresh target dir for heavy `xtask` verification if the default
  `target/` cache is slow:

```sh
CARGO_TARGET_DIR=/tmp/dsq-target cargo run -p xtask -- file-size-check
```

If the main worktree is dirty on July 17, 2026 or later, stop and switch to a
detached or snapshot QA worktree before building artifacts:

```sh
git worktree add --detach /tmp/dropsquash-qa-$(git rev-parse --short HEAD) HEAD
```

or:

```sh
scripts/manual-qa-snapshot-worktree.sh /tmp/dropsquash-qa-snapshot-$(git rev-parse --short HEAD)
```

If you want the shortest snapshot handoff for the distribution proof, use:

```sh
scripts/manual-qa-distribution-handoff.sh /tmp/dropsquash-qa-snapshot-$(git rev-parse --short HEAD)
```

That helper prints the snapshot path plus the next
`productization-status --track "Paid beta"`, desktop/web install,
`file-size-check`, `release-check`, unsigned build,
`manual-qa-prepare --reset-trial`, `macos-signing-check`,
`manual-qa-distribution-rerun`, and `paid-beta-check` commands for that exact
snapshot.

Run the remaining commands from that QA worktree so the tested DMG can be tied
to one exact source snapshot.

From that snapshot or detached worktree, rerun the gate and create the prepared
manual-QA markdown before signing-specific helper commands:

```sh
cd /tmp/dropsquash-qa-snapshot-$(git rev-parse --short HEAD)
CARGO_TARGET_DIR=/tmp/dsq-target cargo run -p xtask -- paid-beta-check
nix develop --command pnpm --dir apps/desktop install --frozen-lockfile
nix develop --command pnpm --dir apps/desktop/web install --frozen-lockfile
nix develop --command pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci
CARGO_TARGET_DIR=/tmp/dsq-target cargo run -p xtask -- manual-qa-prepare --reset-trial --app-artifact target/release/bundle/dmg/DropSquash.dmg --input-sample-set "short, medium, and large local recordings" --app-state-dir "/tmp/dropsquash-manual-qa-app-state/Library/Application Support/DropSquash" --state-dir /tmp/dropsquash-manual-qa-state --output-dir /tmp/dropsquash-manual-qa-output --markdown-output /tmp/dropsquash-manual-qa-prepared-<app-build>.md
```

## Unsigned Artifact

Build and normalize the unsigned QA artifact first:

```sh
nix develop --command pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci
cargo run -p xtask -- normalize-dmg target/release/bundle/dmg
cargo run -p xtask -- artifact-check target/release/bundle/dmg/DropSquash.dmg
```

If the default `target/` tree is stale or the unsigned bundle no longer matches
the current sources, rebuild into an isolated target and normalize there:

```sh
nix develop --command env CARGO_TARGET_DIR=/tmp/dsq-build-target pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci
cargo run -p xtask -- normalize-dmg /tmp/dsq-build-target/release/bundle/dmg
cargo run -p xtask -- artifact-check /tmp/dsq-build-target/release/bundle/dmg/DropSquash.dmg
```

## Signing Plan

Print the deterministic macOS signing sequence:

```sh
cargo run -p xtask -- macos-signing-plan target/release/bundle/dmg/DropSquash.dmg /tmp/dropsquash-signed-release
```

The signed target must stay isolated from the unsigned input.

## Execute Signing

Run the generated steps in order against `/tmp/dropsquash-signed-release/DropSquash.dmg`:

```sh
cargo run -p xtask -- signed-dmg-prepare target/release/bundle/dmg/DropSquash.dmg /tmp/dropsquash-signed-release
cargo run -p xtask -- signed-dmg-copy target/release/bundle/dmg/DropSquash.dmg /tmp/dropsquash-signed-release
cargo run -p xtask -- macos-keychain-plan /tmp/dropsquash-signed-release/keychain
cargo run -p xtask -- macos-codesign-plan /tmp/dropsquash-signed-release/DropSquash.dmg "Developer ID Application: ..."
cargo run -p xtask -- macos-codesign-verify-plan /tmp/dropsquash-signed-release/DropSquash.dmg
cargo run -p xtask -- macos-notary-plan /tmp/dropsquash-signed-release/DropSquash.dmg --api-key
cargo run -p xtask -- macos-stapler-plan /tmp/dropsquash-signed-release/DropSquash.dmg
cargo run -p xtask -- macos-spctl-plan /tmp/dropsquash-signed-release/DropSquash.dmg
cargo run -p xtask -- signed-dmg-check /tmp/dropsquash-signed-release/DropSquash.dmg target/release/bundle/dmg/DropSquash.dmg
cargo run -p xtask -- checksum /tmp/dropsquash-signed-release/DropSquash.dmg --output /tmp/dropsquash-signed-release/SHA256SUMS
cargo run -p xtask -- macos-keychain-cleanup-plan /tmp/dropsquash-signed-release/keychain
```

## Manual QA Rows

Release-blocker mapping:

- `Codesign verification` -> `Signed DMG`
- `Notarization staple verification` -> `Notarized and stapled DMG`
- `Gatekeeper open test` -> `Gatekeeper clean-machine open`

For the distribution rows in `docs/manual-qa.md`, use:

```sh
CARGO_TARGET_DIR=/tmp/dsq-target cargo run -p xtask -- manual-qa-distribution-rerun /tmp/dropsquash-manual-qa-prepared-<app-build>.md
CARGO_TARGET_DIR=/tmp/dsq-target cargo run -p xtask -- manual-qa-pending /tmp/dropsquash-manual-qa-prepared-<app-build>.md --section distribution
cargo run -p xtask -- paid-beta-check
```

That output now prints the helper commands plus row candidates for:

- `Codesign verification`
- `Notarization staple verification`
- `Gatekeeper open test`

It also prints the fresh-build helper commands for:

- `distribution fresh build command`
- `distribution fresh normalize command`
- `distribution fresh dmg artifact`
- `distribution fresh artifact check`
- `distribution fresh signing plan`
- `distribution fresh checksum command`

Read the current `manual-qa-distribution-rerun` output in this order:

1. `cargo run -p xtask -- manual-qa-pending <manual-qa.md> --section distribution | rg 'distribution .*markdown row:'`
2. `distribution quickstart 1`
3. `distribution quickstart 2`
4. `distribution quickstart 3`
5. `distribution quickstart 4`
6. `distribution quickstart 5`
7. `distribution quickstart 6`

## Current Isolated Commands

As of Saturday, July 18, 2026, `manual-qa-distribution-rerun docs/manual-qa.md`
prints this isolated distribution path set. Use these exact commands unless the
prepared file changes again.

Current preflight result:

```text
macOS signing requires APPLE_SIGNING_IDENTITY or APPLE_CERTIFICATE with APPLE_CERTIFICATE_PASSWORD; notarization also needs APPLE_API_KEY/APPLE_API_ISSUER/APPLE_API_KEY_PATH or APPLE_ID/APPLE_PASSWORD/APPLE_TEAM_ID
```

Interpret that literally: release credentials are still missing locally, so the
distribution proof is blocked on environment setup rather than product code.

Current quickstart from the helper:

```sh
CARGO_TARGET_DIR=/tmp/dsq-build-target pnpm --dir apps/desktop tauri build
cargo run -p xtask -- normalize-dmg /tmp/dsq-build-target/release/bundle/dmg
cargo run -p xtask -- artifact-check '/tmp/dsq-build-target/release/bundle/dmg/DropSquash.dmg'
cargo run -p xtask -- macos-signing-plan '/tmp/dsq-build-target/release/bundle/dmg/DropSquash.dmg' '/tmp/dropsquash-signed-release'
cargo run -p xtask -- macos-codesign-verify-plan '/tmp/dropsquash-signed-release/DropSquash.dmg'
cargo run -p xtask -- macos-stapler-plan '/tmp/dropsquash-signed-release/DropSquash.dmg'
cargo run -p xtask -- macos-spctl-plan '/tmp/dropsquash-signed-release/DropSquash.dmg'
cargo run -p xtask -- checksum '/tmp/dropsquash-signed-release/DropSquash.dmg' --output '/tmp/dropsquash-signed-release/SHA256SUMS'
```

Current helper observations before credentials are loaded:

- unsigned public DMG status:
  `missing target/release/bundle/dmg/DropSquash.dmg; run normalize command first`
- fresh DMG artifact:
  `/tmp/dsq-build-target/release/bundle/dmg/DropSquash.dmg`
- signed release staging dir:
  `/tmp/dropsquash-signed-release`

Current copy-ready markdown rows from the helper:

```text
| Codesign verification | Public DMG/app artifact verifies with Developer ID signature | codesign verified Developer ID Application signature for public DropSquash.dmg |
| Notarization staple verification | Public DMG/app artifact passes notary, stapler validate or stapled status, and `spctl` assessment | notary accepted, stapler validate passed, and spctl accepted for public DropSquash.dmg |
| Gatekeeper open test | Signed, notarized, stapled app from public `DropSquash.dmg` matching the release notes Artifact URL opens cleanly without Gatekeeper warning | Gatekeeper opened signed, notarized, stapled app from public DropSquash.dmg cleanly in fresh macOS account without Gatekeeper warning |
```

Treat those row bodies as templates, not proof. Only paste them after the
signed public artifact, notarization result, stapler verification, and clean
Gatekeeper open have been observed for the exact release candidate.
8. `distribution quickstart 7`
9. `pending distribution markdown rows`

Use those quickstart lines as the literal preflight order before you move into
codesign, notarization, stapler, and Gatekeeper observations.
Run `CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- macos-signing-check` before quickstart 4 so missing
signing environment variables are caught before you start the signing-plan
steps.
If that command fails, treat the run as blocked on release credentials rather
than on product code. Do not write placeholder evidence into `docs/manual-qa.md`
or the release notes.
On local macOS, that means APPLE_SIGNING_IDENTITY or APPLE_CERTIFICATE with
APPLE_CERTIFICATE_PASSWORD, plus either APPLE_API_KEY/APPLE_API_ISSUER/
APPLE_API_KEY_PATH or APPLE_ID/APPLE_PASSWORD/APPLE_TEAM_ID for notarization.
On CI, signing also requires APPLE_KEYCHAIN_PASSWORD and
APPLE_CODESIGN_IDENTITY.
After `distribution quickstart 4`, run the emitted signing-plan steps so
`/tmp/dropsquash-signed-release/DropSquash.dmg` exists before quickstart 5..7.

## Release Notes Evidence

Fill `docs/release-notes-template.md` only after the signed artifact is public.
Use concrete wording with the exact Artifact URL.

Minimum shape:

- `codesign`: mention `codesign --verify`, `codesign -dv`, `Developer ID`, `public`, and the exact Artifact URL
- `spctl`: mention `spctl --assess --type open`, `accepted`, `public`, and the exact Artifact URL
- `stapler`: mention `stapler validate` or stapled status, `public`, and the exact Artifact URL
- `Apple notary log`: mention `notarytool`, `accepted`, `log`, `public`, and the exact Artifact URL
- `Gatekeeper clean-machine open`: mention `Gatekeeper`, `fresh` or `clean`, `signed`, `notarized`, `stapled`, `public`, the exact Artifact URL, and `without warning`

Copy-ready release-note templates:

```text
codesign --verify --deep --strict --verbose=4 and codesign -dv --verbose=4 confirmed Developer ID for the public DropSquash.dmg at <Artifact URL>.
spctl --assess --type open --verbose=4 returned accepted for the public DropSquash.dmg at <Artifact URL>.
xcrun stapler validate confirmed stapled status for the public DropSquash.dmg at <Artifact URL>.
xcrun notarytool log showed accepted for the public DropSquash.dmg at <Artifact URL>.
Gatekeeper opened the fresh signed, notarized, stapled public DropSquash.dmg at <Artifact URL> on a clean machine without warning.
```

## Exit Criteria

Do not mark signing done until all of these are true:

- `docs/manual-qa.md` has concrete distribution rows
- `docs/release-blockers.md` can move `Signed DMG`, `Notarized and stapled DMG`,
  and `Gatekeeper clean-machine open` to `Verified`
- release notes name the same public Artifact URL
- `SHA256SUMS` matches the same public `DropSquash.dmg`
- `cargo run -p xtask -- paid-beta-check` no longer reports signing blockers
