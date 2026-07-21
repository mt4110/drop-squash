# License Sandbox Runbook

Use this when recording the remaining Lemon Squeezy sandbox proof in
`docs/manual-qa.md`.

Do not paste raw license keys, private store IDs, or checkout secrets into this
repository.

## Target Rows

- `Sandbox product setup`
- `Sandbox purchase`
- `Valid sandbox activation`

Release-blocker mapping:

- `Sandbox product setup` -> `Lemon Squeezy product setup`
- `Sandbox purchase` -> `Lemon Squeezy sandbox purchase`
- `Valid sandbox activation` -> `Valid sandbox activation`

The empty-key, invalid-key, network-failure, expired-refresh, and local-forget
rows already have helper commands and candidate wording through
`manual-qa-pending --section license`.

## Recommended Order

迷ったら、この順番を崩しません。

1. `cargo run -p xtask -- paid-beta-check`
2. fresh prepared manual-QA draft を作る
3. checked benchmark CSV を用意する
4. `cargo run -p xtask -- manual-qa-ready-all <manual-qa.md> <results.csv>`
5. `cargo run -p xtask -- manual-qa-license-rerun <manual-qa.md>`

`manual-qa-ready-all` は deterministic に埋まる行を先に整えるための入口です。
Sandbox product setup / purchase / valid activation の3行だけが残る状態を先に
作ってから、この runbook に入ってください。

## Short Execution Memo

急ぎで sandbox 3行だけ埋めるときは、この順だけ守ります。

1. `cargo run -p xtask -- paid-beta-check`
2. `cargo run -p xtask -- manual-qa-license-rerun /tmp/dropsquash-manual-qa-prepared-<app-build>.md`
3. helper が `Sign in to Lemon Squeezy` / `auth.lemonsqueezy.com/login` を示すなら、Lemon Squeezy dashboard にサインインする
4. `next sandbox markdown rows command`
5. `sandbox quickstart 1`
6. `sandbox quickstart 2`
7. `sandbox quickstart 3`
8. アプリで sandbox key を入れて `Pro` まで進める
9. `sandbox quickstart 4`
10. `docs/manual-qa.md` に 3 行を貼る
11. `cargo run -p xtask -- manual-qa-check docs/manual-qa.md --section license`
12. `cargo run -p xtask -- paid-beta-check`

最低限の観測ポイント:

- `Sandbox product setup`: intended product が `DropSquash`、sandbox、license keys enabled
- `Sandbox purchase`: `test buyer` と order id or order number
- `Valid sandbox activation`: `Activating` 中の disabled submit、`Pro`、checked cache、fingerprint、`instance_id`、raw key absence

## Browser Sign-In Prerequisite

Before trying to fill the remaining 3 sandbox rows, confirm the Lemon Squeezy
operator session is signed in to the Lemon Squeezy dashboard.

Use this as the minimum browser checkpoint:

1. Reach `https://app.lemonsqueezy.com/` or the current Lemon Squeezy
   dashboard host.
2. Confirm you are past the `Sign in to Lemon Squeezy` page.
3. Confirm sandbox mode is active before checking the intended product.
4. Continue only after the dashboard is open for the intended `DropSquash`
   product view or product list.

If the browser is still on `https://auth.lemonsqueezy.com/login` or shows the
`Sign in to Lemon Squeezy` form, stop there and sign in first. Do not record
`Sandbox product setup`, `Sandbox purchase`, or `Valid sandbox activation`
until the dashboard session is actually open.

## Start From The Prepared View

Use the prepared manual-QA file or the main record:

```sh
CARGO_TARGET_DIR=/tmp/dsq-target cargo run -p xtask -- manual-qa-pending docs/manual-qa.md --section license
```

That output prints:

- cache inspection commands
- cache helper commands
- launch commands
- license diagnostics commands
- row candidates and markdown row templates

Prefer a prepared manual-QA file whose `Config path`, `History path`, and
`License cache path` already point under `/tmp/.../Application Support/DropSquash`
instead of your everyday `~/Library/Application Support/DropSquash`. Running
`manual-qa-license-rerun` on such an isolated file now keeps the next launch,
diagnostics, and cache-inspection commands on that isolated state. When the
cache path is not under `/tmp`, `manual-qa-license-rerun` prints an explicit
isolation note before you continue.

If you do not already have an isolated prepared file, stop and create or reuse
one before continuing. The sandbox rows are much easier to verify when the app
state is disposable and the resulting `license.json` and `history.jsonl`
observations are not mixed with day-to-day testing.

If `cargo run -p xtask -- paid-beta-check` reports a dirty worktree, do not use
the everyday tree for sandbox proof. Move into:

```sh
git worktree add --detach /tmp/dropsquash-qa-$(git rev-parse --short HEAD) HEAD
```

or snapshot the current dirty tree exactly:

```sh
scripts/manual-qa-snapshot-worktree.sh /tmp/dropsquash-qa-snapshot-$(git rev-parse --short HEAD)
```

If you want the shortest snapshot handoff for the sandbox proof, use:

```sh
scripts/manual-qa-license-sandbox-handoff.sh /tmp/dropsquash-qa-snapshot-$(git rev-parse --short HEAD)
```

That helper prints the snapshot path plus the next
`productization-status --track "Paid beta"`, desktop/web install,
`file-size-check`, `release-check`, unsigned build,
`manual-qa-prepare --reset-trial`, `manual-qa-license-rerun`, and
`paid-beta-check` commands for that exact snapshot.

Then continue the sandbox flow from that detached or snapshot worktree so the
packaged app and the recorded cache evidence refer to one exact source state.

From the snapshot worktree, rerun the gate and create the isolated prepared
markdown before any sandbox observations:

```sh
cd /tmp/dropsquash-qa-snapshot-$(git rev-parse --short HEAD)
CARGO_TARGET_DIR=/tmp/dsq-target cargo run -p xtask -- paid-beta-check
nix develop --command pnpm --dir apps/desktop install --frozen-lockfile
nix develop --command pnpm --dir apps/desktop/web install --frozen-lockfile
nix develop --command pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci
CARGO_TARGET_DIR=/tmp/dsq-target cargo run -p xtask -- manual-qa-prepare --reset-trial --app-artifact target/release/bundle/dmg/DropSquash.dmg --input-sample-set "short, medium, and large local recordings" --app-state-dir "/tmp/dropsquash-manual-qa-app-state/Library/Application Support/DropSquash" --state-dir /tmp/dropsquash-manual-qa-state --output-dir /tmp/dropsquash-manual-qa-output --markdown-output /tmp/dropsquash-manual-qa-prepared-<app-build>.md
```

If the default `target/` tree is stale, rebuild the packaged app into an
isolated target first:

```sh
nix develop --command env CARGO_TARGET_DIR=/tmp/dsq-build-target pnpm --dir apps/desktop tauri build --bundles app --no-sign --ci
```

Then use:

- `/tmp/dsq-build-target/release/bundle/macos/DropSquash.app`
- `license fresh app launch command: ...`
- `license fresh app network failure launch command: ...`

from `manual-qa-pending --section license` instead of pretending the stale
default bundle was retested.

For the cleanest handoff into sandbox work, start from:

```sh
CARGO_TARGET_DIR=/tmp/dsq-target cargo run -p xtask -- manual-qa-license-rerun /tmp/dropsquash-manual-qa-prepared-<app-build>.md
cargo run -p xtask -- paid-beta-check
```

and then follow the printed:

- `next sandbox row candidates`
- `sandbox quickstart 1`
- `sandbox quickstart 2`
- `sandbox quickstart 3`
- `sandbox quickstart 4`

Treat those printed commands as the source of truth for the current isolated
paths. Do not rewrite them by hand unless the prepared file itself changed.

The current `manual-qa-license-rerun` output is intentionally front-loaded for
the paid-beta sandbox proof. Read it in this order:

1. `next sandbox markdown rows command`
2. `next sandbox row candidates`
3. `license browser sign-in checkpoint`
4. `sandbox quickstart 1` to launch the fresh app
5. `sandbox quickstart 2` to record the before-state
6. `sandbox quickstart 3` to inspect `license.json`
7. `sandbox quickstart 4` after the UI action

The later helper lines remain useful, but they are secondary during the
three-row paid-beta proof.

Use those printed quickstart lines as the literal command order for the
activation loop. Do not jump ahead to the broader helper output unless the
quickstart path fails or you intentionally need a non-sandbox row such as
network-failure or local-forget evidence later.
Run `sandbox quickstart 2` before the UI action and `sandbox quickstart 4`
after `Pro` appears so the cache delta is recorded instead of inferred.

## Current Isolated Commands

As of Saturday, July 18, 2026, `manual-qa-license-rerun docs/manual-qa.md`
prints this isolated sandbox path set. Use these exact commands unless the
prepared file changes again.

Fresh packaged app:

```sh
CARGO_TARGET_DIR=/tmp/dsq-build-target pnpm --dir apps/desktop tauri build
cargo run -p xtask -- manual-qa-launch-app '/tmp/dsq-build-target/release/bundle/macos/DropSquash.app' '/tmp/dropsquash-manual-qa-app-state/Library/Application Support/DropSquash/config.json'
```

Cache and diagnostics:

```sh
cargo run -p dropsquash -- license status --history '/tmp/dropsquash-manual-qa-app-state/Library/Application Support/DropSquash/history.jsonl' --cache-path '/tmp/dropsquash-manual-qa-app-state/Library/Application Support/DropSquash/license.json'
sed -n '1,160p' "/tmp/dropsquash-manual-qa-app-state/Library/Application Support/DropSquash/license.json"
```

Current helper observation before a real sandbox activation:

- `license cache path`:
  `/tmp/dropsquash-manual-qa-app-state/Library/Application Support/DropSquash/license.json`
- `raw license key persisted`: `no`
- `fingerprint`: `missing`
- `instance_id`: `missing`

Current copy-ready markdown rows from the helper:

```text
| Sandbox product setup | Intended product is DropSquash, sandbox license keys are enabled, and private store IDs are not recorded | Intended product was DropSquash; Lemon Squeezy sandbox mode was active; sandbox license keys were enabled; confirmed private store IDs were not recorded |
| Sandbox purchase | Sandbox checkout completes for the intended product and test buyer order | Sandbox checkout completed for the intended DropSquash product and test buyer order; recorded the sandbox order id or order number without private store IDs |
| Valid sandbox activation | Lemon Squeezy sandbox activation request disables submit while Activating; Pro state; checked license cache has 64-character lowercase hex fingerprint and `instance_id` fields with no raw key | Lemon Squeezy sandbox activation request disabled submit while Activating; Pro state; checked license cache diagnostics showed raw license key persisted: no, fingerprint present 64-character lowercase hex, and instance_id present |
```

Treat those row bodies as templates, not proof. Only paste them into
`docs/manual-qa.md` after the real Lemon Squeezy dashboard and app-side
activation loop have been observed.

## Sandbox Product Setup

Confirm all of these outside the repository:

1. The intended product is `DropSquash`.
2. Lemon Squeezy sandbox mode is active.
3. License keys are enabled for the sandbox product.
4. Private store IDs are not recorded in repo docs.

When you copy the result into `docs/manual-qa.md`, make sure the sentence still
includes:

- `DropSquash`
- `sandbox`
- `intended product`
- `license keys enabled`
- `private store IDs not recorded`

Preferred result shape:

`Intended product was DropSquash; sandbox license keys were enabled; confirmed private store IDs were not recorded`

Copy-ready template:

```text
Intended product was DropSquash; confirmed Lemon Squeezy sandbox mode for the intended product, license keys were enabled, and private store IDs were not recorded in repository docs.
```

## Sandbox Purchase

Complete one sandbox checkout for the intended DropSquash product.

Record:

- sandbox checkout completed
- intended product
- `test buyer`
- concrete order id or order number
- no private store IDs pasted

The checker expects a real order identifier, not only `order completed`.
Keep the order id or order number outside secrets, but include enough digits or
characters for the result to be concrete.

Preferred result shape:

`Sandbox checkout completed for the intended DropSquash product and test buyer order; recorded sandbox order id or order number without private store IDs`

Copy-ready template:

```text
Sandbox checkout completed for the intended DropSquash product and test buyer order; recorded sandbox order id or order number <replace-with-order-id> without private store IDs.
```

## Valid Sandbox Activation

Use a sandbox key outside the repository, then record the app behavior plus the
local cache diagnostics.

Recommended sequence:

```sh
sandbox quickstart 1
sandbox quickstart 2
sandbox quickstart 3
```

Then in the app:

1. Enter the sandbox key.
2. Confirm submit disables while `Activating`.
3. Confirm the app reaches `Pro`.
4. Re-run `sandbox quickstart 4` and inspect `license.json`.

When the default `target/` tree is stale, prefer the fresh app launch helper
printed by `manual-qa-pending --section license`:

```sh
cargo run -p xtask -- manual-qa-launch-app '/tmp/dsq-build-target/release/bundle/macos/DropSquash.app' '/path/to/config.json'
```

The recorded evidence must include:

- Lemon Squeezy sandbox activation request
- disabled `Activating` state
- `Pro` state
- checked cache evidence
- 64-character lowercase hex fingerprint
- `instance_id` present
- raw key absent

Before pasting the result, sanity-check that it still says:

- `Lemon Squeezy`
- `sandbox`
- `Activating`
- `disabled submit`
- `Pro`
- `checked` or `inspected` cache
- `fingerprint`
- `instance_id`
- `raw key persisted: no` or equivalent raw-key absence

If the cache helper output does not report `fingerprint present 64-character
lowercase hex` and `instance_id present`, do not soften the wording in
`docs/manual-qa.md`; fix the activation evidence first.

Preferred result shape:

`Lemon Squeezy sandbox activation request disabled submit while Activating; Pro state; checked license cache diagnostics showed raw license key persisted: no, fingerprint present 64-character lowercase hex, and instance_id present`

Copy-ready template:

```text
Lemon Squeezy sandbox activation request disabled submit while Activating; Pro state appeared; checked license cache diagnostics showed raw license key persisted: no, fingerprint present 64-character lowercase hex, and instance_id present.
```

## Finish

After recording the rows, run:

```sh
CARGO_TARGET_DIR=/tmp/dsq-target cargo run -p xtask -- paid-beta-check
CARGO_TARGET_DIR=/tmp/dsq-target cargo run -p xtask -- manual-qa-check docs/manual-qa.md
```

If only sandbox rows remain, the next missing rows should move from license to
distribution or public-web proof.
