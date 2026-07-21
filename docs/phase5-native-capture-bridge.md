# Phase 5 Native Capture Bridge

## Decision

Phase 5 uses a small macOS-native bridge at the capture boundary. Rust owns
product policy, canonical MaskPlan, evidence signing, and final output
verification. The native bridge owns Apple reference types for their complete
lifetime.

The initial implementation uses ScreenCaptureKit's Objective-C completion
handler API. Swift remains a valid implementation choice for later isolated
workers, but the bridge does not depend on Swift Concurrency or transfer an
Apple object through Rust.

## Ownership Boundary

```text
Tauri/Rust -> C ABI -> native Apple bridge
                         ScreenCaptureKit
                         Vision (JA/EN rectangles only)
                         CVPixelBuffer
                         AVAssetWriter
                      -> C ABI value callback -> Rust
                                                   MaskPlan
                                                   receipt
                                                   final decode verification
```

No Apple object pointer, captured pixel buffer, or recognized text crosses the
C ABI. The bridge exchanges integer times, Vision rectangles and confidence,
scale values, opaque session identifiers, and error codes only.

The signed MaskPlan sidecar records `apple_native_capture_v1` for the native
bridge route. A `phase5-native-bridge-v1` sidecar without that marker is
rejected by the evidence verifier. This proves the route used for an attempt,
not selective masking, absence of every leak, or enterprise readiness.

## Bridge Responsibilities

The native bridge must discover and select the requested window; own and stop
its SCStream; run local Japanese/English Vision text and text-rectangle
requests; validate accepted-frame metadata; destructively overwrite its native
output buffer in timestamp order; and discard temporary output on any gap,
capacity error, Vision failure, metadata failure, ordering violation, or
cancellation.

Rust must reject a final report unless continuity, final decode, no-audio,
metadata, MaskPlan, and evidence checks all pass. The legacy Rust callback
route is explicitly excluded from this publication boundary because it moves
an Apple reference type across a Rust thread boundary without a supported
binding contract.

## Acceptance Criteria

- No `Retained::into_raw(... as usize)` cross-thread transfer remains in the
  high-assurance capture path.
- The bridge retains ScreenCaptureKit, Vision, CoreVideo, and AVFoundation
  objects until their native work completes.
- Rust receives only value callbacks and independently verifies the MP4.
- A frame that cannot be copied into the bounded queue rejects the recording.
- Strict Shield remains the conservative baseline.

## Strict Shield Recording ABI

The current native session accepts a selected `windowID`, fixed output size,
output path, and a value-only completion callback. It owns `SCStream`,
`CMSampleBuffer`, `CVPixelBuffer`, `AVAssetWriter`, and its pixel-buffer pool.
It writes no audio and overwrites every emitted frame with black before append.

The callback status is `1` for started, `2` for completed with its frame
count, and `3` for fail-closed cleanup. Failure callbacks carry a stable class:
authorization, selected target, setup, environment change, writer, or frame
continuity. Rust must keep its callback context alive until status `2` or `3`;
it receives no media object.

The metadata-enabled native entry additionally emits one value struct per
appended frame: ScreenCaptureKit display-time ticks, frame status, scale
factor, content scale, content rect, and bounding rect. Missing, malformed, or
out-of-range attachments reject and delete the recording. The Rust consumer is
represented by a `#[repr(C)]` value struct and callback. The Tauri recording
state is the next integration step; the metadata ABI is deliberately separate
so the old product path cannot accidentally claim this coverage.

Before it appends a black frame, the native recorder itself requires a complete
frame status, strictly increasing display time, a gap of no more than five
30fps intervals after `mach_timebase_info` conversion, and unchanged content
and bounding rectangles plus scale values. A violation deletes the partial MP4.
This is a conservative native capture boundary; it is not a substitute for
AX/Vision agreement, final decode, and evidence gates.

The attested entry also runs `VNRecognizeTextRequest` with `ja-JP` and
`en-US`, plus `VNDetectTextRectanglesRequest`, against the native source pixel
buffer before it is blackened. It sends only normalized rectangles, kind, frame
index, and confidence to Rust. Strict Shield retains aggregate observation
counts in its MaskPlan; it does not retain recognized strings or Vision
rectangles in the signed sidecar. A malformed observation, queue backpressure,
or Vision request failure deletes the partial output. This is observation
evidence only, not a selective-masking authorization.

The attested entry additionally receives only the selected window's ID, owner
PID, and CoreGraphics frame as scalar values. Objective-C checks that tuple
against the current on-screen CoreGraphics window list at start, before every
accepted frame, and again before writer completion. It rejects a missing,
replaced, moved, or resized target. This closes the direct target-substitution
path for the native strict path. The same native session observes display
reconfiguration, workspace sleep/session/Space lifecycle notifications, and
foreground activation. A display change, watched lifecycle event, or observed
third-party foreground application rejects the partial output. Accessibility,
AX/Vision agreement, final decode, and receipt coverage remain separate
required gates.

The attested entry also requires macOS Accessibility trust before it asks
ScreenCaptureKit to start. Before it requests capture, it retains the matching
AX window only inside the native session and subscribes to its move, resize, and destruction notifications,
and rejects the partial output when one arrives. It additionally checks AX
geometry at start, before each accepted frame, and before writer completion.
This strengthens target continuity, but is not AX/Vision agreement proof or a
selective-masking claim. Packaged-app adversarial QA is still required.

`native/AppleSecureShareBridgeTests.m` builds an in-memory nonzero 32BGRA
sample, invokes the native copy routine in Strict Shield mode, and asserts that
every returned owned byte is zero. It verifies destructive overwrite of the
copied buffer, not a real ScreenCaptureKit permission or recording session.

## Current Checkpoint

`AppleSecureShareBridge.m` exports a C ABI version, asynchronous
ScreenCaptureKit discovery, and a selected-window single-frame probe. The
probe copies 32BGRA pixels before its callback, can destroy every copied pixel
for Strict Shield, and has an explicit native release function. This is an ABI
smoke test. The native Strict Shield recorder also compiles and links with
ScreenCaptureKit, CoreVideo, CoreMedia, and AVFoundation. The desktop command
now starts the attested native session in a dedicated Rust control thread, but
only receives bounded value metadata. It drains that metadata during capture,
rejects callback backpressure, requires the native terminal frame count to
match, then enters the existing independent MP4 verification and signed-sidecar
publication boundary. Its receipt records `phase5-native-bridge-v1` as the
verification-policy version. The former raw `SCShareableContent` transfer is
no longer used by that command.

This is not a selective-sharing claim or a packaged-app pass. On the current
Mac, `dropsquash-platform` debug test execution also stops before listing
tests, and even `codesign -d` on that debug executable waits indefinitely.
`cargo check -p dropsquash-platform --tests` succeeds, but it is compilation
evidence only. The new route still requires a signed packaged-app permission
run plus adversarial fixture QA before any product claim can change.
