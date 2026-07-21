# Phase 2 R&D Review

Use this file as the review gate for high-assurance Secure Share.

Date baseline:

```text
Sunday, July 19, 2026
```

## Current Objective

```text
Build the Phase 2 R&D foundation for high-assurance Secure Share. The first
checkpoint is a documented and prototyped mask-candidate engine that combines
ScreenCaptureKit frame metadata, Accessibility structure, and local Vision
text/shape observations into a deterministic MaskPlan. Do not reopen public
checkout, Lemon Squeezy production, Stripe production, or broad public claims
until Strict Reveal, fail-closed export, and independent verification pass.
```

## Product Boundary

The earlier manual-rectangle alpha proved only this:

- DropSquash can enter a frame-aware export path.
- Pixels can be destructively overwritten before encode.
- The output can be hashed.
- A receipt sidecar can be written.

That is important engineering groundwork, but it is not a sellable privacy
feature. It must not be presented as leak-zero, enterprise-safe, or automatic
PII protection.

## In Scope

- macOS only
- built-in capture only for high-assurance Secure Share
- ScreenCaptureKit frame metadata
- Accessibility window, modal, and text-frame structure
- local Vision Japanese and English text/shape observations
- canonical coordinate mapping
- deterministic frame-exact `MaskPlan`
- Strict Reveal fail-closed policy
- final-size CVPixelBuffer destructive overwrite
- independent decode and residual-leak verification
- local receipt with sources, reasons, confidence, and verification result

## Explicit Non-Goals

- general screen recorder
- timeline editor
- YouTube downloader
- media library
- broad format support
- advertising SDK
- production payment setup
- public leak-zero claim before evidence exists

## Minimum R&D Checkpoint

Phase 2 R&D is reviewable only when all of these are true:

1. Threat model exists and names what can and cannot be protected:
   [docs/secure-share-threat-model.md](/Users/masakitakemura/_workspace/drop-squash/docs/secure-share-threat-model.md)
2. Supported app/window/language/display matrix exists:
   [docs/secure-share-support-matrix.md](/Users/masakitakemura/_workspace/drop-squash/docs/secure-share-support-matrix.md)
3. Japanese and English adversarial corpus plan exists:
   [docs/secure-share-adversarial-corpus.md](/Users/masakitakemura/_workspace/drop-squash/docs/secure-share-adversarial-corpus.md)
4. ScreenCaptureKit prototype records per-frame metadata from a GUI-hosted
   packaged-app or signed-helper harness.
5. Accessibility prototype maps at least window bounds and then text-element
   bounds.
6. Vision prototype detects Japanese and English text/shape regions locally.
7. Canonical coordinate transform is tested across scale and resize cases.
8. `MaskPlan` schema records frame time, region, policy, reason, source, and
   confidence:
   [docs/secure-share-maskplan.md](/Users/masakitakemura/_workspace/drop-squash/docs/secure-share-maskplan.md)
9. Strict Reveal policy destroys unknown regions.
10. Independent verification plan exists for final encoded output.

## Stronger Commercial Checkpoint

The product may return to public website, distribution, and payment work only
after all of these are true:

1. Strict Reveal works in the packaged macOS app.
2. The app does not write durable plaintext unmasked raw capture artifacts.
3. Missing Accessibility permission blocks high-assurance export.
4. Vision/AX gaps become unknown and are destroyed.
5. Audio is disabled by default in secure modes.
6. The final encoded output is independently decoded and checked.
7. Verification failure deletes or quarantines the output.
8. Receipt records the supported scope and verification result.
9. Manual QA covers transient notifications, Japanese text, English text,
   browser/Electron UI, modal/sheet overlays, display scale, and window resize.
10. The user explicitly accepts the result as commercially convincing.

## Review Decision Options

At review, choose one:

1. continue toward commercial Secure Share
2. hold and improve detection/verification quality
3. narrow the public product back to compression only

Do not mix this decision with Lemon Squeezy, Stripe, public website, or release
publication. Those are downstream only.

## Current Decision

Current decision on Sunday, July 19, 2026:

- continue Phase 2 R&D
- demote manual rectangles and fixed-bar masking from product value
- remove manual rectangle/mask-mode controls from the normal desktop UI
- treat CLI ScreenCaptureKit capture start as unsafe; CLI may list redacted
  candidates only
- make high-assurance mask-candidate generation the core engine
- keep payment and public launch paused

## Next Engineering Step

Build the first GUI-hosted observation harness.

Acceptance:

- it runs from the packaged app or a signed helper, not `xtask`
- it asks for or reports Screen Recording permission clearly
- it records real frame metadata for one selected window
- it stops capture cleanly
- it writes only redacted QA evidence, not raw unmasked frames
- missing permission, no frames, or incomplete metadata fails closed

Current harness:

- enumerate candidates with `cargo run -p xtask -- manual-qa-secure-share-observe list`
- launch the packaged app with `DROP_SQUASH_QA_SCK_OBSERVE_WINDOW_ID=<id>`
- write evidence to `DROP_SQUASH_MANUAL_QA_EVENT_LOG=/tmp/dsq-secure-share-observe.jsonl`
- use `DROP_SQUASH_QA_SCK_OBSERVE_QUIT_AFTER=1` for a one-shot QA run

Current evidence on Sunday, July 19, 2026:

- the packaged-app startup harness writes JSONL evidence
- the harness can run under a separate QA instance lock with
  `DROP_SQUASH_QA_INSTANCE_ID`
- callback execution is deferred outside the shareable-content completion
  callback, avoiding the prior `SCStream` start timeout
- sample-level `SCStreamFrameInfo` attachments are read with
  `CMSampleBufferGetSampleAttachmentsArray`, matching Apple's ScreenCaptureKit
  sample pattern
- latest successful evidence:
  `/tmp/dsq-secure-share-observe-frame-metadata-1784451193.jsonl`
- observed result:
  `status = ok`, `windowId = 3328`, `frameWidth = 1691`,
  `frameHeight = 950`, `frameCount = 18`,
  `firstFrameTimeNs = 4672453313239`,
  `lastFrameTimeNs = 4672460422746`
- this is not yet a sellable masking feature; AX structure, Vision OCR,
  MaskPlan synthesis, destructive overwrite, and residual-leak verification
  are still required
- next code checkpoint added on Sunday, July 19, 2026: selected
  ScreenCaptureKit window targets now carry only owner process ID into the AX
  bridge; `AXIsProcessTrusted` is checked; `AXWindows`, `AXPosition`, and
  `AXSize` can produce redacted window-rectangle observations; the QA DTO now
  exposes `axObservationCount`
- this still is not sellable: Accessibility text-element traversal, modal/sheet
  classification, local Vision Japanese/English observation, canonical
  coordinate transforms, destructive pixel writing from `MaskPlan`, and
  independent residual-leak verification remain required
- earlier Accessibility-enabled packaged-app run:
  `/tmp/dsq-secure-share-observe-ax-1784453411.jsonl`
- observed result:
  `status = error`; ScreenCaptureKit discovery failed closed because macOS TCC
  reported that the user denied app/window/display capture
- this is useful fail-closed evidence; the later permission-granted Vision run
  below records both `axObservationCount` and `visionObservationCount`
- Vision checkpoint added on Sunday, July 19, 2026: core
  `VisionNormalizedRect` converts Vision lower-left normalized bounding boxes
  into DropSquash top-left output pixels; macOS builds now expose an
  `objc2-vision` boundary for `VNRecognizeTextRequest` and
  `VNDetectTextRectanglesRequest`; captured ScreenCaptureKit `CMSampleBuffer`
  values can now be validated and wrapped in a `VNImageRequestHandler`; local
  Japanese/English text recognition and text-shape requests can be performed;
  text-shape results map to redacted `VisionObservation` geometry without
  storing recognized text; accepted stream callbacks now run Vision immediately
  and GUI-hosted observation reports can expose `visionObservationCount`
- packaged-app AX/Vision evidence on Sunday, July 19, 2026:
  `/tmp/dsq-secure-share-observe-vision-1784457000.jsonl` recorded
  `status = ok`, `windowId = 3328`, `frameCount = 2`,
  `axObservationCount = 6`, and `visionObservationCount = 268`; a shorter run
  at `/tmp/dsq-secure-share-observe-vision-1784456934.jsonl` recorded
  `frameCount = 1`, `axObservationCount = 6`, and
  `visionObservationCount = 133`
- this still does not satisfy the commercial Vision checkpoint: Japanese and
  English adversarial text must be checked for detection quality, and the
  resulting observations must feed frame-exact `MaskPlan` synthesis,
  destructive masking, and independent output verification before review
