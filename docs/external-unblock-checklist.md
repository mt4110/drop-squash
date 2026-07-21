# External Unblock Checklist

Use this when the repository is ready but paid beta completion still depends on
outside systems.

Date baseline:

```text
Saturday, July 18, 2026
```

## Current Snapshot

Use this as the current starting point before touching outside systems.

```text
paid beta check: 7 required, 1 verified, 6 remaining
remaining paid-beta blockers:
- Lemon Squeezy product setup
- Lemon Squeezy sandbox purchase
- Valid sandbox activation
- Signed DMG
- Notarized and stapled DMG
- Gatekeeper clean-machine open
```

```text
public web proof: canonical host is live; remaining rows are pricing copy, refund copy, and live checkout
current Sites state:
- project status: active
- current live URL: https://dropsquash-app.system-obj-gg.chatgpt.site
- access mode: public
- available access modes: custom, public
- custom domain: dropsquash.app
- custom domain status: active
- ssl status: active
- canonical HTTP checks:
  - https://dropsquash.app/ -> 200
  - https://dropsquash.app/release-status -> 200
  - https://dropsquash.app/pricing -> 200
  - https://dropsquash.app/refund -> 200
```

```text
current signing preflight:
macOS signing requires APPLE_SIGNING_IDENTITY or APPLE_CERTIFICATE with APPLE_CERTIFICATE_PASSWORD; notarization also needs APPLE_API_KEY/APPLE_API_ISSUER/APPLE_API_KEY_PATH or APPLE_ID/APPLE_PASSWORD/APPLE_TEAM_ID
```

## 1. Lemon Squeezy Sandbox

Goal:

- fill `Sandbox product setup`
- fill `Sandbox purchase`
- fill `Valid sandbox activation`

Start here:

```sh
cargo run -p xtask -- manual-qa-license-rerun
```

Do this:

1. Sign in to the Lemon Squeezy dashboard.
2. Confirm sandbox mode is visible.
3. Confirm the intended product is `DropSquash`.
4. Confirm sandbox license keys are enabled.
5. Complete one sandbox checkout and keep a non-secret order id or order
   number.
6. Launch the app through:
   `cargo run -p xtask -- manual-qa-license-rerun`
7. Activate one real sandbox key and record:
   - submit disabled while `Activating`
   - `Pro` state
   - fingerprint present
   - `instance_id` present
   - raw key absent

Record in:

- [docs/manual-qa.md](/Users/masakitakemura/_workspace/drop-squash/docs/manual-qa.md)

Reference:

- [docs/license-sandbox-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/license-sandbox-runbook.md)

## 2. Apple Signing And Notarization

Goal:

- fill `Signed DMG`
- fill `Notarized and stapled DMG`
- fill `Gatekeeper clean-machine open`

Start here:

```sh
cargo run -p xtask -- macos-signing-check
cargo run -p xtask -- manual-qa-distribution-rerun
```

Do this:

1. Load one signing credential group:
   - `APPLE_SIGNING_IDENTITY`
   - or `APPLE_CERTIFICATE` + `APPLE_CERTIFICATE_PASSWORD`
2. Load one notarization credential group:
   - `APPLE_API_KEY` + `APPLE_API_ISSUER` + `APPLE_API_KEY_PATH`
   - or `APPLE_ID` + `APPLE_PASSWORD` + `APPLE_TEAM_ID`
3. Run:
   `cargo run -p xtask -- macos-signing-check`
4. Only after that passes, continue with:
   `cargo run -p xtask -- manual-qa-distribution-rerun`
5. Record:
   - codesign verification
   - notary accepted
   - stapler validate
   - `spctl` accepted
   - clean-machine or fresh-account open without Gatekeeper warning

Record in:

- Release notes
- [docs/manual-qa.md](/Users/masakitakemura/_workspace/drop-squash/docs/manual-qa.md)

Reference:

- [docs/signed-dmg-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/signed-dmg-runbook.md)

## 3. Public Website

Goal:

- fill `Public website deployment`
- fill `Pricing finalized`
- fill `Refund policy finalized`
- fill `Live checkout link`

Start here:

```sh
cargo run -p xtask -- public-web-ready
cargo run -p xtask -- public-web-probe
```

Do this:

1. Keep `https://dropsquash.app/release-status` as the canonical evidence URL
   for `Public website deployment`.
2. Revise the pricing page until it no longer says `draft` and still matches
   the real product state.
3. Revise the refund page until it no longer says `draft` or `Checkout is not
   live yet`, unless that wording is intentionally part of the final policy.
4. Add and verify the real
   `store.lemonsqueezy.com/checkout/buy/<id>` link only after the product,
   legal, and distribution truth is aligned.
5. Re-run:
   - `cargo run -p xtask -- public-web-probe`
   - `cargo run -p xtask -- productization-status --track 'Public web proof'`

Record in:

- [docs/release-blockers.md](/Users/masakitakemura/_workspace/drop-squash/docs/release-blockers.md)

Reference:

- [docs/public-beta-operator-checklist.md](/Users/masakitakemura/_workspace/drop-squash/docs/public-beta-operator-checklist.md)
- [docs/website-deployment-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/website-deployment-runbook.md)

## 4. Public Distribution

Goal:

- fill `Published checksum`
- fill `Homebrew cask install`

Start here:

```sh
cargo run -p xtask -- publish-check /absolute/path/to/release-notes.md
cargo run -p xtask -- homebrew-cask-check packaging/homebrew/Casks/dropsquash.rb /absolute/path/to/release-notes.md
```

Do this:

1. Attach `SHA256SUMS` to the public GitHub Release for the same public
   `DropSquash.dmg`.
2. Open the public Homebrew tap PR.
3. Verify install/uninstall against the same public artifact and matching
   SHA-256.

Record in:

- GitHub Release URL
- Homebrew tap PR URL

## 5. Market Validation

Goal:

- collect real cohort evidence before any scope expansion

Start here:

- issue one manual beta seat
- record one activated tester row

Do this:

1. Issue private beta seats manually.
2. Collect one real 3-file session result.
3. Collect repeat-usage and willingness-to-pay follow-up.
4. Update the tracker rollup before changing the market decision memo.

Reference:

- [docs/manual-beta-license-issuance.md](/Users/masakitakemura/_workspace/drop-squash/docs/manual-beta-license-issuance.md)
- [docs/market-validation-tracker.md](/Users/masakitakemura/_workspace/drop-squash/docs/market-validation-tracker.md)
- [docs/market-decision-memo.md](/Users/masakitakemura/_workspace/drop-squash/docs/market-decision-memo.md)
