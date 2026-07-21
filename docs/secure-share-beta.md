# Phase 2: High-Assurance Secure Share R&D

This document replaces the earlier manual-rectangle Secure Share alpha plan.

The current alpha implementation proved that DropSquash can destructively
modify video frames before export, hash the output, and emit a local receipt.
That is useful groundwork, but it is not enough to sell as a privacy or
enterprise security feature.

## Product Decision

DropSquash should not establish the paid sales route until the secure-sharing
engine is strong enough to justify a high-trust privacy and evidence claim.

The new product hypothesis is:

```text
DropSquash is worth selling when it can locally produce screen-recording
exports that are aggressively privacy-preserving, frame-verifiable, and honest
about any residual risk.
```

This makes Secure Share the core research and product-quality track, not a
small add-on.

## Hard Boundary

Do not claim:

- leak-zero
- audit-ready
- PII-safe
- enterprise-safe
- automatic confidential masking

until the implementation has built-in capture, frame-level metadata,
fail-closed masking policy, and independent verification evidence.

Current fixed-bar detection and manual rectangles are not sufficient for those
claims.

## Target Pipeline

The high-assurance path is:

```text
ScreenCaptureKit frames + per-frame metadata
Accessibility event timeline
Vision text/shape observations
  -> canonical coordinates
  -> Policy Resolver: safe / sensitive / unknown
  -> frame-exact MaskPlan with time, reason, confidence, source
  -> final-size CVPixelBuffer destructive overwrite
  -> encode
  -> independent decode and residual-leak verification
  -> pass: receipt
  -> fail: discard output
```

Current implemented foundation:

- core frame metadata types
- AX and Vision observation types with time ranges
- deterministic frame-exact `MaskPlan` draft construction
- Strict Reveal draft behavior for unmatched observations: full-frame
  `verification_required` / `unknown`
- `MaskPlanAudit` summary for unmatched observation reasons and
  verification-required frames
- Secure Share receipt summary hook for `MaskPlanAudit`
- desktop and CLI receipt save paths can accept an optional `MaskPlan`
- pseudo observation snapshot test proves `snapshot -> MaskPlan -> receipt`
- platform observation snapshot boundary
- injectable `SecureShareSnapshotProvider` boundary for future SCK / AX /
  Vision adapters
- separate `FrameMetadataProvider`, `AccessibilityObservationProvider`, and
  `VisionObservationProvider` traits with a deterministic composition helper
- fail-closed `NativeFrameMetadataProvider` boundary for future
  ScreenCaptureKit metadata capture
- macOS-only `NativeSampleBufferProvider` boundary for observed
  `CMSampleBuffer` streams
- `SampleBufferFrameMetadataProvider` adapter that converts observed sample
  buffers into validated `CaptureFrameMetadata`
- fail-closed test proving an empty ScreenCaptureKit sample-buffer stream is
  not accepted as successful frame metadata
- compiled `SCStreamOutput` receiver object that accepts screen sample-buffer
  callbacks and immediately extracts metadata instead of retaining raw frames
- fail-closed test proving the receiver cannot report success before any
  screen frames have been captured
- `SckFrameMetadataStreamRegistration` boundary that creates a real `SCStream`
  from caller-supplied `SCContentFilter` / `SCStreamConfiguration` values and
  registers the screen output receiver on a retained serial dispatch queue
- `SCShareableContent` snapshot boundary that extracts redacted window/display
  candidates with IDs, frames, layer, and visibility state
- window titles and application names are not stored in shareable-content
  candidate snapshots; only title/owner presence flags are retained
- Strict Reveal target selection policy for redacted candidates
- automatic Strict Reveal window selection succeeds only when exactly one
  active, on-screen, app-owned, normal-layer window is eligible
- display capture target selection requires an explicit display ID and fails
  closed when the display is missing
- asynchronous `SCShareableContent` request handling converts completion
  callbacks into redacted candidate snapshots inside the platform boundary
- empty, failed, disconnected, or timed-out shareable-content requests fail
  closed and cannot become trusted capture targets
- `SCStream` start/stop lifecycle helpers now wait for native completion
  callbacks and fail closed on error, disconnection, or timeout
- `SckFrameMetadataStreamRegistration::observe_for` can run a caller-supplied
  stream briefly, stop it, and return observed frame metadata only if frames
  were actually captured
- redacted Strict Reveal targets can now be resolved against the original
  `SCShareableContent` object to build a real `SCContentFilter`
- stream capture plans now create an `SCStreamConfiguration` with audio off,
  cursor hidden, bounded queue depth, BGRA pixels, and target-derived frame size
- `SckFrameMetadataStreamRegistration::from_plan` connects the target plan to
  the metadata observer without product UI exposure
- tests proving observations are assigned by frame time
- desktop UI no longer exposes manual rectangle entry or mask-mode controls;
  those remain internal compatibility/R&D data only
- `manual-qa-secure-share-observe list` can enumerate redacted
  ScreenCaptureKit window/display candidates for local QA
- CLI capture start is intentionally disabled because starting a real
  `SCStream` from the command-line harness can abort inside CoreGraphics before
  DropSquash can return a fail-closed error
- ScreenCaptureKit window candidates now retain only the owner process ID as
  the bridge into Accessibility; titles and application names remain excluded
- macOS Accessibility preflight is wired for the R&D path and fails closed when
  permission is missing
- the GUI-hosted observation report can now include redacted Accessibility
  window-rectangle observation count alongside frame metadata
- core Vision normalized bounding boxes can now be converted from Vision's
  lower-left normalized coordinates into DropSquash top-left output pixels
  without storing recognized text content
- macOS builds now have an `objc2-vision` R&D boundary for
  `VNRecognizeTextRequest` and `VNDetectTextRectanglesRequest`
- macOS builds can now validate a ScreenCaptureKit `CMSampleBuffer` and create
  a `VNImageRequestHandler` for that frame without retaining raw frames in the
  observation callback path
- macOS builds can now configure local Japanese/English text recognition and
  text-shape requests, perform them through `VNImageRequestHandler`, and map
  shape results into `VisionObservation` geometry without reading recognized
  text content
- the ScreenCaptureKit stream callback now performs Vision immediately for
  accepted screen sample buffers and retains only redacted observations, not raw
  frame buffers
- GUI-hosted QA observation DTOs can now report `visionObservationCount`
- `NativeVisionObservationProvider` still fails closed until real Vision
  request output is connected to the generic production snapshot provider

Still not implemented:

- target window/display selection UI
- screen-recording permission request / denial flow
- Accessibility text-element traversal and event timeline
- Japanese/English recognized text and text-shape QA evidence
- Strict Reveal pixel-writer integration
- independent decode and residual-leak verification
- real export path that creates and passes a `MaskPlan` automatically

## ScreenCaptureKit Binding Decision

Use `objc2-screen-capture-kit` for the macOS R&D path.

Rationale:

- it belongs to the same generated `objc2` family already used by DropSquash
- it exposes `SCStream` and `SCStreamFrameInfo*` metadata keys
- the platform crate can enable only `std` and `SCStream` instead of broad
  default features
- keeping the dependency in `dropsquash-platform` preserves the local-first,
  no-ffmpeg media boundary

Current state:

- dependency is present for macOS builds
- `NativeFrameMetadataProvider` references the binding type
- `objc2-core-media` attachment boundary is present for `CMSampleBuffer`
- `RawSckFrameInfo -> CaptureFrameMetadata` mapping exists with validation
- required ScreenCaptureKit attachment presence is checked before conversion
- `CMSampleBuffer` attachment value extraction now reads status, display time,
  scale factor, content scale, content rect, and bounding rect
- observed `CMSampleBuffer` sequences can now be adapted into the generic
  `FrameMetadataProvider` boundary
- an empty observed sample-buffer sequence fails closed instead of producing a
  fake successful capture
- `SCStreamOutput` callback conformance is now compiled and converts screen
  sample-buffer callbacks to metadata immediately
- non-screen callbacks are ignored, and raw sample buffers are not retained by
  the receiver
- a real `SCStream` registration boundary now exists for externally supplied
  filters/configuration and keeps both the receiver and serial callback queue
  alive
- redacted `SCShareableContent` snapshot extraction now exists for
  caller-supplied content objects
- shareable window candidates retain geometry and state but not title text or
  application names
- Strict Reveal window selection now fails closed for zero or multiple eligible
  active window candidates
- explicit display target selection now fails closed when the requested display
  is not present
- asynchronous shareable-content discovery now exists and returns only the
  redacted candidate snapshot or a fail-closed error
- shareable-content request tests cover missing content and timeout without
  invoking real screen-recording permission UI
- `SCStream` start/stop lifecycle helpers now exist and use native completion
  callbacks instead of assuming capture started
- registered streams can now be observed for a caller-supplied duration, then
  stopped before frame metadata is read
- stream capture plans now bridge redacted target selection to real
  `SCContentFilter` / `SCStreamConfiguration` objects
- plan construction fails closed if the selected window/display is missing,
  ambiguous, has invalid scale, or has invalid target dimensions
- the packaged-app startup harness can now start `SCStream`, receive real
  sample buffers, and convert sample-level `SCStreamFrameInfo` attachments into
  redacted frame metadata
- the latest packaged-app R&D run recorded 18 frames for a selected window
  without writing raw pixels, window titles, application names, or recognized
  private text

## Accessibility Binding Decision

Use `objc2-application-services` for the macOS Accessibility R&D path.

Current state:

- dependency is present for macOS builds
- `AXIsProcessTrusted` is checked before AX observation
- missing Accessibility permission fails closed instead of silently continuing
- selected ScreenCaptureKit window targets carry an owner process ID so the AX
  application root can be selected without storing window titles or app names
- the first AX prototype reads `AXWindows`, `AXPosition`, and `AXSize`, then
  maps those bounds into redacted `AxObservation` rectangles
- the QA DTO reports `axObservationCount`

Limitations:

- current AX rectangles are a structure signal, not final canonical output
  coordinates
- text-element, modal, sheet, popover, menu, and event-timeline traversal are
  still required
- local Vision observations are still required for custom-rendered UI,
  browsers, Electron apps, canvas surfaces, and image-embedded text

## Vision Binding Decision

Use `objc2-vision` for the macOS local Vision R&D path.

Current state:

- dependency is present for macOS builds
- `VNRecognizeTextRequest` and `VNDetectTextRectanglesRequest` are the intended
  local text and text-shape request boundaries
- Vision normalized rectangles are converted into output-frame `PixelRect`
  values with the required lower-left-to-top-left origin flip
- the conversion is tested for clamping and floating-point rounding stability
- captured ScreenCaptureKit `CMSampleBuffer` values can now be validated and
  wrapped in a `VNImageRequestHandler`
- local Japanese/English text recognition requests are configured with
  language correction disabled and text-shape requests omit character boxes
- Vision text-shape results can now be converted into redacted
  `VisionObservation` values using geometry and confidence only
- accepted ScreenCaptureKit screen sample-buffer callbacks now attempt Vision
  immediately and fail closed if the request fails
- GUI-hosted observation reports can now include a redacted Vision observation
  count
- `NativeVisionObservationProvider` returns a fail-closed unsupported-media
  error until captured-frame request output is wired into the generic snapshot
  provider

Limitations:

- no permission-granted packaged-app run has recorded real OCR/text-shape
  results against a captured frame yet
- no Japanese or English OCR QA evidence has been recorded yet
- recognized text content must not be stored by default; only geometry,
  reason, source, confidence, and timing should enter `MaskPlan`

## Observation Harness Boundary

`manual-qa-secure-share-observe list` is valid evidence only for redacted
candidate discovery. It proves that DropSquash can ask ScreenCaptureKit for
window/display candidates without storing titles or application names.

It is not valid evidence for captured frame metadata.

Real frame observation must run inside a GUI-hosted harness, preferably the
packaged DropSquash app or a small signed helper app. The command-line xtask
must not start capture, because the current macOS stack can abort inside
CoreGraphics before DropSquash can report a normal fail-closed error.

The next acceptable evidence is:

- selected redacted target ID
- screen-recording permission result
- captured frame count
- first/last frame times
- frame status distribution
- content rect / scale / bounding rect samples
- stop result
- no durable raw unmasked capture artifact

Current GUI-hosted harness entrypoint:

- Tauri command: `manual_qa_secure_share_observe_window`
- gate: `DROP_SQUASH_QA_SCK_OBSERVE=1`
- input: explicit redacted `window_id`, `capture_ms`, `timeout_ms`
- output: frame size, frame count, first/last frame timestamps, and
  Accessibility observation count when AX permission is present

This command is intentionally not exposed in the normal UI. It exists only for
manual QA/R&D runs after a candidate window has been selected from the redacted
candidate list. For high-assurance Secure Share R&D, Screen Recording and
Accessibility permissions are both required; missing Accessibility permission is
a fail-closed result, not a soft warning.

Packaged-app startup harness:

- set `DROP_SQUASH_QA_SCK_OBSERVE_WINDOW_ID` to the redacted candidate window ID
- set `DROP_SQUASH_MANUAL_QA_EVENT_LOG` to a JSONL path
- optional: `DROP_SQUASH_QA_SCK_OBSERVE_CAPTURE_MS`
- optional: `DROP_SQUASH_QA_SCK_OBSERVE_TIMEOUT_MS`
- optional: `DROP_SQUASH_QA_SCK_OBSERVE_QUIT_AFTER=1`

When the window ID is present, the packaged app records one
`secure-share-observe-window` event. The event `detail` string contains JSON
with either `status: "ok"` and redacted frame metadata, or `status: "error"`
with a fail-closed reason. It must not contain raw frame pixels, window titles,
application names, or recognized private text.

Current local evidence:

- app artifact:
  `/Users/masakitakemura/_workspace/drop-squash/target/release/bundle/macos/DropSquash.app`
- `DROP_SQUASH_QA_INSTANCE_ID=sck_frame_metadata` successfully used a separate
  QA instance lock without closing the user's existing DropSquash app
- startup harness wrote `secure-share-observe-window` JSONL events
- previous successful frame-metadata event:
  `/tmp/dsq-secure-share-observe-frame-metadata-1784451193.jsonl`
- observed redacted result:
  `status = ok`, `windowId = 3328`, `frameWidth = 1691`,
  `frameHeight = 950`, `frameCount = 18`,
  `firstFrameTimeNs = 4672453313239`,
  `lastFrameTimeNs = 4672460422746`
- this proves frame metadata capture only; AX structure, Vision observations,
  frame-exact MaskPlan synthesis, destructive masking, and independent final
  output verification remain incomplete
- earlier Accessibility-enabled harness run:
  `/tmp/dsq-secure-share-observe-ax-1784453411.jsonl`
- observed fail-closed result:
  `status = error`; ScreenCaptureKit discovery failed because macOS TCC
  reported that the user denied app/window/display capture
- this showed that Screen Recording permission had to be granted before AX or
  Vision observation quality could be measured
- latest permission-granted AX/Vision harness run:
  `/tmp/dsq-secure-share-observe-vision-1784457000.jsonl`
- observed redacted result:
  `status = ok`, `windowId = 3328`, `frameWidth = 1691`,
  `frameHeight = 950`, `frameCount = 2`, `axObservationCount = 6`,
  `visionObservationCount = 268`,
  `firstFrameTimeNs = 4811040017270`,
  `lastFrameTimeNs = 4811040428285`
- a shorter prior run at
  `/tmp/dsq-secure-share-observe-vision-1784456934.jsonl` recorded
  `frameCount = 1`, `axObservationCount = 6`, and
  `visionObservationCount = 133`, which is consistent with the two-frame run
- this proves that the packaged GUI-hosted harness can run local Vision on
  captured ScreenCaptureKit frames and record only redacted observation counts;
  it still does not prove Japanese/English detection quality, Strict Reveal
  mask synthesis, destructive pixel writing, or independent final-output leak
  verification

## Two Product Modes

### Smart Mask

Smart Mask automatically destroys likely sensitive text and UI regions while
trying to keep enough context for QA review.

It may use:

- Accessibility element frames
- window and sheet structure
- local Vision text and text-shape detection
- temporal tracking
- fixed toolbar and notification heuristics

Smart Mask is helpful, but it must not be sold as guaranteed private or
complete.

### Strict Reveal

Strict Reveal is the stronger commercial candidate.

Default behavior:

- unknown regions are destroyed
- client-area text is destroyed
- transient overlays are destroyed
- raw window titles are destroyed
- only explicitly safe regions are preserved or synthetically re-rendered

This mode is less visually rich, but it is the only realistic route toward a
high-assurance security claim.

## Required Architecture

### 1. Window Scope Layer

Use ScreenCaptureKit for built-in capture when high assurance is required.

Owns:

- selected display, window, or app scope
- per-frame capture metadata
- frame status checks
- scale and coordinate metadata
- cursor and audio policy

Rules:

- process only complete frames
- treat missing frame metadata as unknown
- keep audio disabled by default in secure modes
- never depend on metadata from an already-existing `.mov` for provenance

### 2. Accessibility Structure Layer

Use Accessibility as the strongest first-party structure signal.

Owns:

- focused window and modal structure
- title bar, sheet, dialog, popover, menu, and text-element frames
- text range bounds when available
- process and app identity hints

Rules:

- permission missing means the high-assurance mode cannot run
- incomplete AX structure marks the affected region unknown
- raw AX text is not required for masking; frames are enough

### 3. Vision Observation Layer

Use local Apple Vision to cover custom-rendered UI, browsers, Electron apps,
canvas surfaces, and image-embedded text.

Owns:

- Japanese and English text recognition
- text-rectangle and character-box observations
- text-shape regions even when OCR content is uncertain
- confidence scores

Rules:

- OCR classification can refine policy, but text existence must not depend on
  recognized strings only
- transient one-frame detections still mask immediately
- Vision is local-only, but it is still a neural-network analyzer; public copy
  must not pretend this is a simple deterministic OCR engine

### 4. Temporal Resolver

Use time to stabilize and expand masks, not to delay them.

Owns:

- observation tracking across frames
- unioning repeated detections
- expanding uncertain bounds
- suppressing obvious visual noise only when fail-closed policy permits it

Rules:

- do not require 5-15 frames before masking a possible secret
- one-frame secrets, notifications, tokens, and dialogs must be destroyed
- temporal voting may only make masks bigger or more stable

### 5. Policy Resolver

Convert all observations into a frame-exact mask plan.

Policy classes:

- `safe`: may remain visible
- `sensitive`: must be destroyed
- `unknown`: destroyed in Strict Reveal; risky in Smart Mask

Rules:

- raw title bars are not safe by default
- URLs, filenames, ticket titles, customer names, and notification content are
  sensitive
- "function names" are safe only from an explicit static allowlist or synthetic
  re-rendering, not because OCR guessed them
- overlapping regions are unioned before writing pixels
- observations must apply only to matching frame times; unmatched observations
  become full-frame verification-required masks in Strict Reveal and still need
  export-path failure/deletion handling

### 6. Destructive Pixel Writer

Apply masks after final-size scaling and immediately before encode.

Owns:

- CVPixelBuffer mutation
- row-stride-safe writes
- color format handling
- padding for scaling, chroma, and filter edge bleed

Rules:

- blur is prohibited for secure modes
- destructive overwrite failure aborts the export
- masked output must never silently fall back to a plain export

### 7. Independent Verification

Decode the final output and run an independent residual-leak check.

Owns:

- output decoding
- residual text/shape scan
- receipt verification fields
- fail/delete behavior

Rules:

- do not use the exact same detector as the only verification proof
- if verification finds unmasked residual sensitive regions, delete the output
- receipts are tamper-evident, not proof that detection was perfect

## Existing Recording Boundary

Existing `.mov` / `.mp4` files can still be post-processed, but they cannot
provide high-assurance provenance because they lack:

- ScreenCaptureKit frame metadata
- trusted window identity
- Accessibility event timeline
- exact capture-time scale and coordinate state
- capture-time transient overlay context

Therefore existing-file Secure Share is an assistive sanitizer, not the final
paid security product.

## GStreamer Boundary

GStreamer can be a media transport and pixel-mutation backend on Linux.

It cannot provide macOS window identity, Accessibility structure, or Apple
Vision semantics. Keep the core `MaskPlan` platform-neutral, but generate the
best observations with each OS-native stack.

## R&D Gates Before Sales

Do not resume public checkout or production payment setup until these gates are
accepted:

1. threat model is written and accepted:
   [docs/secure-share-threat-model.md](/Users/masakitakemura/_workspace/drop-squash/docs/secure-share-threat-model.md)
2. supported app/window/language/display matrix is written:
   [docs/secure-share-support-matrix.md](/Users/masakitakemura/_workspace/drop-squash/docs/secure-share-support-matrix.md)
3. adversarial Japanese and English corpus exists:
   [docs/secure-share-adversarial-corpus.md](/Users/masakitakemura/_workspace/drop-squash/docs/secure-share-adversarial-corpus.md)
4. ScreenCaptureKit capture prototype records frame metadata
5. Accessibility prototype maps window and text-element frames
6. Vision prototype finds Japanese and English text regions locally
7. Policy Resolver emits deterministic frame-exact `MaskPlan`:
   [docs/secure-share-maskplan.md](/Users/masakitakemura/_workspace/drop-squash/docs/secure-share-maskplan.md)
8. Strict Reveal fail-closed export destroys unknown regions
9. independent decode verification exists
10. receipts explain sources, reasons, confidence, and verification result
11. packaged-app manual QA proves pass and fail paths
12. user review says the result is commercially convincing

Current implementation foundation:

- platform-neutral `MaskPlan` types exist in
  `crates/dropsquash-core/src/secure_share`
- ScreenCaptureKit-like frame metadata can be converted into `FrameMaskPlan`
- Strict Reveal treats untrusted capture frames as full-frame unknown regions
- Accessibility and Vision observation result types can become sensitive
  `MaskRegion` values without storing recognized private text
- `CoordinateSpace` can map capture-space rectangles into final output pixels
  with clamp and invalid-rectangle rejection
- `dropsquash-platform` has real macOS ScreenCaptureKit, Accessibility, and
  local Vision adapters for the packaged Phase 5 alpha
- `MaskPlanDraft` can combine capture metadata, Accessibility observations,
  and Vision observations into a `MaskPlan`
- platform observation snapshots can be converted into `MaskPlan` with an
  explicit policy and verification expectation
- historical packaged Strict Shield evidence exists for
  `tests/fixtures/secure-share/NativeAccessibilityFixture.swift`; current
  Phase 5 native sheet, popover, and Japanese IME evidence is summarized in
  `docs/phase5-alpha.md`

Still not implemented:

- local Vision quality comparison against the adversarial annotation corpus
- OS-native coordinate extraction and display-scale edge-case proof
- frame-exact AX/Vision observation timing; current composition is a first
  deterministic bridge, not final timing behavior
- selective output that can retain any pixels without the Strict Shield guard

## Next Implementation Goal

The next work should not be Lemon Squeezy, public checkout, or a broader media
feature.

The next work is:

```text
Build selective-mask evidence for high-assurance Secure Share. The existing
macOS Strict Shield path already produces a deterministic MaskPlan, destructive
final frames, fail-closed output deletion, and independent verification. Do
not sell or publicly imply completed privacy protection until selective retained
regions have adversarial evidence.
```

Start with:

- [docs/secure-share-threat-model.md](/Users/masakitakemura/_workspace/drop-squash/docs/secure-share-threat-model.md)
- [docs/secure-share-support-matrix.md](/Users/masakitakemura/_workspace/drop-squash/docs/secure-share-support-matrix.md)
- [docs/secure-share-adversarial-corpus.md](/Users/masakitakemura/_workspace/drop-squash/docs/secure-share-adversarial-corpus.md)
- [docs/secure-share-maskplan.md](/Users/masakitakemura/_workspace/drop-squash/docs/secure-share-maskplan.md)

## Non-Goals

Do not add these while this R&D gate is active:

- YouTube downloading
- advertising SDKs
- timeline editing
- broad format expansion
- media-library workflows
- production Stripe or Lemon Squeezy setup
- public claims that imply leak-zero
