# Changelog

## Unreleased

- Add Phase 0 Rust workspace scaffold.
- Replace the external media CLI plan with native macOS, Windows, and Linux backend contracts.
- Add the media threat model and an optional pinned Nix development environment.
- Add macOS native conversion through AVFoundation and the desktop Drop Zone MVP.
- Persist desktop output, profile, and size settings locally.
- Replace macOS output hard-link finalization with no-clobber atomic rename.
- Split oversized Rust production files to keep modules within the repository size rules.
- Require verified outputs to be smaller `.mp4` files with an MP4 file-type box and non-zero MP4 duration.
- Add a desktop cancel action wired through the active conversion state and native encoder path.
- Add the first sequential queue model and desktop queue UI for multi-file drops.
- Persist the original-file policy and wire safe post-conversion Trash handling through macOS NSFileManager.
- Add an explicit Ask-mode action for moving the original to Trash after re-validating the output.
- Show trial usage in the desktop window so the 10-conversion limit is visible before lockout.
- Add a raw-key-free license cache with offline grace checks for future Pro activation.
- Add a locked-state license activation form wired to the provider boundary without adding license networking.
- Add a release readiness gate that rejects secret-like files and keeps unsigned releases blocked.
- Implement the Lemon Squeezy License API provider with form-encoded HTTPS requests and raw-key-free cache writes.
- Add a static sales-site draft with landing, pricing, privacy, download, support, license, and changelog pages.
- Add an `xtask checksum` command for release SHA-256 artifact manifests.
- Add an `xtask macos-signing-check` command for preflight signing and notarization environment validation.
- Add an `xtask artifact-check` command to reject release artifacts containing `/nix/store` references.
- Add an `xtask homebrew-cask` command to generate the desktop cask from release metadata.
