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
| Packaged macOS manual QA | Blocked | Filled manual QA table for the exact `.app` or `.dmg` artifact | TBD | `docs/manual-qa.md` |
| Lemon Squeezy product setup | Blocked | Sandbox product is configured for DropSquash with license keys enabled | TBD | `docs/manual-qa.md` |
| Lemon Squeezy sandbox purchase | Blocked | Sandbox checkout completes with the intended product, test buyer, and order | TBD | `docs/manual-qa.md` |
| Valid sandbox activation | Blocked | App reaches Pro state and raw key is absent from local cache | TBD | `docs/manual-qa.md` |
| Invalid license key handling | Blocked | Friendly error appears and raw key is absent from local cache | TBD | `docs/manual-qa.md` |
| Local license forget | Blocked | Local cache is removed and app returns to trial or locked state | TBD | `docs/manual-qa.md` |
| Public website deployment | Blocked | Production website serves the release-status, privacy, pricing, support, and download pages | TBD | `https://...` |
| Refund policy finalized | Blocked | Production refund policy is final and linked before checkout goes live | TBD | `https://...` |
| Live checkout link | Blocked | Public pricing page opens the tested Lemon Squeezy checkout for the intended product | TBD | `https://...` |
| Signed DMG | Blocked | `codesign` verification shows Developer ID for the public DMG artifact | TBD | Release notes |
| Notarized and stapled DMG | Blocked | `spctl`, notary, and stapled evidence for the public DMG artifact | TBD | Release notes |
| Gatekeeper clean-machine open | Blocked | Fresh macOS account or clean machine opens the stapled app | TBD | `docs/manual-qa.md` |
| Published checksum | Blocked | SHA-256 line for the public DMG is attached to the release | TBD | GitHub Release |
| Homebrew cask install | Blocked | `brew install --cask mt4110/tap/dropsquash` installs the versioned artifact and cask includes `zap` cleanup | TBD | Homebrew tap PR |

## Evidence Classes

Use this table to group the remaining release work without weakening the
blocker table. Every row still stays `Blocked` until its concrete evidence is
recorded in the location above.

| Blocker | Class | Next action | Evidence owner |
|---|---|---|---|
| Packaged macOS manual QA | Manual packaged-app | Run the packaged artifact through the manual QA table | `docs/manual-qa.md` |
| Lemon Squeezy product setup | License sandbox | Confirm the sandbox product is DropSquash and license keys are enabled | `docs/manual-qa.md` |
| Lemon Squeezy sandbox purchase | License sandbox | Complete sandbox checkout for the intended product, test buyer, and order | `docs/manual-qa.md` |
| Valid sandbox activation | License sandbox | Activate the packaged app and inspect the local license cache | `docs/manual-qa.md` |
| Invalid license key handling | License sandbox | Enter an invalid key and inspect the local license cache | `docs/manual-qa.md` |
| Local license forget | License sandbox | Use the local forget action and inspect the returned app state | `docs/manual-qa.md` |
| Public website deployment | Public web | Deploy the production site and verify every public release page | Public website URL |
| Refund policy finalized | Public web | Publish the final refund policy before checkout goes live | Refund policy URL |
| Live checkout link | Public web | Verify the pricing page opens the live checkout for the product | Public website URL |
| Signed DMG | Signing/notarization | Sign the public DMG and capture Developer ID verification output | Release notes |
| Notarized and stapled DMG | Signing/notarization | Notarize, staple, and assess the public DMG with captured verification output | Release notes |
| Gatekeeper clean-machine open | Manual packaged-app | Open the stapled app from a fresh macOS account or clean machine | `docs/manual-qa.md` |
| Published checksum | Distribution | Attach SHA256SUMS to the public GitHub Release | GitHub Release URL |
| Homebrew cask install | Distribution | Open the Homebrew tap PR and verify the cask install command plus `zap` cleanup path | Homebrew tap PR URL |
