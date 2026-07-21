# Paid Beta Operator Checklist

Use this when finishing the last manual proof for the testable paid beta.
For the shortest owner-side external action list, also see
[docs/external-unblock-checklist.md](/Users/masakitakemura/_workspace/drop-squash/docs/external-unblock-checklist.md).

This checklist is intentionally narrow:

1. fill the 3 license sandbox rows
2. fill the 3 signing / notarization / Gatekeeper distribution rows
3. rerun the paid beta and manual QA gates

It does not replace the public-website proof tracked in
`docs/release-blockers.md`. Public website deployment, pricing finalization,
refund finalization, and the live checkout URL still must be completed before
production payment onboarding or a public paid beta.
Published checksum and Homebrew cask install also remain deferred to the later
public-proof phase.
Use
[docs/public-beta-operator-checklist.md](/Users/masakitakemura/_workspace/drop-squash/docs/public-beta-operator-checklist.md)
for that later public-proof phase.

Do not paste raw license keys, private store IDs, certificate material, or
notary secrets into this repository.

## Current Gate

Start here:

```sh
CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- paid-beta-check
```

Current signing preflight as of Saturday, July 18, 2026:

```sh
CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- macos-signing-check
```

Current result:

```text
macOS signing requires APPLE_SIGNING_IDENTITY or APPLE_CERTIFICATE with APPLE_CERTIFICATE_PASSWORD; notarization also needs APPLE_API_KEY/APPLE_API_ISSUER/APPLE_API_KEY_PATH or APPLE_ID/APPLE_PASSWORD/APPLE_TEAM_ID
```

Interpret this literally:

- signing is currently blocked by release credentials, not by product code
- local macOS signing still needs either:
  - `APPLE_SIGNING_IDENTITY`
  - `APPLE_CERTIFICATE` with `APPLE_CERTIFICATE_PASSWORD`
- local notarization still needs either:
  - `APPLE_API_KEY`, `APPLE_API_ISSUER`, `APPLE_API_KEY_PATH`
  - `APPLE_ID`, `APPLE_PASSWORD`, `APPLE_TEAM_ID`

Do not enter the distribution proof loop until that preflight passes.

## Dirty Worktree Quickstart

`paid-beta-check` が dirty worktree を返したら、普段の作業ツリーでそのまま
packaged-app / signing proof に入らないでください。最短でもこの順です。

1. snapshot worktree を作る
2. snapshot 側で `productization-status --track "Paid beta"`
3. Nix build で unsigned DMG を作る
4. `manual-qa-prepare --reset-trial`
5. `benchmark --release-set`
6. `benchmark-csv-check`
7. `manual-qa-ready-all`

最短 handoff:

```sh
snapshot=$(scripts/manual-qa-snapshot-worktree.sh /tmp/dropsquash-qa-snapshot-$(git rev-parse --short HEAD) | sed -n 's/^snapshot worktree: //p')
cd "$snapshot"
CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- productization-status --track "Paid beta"
nix develop --command pnpm --dir apps/desktop install --frozen-lockfile
nix develop --command pnpm --dir apps/desktop/web install --frozen-lockfile
nix develop --command pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci
CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- manual-qa-prepare --reset-trial --app-artifact target/release/bundle/dmg/DropSquash.dmg --input-sample-set "short, medium, and large local recordings" --app-state-dir "/tmp/dropsquash-manual-qa-app-state/Library/Application Support/DropSquash" --state-dir /tmp/dropsquash-manual-qa-state --output-dir /tmp/dropsquash-manual-qa-output --markdown-output /tmp/dropsquash-manual-qa-prepared-<app-build>.md
CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- benchmark --release-set --input /absolute/path/to/short.mov --input /absolute/path/to/medium.mov --input /absolute/path/to/large.mov --output-dir /tmp/dropsquash-manual-qa-output --csv-output /tmp/dropsquash-manual-qa-output/benchmark-results-<app-build>.csv
CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- benchmark-csv-check /tmp/dropsquash-manual-qa-output/benchmark-results-<app-build>.csv
CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- manual-qa-ready-all /tmp/dropsquash-manual-qa-prepared-<app-build>.md /tmp/dropsquash-manual-qa-output/benchmark-results-<app-build>.csv
```

sandbox 3 行を急いで進めるなら、snapshot 作成から `manual-qa-license-rerun`
までの next commands をまとめて出す shortcut も使えます:

```sh
scripts/manual-qa-license-sandbox-handoff.sh /tmp/dropsquash-qa-snapshot-$(git rev-parse --short HEAD)
```

license handoff script も snapshot path に加えて `file-size-check` と
`release-check` 付きの next commands をまとめて出します。

distribution 3 行を急いで進めるなら、snapshot 作成から
`manual-qa-distribution-rerun` までの next commands をまとめて出す
shortcut も使えます:

```sh
scripts/manual-qa-distribution-handoff.sh /tmp/dropsquash-qa-snapshot-$(git rev-parse --short HEAD)
```

ここまで通ってから、`manual-qa-license-rerun` / `manual-qa-distribution-rerun`
へ進めます。distribution handoff script は snapshot path に加えて
`file-size-check`、`release-check`、`macos-signing-check` 付きの next
commands もまとめて出します。

## Rerun Entrypoints

最初にどの helper を叩くべきか迷ったら、この表だけ見ます。

| Need | Start with | What it prints next |
|---|---|---|
| paid beta 全体の rerun 導線を一度に見たい | `cargo run -p xtask -- manual-qa-paid-beta-rerun` | packaged / license / distribution の rerun entrypoint、`next license sandbox runbook`、`next signed DMG runbook`、`paid beta license browser sign-in checkpoint`、`paid beta license browser sign-in success`、`paid beta license activation loop`、`paid beta license markdown rows`、`paid beta distribution markdown rows`、section gate / final gate |
| packaged-app proof だけ進めたい | `cargo run -p xtask -- manual-qa-packaged-rerun` | sample-link helper, installed-app status/stash/restore, local-proof gate |
| sandbox product setup / purchase / valid activation を埋めたい | `cargo run -p xtask -- manual-qa-license-rerun` | pending rows, browser sign-in checkpoint, browser sign-in success, sandbox quickstart 1..4, activation loop, cache diagnostics, next paid-beta gate |
| signing / notarization / Gatekeeper rowsを埋めたい | `cargo run -p xtask -- manual-qa-distribution-rerun` | signing preflight, signing plan, codesign/stapler/spctl plans, checksum command |
| fresh prepared markdown と benchmark CSV から deterministic に整えたい | `cargo run -p xtask -- manual-qa-ready-all /tmp/dropsquash-manual-qa-prepared-<app-build>.md /tmp/dropsquash-manual-qa-output/benchmark-results-<app-build>.csv` | next operator checklist, section gates, final `manual-qa-check`, `paid-beta-check` |

helper output は一時表示なので、証拠そのものではありません。完成判定は
必ず
[docs/release-blockers.md](/Users/masakitakemura/_workspace/drop-squash/docs/release-blockers.md)
と
[docs/manual-qa.md](/Users/masakitakemura/_workspace/drop-squash/docs/manual-qa.md)
の実記録で行います。

`manual-qa-distribution-rerun` の `macos-signing-check` が落ちた場合は、
まず資格情報の不足を疑います。ここで止まるのは通常、product code の不具合
ではなく release credentials の未投入です。
もし `xtask` の verification 自体が普段の `target/` で詰まるなら、
distribution helper 群は `CARGO_TARGET_DIR=/tmp/dsq-xtask-target` を付けて
isolated target に逃がしてから rerun します。

Saturday, July 18, 2026 時点の sandbox helper 実出力は
[docs/license-sandbox-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/license-sandbox-runbook.md)
の `Current Isolated Commands` に固定してあります。`/tmp/dsq-build-target`
の fresh app、isolated `license.json`、copy-ready markdown row をそのまま
再確認したいときは、まずそちらを見てから UI 側の実観測に進んでください。

Saturday, July 18, 2026 時点の distribution helper 実出力は
[docs/signed-dmg-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/signed-dmg-runbook.md)
の `Current Isolated Commands` に固定してあります。`/tmp/dsq-build-target`
の fresh DMG、`/tmp/dropsquash-signed-release` の staging path、そして
`macos-signing-check` の現在の失敗文言まで残してあるので、資格情報投入後は
そこから signing / notarization の観測へそのまま進んでください。

## Shortest Evidence Flow

迷ったら、まずこの順番だけ守ります。

1. `paid-beta-check` で現在の blocker を確認する
2. isolated prepared markdown と fresh benchmark CSV を用意する
3. `manual-qa-ready-all` で deterministic な行を一気に整える
4. `manual-qa-license-rerun` と `manual-qa-distribution-rerun` で残りの manual rows を埋める

最短コマンド列:

```sh
CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- paid-beta-check
CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- manual-qa-prepare --reset-trial --app-artifact target/release/bundle/dmg/DropSquash.dmg --input-sample-set "short, medium, and large local recordings" --app-state-dir "/tmp/dropsquash-manual-qa-app-state/Library/Application Support/DropSquash" --state-dir /tmp/dropsquash-manual-qa-state --output-dir /tmp/dropsquash-manual-qa-output --markdown-output /tmp/dropsquash-manual-qa-prepared-<app-build>.md
CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- benchmark --release-set --input /absolute/path/to/short.mov --input /absolute/path/to/medium.mov --input /absolute/path/to/large.mov --output-dir /tmp/dropsquash-manual-qa-output --csv-output /tmp/dropsquash-manual-qa-output/benchmark-results-<app-build>.csv
CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- benchmark-csv-check /tmp/dropsquash-manual-qa-output/benchmark-results-<app-build>.csv
CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- manual-qa-ready-all /tmp/dropsquash-manual-qa-prepared-<app-build>.md /tmp/dropsquash-manual-qa-output/benchmark-results-<app-build>.csv
```

その後は emitted lines をそのまま使います。

- license の残り:
  `manual-qa-license-rerun`
- distribution の残り:
  `manual-qa-distribution-rerun`
- section gate:
  `manual-qa-check --section license` / `manual-qa-check --section distribution`
- 最終 gate:
  `manual-qa-check` と `paid-beta-check`

`manual-qa-ready-all` が通ったら、次の判断は自分で組み立てず、出力された
`next operator checklist`、`next license section gate`、
`next distribution section gate`、`next final manual QA gate`、
`next paid beta check command` をそのまま辿ります。

## Remaining Proof Map

| Remaining blocker | Start with | Record in |
|---|---|---|
| Lemon Squeezy product setup | `cargo run -p xtask -- manual-qa-paid-beta-rerun`, then `paid beta license markdown rows`, then [docs/license-sandbox-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/license-sandbox-runbook.md) | `docs/manual-qa.md` |
| Lemon Squeezy sandbox purchase | `cargo run -p xtask -- manual-qa-paid-beta-rerun`, then `paid beta license markdown rows`, then [docs/license-sandbox-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/license-sandbox-runbook.md) | `docs/manual-qa.md` |
| Valid sandbox activation | `cargo run -p xtask -- manual-qa-paid-beta-rerun`, then `paid beta license markdown rows`, then [docs/license-sandbox-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/license-sandbox-runbook.md) | `docs/manual-qa.md` |
| Signed DMG | `cargo run -p xtask -- manual-qa-paid-beta-rerun`, then `paid beta distribution markdown rows`, then [docs/signed-dmg-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/signed-dmg-runbook.md) | Release notes |
| Notarized and stapled DMG | `cargo run -p xtask -- manual-qa-paid-beta-rerun`, then `paid beta distribution markdown rows`, then [docs/signed-dmg-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/signed-dmg-runbook.md) | Release notes |
| Gatekeeper clean-machine open | `cargo run -p xtask -- manual-qa-paid-beta-rerun`, then `paid beta distribution markdown rows`, then [docs/signed-dmg-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/signed-dmg-runbook.md) | `docs/manual-qa.md` |

As of Saturday, July 18, 2026, packaged-app proof is already verified in
[docs/release-blockers.md](/Users/masakitakemura/_workspace/drop-squash/docs/release-blockers.md),
so it is intentionally excluded from the remaining-blocker map above.

Treat this table as a routing map only. Source-of-truth completion evidence
still lives in [docs/release-blockers.md](/Users/masakitakemura/_workspace/drop-squash/docs/release-blockers.md).

If `paid-beta-check` reports a dirty worktree, do not run the packaged-app or
signing proof from the everyday tree. Use one of these first:

```sh
git worktree add --detach /tmp/dropsquash-qa-$(git rev-parse --short HEAD) HEAD
```

or snapshot the current dirty tree without mutating it:

```sh
scripts/manual-qa-snapshot-worktree.sh /tmp/dropsquash-qa-snapshot-$(git rev-parse --short HEAD)
```

Then run the remaining build and QA commands inside that detached or snapshot
worktree.

Recommended snapshot handoff:

```sh
snapshot=$(scripts/manual-qa-snapshot-worktree.sh /tmp/dropsquash-qa-snapshot-$(git rev-parse --short HEAD) | sed -n 's/^snapshot worktree: //p')
cd "$snapshot"
CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- productization-status --track "Paid beta"
nix develop --command pnpm --dir apps/desktop install --frozen-lockfile
nix develop --command pnpm --dir apps/desktop/web install --frozen-lockfile
nix develop --command pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci
```

From that snapshot worktree, create the prepared manual-QA markdown before
running `manual-qa-license-rerun` or `manual-qa-distribution-rerun`:

```sh
CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- manual-qa-prepare --reset-trial --app-artifact target/release/bundle/dmg/DropSquash.dmg --input-sample-set "short, medium, and large local recordings" --app-state-dir "/tmp/dropsquash-manual-qa-app-state/Library/Application Support/DropSquash" --state-dir /tmp/dropsquash-manual-qa-state --output-dir /tmp/dropsquash-manual-qa-output --markdown-output /tmp/dropsquash-manual-qa-prepared-<app-build>.md
```

Treat every `/tmp/.../benchmark-results-*.csv` path as disposable evidence for
that exact run. If an older row in `docs/manual-qa.md` points at a missing CSV,
do not reuse it. Regenerate a fresh prepared file and fresh benchmark CSV for
the current app build before continuing.

If you already have the fresh prepared markdown and checked benchmark CSV,
prefer one deterministic pass first:

```sh
CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- manual-qa-ready-all /tmp/dropsquash-manual-qa-prepared-<app-build>.md /tmp/dropsquash-manual-qa-output/benchmark-results-<app-build>.csv
```

That command now prints:

- `next operator checklist`
- `next license section gate`
- `next distribution section gate`
- `next final manual QA gate`
- `next paid beta check command`

If the referenced benchmark CSV no longer exists, do not reuse stale `/tmp`
evidence. Follow the printed recovery commands in order:

1. `manual-qa-prepare --reset-trial`
2. `benchmark --release-set`
3. `benchmark-csv-check`
4. `manual-qa-ready-all`

As of the current repository state, that command should report:

- `manual QA local proof rows: none pending`
- `manual QA license rows still empty`
- `manual QA distribution rows still empty`

ここで見える distribution 側の pending 行は、release blocker 3件
(`Signed DMG` / `Notarized and stapled DMG` / `Gatekeeper clean-machine open`)
そのものに加えて、preflight や public-release 側で再利用する helper 行も
含みます。paid beta technical proof の実質的な残 blocker は、その 3 件です。

If you want one short reminder of every paid-beta manual-QA rerun entrypoint
before choosing a section, run:

```sh
CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- manual-qa-paid-beta-rerun
```

That helper prints the packaged, license, and distribution rerun entrypoints
plus the focused section gates and final `manual-qa-check` / `paid-beta-check`
commands in one place.
It now also prints:

- `next license sandbox runbook`
- `next signed DMG runbook`
- `paid beta license browser sign-in checkpoint`
- `paid beta license markdown rows`
- `paid beta distribution markdown rows`

Use those lines before the section gates when you only need the remaining
copy-ready manual QA markdown rows for sandbox or signing proof.

If you are advancing only the packaged-app proof after the benchmark CSV is
ready, run:

```sh
CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- manual-qa-ready-local-proof /tmp/dropsquash-manual-qa-prepared-<app-build>.md /tmp/dropsquash-manual-qa-output/benchmark-results-<app-build>.csv
```

If `docs/manual-qa.md` already contains the checked benchmark CSV and you want
the shortest rerun path for the public `DropSquash.dmg`, use:

```sh
CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- manual-qa-packaged-rerun
```

That helper now also prints:

- `fresh packaged-app sample-link command`
- `fresh packaged-app installed-app status`
- `fresh packaged-app installed-app stash`
- `fresh packaged-app installed-app restore`
- `packaged-app mounted dmg window probe command`

Use the sample-link command before chooser-based packaged-app checks, and use
the installed-app commands before mounted-DMG checks so an older
`/Applications/DropSquash.app` does not quietly interfere with the observation.
Use the window-probe command before and after relaunch so the mounted app pid
count, pid list, and AX window count are recorded together.

If you want a focused gate before every row in other sections is filled, use:

```sh
CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- manual-qa-check docs/manual-qa.md --section local-proof
CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- manual-qa-check docs/manual-qa.md --section license
CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- manual-qa-check docs/manual-qa.md --section distribution
```

## License Sandbox Rows

Prefer an isolated prepared manual-QA file before touching the sandbox rows.
Using `docs/manual-qa.md` directly is acceptable only when its `Config path`,
`History path`, and `License cache path` already point under
`/tmp/.../Application Support/DropSquash`.
急ぎで 3 行だけ埋めるなら、まず
[docs/license-sandbox-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/license-sandbox-runbook.md)
の `Short Execution Memo` を見てから、この節に戻ってください。

Recommended flow:

```sh
nix develop --command pnpm --dir apps/desktop install --frozen-lockfile
nix develop --command pnpm --dir apps/desktop/web install --frozen-lockfile
nix develop --command pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci
CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- manual-qa-prepare --reset-trial --app-artifact target/release/bundle/dmg/DropSquash.dmg --input-sample-set "short, medium, and large local recordings" --app-state-dir "/tmp/dropsquash-manual-qa-app-state/Library/Application Support/DropSquash" --state-dir /tmp/dropsquash-manual-qa-state --output-dir /tmp/dropsquash-manual-qa-output --markdown-output /tmp/dropsquash-manual-qa-prepared-<app-build>.md
CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- manual-qa-license-rerun /tmp/dropsquash-manual-qa-prepared-<app-build>.md
```

That command prints the exact pending rows plus:

- `next sandbox row candidates`
- `sandbox quickstart 1`
- `sandbox quickstart 2`
- `sandbox quickstart 3`
- `sandbox quickstart 4`

Helper-row entrypoint when you only need the already-defined safety rows:

```sh
CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- manual-qa-pending docs/manual-qa.md --section license
```

If `App artifact` in `docs/manual-qa.md` still points at a `.dmg`, prefer the
printed fresh app launch commands for sandbox activation loops. Use the mounted
DMG launch only when you intentionally need mounted-DMG behavior.

If you only have the main record, print the exact pending rows and helper
commands with:

```sh
CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- manual-qa-license-rerun 'docs/manual-qa.md'
```

Copy the emitted sandbox markdown rows into `docs/manual-qa.md` after the real
sandbox observations are complete:

- `Sandbox product setup`
- `Sandbox purchase`
- `Valid sandbox activation`

For the sandbox observation flow, use
[docs/license-sandbox-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/license-sandbox-runbook.md).

Before copying any sandbox result back into `docs/manual-qa.md`, confirm the
observed cache path stayed under `/tmp/.../Application Support/DropSquash` and
that no raw key or private store identifier was pasted into the repository.

## Distribution Rows

Print the exact pending rows and helper commands:
急ぎで distribution 3 行だけ埋めるなら、まず
[docs/signed-dmg-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/signed-dmg-runbook.md)
の `Short Execution Memo` を見てから、この節に戻ってください。

```sh
nix develop --command pnpm --dir apps/desktop install --frozen-lockfile
nix develop --command pnpm --dir apps/desktop/web install --frozen-lockfile
nix develop --command pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci
CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- manual-qa-prepare --reset-trial --app-artifact target/release/bundle/dmg/DropSquash.dmg --input-sample-set "short, medium, and large local recordings" --app-state-dir "/tmp/dropsquash-manual-qa-app-state/Library/Application Support/DropSquash" --state-dir /tmp/dropsquash-manual-qa-state --output-dir /tmp/dropsquash-manual-qa-output --markdown-output /tmp/dropsquash-manual-qa-prepared-<app-build>.md
CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- manual-qa-distribution-rerun /tmp/dropsquash-manual-qa-prepared-<app-build>.md
```

When building the unsigned QA artifact for those rows, prefer the Nix shell in
the detached or snapshot worktree:

```sh
nix develop --command pnpm --dir apps/desktop install --frozen-lockfile
nix develop --command pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci
```

After the fresh benchmark CSV exists, create ASCII sample aliases from that
same CSV before packaged-app chooser work:

```sh
cargo run -p xtask -- manual-qa-link-samples /tmp/dropsquash-manual-qa-output/benchmark-results-<app-build>.csv /tmp/dropsquash-qa-open-panel
```

Copy the emitted `pending distribution markdown rows:` into
`docs/manual-qa.md` after the real signing and release observations are
complete:

必要な markdown 行だけ先に抜くなら:

```sh
CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- manual-qa-pending /tmp/dropsquash-manual-qa-prepared-<app-build>.md --section distribution | rg 'distribution .*markdown row:'
```

Before those rows, follow the printed:

- `distribution signing environment check`
- `distribution quickstart 1`
- `distribution quickstart 2`
- `distribution quickstart 3`
- `distribution quickstart 4`
- `distribution quickstart 5`
- `distribution quickstart 6`
- `distribution quickstart 7`

2026年7月18日時点で、`distribution signing environment check` が通るには
少なくとも次が必要です。

- local macOS signing:
  - `APPLE_SIGNING_IDENTITY`
  - または `APPLE_CERTIFICATE` と `APPLE_CERTIFICATE_PASSWORD`
- local notarization:
  - `APPLE_API_KEY`, `APPLE_API_ISSUER`, `APPLE_API_KEY_PATH`
  - または `APPLE_ID`, `APPLE_PASSWORD`, `APPLE_TEAM_ID`
- CI signing の追加要件:
  - `APPLE_KEYCHAIN_PASSWORD`
  - `APPLE_CODESIGN_IDENTITY`

ここが欠けているときは product code ではなく release credentials が未投入
なだけなので、signing plan に進まず先に環境を揃えてください。

`distribution quickstart 4` の signing plan を実行して
`/tmp/dropsquash-signed-release/DropSquash.dmg` を作ってから
`distribution quickstart 5..7` に進んでください。

- ``cargo run -p xtask -- manual-qa-check``
- ``cargo run -p xtask -- homebrew-cask-check packaging/homebrew/Casks/dropsquash.rb path/to/release-notes.md``
- ``CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- macos-signing-check``
- `Codesign verification`
- `Notarization staple verification`
- `Gatekeeper open test`

When `manual-qa-pending --section distribution` says `App artifact is the local
unsigned QA DMG`, treat the printed plan/check commands as preflight only.
Record the final distribution rows against the signed public `DropSquash.dmg`
and its public Artifact URL, not the local unsigned QA artifact.

For the signing/notarization flow, use
[docs/signed-dmg-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/signed-dmg-runbook.md).

Before mounted DMG manual QA, check whether an older installed app is still at
`/Applications/DropSquash.app`:

```sh
cargo run -p xtask -- manual-qa-installed-app status /tmp/dropsquash-manual-qa-installed-app
```

If it reports `installed app: present`, stash it before mounted DMG QA:

```sh
cargo run -p xtask -- manual-qa-installed-app stash /tmp/dropsquash-manual-qa-installed-app
```

After the mounted DMG observations are finished, restore the stashed app:

```sh
cargo run -p xtask -- manual-qa-installed-app restore /tmp/dropsquash-manual-qa-installed-app
```

## Final Commands

After the manual rows are filled with real evidence:

```sh
CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- manual-qa-check 'docs/manual-qa.md'
CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- paid-beta-check
```

The paid beta proof is ready only when:

- `manual-qa-check` passes
- `paid-beta-check` reports no remaining blockers
- the signed/notarized artifact evidence matches the same public
  `DropSquash.dmg`

## Related Docs

- [docs/paid-beta-readiness.md](/Users/masakitakemura/_workspace/drop-squash/docs/paid-beta-readiness.md)
- [docs/manual-beta-license-issuance.md](/Users/masakitakemura/_workspace/drop-squash/docs/manual-beta-license-issuance.md)
- [docs/license-sandbox-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/license-sandbox-runbook.md)
- [docs/signed-dmg-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/signed-dmg-runbook.md)
