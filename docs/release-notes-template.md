# DropSquash Release Notes Template

Copy this into the GitHub Release after the public artifact is signed,
notarized, stapled, checked, and checksummed. Do not paste signing secrets,
license keys, private store IDs, or certificate material.
Do not copy preflight wording from local unsigned QA artifacts into this file.
Every artifact, verification, checksum, and Gatekeeper line here must describe
the same signed public `DropSquash.dmg` and its public Artifact URL.

## Artifact

- Version:
- Artifact: DropSquash.dmg
- Artifact URL:
- SHA-256:
- Git commit:

## macOS Verification

- `codesign`:
- `spctl`:
- `stapler`:
- Apple notary log:
- Gatekeeper clean-machine open:

Example fill shape:

```text
- `codesign`: codesign --verify --deep --strict --verbose=4 and codesign -dv --verbose=4 confirmed Developer ID for the public DropSquash.dmg at <Artifact URL>.
- `spctl`: spctl --assess --type open --verbose=4 returned accepted for the public DropSquash.dmg at <Artifact URL>.
- `stapler`: xcrun stapler validate confirmed stapled status for the public DropSquash.dmg at <Artifact URL>.
- Apple notary log: xcrun notarytool log showed accepted for the public DropSquash.dmg at <Artifact URL>.
- Gatekeeper clean-machine open: Gatekeeper opened the fresh signed, notarized, stapled public DropSquash.dmg at <Artifact URL> without warning.
```

## Productization Evidence

- `docs/release-blockers.md` status:
- Manual QA record:
- Conversion safety evidence:
- Queue evidence:
- Trash source policy:
- Benchmark sample set:
- Benchmark regression threshold:
- Lemon Squeezy product setup:
- Lemon Squeezy sandbox purchase:
- Valid sandbox activation:
- Empty key activation:
- Invalid license key handling:
- License network failure:
- Expired license refresh:
- Local license forget:
- Public website URL:
- Pricing URL:
- Refund policy URL:
- Live checkout URL:

## Distribution

- GitHub Release checksum:
- GitHub Release URL:
- Homebrew tap PR:
- Homebrew tap PR URL:
- Homebrew install result:

## Notes

- Known limitations:
- Support contact:

## Evidence Wording Checklist

Use concrete wording in each filled field so `release-notes-check` can reject
weak public evidence before the release is published.
Before publish, fill every required URL field: Artifact URL, Public website URL, Pricing URL, Refund policy URL,
Live checkout URL, GitHub Release URL, and Homebrew tap PR URL.
When those public URLs and distribution references are still being assembled
after the private/manual paid beta technical proof, use
`cargo run -p xtask -- public-web-ready`, then
[docs/public-beta-operator-checklist.md](/Users/masakitakemura/_workspace/drop-squash/docs/public-beta-operator-checklist.md)
as the shorter operator sequence for the deferred public-proof phase.
Use
[docs/website-deployment-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/website-deployment-runbook.md)
when filling the production `dropsquash.app` URL fields so the evidence comes
from the real host rather than a local preview.
Use `release-notes-prepare` to generate the macOS verification command drafts
for the same checked DMG:

- `codesign --verify --deep --strict --verbose=4`
- `codesign -dv --verbose=4`
- `spctl --assess --type open --verbose=4`
- `xcrun stapler validate`

- Git commit: use the current lowercase 7-character or full 40-character commit hash.
- `codesign`: include `codesign --verify`, `codesign -dv`, `Developer ID`, `public`, and the exact Artifact URL.
- `spctl`: include `spctl --assess --type open`, `accepted`, `public`, and the exact Artifact URL.
- `stapler`: include `stapler` or `staple`, `stapled` or `validate`, confirm stapled status, `public`, and the exact Artifact URL.
- Apple notary log: include `notarytool`, `accepted`, `log`, `public`, and the exact Artifact URL.
- Gatekeeper clean-machine open: include `Gatekeeper`, `opened` or `opens`, `clean` or `fresh`, `public`, the exact Artifact URL, `signed`, `notarized`, `stapled`, and `without warning` or `no warning`.
- `docs/release-blockers.md` status: mention `docs/release-blockers.md` and `all rows Verified`.
- Manual QA record: mention `docs/manual-qa.md`, the tested exact Artifact URL, and `manual-qa-check` passing.
- Conversion safety evidence: mention cancellation returning ready after temp cleanup, failed conversion, larger/not-smaller kept-original result, original preservation, unchanged trial count, history, and no new success.
- Queue evidence: mention multi-file queue, waiting or queued row cancellation, batch summary, trial or license lock blocked jobs, finished count, saved bytes, and failed/cancelled/blocked counts.
- Trash source policy: mention Moving original or moving state, disabled action state, verified smaller output, and Trash.
- Benchmark sample set: mention three short, medium, and large original local recordings, smaller outputs, backend, saved percent, duration, speed ratio, existing CSV path outside the repository, machine, and OS context.
- Benchmark regression threshold: mention the 20% regression threshold, whether any sample exceeded it, the same-machine comparison, and the release candidate baseline.
- Lemon Squeezy product setup: mention DropSquash, the sandbox product, the intended product, license keys enabled, and private store IDs absent or not recorded.
- Lemon Squeezy sandbox purchase: mention the sandbox checkout, intended product, `test buyer`, and concrete order id or order number.
- Valid sandbox activation: mention the Lemon Squeezy sandbox request, disabled action state, Pro state, checked cache evidence, 64-character lowercase hex fingerprint, `instance_id`, and `raw key` absence.
- Empty key activation: mention Activate disabled for empty input, checked cache evidence, `raw key`, no fingerprint, and no instance.
- Invalid license key handling: mention disabled action state, friendly error, inspected cache evidence, `raw key`, no fingerprint, and no instance.
- License network failure: mention a friendly network error, checked preserved existing valid cache, 64-character lowercase hex fingerprint, `instance_id`, and `raw key` absence.
- Expired license refresh: mention the attempted conversion, expired offline grace cache, reconnect prompt, conversion blocked before starting, checked cache evidence, and `raw key` absence.
- Local license forget: mention disabled action state, confirmed cache removal, and observed trial or locked state.
- Public website URL: use the production `https://dropsquash.app/release-status` URL after that page links release-status, privacy, pricing, terms, license, support, and download.
- Pricing URL: use the production `https://dropsquash.app/pricing` URL after the price is final and draft price copy is removed.
- Refund policy URL: use the production `https://dropsquash.app/refund` URL after the policy is final.
- Do not fill Public website URL, Pricing URL, or Refund policy URL with the
  owner-only Sites live URL `https://dropsquash-app.system-obj-gg.chatgpt.site`;
  that URL is useful for deployment preflight only and does not satisfy the
  canonical `dropsquash.app` blocker evidence.
- Live checkout URL: use the `https://store.lemonsqueezy.com/checkout/buy/<id>` URL for the product.
- GitHub Release checksum: mention `SHA256SUMS` or `SHA-256`, public `DropSquash.dmg`, attached to the GitHub Release URL above, the Artifact URL above, and the exact lowercase SHA-256 digest above.
- Homebrew tap PR: mention the cask, public PR, the Homebrew tap PR URL above, versioned `DropSquash.dmg`, the Artifact URL above, the exact lowercase SHA-256 digest above, `auto_updates false`, and `zap` cleanup path.
- Homebrew install result: mention `brew install --cask mt4110/tap/dropsquash`, `brew uninstall --cask mt4110/tap/dropsquash`, removes it cleanly, the Homebrew tap PR URL above, the versioned `DropSquash.dmg` artifact, the Artifact URL above, and the exact lowercase SHA-256 digest above.
- Known limitations: mention the macOS MVP and unreleased Windows/Linux platform builds.
- Support contact: mention support through GitHub Issues or a support email address.
