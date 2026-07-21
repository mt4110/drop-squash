# Public Beta Operator Checklist

Use this only after the narrower technical proof for the private/manual paid
beta is complete.

For the shortest cross-track snapshot, also see
[docs/objective-audit.md](/Users/masakitakemura/_workspace/drop-squash/docs/objective-audit.md).
For the shortest owner-side external action list, also see
[docs/external-unblock-checklist.md](/Users/masakitakemura/_workspace/drop-squash/docs/external-unblock-checklist.md).

This checklist exists to prevent a common mistake:

- `paid-beta-check` passing means the local app, trial, license sandbox, and
  signed/notarized release proof are ready for a testable private/manual beta
- it does not mean production payment onboarding or a public paid beta is ready

Before public launch or production onboarding, all deferred public-proof rows
still need concrete evidence.

## Required Before Production Onboarding

These items remain required after the private/manual paid beta technical proof:

1. Public website deployment
2. Pricing finalized
3. Refund policy finalized
4. Live checkout link
5. Published checksum
6. Homebrew cask install

The blocker source of truth remains
[docs/release-blockers.md](/Users/masakitakemura/_workspace/drop-squash/docs/release-blockers.md).

## Current Gate

Start here:

```sh
cargo run -p xtask -- productization-status
```

That output should explicitly list the deferred items above before production
payment onboarding or a public paid beta.

Current repository state note:

- the static website files exist under `website/`
- `cargo run -p xtask -- website-check` passes locally
- this repository now contains `.openai/hosting.json` with a Sites `project_id`
- the Sites project now has a public live URL:
  `https://dropsquash.app`
- the latest saved Sites version is now `3`, pointing to commit
  `1017927b9c1ab26e4fd251ad81ffbd85fd0c88b5`
- that saved version has id
  `appgprj_6a5a4a7d557c8191ac7381f80c671354~appgver_f4d9d0220ef081919a8e2dfb1cd789b2`
- the current local repository `HEAD` is
  `5a69fea9240adfddff77425c83faf72ea3b61751`, so the current public site
  should still be treated as potentially older than the current local tree
  until version `3` is deliberately deployed
- the Site project itself is currently `active`
- the current Sites access mode is `public`
- therefore, local website correctness is still not production deployment proof
- record blocker evidence only after the real `https://dropsquash.app/...` URLs
  are live and manually verified
- use
  [docs/website-deployment-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/website-deployment-runbook.md)
  when moving from local site proof to the real production host
- the custom domain is now active and the canonical host resolves publicly
- `cargo run -p xtask -- public-web-probe` now reports HTTP 200 for the owner
  host, canonical host, pricing, and refund URLs
- the remaining public-web blockers are now content-state blockers, not DNS
  blockers: the current live pricing/refund pages still serve older draft-era
  copy until saved version `3` is deployed

## Requirement Audit As Of Saturday, July 18, 2026

Use this table when deciding whether production onboarding or a public paid
beta is actually ready. This is an audit view, not a replacement for
`docs/release-blockers.md`.

| Requirement | Current status | Strongest evidence now | Remaining proof |
|---|---|---|---|
| Public website deployment | Verified on canonical host | `cargo run -p xtask -- public-web-probe` now returns HTTP 200 for `https://dropsquash.app/release-status`, and `docs/release-blockers.md` records `Public website deployment` as verified | Keep the canonical host public and aligned with the intended site surface |
| Pricing finalized | Staged but private | `https://dropsquash.app/pricing` copy is aligned, but the site is intentionally owner-only and returns `401` to non-owners | Keep blocked until the site is intentionally made public again |
| Refund policy finalized | Staged but private | `https://dropsquash.app/refund` copy is aligned, but the site is intentionally owner-only and returns `401` to non-owners | Keep blocked until the site is intentionally made public again |
| Live checkout link | Missing | Public-web track still lists `Live checkout link` as blocked; checklist keeps checkout blocked until a tested `store.lemonsqueezy.com/checkout/buy/<id>` exists | Verify the production pricing page opens the real live checkout URL |
| Published checksum | Missing | No public GitHub Release evidence is recorded yet | Attach `SHA256SUMS` for the exact public `DropSquash.dmg` and verify with `publish-check` |
| Homebrew cask install | Missing | No public tap PR evidence is recorded yet | Open the public tap PR and verify install/uninstall against the same public artifact |

Current public-proof gate summary from current commands:

- `cargo run -p xtask -- productization-status --track "Public web proof"`:
  `3/4 remaining`
- `cargo run -p xtask -- public-web-ready`:
  local content and deployable packaging passed on Saturday, July 18, 2026
- `cargo run -p xtask -- public-web-probe`:
  owner-only HTTP 200; canonical HTTP 200; pricing HTTP 200; refund HTTP 200

Current public-web-ready snapshot:

```text
public web local gate passed: cargo run -p xtask -- website-check
public web deployable site passed: npm --prefix apps/site run verify:site
public web ready: local website content and deployable site packaging passed on Saturday, July 18, 2026
public web next step 1: make the canonical host resolve and serve https://dropsquash.app/release-status
public web next step 2: verify the final production URLs https://dropsquash.app/pricing and https://dropsquash.app/refund
public web next step 3: keep checkout blocked until the tested live https://store.lemonsqueezy.com/checkout/buy/<id> URL exists
```

DNS note for Saturday, July 18, 2026:

- treat `cargo run -p xtask -- public-web-probe` as the authoritative current
  observation
- the canonical host is now live; the remaining blockers are content-state
  blockers on pricing and refund plus the later live checkout URL
- the owner-history host remains useful only as a provenance reference

Current probe snapshot:

```text
public web probe date: Saturday, July 18, 2026
dns A: 172.67.131.70
expected dns A: 162.159.143.30 172.66.3.26
dns TXT verification: "openai-site-verification=1B5z4jz2Z2ifY2MsN3UggSHPMaSsaGwrOgttmLnIw6g"
expected TXT verification: openai-site-verification=1B5z4jz2Z2ifY2MsN3UggSHPMaSsaGwrOgttmLnIw6g
dns TXT custom-hostname: "c70e75c8-8887-4b4a-a390-72fd4a8400fd"
expected TXT custom-hostname: c70e75c8-8887-4b4a-a390-72fd4a8400fd
owner-only HTTP: HTTP/2 200
canonical HTTP: HTTP/2 200
pricing HTTP: HTTP/2 200
refund HTTP: HTTP/2 200
owner-only status: recheck owner-only deploy manually
expected domain status: active
canonical status: canonical host may be ready; verify release-status, pricing, and refund pages
blocker state: check release-status, pricing, and refund production URLs before moving blockers
```

Current Sites snapshot from Saturday, July 18, 2026:

```text
project status: active
current live URL: https://dropsquash.app
access mode: public
available access modes: custom, public
latest saved version: 3
latest saved version commit: 1017927b9c1ab26e4fd251ad81ffbd85fd0c88b5
custom domain refresh: dropsquash.app -> status active, provider_status active, ssl_status active
custom domain updated_at: 2026-07-18T06:23:16.868939+00:00
```

Interpretation:

- `active` means the Sites project itself is healthy
- the canonical host and public access condition are already satisfied
- do not close `Pricing finalized` or `Refund policy finalized` until the live
  pages stop serving stale draft-era copy

## Short Execution Memo

急ぎで deferred public proof だけ回収するときは、この順だけ守ります。

1. `cargo run -p xtask -- productization-status --track "Public web proof"`
2. `cargo run -p xtask -- public-web-ready`
3. 必要なら `cargo run -p xtask -- public-web-rerun`
4. dirty worktree のまま本番候補を更新しないなら `scripts/public-web-handoff.sh /tmp/dropsquash-public-web-$(git rev-parse --short HEAD)` で clean snapshot を切る
5. `cargo run -p xtask -- public-web-probe`
6. canonical host が HTTP 200 を返していることを確認する
7. production `https://dropsquash.app/release-status` / `pricing` / `refund` / live checkout URL を確認する
8. `docs/release-blockers.md` の `Public website deployment` / `Pricing finalized` / `Refund policy finalized` / `Live checkout link` を更新する
9. `cargo run -p xtask -- homebrew-cask-check packaging/homebrew/Casks/dropsquash.rb path/to/release-notes.md`
10. `docs/release-blockers.md` の `Published checksum` / `Homebrew cask install` を更新する
11. `cargo run -p xtask -- publish-check path/to/release-notes.md`
12. `cargo run -p xtask -- productization-status`

最低限の観測ポイント:

- `Public website deployment`: `https://dropsquash.app/release-status` で release-status / privacy / pricing / terms / license / support / download が見える
- `Pricing finalized`: `https://dropsquash.app/pricing` が最終価格で draft copy が残っていない
- `Refund policy finalized`: `https://dropsquash.app/refund` が最終版でリンクされている
- `Live checkout link`: pricing page から live `store.lemonsqueezy.com/checkout/buy/<id>` が開く
- canonical-host gate: `cargo run -p xtask -- public-web-probe` で canonical / pricing / refund が HTTP 200 を返している
- `Published checksum`: public GitHub Release に `SHA256SUMS` が付き、release notes Artifact URL と一致する public `DropSquash.dmg` の lowercase SHA-256 line が確認できる
- `Homebrew cask install`: public tap PR で versioned `DropSquash.dmg`, matching lowercase SHA-256, `brew uninstall --cask`, `auto_updates false`, `zap` が確認できる

Remaining proof map:

| Remaining blocker | Start with | Record in |
|---|---|---|
| Public website deployment | `cargo run -p xtask -- public-web-ready`, then verify `https://dropsquash.app/release-status` on the production host | `https://dropsquash.app/release-status` |
| Pricing finalized | `cargo run -p xtask -- public-web-ready`, then verify the final production pricing page at `https://dropsquash.app/pricing` | `https://dropsquash.app/pricing` |
| Refund policy finalized | `cargo run -p xtask -- public-web-ready`, then verify the final production refund policy at `https://dropsquash.app/refund` | `https://dropsquash.app/refund` |
| Live checkout link | `cargo run -p xtask -- public-web-ready`, then verify the public pricing page opens the live `store.lemonsqueezy.com/checkout/buy/<id>` URL | Live checkout URL |
| Published checksum | `cargo run -p xtask -- publish-check path/to/release-notes.md` after attaching `SHA256SUMS` to the public GitHub Release | GitHub Release URL |
| Homebrew cask install | `cargo run -p xtask -- homebrew-cask-check packaging/homebrew/Casks/dropsquash.rb path/to/release-notes.md` after opening the public tap PR and testing install/uninstall | Homebrew tap PR URL |

Treat this table as a routing map only. Source-of-truth completion evidence
still lives in
[docs/release-blockers.md](/Users/masakitakemura/_workspace/drop-squash/docs/release-blockers.md).

## Public Web Proof

Required URLs:

- `https://dropsquash.app/release-status`
- `https://dropsquash.app/pricing`
- `https://dropsquash.app/refund`

The production site must also expose and link:

- Privacy Policy
- Terms of Service
- Support
- License policy
- Download
- the live Lemon Squeezy checkout URL

Before recording those rows, verify:

```sh
cargo run -p xtask -- public-web-ready
```

Then deploy the static site to the real `dropsquash.app` host and verify the
production URLs directly in a browser. A local file preview, local static
server, or draft preview URL is not enough for these blocker rows.

Then update the public-web blockers in
[docs/release-blockers.md](/Users/masakitakemura/_workspace/drop-squash/docs/release-blockers.md):

- `Public website deployment`
- `Pricing finalized`
- `Refund policy finalized`
- `Live checkout link`

## Public Distribution Proof

Before recording the public distribution rows, the exact public
`DropSquash.dmg` must already be signed, notarized, stapled, and matched to the
release notes Artifact URL.

Then complete:

- GitHub Release with attached `SHA256SUMS` for the same public
  `DropSquash.dmg` matching the release notes Artifact URL
- Homebrew tap PR with the matching cask, versioned `DropSquash.dmg`
  Artifact URL, matching lowercase SHA-256, `brew uninstall --cask`,
  `auto_updates false`, and `zap`

Useful checks:

```sh
cargo run -p xtask -- publish-check path/to/release-notes.md
cargo run -p xtask -- homebrew-cask-check packaging/homebrew/Casks/dropsquash.rb path/to/release-notes.md
```

Use
[docs/release-notes-template.md](/Users/masakitakemura/_workspace/drop-squash/docs/release-notes-template.md)
when filling the final public Artifact URL, website URLs, checkout URL,
GitHub Release URL, and Homebrew tap PR URL so the release notes and blocker
evidence stay aligned.
Run `cargo run -p xtask -- publish-check path/to/release-notes.md` only after
the production URLs are verified, blocker rows are updated, release notes are
filled, and the public distribution evidence exists.

## Finish Line

Production payment onboarding or a public paid beta is ready only when all of
these are true:

- `paid-beta-check` has no remaining blockers
- `manual-qa-check` passes with the final public artifact evidence
- public website, pricing, refund, and checkout blockers are verified
- GitHub Release checksum and Homebrew cask blockers are verified

Do not start Stripe or Lemon Squeezy production onboarding before that point.
