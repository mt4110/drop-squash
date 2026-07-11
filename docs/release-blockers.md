# Release Blockers

Do not publish a paid beta until every row has concrete evidence in the named
location. Keep secrets, license keys, certificates, and private store IDs out of
this file.

| Blocker | Status | Completion evidence | Record in |
|---|---|---|---|
| Packaged macOS manual QA | Blocked | Filled manual QA table for the exact `.app` or `.dmg` artifact | `docs/manual-qa.md` |
| Lemon Squeezy sandbox purchase | Blocked | Sandbox checkout completes with the intended product and test buyer | `docs/manual-qa.md` |
| Valid sandbox activation | Blocked | App reaches Pro state and raw key is absent from local cache | `docs/manual-qa.md` |
| Signed DMG | Blocked | `codesign` verification for the public DMG artifact | Release notes |
| Notarized and stapled DMG | Blocked | `spctl`/notary evidence for the public DMG artifact | Release notes |
| Gatekeeper clean-machine open | Blocked | Fresh macOS account or clean machine opens the stapled app | `docs/manual-qa.md` |
| Published checksum | Blocked | SHA-256 line for the public DMG is attached to the release | GitHub Release |
| Homebrew cask install | Blocked | `brew install --cask mt4110/tap/dropsquash` installs the versioned artifact | Homebrew tap PR |
