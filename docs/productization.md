# Productization Plan

DropSquash is currently a macOS MVP: a small local Drop Zone that converts user-selected screen recordings into numbered `.squashed.mp4` outputs through Apple's native AVFoundation export pipeline.

The v1 promise stays intentionally narrow:

```text
Drag a screen recording.
Get a smaller MP4.
Original stays safe.
Nothing uploads.
```

## Operating Rules

| Rule | Decision |
|---|---|
| Media path | No `ffmpeg`, no `ffprobe`, no shell, no `PATH` lookup |
| Privacy | No media upload, no default telemetry |
| Original handling | Keep original unless verified success and user policy allow Trash |
| Trial | Count successful, smaller verified conversions only |
| UI | Keep the main window small, calm, and task-focused |
| Platform order | macOS first; Windows and Linux after macOS product hardening |
| Claims | Say AVFoundation/native macOS pipeline now; do not overclaim explicit VideoToolbox hardware encoding |
| File size | Rust production files <= 128 lines; Rust test files may exceed 1000 lines; TS/TSX files <= 512 lines |

## Plan Table

| Phase | Status | Goal | Scope | Acceptance | Verification |
|---|---|---|---|---|---|
| 0.5 Repo alignment | Done | Make names, docs, and metadata consistent before adding product surface | README/docs current-state wording, product invariants, repository URL decision | Docs match implementation; no unsupported hardware claims; no secrets added | `cargo fmt --all -- --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace` |
| 0.6 File-size architecture cleanup | Done | Bring existing code under the repository's size rules before adding more behavior | Split large Rust production files by responsibility; keep TS/TSX under 512 lines | No Rust production file exceeds 128 lines; no TS/TSX file exceeds 512 lines; behavior unchanged | `cargo run -p xtask -- file-size-check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings` |
| 1 macOS encoder hardening | In progress | Make single-file macOS conversion safe enough for paid beta | Replace hard-link finalization, strengthen output verification, clean temp files, friendly failure states | Existing outputs are never overwritten; failed/larger/cancelled conversions do not count; original remains untouched | Unit tests plus `docs/manual-qa.md` packaged-app conversion record; `manual-qa-check` before completion |
| 2 Cancellation | In progress | Let users stop an active conversion cleanly | Cancellation token through command/encoder boundary, UI cancel action, temp cleanup | Cancel returns app to ready state; no success history; no trial count | Unit tests plus `docs/manual-qa.md` cancellation record; `manual-qa-check` before completion |
| 3 Sequential queue | In progress | Handle multiple dropped files deterministically | Queue states, per-job progress, one active conversion at a time | Multiple drops create rows; one job runs at a time; failures do not block unrelated jobs | Queue unit tests plus `docs/manual-qa.md` multi-file record; `manual-qa-check` before completion |
| 4 Source postprocess | In progress | Safely move originals to Trash only after verified success | Source policy setting, macOS Trash adapter, ask-after-success flow | Keep never moves; Ask prompts; Trash moves only after all safety gates | Setting, Ask UI, Trash adapter, and `docs/manual-qa.md` Trash record; `manual-qa-check` before completion |
| 5 License and trial UI | In progress | Convert trial usage into Pro unlock without account creation | License cache, instance id, activation/validation provider, local forget action, locked/Pro UI | Raw license key is not persisted; invalid/network errors are friendly; valid cache survives grace period | Provider, trial UI, activation shell, local raw-key-free forget action, and safe cache/grace are implemented; Lemon Squeezy sandbox test remains |
| 6 Release pipeline | In progress | Ship a trusted macOS beta | Signed app, notarized DMG, checksums, release checklist, Homebrew cask draft | Gatekeeper opens cleanly; secrets stay in CI; artifact checksum published | Readiness, media/privacy security, artifact, checksum, signing preflight, DMG bundle target, and cask generation gates exist; signing/notarization remain |
| 7 Sales site | In progress | Let a stranger understand, download, try, and buy | Landing, pricing, privacy, download, refund, FAQ, support | Privacy claims match implementation; CTA works; download path works | Static site draft, required-page gate, and `website-check` exist; hosting and live checkout remain |
| 8 Windows/Linux | Later | Expand after macOS signal | Media Foundation and GStreamer allowlist backends | Same core invariants; no silent software fallback | Native OS CI and real-device smoke tests |

## Immediate Backlog

| Order | Item | Status | Notes |
|---:|---|---|---|
| 1 | Persist output/profile/size settings | Done | Stored in platform app config path |
| 2 | Update README/docs from Phase 0 wording | Done | Current macOS path is described as AVFoundation MVP |
| 3 | Decide canonical repository slug | Done | Metadata follows the current GitHub remote, `mt4110/drop-squash` |
| 4 | Replace macOS hard-link finalization | Done | Uses no-clobber atomic rename on macOS |
| 5 | Split oversized production files | Done | All Rust production files are now <= 128 lines; TS/TSX remain <= 512 lines |
| 6 | Add output media validation beyond size | Done | Requires smaller `.mp4`, MP4 file-type box, and non-zero `mvhd` duration |
| 7 | Add cancellation | In progress | Command/UI/encoder path is implemented; `docs/manual-qa.md` packaged-app record remains |
| 8 | Add sequential queue | In progress | Queue model and React sequential runner are implemented; `docs/manual-qa.md` multi-file record remains |
| 9 | Wire source policy and Trash | In progress | Setting is persisted; Ask has explicit Trash action; Trash uses macOS NSFileManager after safety gates; `docs/manual-qa.md` Trash record remains |
| 10 | Add license activation UI | In progress | Trial UI, locked activation form, local forget action, provider networking, and raw-key-free cache exist; sandbox/manual validation remains |
| 11 | Build release pipeline | Pending | Signing/notarization before public beta |

## Release Gate

Do not start a public paid beta until all of these are true:

```text
macOS single-file conversion is verified
cancel does not count trial
failed conversion does not count trial
larger output is treated as failure
original is never moved without verified success
app is signed and notarized
privacy claims match implementation
license secrets are not in the repository
```
