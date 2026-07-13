# DropSquash Release Notes Template

Copy this into the GitHub Release after the public artifact is signed,
notarized, stapled, checked, and checksummed. Do not paste signing secrets,
license keys, private store IDs, or certificate material.

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

- Git commit: use the current lowercase 7-character or full 40-character commit hash.
- `codesign`: include `codesign`, `Developer ID`, `public`, and `DropSquash.dmg`.
- `spctl`: include `spctl`, `accepted`, `public`, and `DropSquash.dmg`.
- `stapler`: include `stapler` or `staple`, `stapled` or `validate`, `public`, and `DropSquash.dmg`.
- Apple notary log: include `notary` or `notarytool`, `accepted`, `public`, and `DropSquash.dmg`.
- Gatekeeper clean-machine open: include `Gatekeeper`, `opened` or `opens`, `clean` or `fresh`, `public`, `DropSquash.dmg`, `signed`, `notarized`, `stapled`, and `without warning` or `no warning`.
- `docs/release-blockers.md` status: mention `docs/release-blockers.md` and `all rows Verified`.
- Manual QA record: mention `docs/manual-qa.md`, the tested public `DropSquash.dmg`, and `manual-qa-check` passing.
- Conversion safety evidence: mention cancellation, failed conversion, larger/not-smaller output, original preservation, and unchanged trial count.
- Queue evidence: mention multi-file queue, queued cancellation, batch summary, trial or license lock blocked jobs, finished count, saved bytes, and failed/cancelled/blocked counts.
- Trash source policy: mention Moving original or moving state, disabled action state, verified smaller output, and Trash.
- Benchmark sample set: mention short, medium, and large samples, smaller outputs, backend, saved percent, duration, speed ratio, CSV path outside the repository, machine, and OS context.
- Benchmark regression threshold: mention the 20% regression threshold, whether any sample exceeded it, the same-machine comparison, and the release candidate baseline.
- Lemon Squeezy product setup: mention DropSquash, the sandbox product, the intended product, license keys enabled, and private store IDs absent or not recorded.
- Lemon Squeezy sandbox purchase: mention the sandbox checkout, intended product, `test buyer`, and `order`.
- Valid sandbox activation: mention the Lemon Squeezy sandbox request, disabled action state, Pro state, 64-character lowercase hex fingerprint, `instance_id`, `raw key`, and cache evidence.
- Empty key activation: mention Activate disabled for empty input, `raw key`, no fingerprint, no instance, and cache evidence.
- Invalid license key handling: mention disabled action state, friendly error, `raw key`, no fingerprint, no instance, and cache evidence.
- License network failure: mention a friendly network error, preserved existing valid cache, 64-character lowercase hex fingerprint, `instance_id`, and `raw key` absence.
- Expired license refresh: mention the expired offline grace cache, reconnect prompt, conversion blocked before starting, `raw key` absence, and cache evidence.
- Local license forget: mention disabled action state, cache removal, and trial or locked state.
- Public website URL: use the production `/release-status` URL.
- Refund policy URL: use the production `/refund` URL after the policy is final.
- Live checkout URL: use the Lemon Squeezy `/checkout/buy/` URL for the product.
- GitHub Release checksum: mention `SHA256SUMS` or `SHA-256`, `DropSquash.dmg`, that it is attached, the GitHub Release URL above, the Artifact URL above, and the exact lowercase SHA-256 digest above.
- Homebrew tap PR: mention the cask, PR, the Homebrew tap PR URL above, versioned `DropSquash.dmg`, the Artifact URL above, the exact lowercase SHA-256 digest above, `auto_updates false`, and `zap` cleanup path.
- Homebrew install result: mention `brew install --cask mt4110/tap/dropsquash`, the versioned `DropSquash.dmg` artifact, and the exact lowercase SHA-256 digest above.
- Known limitations: mention the macOS MVP and unreleased Windows/Linux platform builds.
- Support contact: mention support through GitHub Issues or a support email address.
