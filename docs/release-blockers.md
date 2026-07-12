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
`GitHub Release` and `Homebrew tap PR` references must include the public URL.

| Blocker | Status | Completion evidence | Evidence reference | Record in |
|---|---|---|---|---|
| Packaged macOS manual QA | Blocked | Tested the public `DropSquash.dmg` artifact with the filled manual QA table and `manual-qa-check` passing | TBD | `docs/manual-qa.md` |
| Lemon Squeezy product setup | Blocked | Sandbox product is configured for DropSquash with license keys enabled | TBD | `docs/manual-qa.md` |
| Lemon Squeezy sandbox purchase | Blocked | Sandbox checkout completes with the intended product, test buyer, and order | TBD | `docs/manual-qa.md` |
| Empty key activation | Blocked | Activate stays disabled for empty input and raw key is absent from local cache | TBD | `docs/manual-qa.md` |
| Valid sandbox activation | Blocked | Activating state disables submit, app reaches Pro state, and raw key is absent from local cache | TBD | `docs/manual-qa.md` |
| Invalid license key handling | Blocked | Activating state disables submit, friendly error appears, and raw key is absent from local cache | TBD | `docs/manual-qa.md` |
| License network failure | Blocked | Friendly network error appears, existing valid local cache remains intact, and raw key is absent from local cache | TBD | `docs/manual-qa.md` |
| Local license forget | Blocked | Forgetting state disables action, local cache is removed, and app returns to trial or locked state | TBD | `docs/manual-qa.md` |
| Public website deployment | Blocked | Production website serves the release-status, privacy, pricing, support, and download pages | TBD | `https://...` |
| Refund policy finalized | Blocked | Production refund policy is final and linked before checkout goes live | TBD | `https://...` |
| Live checkout link | Blocked | Public pricing page opens the tested Lemon Squeezy checkout for the intended product | TBD | `https://...` |
| Signed DMG | Blocked | `codesign` verification shows Developer ID for the public `DropSquash.dmg` artifact | TBD | Release notes |
| Notarized and stapled DMG | Blocked | `spctl`, notary, and stapled evidence for the public `DropSquash.dmg` artifact | TBD | Release notes |
| Gatekeeper clean-machine open | Blocked | Fresh macOS account or clean machine opens the signed, notarized, stapled app without Gatekeeper warning | TBD | `docs/manual-qa.md` |
| Benchmark release set | Blocked | Release-set benchmark absolute CSV path outside repo is recorded and covers short, medium, and large local samples, smaller outputs, machine/OS context, and 20% regression threshold | TBD | `docs/manual-qa.md` |
| Published checksum | Blocked | SHA256SUMS with the SHA-256 line for public `DropSquash.dmg` is attached to the release | TBD | GitHub Release |
| Homebrew cask install | Blocked | `brew install --cask mt4110/tap/dropsquash` installs the versioned artifact `DropSquash.dmg` with matching SHA-256 and cask includes `auto_updates false` plus `zap` cleanup | TBD | Homebrew tap PR |

## Evidence Classes

Use this table to group the remaining release work without weakening the
blocker table. Every row still stays `Blocked` until its concrete evidence is
recorded in the location above. Do not use placeholder text such as `TBD`,
`TODO`, or `...` in `Next action` or `Evidence owner`.

| Blocker | Class | Next action | Evidence owner |
|---|---|---|---|
| Packaged macOS manual QA | Manual packaged-app | Run the public `DropSquash.dmg` artifact through the manual QA table and record `manual-qa-check` passing | `docs/manual-qa.md` |
| Lemon Squeezy product setup | License sandbox | Confirm the sandbox product is DropSquash and license keys are enabled | `docs/manual-qa.md` |
| Lemon Squeezy sandbox purchase | License sandbox | Complete sandbox checkout for the intended product, test buyer, and order | `docs/manual-qa.md` |
| Empty key activation | License sandbox | Leave the key empty, confirm Activate is disabled, and inspect the local license cache | `docs/manual-qa.md` |
| Valid sandbox activation | License sandbox | Activate the packaged app, confirm submit is disabled while activating, and inspect the local license cache | `docs/manual-qa.md` |
| Invalid license key handling | License sandbox | Enter an invalid key, confirm submit is disabled while activating, and inspect the local license cache | `docs/manual-qa.md` |
| License network failure | License sandbox | Simulate a failed activation request and inspect the friendly error plus preserved local cache | `docs/manual-qa.md` |
| Local license forget | License sandbox | Use the local forget action, confirm the action is disabled while forgetting, and inspect the returned app state | `docs/manual-qa.md` |
| Public website deployment | Public web | Deploy the production site and verify every public release page | Public website URL |
| Refund policy finalized | Public web | Publish the final refund policy before checkout goes live | Refund policy URL |
| Live checkout link | Public web | Verify the pricing page opens the live checkout for the product | Live checkout URL |
| Signed DMG | Signing/notarization | Sign the public `DropSquash.dmg` and capture Developer ID verification output | Release notes |
| Notarized and stapled DMG | Signing/notarization | Notarize, staple, and assess the public `DropSquash.dmg` with captured verification output | Release notes |
| Gatekeeper clean-machine open | Manual packaged-app | Open the signed, notarized, stapled app from a fresh macOS account or clean machine and confirm no Gatekeeper warning | `docs/manual-qa.md` |
| Benchmark release set | Benchmark | Run the release-set benchmark with short, medium, and large local recordings, record the absolute CSV path outside repo, and record threshold evidence | `docs/manual-qa.md` |
| Published checksum | Distribution | Attach SHA256SUMS containing the public `DropSquash.dmg` SHA-256 line to the GitHub Release | GitHub Release URL |
| Homebrew cask install | Distribution | Open the Homebrew tap PR and verify the cask install command, versioned `DropSquash.dmg` URL, matching SHA-256, `auto_updates false`, and `zap` cleanup path | Homebrew tap PR URL |
