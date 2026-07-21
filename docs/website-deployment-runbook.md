# Website Deployment Runbook

Use this when the static `website/` files are ready and the next job is to make
the production `dropsquash.app` URLs real.

This runbook is only for the public web proof:

- `Public website deployment`
- `Pricing finalized`
- `Refund policy finalized`
- `Live checkout link`

Do not mark any of those blockers `Verified` from a local preview, draft host,
or repository file path.

## Preconditions

- `cargo run -p xtask -- website-check` passes
- the content in `website/` matches the current app behavior
- the release path still says public download and checkout are blocked until the
  signed/notarized release and checkout proof exist
- the intended production host is `https://dropsquash.app`

Current repository note:

- this repository now includes `.openai/hosting.json`
- the Sites project now serves the canonical public host
  `https://dropsquash.app`
- Sites version `3` is now saved from commit
  `1017927b9c1ab26e4fd251ad81ffbd85fd0c88b5`
- that saved version has id
  `appgprj_6a5a4a7d557c8191ac7381f80c671354~appgver_f4d9d0220ef081919a8e2dfb1cd789b2`
- the current live site still reflects an older saved version until version `3`
  is explicitly deployed
- the Sites project access mode is now `public`
- the `dropsquash.app` custom domain is active
- Sites packaging requires a deployable build with `dist/server/index.js` and
  `dist/.openai/hosting.json`
- `apps/site` is now the deployable Sites surface; `website/` remains the
  static draft and copy reference
- blocker rows still stay open until a real deployed production URL exists and
  is manually verified

## Short Execution Memo

1. run `cargo run -p xtask -- website-check`
2. run `cargo run -p xtask -- public-web-ready`
3. run `cargo run -p xtask -- public-web-rerun` if you want the current local public-web sequence in one place
4. if the intended site source is not already isolated and committed, run `scripts/public-web-handoff.sh /tmp/dropsquash-public-web-$(git rev-parse --short HEAD)` and continue from that snapshot
5. run `cargo run -p xtask -- public-web-probe`
6. if the probe does not return HTTP 200 for canonical / pricing / refund, stop and fix the canonical host before claiming blocker evidence
7. save and deploy the intended Sites version for the real `dropsquash.app` host
8. confirm the site is still public on the canonical host after deployment
9. open the production URLs in a browser
10. confirm page-to-page links work on the production host
11. confirm pricing has no draft copy before live checkout goes public
12. confirm refund is final and linked
13. confirm checkout is still non-live until the real
   `store.lemonsqueezy.com/checkout/buy/<id>` URL exists
14. record the real production URLs in `docs/release-blockers.md`
15. rerun `cargo run -p xtask -- productization-status`
16. run `cargo run -p xtask -- publish-check path/to/release-notes.md` only after public release notes and blocker evidence are ready

Current Sites sequencing note:

- the Sites connector rejects `dropsquash.app` custom-domain attachment until
  the Site has already been published
- finish the first publish, confirm the deployed URL exists, then add the
  custom domain and DNS records
- as of Saturday, July 18, 2026, the canonical host step is complete
- the remaining public-web blockers are production copy alignment on pricing and
  refund plus the later live checkout proof

## Packaging Gate

Before trying to save a Sites version, confirm the site source can produce all
required deployment artifacts:

- `dist/server/index.js`
- `dist/.openai/hosting.json`

Current build target:

- `apps/site`
- preferred local verification command: `npm run verify:site`
- final public-release gate after blocker evidence and release notes exist: `cargo run -p xtask -- publish-check path/to/release-notes.md`

If those outputs do not exist yet, do not treat the Sites project as deployable.
Finish the hosting surface first, then save a version, then deploy.

## Required Production URLs

- `https://dropsquash.app/release-status`
- `https://dropsquash.app/pricing`
- `https://dropsquash.app/refund`

The production site must also expose:

- Privacy Policy
- Terms of Service
- Support
- License policy
- Download

## Evidence Rules

Use these exact evidence shapes:

- `Public website deployment`:
  `https://dropsquash.app/release-status`
- `Pricing finalized`:
  `https://dropsquash.app/pricing`
- `Refund policy finalized`:
  `https://dropsquash.app/refund`
- `Live checkout link`:
  the final `https://store.lemonsqueezy.com/checkout/buy/<id>` URL

Do not use:

- preview URLs
- localhost URLs
- owner-only or access-controlled Sites URLs
- query-string tracking links
- fragment URLs
- copied screenshots instead of the real URL

Owner-only preflight evidence is still useful before DNS completes:

- the current owner-only live URL
  `https://dropsquash-app.system-obj-gg.chatgpt.site`
  remains useful as provenance for older saves
- canonical-host blocker evidence must still come from `https://dropsquash.app/...`

## Production Browser Check

Confirm all of these on the real host:

1. `release-status` links pricing, privacy, terms, license, support, refund,
   and download
2. `pricing` matches the current app promise and trial count
3. `pricing` does not leave draft-price wording in place once checkout is live
4. `refund` is final and linked from pricing or the site navigation
5. `download` still matches the real release state
6. no page claims Windows/Linux public support that does not exist

## DNS Records For `dropsquash.app`

Target records for the custom domain:

- A `dropsquash.app` -> `162.159.143.30`
- A `dropsquash.app` -> `172.66.3.26`
- TXT `_openai-site-verification.dropsquash.app` ->
  `openai-site-verification=1B5z4jz2Z2ifY2MsN3UggSHPMaSsaGwrOgttmLnIw6g`
- TXT `_cf-custom-hostname.dropsquash.app` ->
  `c70e75c8-8887-4b4a-a390-72fd4a8400fd`

Refresh the custom-domain status after DNS changes before claiming production
`https://dropsquash.app/...` evidence.

Current status interpretation as of Saturday, July 18, 2026:

- the current Sites project id is
  `appgprj_6a5a4a7d557c8191ac7381f80c671354`
- the current custom-domain id is
  `appgdom_6a5a4f5e5f608191871669316293988c`
- the Sites custom-domain status now refreshes as `status: active` with
  `ssl_status: active`
- the latest refresh updated the custom-domain record at
  `2026-07-18T06:23:16.868939+00:00` with `provider_status: active` and no
  `last_error`
- `cargo run -p xtask -- public-web-probe` currently reports:
  - `dns A: 172.67.131.70`
  - `dns TXT verification: "openai-site-verification=1B5z4jz2Z2ifY2MsN3UggSHPMaSsaGwrOgttmLnIw6g"`
  - `dns TXT custom-hostname: "c70e75c8-8887-4b4a-a390-72fd4a8400fd"`
  - `owner-only HTTP: HTTP/2 200`
  - `canonical HTTP: HTTP/2 200`
- move the public-web blockers only after the live pages themselves match the
  intended product copy

Current deployed-version note as of Saturday, July 18, 2026:

- Sites version `3` is now the latest saved version
- that saved version points to commit
  `1017927b9c1ab26e4fd251ad81ffbd85fd0c88b5`
- the previous saved version `2` pointed to commit
  `91dc159340da4456e18a4b75e9250e192c0233a2`
- the current local repository `HEAD` is
  `5a69fea9240adfddff77425c83faf72ea3b61751`
- because the saved Sites version commit now differs from the local repository
  `HEAD`, do not assume the canonical host reflects the newest local site edits
  until version `3` is explicitly deployed
- the current worktree is also dirty, so do not save or deploy a new Sites
  version until the intended source state is isolated and committed
- if the local repository HEAD is newer than the saved Sites version commit,
  do not assume the canonical host reflects the newest local site edits until a
  new version is saved and deployed intentionally

## Current Probe Snapshot

As of Saturday, July 18, 2026, `cargo run -p xtask -- public-web-probe`
returned this exact shape:

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
blocker state: keep pricing/refund blocked until the live pages match the intended copy
```

Interpretation:

- the canonical host is active and reachable
- `Public website deployment` can be verified from the canonical host
- `Pricing finalized`, `Refund policy finalized`, and `Live checkout link`
  still stay blocked until the live pages match the intended copy and checkout
  state

## After Deployment

Update:

- [docs/release-blockers.md](/Users/masakitakemura/_workspace/drop-squash/docs/release-blockers.md)
- [docs/release-notes-template.md](/Users/masakitakemura/_workspace/drop-squash/docs/release-notes-template.md)
- [docs/public-beta-operator-checklist.md](/Users/masakitakemura/_workspace/drop-squash/docs/public-beta-operator-checklist.md)

Then continue to the distribution proof for:

- published checksum
- GitHub Release URL
- Homebrew tap PR URL
