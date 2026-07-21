# Phase 5 Native Selective Pipeline

## Status

This is the implementation contract for the next Secure Share research slice.
It does not enable a product mode, a signed receipt, or a selective-sharing
claim. The only publishable Secure Share output remains Strict Shield: every
accepted capture frame is black before encoding.

## Why The Native Boundary Is Required

`SCShareableContent`, `SCStream`, `CMSampleBuffer`, `CVPixelBuffer`, Vision,
Accessibility, and `AVAssetWriter` have lifetime and queue contracts defined by
Apple frameworks. They stay in Objective-C/Swift. Rust receives only copied,
redacted values through C ABI callbacks: timestamps, frame status, normalized
or canonical rectangles, counts, and terminal error classes. No Apple object
pointer, frame pointer, AX value, recognized text, title, or raw frame bytes
crosses into Rust.

This is an ownership boundary, not an operating-system limitation. A native
implementation can destructively edit pixels and write MP4 output; the current
Strict Shield recorder already does both in
`AppleSecureShareRecording.m`.

## Required Per-Frame Order

```text
ScreenCaptureKit complete frame
  -> native metadata / target / lifecycle checks
  -> owned native analysis input
  -> AX + Vision observations finish for this frame
  -> canonical coordinate validation and MaskPlan decision
  -> destructive overwrite of the same owned frame
  -> AVAssetWriter append
  -> final decoded verification and redacted signed evidence
```

There is no allowed path from a source frame directly to `AVAssetWriter` in a
future selective experiment. A missing observation, queue overflow, malformed
coordinate, target change, frame gap, or late analysis result rejects the
recording and removes the partial file.

## Analysis Lanes

| Lane | Native action | Permission to retain pixels |
| --- | --- | --- |
| Strict Shield | Fill the entire output buffer black | Current covered output path |
| AX geometry | Read only text-element bounds; no AX values | Never sufficient by itself |
| Focused input geometry | Read the selected app's focused text-element bounds on every accepted frame; no input value | Always destructive candidate; native code integration pending packaged-app evidence |
| Vision text/shape | Read local Japanese/English region geometry; no strings retained | Never sufficient by itself |
| Temporal change | Compare two native-only 32BGRA frames and emit only a changed bounding rectangle | Always destructive candidate; never a safe signal |
| Cross-source resolver | Union destructive candidates and identify disagreement | Never authorizes a surviving pixel alone |
| Selective experiment | Runs only with synthetic fixture truth and post-decode residual checks | QA artifact only; no receipt or normal UI |

The Vision worker currently runs asynchronously and is intentionally compatible
with Strict Shield because no source pixels survive. It cannot be reused as a
selective writer lane: a result arriving after its frame was encoded is an
unavoidable leakage path. The selective lane must either complete analysis
before append within a bounded native ownership queue, or fail closed.

The current bridge additionally retains only one preceding native frame in
memory for a byte-exact 32BGRA change comparison. It emits no image bytes,
hashes, or changed pixel values: only the current capture frame index and a
normalized changed bounding rectangle cross the C ABI. Any change becomes an
`UnknownRegion` / `TemporalTracker` destructive candidate in an experimental
MaskPlan. The shipped Strict Shield recorder still destroys the full frame, so
this signal has not opened any retained-pixel path or changed the claim surface.

Focused-input geometry is intentionally separate from the periodic AX tree
walk. A text field can change between tree samples, so the native recorder asks
Accessibility for the selected app's focused text-element bounds for every
accepted capture frame. It emits only that rectangle with a distinct
`AccessibilityFocusedText` provenance. Missing or malformed geometry is not a
permission to retain pixels: Strict Shield remains black, and any future
selective lane must reject that frame.

The focused element must also resolve back to the exact AX window selected for
capture. The native bridge first checks its direct AX window attribute, then
walks only native AX parents if that attribute is unavailable. A different
window in the same application is a target mismatch, not an observation to
reuse. The native recorder terminates rather than attributing that element to
the selected-window evidence chain.

## Admission Gates

A future native selective experiment may append a frame only when all of these
are true for that exact frame:

1. ScreenCaptureKit reports `Complete`, with continuous monotonic time and
   unchanged capture geometry/scale.
2. Native CoreGraphics and Accessibility target attestation still match the
   selected window; required lifecycle, display, and focus watchers are clean.
3. Every required analysis result belongs to this frame and maps into the
   canonical output coordinate space without clipping ambiguity.
4. AX/Vision disagreement is converted into a destructive region, never a
   safe region.
5. The destructive regions are written to the native output buffer before the
   one and only writer append.
6. The bounded queue has capacity. Overflow is a terminal failure, not frame
   dropping or delayed masking.

The current 30fps screen-recording target and accurate Vision latency are
separate facts. If analysis cannot keep up, the experiment fails closed. It
must not lower the sampling rate, interpolate old masks over new frames, or
accept an observation from a neighboring frame just to preserve a visible
video.

## Evidence And Exit Criteria

The experimental path needs all of the following before it can be evaluated as
anything beyond R&D:

- a signed packaged-app run against a synthetic Japanese/English fixture;
- fixture truth that contains rectangles only, never sensitive strings;
- decoded final-frame verification of every declared destructive rectangle;
- a residual local-Vision pass that rejects uncovered text candidates;
- measurements for analysis latency, queue depth, frame loss, and rejected
  recordings; and
- an explicit matrix row for each transient input, IME, sheet, popover,
  notification, geometry, display, and AX/Vision disagreement case.

Even then, this does not establish leak-zero or enterprise readiness. Those
claims remain prohibited until the supported surface and adversarial corpus
prove a materially stronger boundary.
