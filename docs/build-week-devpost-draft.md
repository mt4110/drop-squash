# Build Week Devpost Draft

Use this draft as the source text for the Devpost submission.

## Project Title

DropSquash Secure Share Alpha

## Tagline

Local-first screen-recording evidence for QA teams, with native macOS
observation and redacted MaskPlan previews.

## Track

Developer Tools

## Short Description

DropSquash started as a local Mac app for compressing screen recordings without
uploading media. For Build Week, it now has a Secure Share R&D alpha: a native
macOS observation pipeline that uses ScreenCaptureKit frame metadata,
Accessibility structure, and local Apple Vision text/shape observations to
preview where sensitive screen regions should be destructively masked. It also
destructively blackens local Vision regions on the same ScreenCaptureKit frame
and reads back black pixels without storing recognized text.

The alpha does not claim completed privacy protection. It produces redacted
evidence, a Strict Reveal `MaskPlan` preview, and a black-fill proof without
storing recognized private OCR text. The next gates are final-video
destructive export, independent final-output residual verification, and
fail-closed deletion before any commercial security claim.

## What It Does

- compresses local Mac screen recordings through native Apple media APIs
- preserves originals and does not upload media
- opens a Japanese/English adversarial browser fixture
- observes the selected browser window from the packaged macOS app
- records ScreenCaptureKit frame size and frame count
- records Accessibility observation count
- records local Vision text/shape observation count
- emits a redacted Strict Reveal `MaskPlan` preview
- blackens Vision regions on the live captured frame and reads back black pixels
- keeps recognized private text out of evidence JSON

## Why It Matters

QA teams often need to share screen recordings that contain customer names,
emails, tokens, filenames, modals, notifications, or low-contrast private text.
Traditional blur or manual rectangle tools are easy to misuse and hard to
audit. DropSquash is exploring a stricter route: local native observation,
deterministic mask planning, destructive pixel overwrite, and independent
verification before a recording can be shared.

The current Build Week submission is the R&D alpha that proves the observation
and planning foundation, not the final security product.

## Demo Evidence

Recorded local packaged-app evidence:

```text
Fixture: tests/fixtures/secure-share/ja-en-browser-form.html
Window: 1224x968 browser fixture
Frames observed: 3
Accessibility observations: 1
Local Vision observations: 156
MaskPlan policy: strict_reveal
MaskPlan frames: 3
First frame regions after coalescing: 47
Black-fill mask rects: 47
First black-fill sample blackened: true
Live frames blackened: 3
Live text regions blackened: 156
Live black pixel readbacks: 3
Recognized private text stored: no
```

See:

- `docs/build-week-evidence.json`
- `docs/build-week-judge-runbook.md`
- `docs/build-week-demo-script.md`

## How To Run

```bash
nix develop
pnpm --dir apps/desktop tauri build
open -a "Google Chrome" tests/fixtures/secure-share/ja-en-browser-form.html
cargo run -p xtask -- manual-qa-secure-share-observe list
```

Then run the packaged app harness with the browser window ID:

```bash
env \
  DROP_SQUASH_QA_INSTANCE_ID=build_week_maskplan_preview \
  DROP_SQUASH_QA_SCK_OBSERVE=1 \
  DROP_SQUASH_QA_SCK_OBSERVE_WINDOW_ID=3301 \
  DROP_SQUASH_QA_SCK_OBSERVE_CAPTURE_MS=1000 \
  DROP_SQUASH_QA_SCK_OBSERVE_TIMEOUT_MS=15000 \
  DROP_SQUASH_QA_SCK_OBSERVE_QUIT_AFTER=1 \
  DROP_SQUASH_MANUAL_QA_EVENT_LOG=/tmp/dsq-build-week-maskplan-preview.jsonl \
  target/release/bundle/macos/DropSquash.app/Contents/MacOS/dropsquash-desktop
```

Inspect the final JSONL event:

```bash
tail -n 1 /tmp/dsq-build-week-maskplan-preview.jsonl
```

## How Codex And GPT-5.6 Were Used

Codex was used as the primary engineering partner for:

- converting a broad product idea into a strict Build Week alpha scope
- rejecting overclaims such as leak-zero, audit-ready, or complete PII safety
- splitting Rust/Tauri work under strict file-size limits
- adding the Japanese/English adversarial fixture and annotation sidecar
- wiring the packaged-app ScreenCaptureKit, Accessibility, and Vision
  observation evidence into a redacted `MaskPlan` preview and live-frame proof
- preparing README, judging runbook, evidence summary, and demo script
- keeping no-upload, no-ffmpeg, original-safety, and local-first constraints
  visible while the prototype evolved

## What Is Not Finished Yet

- final-video destructive CVPixelBuffer overwrite from the `MaskPlan`
- independent final-output residual verification
- fail-closed output deletion when verification fails
- production Secure Share recording/export UI
- Windows/Linux Secure Share support
- commercial checkout
- any claim that the app is leak-zero or enterprise audit-ready

## Repository Access

The repository is private for submission safety. Read-only invitations are
prepared for:

- `testing@devpost.com`
- `build-week-event@openai.com`

This avoids publishing private productization work while still giving judges
access to source, setup instructions, fixtures, and evidence.

## Suggested Video Description

```text
DropSquash Secure Share Alpha is a local-first macOS prototype for QA teams
that need safer screen-recording evidence. The demo shows a synthetic
Japanese/English sensitive-data fixture, packaged-app ScreenCaptureKit +
Accessibility + local Vision observation, and a redacted Strict Reveal
MaskPlan preview that stores geometry/reasons/sources/confidence but not
recognized private OCR text. It also shows a black-fill proof. This is an R&D
alpha, not a leak-zero claim.
```
