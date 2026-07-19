# Build Week Judge Runbook

This runbook lets judges inspect the DropSquash Build Week alpha without
needing private data or cloud services.

## What This Alpha Shows

- local macOS screen-recording compression foundation
- Japanese/English adversarial browser fixture
- packaged-app ScreenCaptureKit observation path
- Accessibility observation count
- local Apple Vision text/shape observation count
- redacted Strict Reveal `MaskPlan` preview
- live ScreenCaptureKit-frame black-fill and pixel readback

It does not claim completed privacy protection. Final-video destructive export
and independent final-output verification are the next gates.

## Prerequisites

- macOS on Apple Silicon
- Screen Recording permission for DropSquash
- Accessibility permission for DropSquash when testing AX observations
- Google Chrome or another browser for the fixture
- Rust/Node/pnpm through `nix develop` if rebuilding from source

### Grant macOS Permissions

At the first capture attempt, macOS shows a native permission dialog. Choose
`Open System Settings`, then enable `DropSquash` in `Privacy & Security` >
`Screen Recording`. Enable it in `Accessibility` as well when inspecting AX
observations. Quit and reopen DropSquash after changing either permission.

These permissions are an OS security boundary; the app never attempts to
enable them programmatically.

Grant permission to the final signed app bundle. An ad-hoc development build
and a Developer ID-signed bundle are different TCC identities in practice; do
not rebuild or re-sign the bundle after granting permission. If macOS keeps
showing the request while its switch is on, turn that `DropSquash` switch off
and on after the final signing step, then relaunch the exact same bundle.

## Build

```bash
nix develop
pnpm --dir apps/desktop tauri build
```

The packaged app is:

```text
target/release/bundle/macos/DropSquash.app
```

## Fixture

Open:

```bash
open -a "Google Chrome" tests/fixtures/secure-share/ja-en-browser-form.html
```

The fixture contains only synthetic Japanese and English private-looking data.

## Observe Candidate Windows

```bash
cargo run -p xtask -- manual-qa-secure-share-observe list
```

Pick the browser window ID. In the recorded evidence run it was `3301`.

## Run Packaged Observation

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

Inspect the last JSONL event:

```bash
tail -n 1 /tmp/dsq-build-week-maskplan-preview.jsonl
```

Expected shape:

- `status = ok`
- `frameCount > 0`
- `visionObservationCount > 0`
- `maskPlanPreview.policy = strict_reveal`
- `maskPlanPreview.frames` contains sensitive regions with geometry, reason,
  source, confidence, and timing
- `blackFillProof.firstSampleBlackened = true`
- `liveMaskedFrameCount > 0`
- `liveMaskedRectCount > 0`
- `liveVerifiedPixelCount > 0`

The log must not contain recognized private OCR text.

## Recorded Evidence

See:

- [docs/build-week-evidence.json](/Users/masakitakemura/_workspace/drop-squash/docs/build-week-evidence.json)
- [docs/build-week-demo-script.md](/Users/masakitakemura/_workspace/drop-squash/docs/build-week-demo-script.md)
