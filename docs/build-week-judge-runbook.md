# Build Week Judge Runbook

This runbook lets judges inspect the DropSquash Build Week alpha without
needing private data or cloud services.

## What This Alpha Shows

- local macOS screen-recording compression foundation
- Japanese/English adversarial browser fixture
- packaged-app selected-window recording path
- ScreenCaptureKit, Accessibility, and local Apple Vision observations
- redacted Strict Reveal `MaskPlan`
- live ScreenCaptureKit-frame black-fill and pixel readback
- a saved clip path that only finalizes after an independent
  decoded-frame, audio, and metadata verification
- a redacted, Ed25519-signed MaskPlan JSON sidecar with the output SHA-256 and
  no OCR text

The current signed-app Phase 5 evidence is in
`docs/build-week-phase5-evidence.json` and its native AppKit-fixture boundary
is recorded in `docs/phase5-alpha.md`. The UI-driven browser fixture is
historical evidence in `docs/build-week-ui-fixture-evidence.json`; Phase 4
observation-only evidence remains in `docs/phase4-strict-shield-evidence.json`
and `docs/build-week-evidence.json`.
It does not claim leak-zero, completed privacy protection, or enterprise audit
readiness. The local Ed25519 signature binds the sidecar payload in this R&D
alpha; it is not hardware-backed or an independent audit authority. The
repository includes a MaskPlan-frame-size mismatch test and the packaged-app
evidence records a corrupt-signing-key run with no final or partial artifact.

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

## Native Transient Fixture

The AppKit fixture exercises native window surfaces that browser HTML cannot
represent. It contains only synthetic data:

```bash
swiftc tests/fixtures/secure-share/NativeAccessibilityFixture.swift \
  -o /tmp/NativeAccessibilityFixture
nohup /tmp/NativeAccessibilityFixture >/tmp/NativeAccessibilityFixture.log 2>&1 &
```

Start a named scenario instead of entering fixture environment variables by
hand:

```bash
scripts/run-secure-share-native-fixture.sh rapid-burst
```

Supported names are `baseline`, `hide`, `resize`, `move`, `move-return`,
`focus-loss`, `same-app-focus`, `rapid-burst`, `input-burst`, `popover`,
`sheet`, and `title`.
Every automatic action waits ten seconds so the target can be selected and
recording started first. `hide`, `resize`, `move`, `move-return`, and
`focus-loss` and `same-app-focus` must fail closed without an MP4, MaskPlan,
or partial artifact.
`title` is intentionally `detected-but-not-covered`: the native path does not
retain or compare titles. The other content scenarios may publish only an
independently verified Strict Shield black output.

In the native fixture:

1. **Show confirmation** opens an AppKit sheet.
2. **Show transient popover** opens an AppKit popover containing a synthetic
   autofill address and one-time code.
3. Click the editable field and type Japanese with a Japanese input source to
   open the macOS IME candidate window.
4. **Change window title** toggles a synthetic title without moving or resizing
   the window. It is a `detected-but-not-covered` research fixture: the native
   path intentionally does not retain or compare titles, so it must not be
   presented as a fail-closed test or a user feature.
5. **Run rapid secret burst** changes synthetic name, code, token, and file
   path text every 50ms. Keep recording through the burst, then save and run
   the independent verifier. Strict Shield must destroy every captured frame;
   this does not claim that every state was independently detected.
6. **Run rapid input updates** focuses the editable AppKit field and changes
   synthetic name, email, code, and token values every 50ms. This is a typed-
   input stress fixture, not an IME simulator. Record it separately and apply
   the same Strict Shield verification rule. Its sidecar should show a nonzero
   `temporalObservationCount`; this is a local changed-rectangle count, not
   a retained pixel or text value.
7. The custom-drawn canvas token is deliberately not an Accessibility text
   element, though macOS may expose a coarse parent client-area rectangle.
   Strict Shield must still destroy every frame and retain no recognized text.

The signed app has independently verified Strict Shield recordings for the
sheet, popover, and one local Japanese IME configuration. These runs prove
full-frame destructive export for those situations only. They do not prove
selective masking, all IMEs, private browser autofill, password managers, or
Notification Center compositing. A notification was dispatched during one
Strict Shield run, but its compositing into selected-window pixels remains
unproven.

## Record The Fixture In The App

1. Open the final signed `DropSquash.app`.
2. In **黒塗り検証の研究 / Secure Share research**,
   choose the fixture window you are about to exercise: the Chrome fixture for
   browser content, or **DropSquash AX Fixture** for native AppKit sheet,
   popover, and IME states.
   The picker shows a window title; if macOS provides an empty title, it falls
   back to the owning app name or a stable window ID.
3. Choose **黒塗り検証を開始 / Start blackening verification**.
4. Exercise the fixture, then choose **停止して検証結果を保存 / Stop and save
   verification**.
5. Wait for **黒塗り検証動画を保存しました / Blackened verification video saved**,
   then use the video and
   receipt reveal controls.

The saved MP4 is intentionally all black. It is destructive-output evidence,
not a shareable video that preserves fixture content.

Keep the selected window at the same position and size while recording. The
current high-assurance path requires display, CoreGraphics window,
Accessibility geometry and window-event, macOS lifecycle, and
foreground-activation watches to finish without a failure; otherwise it
discards the video instead of publishing it.

After a successful Strict Shield save, the result view shows four aggregate
counts: **Accessibility structure**, **focused-input geometry**, **Vision**,
and **change candidates**. They are separate local signals. The Accessibility
number is only the count of native text-element rectangles; focused-input
geometry is likewise a rectangle count with no input value. Vision is
text/shape observations; a change candidate is the bounding rectangle of a
byte-exact native frame difference. A focus element from another window in the
same app terminates the recording rather than contributing to evidence.
No image bytes, changed pixel values, titles, AX values, or recognized text are
stored. None of the counts means that a pixel was preserved. Confirm the
sidecar's matching aggregate audit fields, not raw observations.

The app processes video locally. Strict Shield destructively blacks unknown
client-area pixels before it encodes. A failure in masking, signature, or
independent verification prevents publication of the MP4.

## Recorded Evidence

See:

- [docs/build-week-ui-fixture-evidence.json](/Users/masakitakemura/_workspace/drop-squash/docs/build-week-ui-fixture-evidence.json)
- [docs/build-week-evidence.json](/Users/masakitakemura/_workspace/drop-squash/docs/build-week-evidence.json)
- [docs/build-week-phase5-evidence.json](/Users/masakitakemura/_workspace/drop-squash/docs/build-week-phase5-evidence.json)
- [docs/build-week-demo-script.md](/Users/masakitakemura/_workspace/drop-squash/docs/build-week-demo-script.md)

## Verify A Saved Evidence Pair

After a successful recording, independently verify the saved MP4 and its
adjacent sidecar without network access:

```bash
cargo run -p xtask -- secure-share-evidence-check \
  /path/to/secure-share-recording-<id>.mp4 \
  /path/to/secure-share-recording-<id>.mask-plan.json
```

The command rejects a changed MP4, a mismatched output name, an invalid
Ed25519 signature, raw `text`, `vision`, or `accessibility` observations in
the sidecar, any Strict Shield frame that is not exactly one full-frame
ScreenCaptureKit `UnknownRegion`, or a decoded frame that lacks Strict Shield's
full-frame black region. It also rejects audio, metadata, captions, subtitles, or other
non-video tracks. It verifies the local evidence binding only; it does not
make an enterprise or leak-zero claim.

For current Phase 5 recordings, the command also prints the evidence schema,
MaskPlan schema, continuity attestation result, and redacted continuity-watch
list. A missing or incomplete watch list is a verification failure, not a
warning. Legacy MaskPlan schema 1 sidecars are also rejected for the current
Phase 5 path.

Current recording-path evidence uses MaskPlan schema 2. Its signed audit must
include `captureContinuityAttested: true` and the redacted required-watch list:
`display_configuration`, `core_graphics_window`, `accessibility_geometry`,
`accessibility_window_events`, `macos_lifecycle_notifications`, and
`foreground_activation_pid_allowlist`. This evidence does not store the window
title, identifier, geometry, app name, notification payload, AX event payload,
or recognized text.

The same schema-2 audit must also classify typed text, IME candidates,
dialogs/popovers, notification overlays, browser autofill/password-manager
surfaces, focus/foreground changes, display geometry/scale changes, frame
drops, target-selection mistakes, AX/Vision disagreement, audio/metadata, and
endpoint exfiltration. The audit distinguishes `covered`, `fail_closed`,
`detected-but-not-covered`, and `out-of-scope`; it does not turn those labels
into a selective-masking completion claim.

## Verify Phase 5 Claims

Before recording or submitting a demo, run the repository-level Build Week
claim gate:

```bash
cargo run -p xtask -- build-week-phase5-check
```

This checks the current Phase 5 evidence JSON and also runs the Phase 5
documentation honesty lint. A row that claims `covered` or `fail-closed` while
its evidence still says `pending`, `unproven`, `untested`, or `proof absent`
is a failure. Fix the claim or gather stronger evidence instead of weakening
the checker.
