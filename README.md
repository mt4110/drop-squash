# DropSquash

Drop huge screen recordings. Squash them locally.

DropSquash turns large screen recordings into small, shareable MP4 files on
macOS today without uploading videos to the cloud.
Windows and Linux support is planned after the macOS product path is hardened
and verified.

## Status

macOS MVP. The desktop app can convert user-selected `.mov`, `.mp4`, and `.m4v` recordings to numbered `.squashed.mp4` outputs through Apple's native AVFoundation export pipeline. The shared Rust workspace also contains deterministic safety logic, cancellation, a sequential queue model, safe source postprocessing, JSONL history, local trial counting, license activation plumbing, and backend contracts for Windows and Linux.

Packaged-app manual QA remains for cancellation, multi-file queueing, Trash handling, and live license activation. The release pipeline can build an unsigned macOS `.app` and `.dmg`, run artifact/checksum gates, and block publication until signing is ready. Windows Media Foundation, Linux GStreamer, signing, notarization, and public release publication are still planned work.

Current Build Week target: Phase 3 alpha / high-assurance Secure Share R&D
prototype. The goal is to show local macOS observation evidence for Japanese
and English screen-recording privacy cases, not to claim completed leak-zero
masking. See [docs/build-week-submission.md](docs/build-week-submission.md).

Build Week judging notes:

- Repository may stay private if shared with `testing@devpost.com` and
  `build-week-event@openai.com`.
- Demo fixture:
  [tests/fixtures/secure-share/ja-en-browser-form.html](tests/fixtures/secure-share/ja-en-browser-form.html)
- Demo script:
  [docs/build-week-demo-script.md](docs/build-week-demo-script.md)
- Devpost draft:
  [docs/build-week-devpost-draft.md](docs/build-week-devpost-draft.md)
- Judge runbook:
  [docs/build-week-judge-runbook.md](docs/build-week-judge-runbook.md)
- Redacted evidence summary:
  [docs/build-week-evidence.json](docs/build-week-evidence.json)
- Current alpha evidence: packaged macOS app observed a 1440x900 browser
  fixture with 2 frames, 2 Accessibility observations, 236 local Vision
  observations, a Strict Reveal `MaskPlan` preview with 42 coalesced
  first-frame regions, 2 live ScreenCaptureKit frames blackened across 236
  text regions with 2 pixel readbacks, and
  `blackFillProof.firstSampleBlackened = true`.

## Principles

- Local-first
- Native OS media pipelines
- No external media executables
- No cloud upload
- Safe original handling
- Screen-recording aware compression
- Small UI
- Scriptable core

## Product Readiness Policy

DropSquash is developed toward a real sellable product, but phase labels are
only planning markers. If a P2 or P3 result still feels weak, misleading, or
unfinished, development continues until the product quality is convincing.

Public business surfaces, checkout links, and production payment onboarding are
not opened just because an intermediate checklist passed. They stay private,
owner-only, or deferred until product truth, UX quality, legal surface, and
distribution quality are all strong enough to publish on purpose.

The current product-quality bet is high-assurance Secure Share. Manual
rectangles and fixed-bar masking are not enough to justify a privacy claim or a
sales route. Production checkout stays paused until DropSquash can combine
ScreenCaptureKit frame metadata, Accessibility structure, local Vision
text/shape observations, deterministic MaskPlan generation, Strict Reveal
fail-closed export, and independent final-output verification in a packaged
macOS app.

## Local Commands

Use the pinned Nix shell when you want the repository's Rust, Node, and pnpm
versions:

```bash
nix develop
node -v
pnpm -v
```

To avoid host Node version drift during checks, run web commands through the
pinned shell:

```bash
nix develop --command pnpm --dir apps/desktop/web test
```

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
pnpm --dir apps/desktop/web build
cargo run -p xtask -- file-size-check
cargo run -p xtask -- release-check
```

On macOS, the CLI uses the native encoder. On Windows and Linux, the backend
contracts are present but the encoders still return a clear native encoder unavailable error:

```bash
cargo run -p dropsquash -- convert ./demo.mov --output-dir ./out --profile auto
cargo run -p dropsquash -- receipt ./out/demo.squashed.mp4
```

The target backend matrix is:

| Platform | Probe / pipeline | Encoder |
|---|---|---|
| macOS | AVFoundation | AVFoundation export MVP now; lower-level VideoToolbox may follow |
| Windows | Media Foundation | planned Hardware MFT backend; not implemented |
| Linux | allowlisted GStreamer via `gstreamer-rs` | planned hardware element; not implemented |

Runtime capability detection is authoritative. DropSquash does not silently switch to an unreviewed codec or external media executable.

## Reproducible Development

Nix is optional and is never shipped with the application:

```bash
nix develop
cargo test --workspace
```

The Nix shell supports Apple Silicon macOS and Linux development and pins Rust
1.95.0, Node 24.16.0, and pnpm 10.34.0 for the desktop build. Native Windows
builds and media tests run on Windows. Do not add mise or another version
manager unless Nix cannot express a required tool.

## Architecture

Business logic lives in `crates/*`. UI and command surfaces in `apps/*` are intentionally thin.

See [native backends](docs/native-backends.md) and the [security model](SECURITY.md) before changing the media pipeline.
