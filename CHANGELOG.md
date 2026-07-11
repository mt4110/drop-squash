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
