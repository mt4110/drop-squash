# Phase 5 Native Bridge QA

Run this only with a Developer ID-signed packaged app. It validates the
`phase5-native-bridge-v1` path and does not authorize a public product claim.

## Preconditions

- Grant Screen Recording and Accessibility to the exact app bundle under test.
- Confirm `LSMinimumSystemVersion` is `14.0` in the bundle's `Info.plist`.
- Use a disposable output directory.
- Build `NativeAccessibilityFixture.swift` locally and select its window.
- Record only synthetic fixture data.

Start a named fixture scenario instead of entering its environment variable by
hand:

```sh
scripts/run-secure-share-native-fixture.sh move-return
```

Supported names are `baseline`, `hide`, `resize`, `move`, `move-return`,
`focus-loss`, `same-app-focus`, `rapid-burst`, `input-burst`, `popover`,
`sheet`, and `title`.

`same-app-focus` must terminate with the native `target` failure class. A
continuity failure is not sufficient evidence for this scenario because it
would not prove the selected-window ownership check ran.
Each automatic scenario waits ten seconds before its action, so start recording
immediately after selecting the fixture target. Override the matching
`DROP_SQUASH_FIXTURE_*_DELAY_SECONDS` variable only when a QA run needs more
setup time.

## Baseline

1. Start Secure Share against the stationary fixture and stop after five seconds.
2. Confirm one final MP4 and one `.mask-plan.json` sidecar exist.
3. Confirm the sidecar has `verificationPolicyVersion` set to
   `phase5-native-bridge-v1`, `captureContinuityAttested` set to `true`, and
   `captureBackend` set to `apple_native_capture_v1`.
4. Run the independent evidence verifier and inspect decoded frames.
5. Record only the count of native AX text-element rectangles. Confirm that no
   AX value, title, role name, or recognized text appears in the sidecar or log.

Expected: every published frame is Strict Shield black. This is destructive
output evidence, not selective-masking evidence. An AX count of zero means
only that this target exposed no usable text-element bounds; it must not be
rewritten as a detector failure or a selective-sharing pass.

## Fail-Closed Cases

For each case, start a new recording, trigger the action, then stop. Confirm
that neither a final MP4 nor final sidecar exists. A dot-prefixed partial may
exist only until the native failure cleanup completes.

| Scenario | Action | Expected classification | Expected native reason class |
| --- | --- | --- | --- |
| Target disappears | Hide or close the fixture window | fail-closed | `target` |
| Geometry changes | Resize or move the fixture | fail-closed | `target` |
| Move and return | Move the fixture, then restore its frame before stop | fail-closed on native AX event | `environment` |
| Display changes | Connect, disconnect, rotate, or mirror a display | fail-closed | `environment` |
| Focus loss | Activate Finder or another third-party app | fail-closed | `environment` |
| Same-app focus mismatch | Front the fixture's secondary input window | fail-closed | `target` |
| Lifecycle change | Sleep, change Space, or resign the user session | fail-closed when macOS emits the watched event | `environment` |
| AX unavailable | Disable Accessibility before start | fail-closed | `authorization` |
| Vision worker cannot retain a frame | Run `scripts/run-secure-share-native-fault-probe.sh`, then record the synthetic fixture | fail-closed with no artifact | `continuity` |
| Missing or discontinuous frame metadata | Trigger only with a deterministic native harness | fail-closed | `continuity` |

The Vision fault injection is QA-only and fail-only: it cannot preserve a
frame, weaken masking, or create an output. Do not set either variable for a
normal recording. The runner refuses to start if DropSquash is already open,
then launches the bundle executable directly so it receives the variables:

```sh
scripts/run-secure-share-native-fault-probe.sh
```

Set `DROP_SQUASH_QA_NATIVE_VISION_FAIL_FRAME` only to move the injected
failure to a later native frame.

## Exposure Fixtures

Run the native fixture and `transient-input-overlay.html` separately with the
Japanese/English typed text, synthetic candidate, popover, modal, and overlay
sequences enabled.

For deterministic short-lived text, start the native fixture with:

```sh
scripts/run-secure-share-native-fixture.sh rapid-burst
```

It rotates synthetic Japanese/English names, one-time codes, a token, and a
local path every 50ms. This tests Strict Shield's destructive output path; it
does not prove that every transient OS surface was observed.

- The current expected result is a fully black Strict Shield MP4 when native
  continuity remains valid.
- For `rapid-burst` or `input-burst`, confirm the signed sidecar audit has a
  nonzero `temporalObservationCount`. It is a native-only frame-difference
  candidate count, not a stored image, string, or a permission to preserve
  pixels. A zero count makes this candidate-engine fixture inconclusive; do
  not rewrite it as coverage.
- Confirm the signed sidecar audit has a nonzero
  `focusedTextObservationCount` while the fixture field has focus. It proves
  geometry capture only, not selective sharing or IME coverage.
- Confirm the result reports a nonzero Vision observation count for the text
  fixture, while the signed sidecar contains only aggregate counts and no
  recognized text or Vision rectangles.
- A real IME candidate, private password-manager UI, Notification Center
  compositing, AX/Vision agreement, and selective masking stay
  `detected-but-not-covered` unless a separately recorded fixture proves them.
- Do not save recognized text, screenshots containing real secrets, or raw AX
  payloads as evidence.

## How To Classify The Result

Record two facts for each exposure path; do not collapse them into one claim.

1. `outputPixelStatus` is `covered` only when the final MP4 independently
   decodes with every frame fully black. It covers only pixels delivered inside
   the selected-window capture.
2. `sourceSurfaceStatus` describes the particular source surface. Use
   `covered` only for the fixture actually exercised, `fail-closed` for a
   recorded rejection, and `detected-but-not-covered` for a real IME,
   password-manager, or notification compositor that has not been proven.

Never change a source-surface result to `covered` merely because the Strict
Shield output is black. Conversely, a `detected-but-not-covered` source does
not weaken the narrow fact that accepted selected-window pixels were destroyed.

## Record

Store only the date, app bundle version, fixture ID, result classification,
output SHA-256, sidecar verification result, decoded-frame count, and failure
reason class. Do not retain titles, recognized text, process names, raw AX
values, or media outside the disposable QA directory.
