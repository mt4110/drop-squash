# Release Blockers

Do not publish a paid beta until every row has concrete evidence in the named
location. Keep secrets, license keys, certificates, and private store IDs out of
this file.
Use `Blocked` until the evidence exists. Change a row to `Verified` only after
the evidence is recorded in the named location.

| Blocker | Status | Completion evidence | Evidence reference | Record in |
|---|---|---|---|---|
| Packaged macOS manual QA | Blocked | Filled manual QA table for the exact `.app` or `.dmg` artifact | TBD | `docs/manual-qa.md` |
| Lemon Squeezy sandbox purchase | Blocked | Sandbox checkout completes with the intended product and test buyer | TBD | `docs/manual-qa.md` |
| Valid sandbox activation | Blocked | App reaches Pro state and raw key is absent from local cache | TBD | `docs/manual-qa.md` |
| Signed DMG | Blocked | `codesign` verification for the public DMG artifact | TBD | Release notes |
| Notarized and stapled DMG | Blocked | `spctl`/notary evidence for the public DMG artifact | TBD | Release notes |
| Gatekeeper clean-machine open | Blocked | Fresh macOS account or clean machine opens the stapled app | TBD | `docs/manual-qa.md` |
| Published checksum | Blocked | SHA-256 line for the public DMG is attached to the release | TBD | GitHub Release |
| Homebrew cask install | Blocked | `brew install --cask mt4110/tap/dropsquash` installs the versioned artifact | TBD | Homebrew tap PR |
