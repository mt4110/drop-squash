# Paid Beta Readiness

This document narrows the current objective to the minimum required for a
testable paid beta.

For the shortest cross-track snapshot, also see
[docs/objective-audit.md](/Users/masakitakemura/_workspace/drop-squash/docs/objective-audit.md).

Primary product hypothesis:

```text
DropSquash is the automatic local post-processor for Mac screen recordings.
```

Current sequencing rule:

1. close the remaining P0 public/distribution alignment
2. then ship Phase 2 Secure Share Beta on macOS
3. only after that return to Lemon Squeezy sandbox proof and live checkout work

## In Scope For The Testable Paid Beta

The paid beta is ready to test only when all of these are true:

1. The single-file Mac screen-recording path is safe:
   - verified smaller-output conversions succeed
   - failed conversions do not count toward trial usage
   - cancelled conversions do not count toward trial usage
   - not-smaller results keep the original and do not count
   - originals are never moved without verified success
2. Trial behavior is trustworthy:
   - only successful smaller conversions count
   - the paid beta trial limit stays at 20
3. License behavior is trustworthy:
   - empty, invalid, network, and expired-grace states are friendly
   - raw license keys are never persisted
   - local forget works
   - sandbox product setup, sandbox purchase, and valid sandbox activation are
     manually verified
4. Release artifacts are trustworthy:
   - signed DMG
   - notarized and stapled DMG
   - Gatekeeper clean-machine open evidence
5. The product remains local-first:
   - no ffmpeg
   - no ffprobe
   - no media upload
   - no automatic original deletion

This document tracks the narrow paid-beta proof only.
Phase 2 Secure Share Beta is the next planned product step. It must add
destructive masking, receipt sidecars, and bounded audit metadata without
weakening the local-first, no-ffmpeg, or original-safety rules.

## Required Evidence For The Current P0 Gate

These evidence rows define the testable paid-beta gate:

- Packaged macOS manual QA
- Signed DMG
- Notarized and stapled DMG
- Gatekeeper clean-machine open

Quick status:

```sh
cargo run -p xtask -- paid-beta-check
cargo run -p xtask -- productization-status --track "Paid beta"
```

Current blocker shape as of Saturday, July 18, 2026:

- packaged-app local-proof rows in
  [docs/manual-qa.md](/Users/masakitakemura/_workspace/drop-squash/docs/manual-qa.md)
  are filled against the tested public `DropSquash.dmg`
- `cargo run -p xtask -- paid-beta-check` currently reports 7 required paid-beta
  blockers, 4 verified, 3 remaining, and no pending local-proof rows
- the remaining paid-beta blocker work is now concentrated in:
  - production deployment of the updated public-surface copy
  - tying signed distribution evidence to the public artifact
- the remaining work is now mostly external-state proof rather than core
  conversion-path implementation:
  - public website / release / checksum publication state
  - later Lemon Squeezy sandbox sign-in and checkout state for the 3 license
    rows after the Phase 2 Secure Share Beta direction is fixed
- `Packaged macOS manual QA` is already verified in
  [docs/release-blockers.md](/Users/masakitakemura/_workspace/drop-squash/docs/release-blockers.md);
  the remaining work is to fill the license and distribution rows in the same
  `docs/manual-qa.md` file and finish the full `manual-qa-check` gate for the
  complete paid-beta evidence set
- the current everyday worktree is dirty, so packaged-app and signing reruns
  should start from the snapshot / detached-worktree flow in
  [docs/paid-beta-operator-checklist.md](/Users/masakitakemura/_workspace/drop-squash/docs/paid-beta-operator-checklist.md)
  instead of rebuilding in place
- `CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- macos-signing-check` now passes once the signing
  credentials are present:

```text
macOS signing environment checks passed
```

- local macOS signing now requires either:
  - `APPLE_SIGNING_IDENTITY`
  - `APPLE_CERTIFICATE` with `APPLE_CERTIFICATE_PASSWORD`
- local notarization now also requires either:
  - `APPLE_API_KEY`, `APPLE_API_ISSUER`, `APPLE_API_KEY_PATH`
  - `APPLE_ID`, `APPLE_PASSWORD`, `APPLE_TEAM_ID`
- CI signing still additionally requires:
  - `APPLE_KEYCHAIN_PASSWORD`
  - `APPLE_CODESIGN_IDENTITY`

## Requirement Audit As Of Saturday, July 18, 2026

Use this table when deciding whether the paid-beta objective is actually done.
This is an audit view, not a replacement for the source records.

| Requirement | Current status | Strongest evidence now | Remaining proof |
|---|---|---|---|
| Single-file conversion safety | Verified for the current macOS path | `docs/manual-qa.md`, `docs/release-blockers.md`, release-set benchmark row, local-proof rows all filled, `manual QA local proof rows: none pending` from `cargo run -p xtask -- paid-beta-check` | Keep evidence current for the tested artifact when rerunning |
| Trial accounting correctness | Verified for paid-beta rules | `docs/manual-qa.md`, verified rows for failed/invalid/not-smaller handling, paid-beta limit documented as `20` here and in product/public docs | No current technical blocker in `paid-beta-check` |
| License friendly failure states | Verified except real sandbox success | `docs/release-blockers.md` shows `Empty key activation`, `Invalid license key handling`, `License network failure`, `Expired license refresh`, and `Local license forget` as `Verified` | Keep rows current if app behavior changes |
| Sandbox product setup | Deferred until after Phase 2 | `cargo run -p xtask -- paid-beta-check` still lists it as blocked in the older technical-proof gate, but commercialization order now defers it | Real Lemon Squeezy dashboard observation in `docs/manual-qa.md` after Phase 2 Secure Share Beta direction is fixed |
| Sandbox purchase | Deferred until after Phase 2 | `cargo run -p xtask -- paid-beta-check` still lists it as blocked in the older technical-proof gate | Real sandbox checkout observation in `docs/manual-qa.md` after Phase 2 Secure Share Beta direction is fixed |
| Valid sandbox activation | Deferred until after Phase 2 | `cargo run -p xtask -- paid-beta-check` still lists it as blocked in the older technical-proof gate | Real app-side activation to `Pro` plus cache observation in `docs/manual-qa.md` after Phase 2 Secure Share Beta direction is fixed |
| Signed DMG | Recorded locally, not yet tied to public release notes | `docs/manual-qa.md` now records `codesign` evidence for `/tmp/dropsquash-signed-release-20260718-1628/DropSquash.dmg` | Public signed artifact plus release-notes evidence |
| Notarized and stapled DMG | Recorded locally, not yet tied to public release notes | `docs/manual-qa.md` now records accepted notary log `42f27444-b954-4b7a-98ad-4fa507f1b08b`, stapler success, and mounted-app `spctl` acceptance for `/tmp/dropsquash-signed-release-20260718-1628/DropSquash.dmg` | Notary acceptance, stapler, and `spctl` evidence in release notes |
| Gatekeeper clean-machine open | Recorded locally | `docs/manual-qa.md` now records the fresh standard user `DropSquash QA` opening `/tmp/dropsquash-signed-release-20260718-1628/DropSquash.dmg` without Gatekeeper warning | Keep the same evidence tied to the final public artifact |
| Local-first invariant | Verified in current implementation | Product rules in `docs/productization.md`; media/privacy checks pass in rerun helpers; no `ffmpeg`, no `ffprobe`, no upload policy remains explicit | Keep invariant checks passing on future changes |

Current gate summary from current commands:

- `cargo run -p xtask -- paid-beta-check`:
  `7 required, 4 verified, 3 remaining`
- `cargo run -p xtask -- productization-status --track "Paid beta"`:
  `13 total, 10 verified, 3 blocked`
- deferred from paid-beta technical proof but still required before production
  onboarding or a public paid beta:
  `Public website deployment`, `Pricing finalized`, `Refund policy finalized`,
  `Live checkout link`, `Published checksum`, `Homebrew cask install`

Interpret those commands carefully:

- they still describe the old technical proof map
- they do not override the newer product order
- the newer product order is: finish P0 public/distribution alignment, then
  Phase 2 Secure Share Beta, then sandbox / checkout proof

Shortest default flow:

1. `paid-beta-check`
2. `manual-qa-prepare --reset-trial`
3. `benchmark --release-set`
4. `benchmark-csv-check`
5. `manual-qa-ready-all`
6. emitted license / distribution follow-up lines

When a fresh prepared manual-QA markdown file and checked benchmark CSV both
exist, treat `manual-qa-ready-all` as the default entrypoint for the remaining
paid-beta rows instead of picking individual section commands first.

Operator checklist:

- [docs/paid-beta-operator-checklist.md](/Users/masakitakemura/_workspace/drop-squash/docs/paid-beta-operator-checklist.md)

Quick routes:

- paid beta manual-QA rerun の入口をまとめて見る:
  `cargo run -p xtask -- manual-qa-paid-beta-rerun`
- sandbox 3 行を急いで埋める:
  [docs/license-sandbox-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/license-sandbox-runbook.md)
  の `Short Execution Memo`
- distribution 3 行を急いで埋める:
  [docs/signed-dmg-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/signed-dmg-runbook.md)
  の `Short Execution Memo`
- deferred public proof 6 行を回収する:
  [docs/public-beta-operator-checklist.md](/Users/masakitakemura/_workspace/drop-squash/docs/public-beta-operator-checklist.md)
  の `Short Execution Memo`
- canonical host 判定を先に見る:
  `cargo run -p xtask -- public-web-probe`

Remaining proof map as of Saturday, July 18, 2026:

| Remaining blocker | Start with | Record in |
|---|---|---|
| Lemon Squeezy product setup | Defer until after Phase 2 Secure Share Beta, then use `cargo run -p xtask -- manual-qa-paid-beta-rerun`, `paid beta license markdown rows`, and [docs/license-sandbox-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/license-sandbox-runbook.md) | `docs/manual-qa.md` |
| Lemon Squeezy sandbox purchase | Defer until after Phase 2 Secure Share Beta, then use `cargo run -p xtask -- manual-qa-paid-beta-rerun`, `paid beta license markdown rows`, and [docs/license-sandbox-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/license-sandbox-runbook.md) | `docs/manual-qa.md` |
| Valid sandbox activation | Defer until after Phase 2 Secure Share Beta, then use `cargo run -p xtask -- manual-qa-paid-beta-rerun`, `paid beta license markdown rows`, and [docs/license-sandbox-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/license-sandbox-runbook.md) | `docs/manual-qa.md` |
| Public website deployment | `cargo run -p xtask -- public-web-ready`, then [docs/public-beta-operator-checklist.md](/Users/masakitakemura/_workspace/drop-squash/docs/public-beta-operator-checklist.md) | `https://dropsquash.app/release-status` |
| Pricing finalized | `cargo run -p xtask -- public-web-ready`, then [docs/public-beta-operator-checklist.md](/Users/masakitakemura/_workspace/drop-squash/docs/public-beta-operator-checklist.md) | `https://dropsquash.app/pricing` |
| Refund policy finalized | `cargo run -p xtask -- public-web-ready`, then [docs/public-beta-operator-checklist.md](/Users/masakitakemura/_workspace/drop-squash/docs/public-beta-operator-checklist.md) | `https://dropsquash.app/refund` |
| Live checkout link | `cargo run -p xtask -- public-web-ready`, then [docs/public-beta-operator-checklist.md](/Users/masakitakemura/_workspace/drop-squash/docs/public-beta-operator-checklist.md) | Live checkout URL |
| Published checksum | `cargo run -p xtask -- publish-check path/to/release-notes.md` after `SHA256SUMS` is attached to the public GitHub Release | GitHub Release URL |
| Homebrew cask install | `cargo run -p xtask -- homebrew-cask-check packaging/homebrew/Casks/dropsquash.rb path/to/release-notes.md` after the public tap PR and install/uninstall proof exist | Homebrew tap PR URL |
| Signed DMG | `cargo run -p xtask -- manual-qa-paid-beta-rerun`, then `paid beta distribution markdown rows`, then [docs/signed-dmg-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/signed-dmg-runbook.md) | Release notes |
| Notarized and stapled DMG | `cargo run -p xtask -- manual-qa-paid-beta-rerun`, then `paid beta distribution markdown rows`, then [docs/signed-dmg-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/signed-dmg-runbook.md) | Release notes |
| Gatekeeper clean-machine open | `cargo run -p xtask -- manual-qa-paid-beta-rerun`, then `paid beta distribution markdown rows`, then [docs/signed-dmg-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/signed-dmg-runbook.md) | `docs/manual-qa.md` |

Treat this table as a routing map only. Source-of-truth completion evidence
still lives in [docs/release-blockers.md](/Users/masakitakemura/_workspace/drop-squash/docs/release-blockers.md).
The packaged-app proof is already verified there, so it is intentionally absent
from the remaining-blocker map above.

When a fresh prepared manual-QA markdown file and checked benchmark CSV both
exist, prefer `manual-qa-ready-all` for one deterministic pass before filling
the remaining paid-beta rows. Use the emitted `next operator checklist`,
license, distribution, `manual-qa-check`, and `paid-beta-check` lines as the
default follow-up. If the referenced benchmark CSV has already disappeared from
`/tmp`, rerun the printed `manual-qa-prepare --reset-trial`,
`benchmark --release-set`, and `benchmark-csv-check` commands first instead of
reusing stale evidence paths.
If you first want one short reminder of every paid-beta manual-QA rerun
entrypoint and gate, run
`cargo run -p xtask -- manual-qa-paid-beta-rerun`.
That output now also prints:

- `next license sandbox runbook`
- `next signed DMG runbook`
- `paid beta license browser sign-in checkpoint`
- `paid beta license browser sign-in success`
- `paid beta license activation loop`
- `paid beta license markdown rows`
- `paid beta distribution markdown rows`

Use those lines before the section gates when you want only the copy-ready
manual QA markdown rows for the remaining sandbox or signing proof.
If `docs/manual-qa.md` already records the checked benchmark CSV, use
`cargo run -p xtask -- manual-qa-packaged-rerun` as the shortest packaged-app
rerun entrypoint for the public `DropSquash.dmg`.

These rows are already supporting that path and should stay verified:

- Empty key activation
- Invalid license key handling
- License network failure
- Expired license refresh
- Local license forget
- Benchmark release set

When re-running the packaged macOS checks behind those rows, prefer a fresh
isolated build such as
`CARGO_TARGET_DIR=/tmp/dsq-build-target pnpm --dir apps/desktop tauri build`
and use `/tmp/dsq-build-target/release/bundle/macos/DropSquash.app` whenever
the default `target/` tree is stale or does not reflect the current sources.

## Deliberately Deferred Until After The Testable Paid Beta

This section is intentionally narrow. These items are deferred only from the
manual-license technical proof tracked by `paid-beta-check`.

They are still required before production payment onboarding, a production
website launch, or a public paid beta:

- Public website deployment
- Pricing finalized
- Refund policy finalized
- Live checkout link
- Published checksum
- Homebrew cask install

They are now sequenced in this order:

1. deploy the updated public site copy to `dropsquash.app`
2. verify `release-status`, `pricing`, and `refund` on the canonical host
3. finish the Phase 2 Secure Share Beta direction
4. return to Lemon Squeezy sandbox proof
5. only then move into live checkout and production payment onboarding

Current deferred public-web status as of Saturday, July 18, 2026:

- the Sites project still has owner-only history at
  `https://dropsquash-app.system-obj-gg.chatgpt.site`, but the canonical host
  is now live too
- the latest saved Sites version is now `3`, from commit
  `1017927b9c1ab26e4fd251ad81ffbd85fd0c88b5`
- the saved version id remains
  `appgprj_6a5a4a7d557c8191ac7381f80c671354~appgver_f4d9d0220ef081919a8e2dfb1cd789b2`
- the Sites access mode is now `public`
- the `dropsquash.app` custom domain status is now `active`
- the `dropsquash.app` SSL status is now `active`
- public canonical checks now return HTTP 200 for:
  - `https://dropsquash.app/`
  - `https://dropsquash.app/release-status`
  - `https://dropsquash.app/pricing`
  - `https://dropsquash.app/refund`
- the canonical site content is staged, but the site is intentionally owner-only
  again and therefore does not close `Public website deployment`,
  `Pricing finalized`, or `Refund policy finalized`
- the remaining public-web work is intentionally paused until we reopen public
  release, at which point the live checkout URL will still be required
- treat `cargo run -p xtask -- public-web-probe` as the authoritative current
  observation before touching public-web blocker rows
- the required canonical-host records remain:
  - A `dropsquash.app` -> `162.159.143.30`
  - A `dropsquash.app` -> `172.66.3.26`
  - TXT `_openai-site-verification.dropsquash.app` ->
    `openai-site-verification=1B5z4jz2Z2ifY2MsN3UggSHPMaSsaGwrOgttmLnIw6g`
  - TXT `_cf-custom-hostname.dropsquash.app` ->
    `c70e75c8-8887-4b4a-a390-72fd4a8400fd`

Manual beta license issuance is acceptable before those public-launch surfaces
exist only for private beta operations. It does not replace the public website,
pricing, refund, and checkout alignment required by
[docs/productization.md](/Users/masakitakemura/_workspace/drop-squash/docs/productization.md)
before Stripe or Lemon Squeezy production onboarding. Use
[docs/manual-beta-license-issuance.md](/Users/masakitakemura/_workspace/drop-squash/docs/manual-beta-license-issuance.md)
for private/manual beta issuance and
[docs/public-beta-operator-checklist.md](/Users/masakitakemura/_workspace/drop-squash/docs/public-beta-operator-checklist.md)
for the deferred public-launch and production-onboarding proof.

## Out Of Scope Until Market Validation

Do not add these before the Market Validation deliverables exist and a market
decision memo justifies them:

- broad format support
- editing features
- third-party advertising SDKs
- YouTube downloading
- media-library features

See [docs/market-validation.md](/Users/masakitakemura/_workspace/drop-squash/docs/market-validation.md).
