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

- `codesign`: include `codesign`, `Developer ID`, and `DropSquash.dmg`.
- `spctl`: include `spctl`, `accepted`, and `DropSquash.dmg`.
- `stapler`: include `stapler` or `staple`, `stapled` or `validate`, and `DropSquash.dmg`.
- Apple notary log: include `notary` or `notarytool`, `accepted`, and `DropSquash.dmg`.
- Gatekeeper clean-machine open: include `Gatekeeper`, `opened` or `opens`, `clean` or `fresh`, `signed`, `notarized`, `stapled`, and `without warning` or `no warning`.
- `docs/release-blockers.md` status: mention `docs/release-blockers.md` and `all rows Verified`.
- Manual QA record: mention `docs/manual-qa.md` and the tested public `DropSquash.dmg`.
- Conversion safety evidence: mention cancellation, failed conversion, larger output, original preservation, and unchanged trial count.
- Queue evidence: mention multi-file queue, queued cancellation, batch summary, finished count, saved bytes, and failed/cancelled/blocked counts.
- Trash source policy: mention Moving original or moving state, disabled action state, verified smaller output, and Trash.
- Benchmark sample set: mention short, medium, and large samples, smaller outputs, machine, and OS context.
- Lemon Squeezy product setup: mention DropSquash, the intended product, and license keys enabled without private store IDs.
- Lemon Squeezy sandbox purchase: mention the intended product, `test buyer`, and `order`.
- Valid sandbox activation: mention the Lemon Squeezy sandbox request, disabled action state, Pro state, `raw key`, and cache evidence.
- Empty key activation: mention Activate disabled for empty input, `raw key`, and cache evidence.
- Invalid license key handling: mention disabled action state, friendly error, `raw key`, and cache evidence.
- License network failure: mention a friendly network error, preserved existing valid cache, and `raw key` absence.
- Local license forget: mention disabled action state, cache removal, and trial or locked state.
- Public website URL: use the production `/release-status` URL.
- Refund policy URL: use the production `/refund` URL after the policy is final.
- Live checkout URL: use the Lemon Squeezy `/checkout/buy/` URL for the product.
- GitHub Release checksum: mention `SHA256SUMS` or `SHA-256`, `DropSquash.dmg`, that it is attached, the Artifact URL above, and the exact SHA-256 digest above.
- Homebrew tap PR: mention the cask, PR, `DropSquash.dmg`, the Artifact URL above, `auto_updates false`, and `zap` cleanup path.
- Homebrew install result: mention `brew install --cask mt4110/tap/dropsquash` and `DropSquash.dmg`.
- Known limitations: mention the macOS MVP and unreleased Windows/Linux platform builds.
- Support contact: mention support through GitHub Issues or a support email address.
