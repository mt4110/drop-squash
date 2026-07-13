# Release Blockers

Do not publish a paid beta until every row has concrete evidence in the named
location. Keep secrets, license keys, certificates, and private store IDs out of
this file.
Use `Blocked` until the evidence exists. Change a row to `Verified` only after
the evidence is recorded in the named location.
Do not leave a row `Blocked` after adding an `Evidence reference`; update the
status and keep the reference traceable.
For `Verified` rows, `Evidence reference` must point to `docs/...`, an
`https://...` URL, `Release notes`, `GitHub Release`, or `Homebrew tap PR`.
Public website and refund references must use the production `dropsquash.app`
host.
`GitHub Release` and `Homebrew tap PR` references must include the public URL.
Use the exact forms `GitHub Release https://...` and
`Homebrew tap PR https://...` without extra words in the Evidence reference.

| Blocker | Status | Completion evidence | Evidence reference | Record in |
|---|---|---|---|---|
| Packaged macOS manual QA | Blocked | Tested the public `DropSquash.dmg` artifact with the filled manual QA table and `manual-qa-check` passing | TBD | `docs/manual-qa.md` |
| Lemon Squeezy product setup | Blocked | Sandbox product is configured for the intended product, DropSquash, with license keys enabled and private store IDs not recorded | TBD | `docs/manual-qa.md` |
| Lemon Squeezy sandbox purchase | Blocked | Sandbox checkout completes with the intended product, test buyer, and order | TBD | `docs/manual-qa.md` |
| Empty key activation | Blocked | Activate stays disabled for empty input, and raw key, fingerprint, and instance are absent from local cache | TBD | `docs/manual-qa.md` |
| Valid sandbox activation | Blocked | Lemon Squeezy sandbox activation reaches Pro state, Activating state disables submit, 64-character lowercase hex fingerprint and `instance_id` fields are present, and raw key is absent from local cache | TBD | `docs/manual-qa.md` |
| Invalid license key handling | Blocked | Activating state disables submit, friendly error appears, and raw key, fingerprint, and instance are absent from local cache | TBD | `docs/manual-qa.md` |
| License network failure | Blocked | Friendly network error appears, existing valid local cache with 64-character lowercase hex fingerprint and `instance_id` fields remains intact, and raw key is absent from local cache | TBD | `docs/manual-qa.md` |
| Expired license refresh | Blocked | Expired offline grace cache shows reconnect prompt, conversion is blocked before starting, and raw key is absent from local cache | TBD | `docs/manual-qa.md` |
| Local license forget | Blocked | Forgetting state disables action, local cache is removed, and app returns to trial or locked state | TBD | `docs/manual-qa.md` |
| Public website deployment | Blocked | Production website production URL serves the release-status, privacy, pricing, support, and download pages on `dropsquash.app` | TBD | `https://...` |
| Refund policy finalized | Blocked | Production refund policy is final on `dropsquash.app` and linked before checkout goes live | TBD | `https://...` |
| Live checkout link | Blocked | Public pricing page opens the live checkout URL for the tested Lemon Squeezy checkout for the intended product | TBD | `https://...` |
| Signed DMG | Blocked | `codesign` verification shows Developer ID for the public `DropSquash.dmg` artifact | TBD | Release notes |
| Notarized and stapled DMG | Blocked | `spctl`, notary, and stapled evidence for the public `DropSquash.dmg` artifact | TBD | Release notes |
| Gatekeeper clean-machine open | Blocked | Fresh macOS account or clean machine opens the signed, notarized, stapled app from public `DropSquash.dmg` without Gatekeeper warning | TBD | `docs/manual-qa.md` |
| Benchmark release set | Blocked | Release-set benchmark absolute CSV path outside repo is recorded and covers backend, saved percent, duration, speed ratio, short, medium, and large local samples, smaller outputs, machine/OS context, 20% regression threshold, and release candidate baseline | TBD | `docs/manual-qa.md` |
| Published checksum | Blocked | SHA256SUMS with the lowercase SHA-256 line for public `DropSquash.dmg` is attached to the GitHub Release | TBD | GitHub Release |
| Homebrew cask install | Blocked | `brew install --cask mt4110/tap/dropsquash` installs the versioned artifact `DropSquash.dmg` with matching lowercase SHA-256 and cask includes `auto_updates false` plus `zap` cleanup | TBD | Homebrew tap PR |

## Evidence Classes

Use this table to group the remaining release work without weakening the
blocker table. Every row still stays `Blocked` until its concrete evidence is
recorded in the location above. Do not use placeholder text such as `TBD`,
`TODO`, or `...` in `Next action` or `Evidence owner`.

| Blocker | Class | Next action | Evidence owner |
|---|---|---|---|
| Packaged macOS manual QA | Manual packaged-app | Run the public `DropSquash.dmg` artifact through the manual QA table and record `manual-qa-check` passing | `docs/manual-qa.md` |
| Lemon Squeezy product setup | License sandbox | Confirm the sandbox product is the intended product, DropSquash, license keys are enabled, and private store IDs are not recorded | `docs/manual-qa.md` |
| Lemon Squeezy sandbox purchase | License sandbox | Complete sandbox checkout for the intended product, test buyer, and order | `docs/manual-qa.md` |
| Empty key activation | License sandbox | Leave the key empty, confirm Activate is disabled, and inspect local license cache raw-key, fingerprint, and instance absence | `docs/manual-qa.md` |
| Valid sandbox activation | License sandbox | Run the Lemon Squeezy sandbox activation request, confirm submit is disabled while Activating, and inspect local license cache 64-character lowercase hex fingerprint, `instance_id`, plus raw-key absence | `docs/manual-qa.md` |
| Invalid license key handling | License sandbox | Enter an invalid key, confirm submit is disabled while activating, friendly error appears, and inspect local license cache raw-key, fingerprint, and instance absence | `docs/manual-qa.md` |
| License network failure | License sandbox | Simulate a failed activation request and inspect the friendly error plus preserved local cache 64-character lowercase hex fingerprint, `instance_id`, and raw-key absence | `docs/manual-qa.md` |
| Expired license refresh | License sandbox | Seed the expired offline grace cache, confirm the reconnect prompt, confirm conversion is blocked before starting, and inspect local license cache raw-key absence | `docs/manual-qa.md` |
| Local license forget | License sandbox | Use the local forget action, confirm the action is disabled while forgetting, and inspect the returned app state | `docs/manual-qa.md` |
| Public website deployment | Public web | Deploy the production site and verify the production `dropsquash.app` URL serves release-status, privacy, pricing, support, and download pages | Public website URL |
| Refund policy finalized | Public web | Publish the final refund policy URL on `dropsquash.app` and confirm it is linked before checkout goes live | Refund policy URL |
| Live checkout link | Public web | Verify the public pricing page opens the live checkout URL for the tested Lemon Squeezy checkout for the intended product | Live checkout URL |
| Signed DMG | Signing/notarization | Sign the public `DropSquash.dmg` and capture `codesign` Developer ID verification output | Release notes |
| Notarized and stapled DMG | Signing/notarization | Notarize, staple, and assess the public `DropSquash.dmg` with captured `spctl`, notary, and stapler verification output | Release notes |
| Gatekeeper clean-machine open | Manual packaged-app | Open the signed, notarized, stapled app from public `DropSquash.dmg` in a fresh macOS account or clean machine and confirm no Gatekeeper warning | `docs/manual-qa.md` |
| Benchmark release set | Benchmark | Run the release-set benchmark with short, medium, and large local recordings, record backend, saved percent, duration, speed ratio, the absolute CSV path outside repo, and threshold evidence | `docs/manual-qa.md` |
| Published checksum | Distribution | Attach SHA256SUMS containing the public `DropSquash.dmg` lowercase SHA-256 line to the GitHub Release | GitHub Release URL |
| Homebrew cask install | Distribution | Open the Homebrew tap PR and verify `brew install --cask`, versioned `DropSquash.dmg` URL, matching lowercase SHA-256, `auto_updates false`, and `zap` cleanup path | Homebrew tap PR URL |

## Execution Order

Use this order to finish the blockers without treating external evidence as a
local code task. Keep each blocker `Blocked` until the matching concrete
evidence is recorded in the blocker table.

| Order | Track | Blockers | Exit condition | Record target |
|---:|---|---|---|---|
| 1 | Local packaged-app proof | Packaged macOS manual QA, Benchmark release set | Public `DropSquash.dmg`, filled manual QA rows, release-set CSV outside the repo, and `manual-qa-check` passing | `docs/manual-qa.md` |
| 2 | License sandbox proof | Lemon Squeezy product setup, Lemon Squeezy sandbox purchase, Empty key activation, Valid sandbox activation, Invalid license key handling, License network failure, Expired license refresh, Local license forget | Sandbox purchase/activation behavior, friendly failures, raw-key absence, fingerprint/instance checks, and forget behavior | `docs/manual-qa.md` |
| 3 | Public web proof | Public website deployment, Refund policy finalized, Live checkout link | Production `dropsquash.app` URLs serve the required pages and checkout/refund links before the public beta | Production website URLs |
| 4 | Signing and distribution proof | Signed DMG, Notarized and stapled DMG, Gatekeeper clean-machine open, Published checksum, Homebrew cask install | Release notes, GitHub Release, Homebrew tap PR, and `docs/manual-qa.md` prove the same public `DropSquash.dmg` is signed, notarized, checksummed, installable, and opens without warning | Release notes and public distribution URLs |
