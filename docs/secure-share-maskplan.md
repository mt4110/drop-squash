# Secure Share MaskPlan Specification

`MaskPlan` is the deterministic bridge between observations and destructive
pixel writing.

It must be stable enough to test without depending on the UI.

## Pipeline Position

```text
ScreenCaptureKit frame metadata
ScreenCaptureKit redacted target candidate snapshot
Native CMSampleBuffer stream adapter
Accessibility observations
Vision observations
  -> canonical coordinates
  -> Policy Resolver
  -> frame-exact MaskPlan
  -> final-size CVPixelBuffer destructive overwrite
  -> encode
  -> independent verification
```

## Coordinate System

Use canonical output-frame coordinates:

- origin: top-left
- unit: final output pixel
- frame size: final encoded frame size
- rectangles: integer pixel bounds
- end bounds: exclusive

All source observations must be transformed before entering the final plan.

Transform inputs:

- selected capture target frame from redacted ScreenCaptureKit candidates
- ScreenCaptureKit content rect
- bounding rect
- scale factor
- content scale
- output size
- window position per frame
- resize state per frame
- Vision normalized bounding boxes, which use normalized coordinates and must
  be flipped from lower-left origin into top-left output-frame pixels

If the transform cannot be proven, the affected frame is untrusted and the
export fails closed or destroys the full affected region.

Vision transform rule:

```text
left = x
right = x + width
top = 1 - (y + height)
bottom = 1 - y
```

The resulting normalized rectangle is clamped to the final frame and then
converted to integer output pixels. The transform must not store recognized
text content by default.

## Core Types

Draft Rust-like shape:

```rust
struct MaskPlan {
    schema_version: u32,
    capture_id: String,
    frame_size: FrameSize,
    frames: Vec<FrameMaskPlan>,
    policy: MaskPolicy,
    audit: MaskPlanAudit,
    verification_expectations: VerificationExpectations,
}

struct MaskPlanAudit {
    unmatched_observations: Vec<UnmatchedObservation>,
    verification_required_frame_count: usize,
}

struct FrameMaskPlan {
    frame_index: u64,
    presentation_time_ns: u64,
    frame_status: FrameStatus,
    regions: Vec<MaskRegion>,
}

struct TimeRangeNs {
    start_ns: u64,
    end_ns: u64,
}

struct AxObservation {
    rect: PixelRect,
    time_range: TimeRangeNs,
    kind: AxObservationKind,
    confidence: Confidence,
}

struct VisionObservation {
    rect: PixelRect,
    time_range: TimeRangeNs,
    kind: VisionObservationKind,
    confidence: Confidence,
}

struct MaskRegion {
    rect: PixelRect,
    policy: RegionPolicy,
    reason: MaskReason,
    sources: Vec<ObservationSource>,
    confidence: Confidence,
    expansion_px: u32,
}
```

## Region Policy

| Policy | Meaning | Strict Reveal |
|---|---|---|
| `safe` | May remain visible | Preserve |
| `sensitive` | Must be destroyed | Destroy |
| `unknown` | Cannot prove safe | Destroy |

Smart Mask may expose unknown regions as risk, but Strict Reveal destroys them.

### Phase 5 Alpha Strict Shield

For a capture that is eligible for sharing during Phase 5 alpha, add an
`ax_unknown_client_area` region covering the selected target. This is an
explicit safety guard, not a detection result: Accessibility and Vision remain
in the plan as redacted research observations, but neither may authorize a
pixel to remain visible. A future selective mode must prove its retained
regions against an adversarial corpus and independent residual-leak checks
before it can replace this guard.

## Mask Reasons

Required reason categories:

- `ax_text_element`
- `ax_modal_body`
- `ax_unknown_client_area`
- `vision_text`
- `vision_text_shape`
- `transient_overlay`
- `dynamic_title`
- `url_or_filename`
- `notification`
- `unknown_region`
- `verification_required`

Reasons must be stable strings or enums. Do not hide reasons in free-form UI
copy.

## Observation Sources

Required source categories:

- `screen_capture_kit_frame`
- `accessibility_window`
- `accessibility_text`
- `accessibility_modal`
- `vision_text_recognition`
- `vision_text_rectangle`
- `temporal_tracker`
- `policy_default`
- `verification_feedback`

Each source should include enough fields for debugging without storing private
text content by default.

## Confidence

Confidence is not permission to preserve sensitive regions.

Use confidence for:

- prioritizing review output
- expanding uncertain regions
- explaining receipt summaries
- deciding when Smart Mask should warn

Do not use confidence to skip destruction in Strict Reveal.

Suggested shape:

```rust
struct Confidence {
    detection: f32,
    transform: f32,
    policy: f32,
}
```

Values are in `0.0..=1.0`.

## Region Union Rules

Before pixel writing:

1. Clamp regions to frame bounds.
2. Expand regions by source-specific padding.
3. Merge overlapping and adjacent destructive regions.
4. Prefer destructive policy over safe policy.
5. Preserve safe regions only after destructive union is complete.

Priority:

```text
sensitive > unknown > safe
```

## Temporal Rules

Core `MaskPlan` construction assigns Accessibility and Vision observations only
to frames whose `presentation_time_ns` falls inside the observation
`time_range`.

Temporal logic may:

- stabilize recurring regions
- expand uncertain moving regions
- track regions between analyzer frames
- improve receipt summaries

Temporal logic must not:

- wait for multiple frames before masking a possible one-frame secret
- shrink a destructive mask unless verification proves the shrink is safe
- convert unknown to safe without an explicit allowlist or trusted structure

If an observation cannot be matched to a frame time, the real high-assurance
export path must fail closed or mark the affected frame unknown. It must not
silently attach the observation to an unrelated frame.

Current core behavior:

- matching observations are attached only to matching frame times
- unmatched observations in Strict Reveal add full-frame
  `verification_required` / `unknown` regions
- unmatched observation source, reason, and time range are recorded in
  `MaskPlanAudit`
- unmatched observations must never be used as proof that a frame is safe

## Pixel Writer Contract

The writer receives only final `MaskRegion` data in final output coordinates.

It must:

- write after final resize
- write before encode
- support row stride
- support the active pixel format
- apply edge padding for filter and chroma bleed
- fail the export on write errors

Blur is not a valid secure write mode.

## Verification Expectations

`MaskPlan` should carry enough expectations for independent verification:

- final frame size
- destructive region list or digest
- expected stripped tracks
- expected no audio unless explicitly allowed
- expected receipt output hash
- verification policy version

The verifier must decode the final output and check the encoded result, not
only trust pre-encode state.

## Receipt Summary

Receipts should not dump every frame by default. They should summarize:

- policy mode
- supported scope
- sources used
- unmatched observation count and reasons
- verification-required frame count
- region counts by reason
- confidence rollup
- verification result
- output hash
- implementation version

Current receipt-facing implementation keeps this summary bounded:

- unmatched observation count
- unmatched reasons
- verification-required frame count

It intentionally does not store raw OCR text, full local paths, or full
per-frame masks in the receipt summary.

Desktop and CLI receipt save paths already accept an optional `MaskPlan`.
Current manual-rectangle Secure Share exports pass `None` because they do not
yet produce trusted ScreenCaptureKit / Accessibility / Vision-derived plans.

A privacy-crate integration test now proves the pure-data path:

```text
SecureShareObservationSnapshot -> MaskPlan -> SecureShareReceipt
```

This is not evidence that real macOS capture works yet. It only proves that the
deterministic data path is wired.

The platform layer also has an injectable `SecureShareSnapshotProvider`
boundary. Real ScreenCaptureKit, Accessibility, and Vision adapters should plug
into that boundary instead of bypassing the deterministic `MaskPlan` path.

For R&D, the snapshot boundary is also split by signal:

- `FrameMetadataProvider`
- `AccessibilityObservationProvider`
- `VisionObservationProvider`

The composition helper builds one `SecureShareObservationSnapshot` from those
three sources, so each adapter can be tested independently.

The current native frame metadata provider is intentionally fail-closed. It
returns an explicit unsupported error until the real ScreenCaptureKit binding
and metadata extraction path exist.

The selected binding is `objc2-screen-capture-kit`. The next implementation
must use ScreenCaptureKit frame metadata keys such as status, display time,
scale factor, content scale, content rect, and bounding rect to populate
`CaptureFrameMetadata`.

Current mapper state:

- `RawSckFrameInfo` maps validated raw ScreenCaptureKit-like values into
  `CaptureFrameMetadata`
- required ScreenCaptureKit attachment presence is checked before raw value
  conversion
- real `CMSampleBuffer` attachment value extraction maps SCK status, display
  time, scale, content rect, and bounding rect into `RawSckFrameInfo`
- `NativeSampleBufferProvider` defines the macOS-only source boundary for
  observed sample-buffer streams
- `SampleBufferFrameMetadataProvider` converts observed sample buffers into the
  generic `FrameMetadataProvider` timeline
- empty sample-buffer streams fail closed and cannot become a trusted
  `MaskPlan` frame timeline
- `SckStreamFrameMetadataOutput` implements the `SCStreamOutput` callback
  receiver and extracts raw frame metadata immediately
- non-screen sample-buffer callback types are ignored
- the callback receiver does not retain raw `CMSampleBuffer` frames
- validated `CMSampleBuffer` frames can now be wrapped in a local
  `VNImageRequestHandler`; local Japanese/English text recognition and
  text-shape requests can be performed and shape results can become redacted
  `VisionObservation` geometry without retaining recognized text
- accepted screen sample-buffer callbacks now run Vision immediately and keep
  only redacted observations; packaged-app evidence at
  `/tmp/dsq-secure-share-observe-vision-1784457000.jsonl` recorded
  `frameCount = 2`, `axObservationCount = 6`, and
  `visionObservationCount = 268`
- `SckFrameMetadataStreamRegistration` creates a real `SCStream`, registers the
  screen output receiver, and keeps a serial dispatch queue alive for callbacks
- `snapshot_shareable_content` extracts redacted window/display candidates from
  `SCShareableContent`
- shareable candidate snapshots keep IDs, geometry, layer, and visibility state
  but do not store window title text or application names
- `select_strict_reveal_window_target` accepts only one eligible active,
  on-screen, app-owned, normal-layer window
- ambiguous or missing Strict Reveal window candidates fail closed
- `select_explicit_display_target` accepts only caller-requested display IDs
  present in the redacted snapshot
- empty content/bounding rects are rejected
- non-finite or non-positive scale values are rejected

The platform crate now has the typed extraction entry point:

```text
CMSampleBuffer -> RawSckFrameInfo
NativeSampleBufferProvider -> SampleBufferFrameMetadataProvider
SampleBufferFrameMetadataProvider -> FrameMetadataProvider
SCStreamOutput callback -> RawSckFrameInfo
SckStreamFrameMetadataOutput -> FrameMetadataProvider
SCContentFilter + SCStreamConfiguration -> SCStream output registration
SCShareableContent -> redacted window/display candidate snapshot
redacted candidate snapshot -> fail-closed target selection
```

This is still not proof that real capture works. The next implementation must
connect asynchronous shareable content discovery, target selection UI,
permission handling, capture start/stop, and then record evidence that captured
sample buffers produce complete, validated `RawSckFrameInfo` values using
`objc2-screen-capture-kit` keys.

Full frame-level MaskPlan logs may be emitted only in explicit debug or QA mode
and must avoid private recognized text by default.

## First Implementation Slices

1. Add platform-neutral `MaskPlan` types in a core or privacy crate. Done in
   `crates/dropsquash-core/src/secure_share`.
2. Add deterministic serialization tests. Done in
   `crates/dropsquash-core/src/secure_share/tests.rs`.
3. Add ScreenCaptureKit-like frame metadata types and strict untrusted-frame
   handling. Done in `CaptureFrameMetadata::to_mask_frame`.
4. Add Accessibility and Vision observation result types without storing
   recognized private text. Done in `AxObservation` and `VisionObservation`.
5. Add coordinate transform unit tests. Initial `CoordinateSpace` tests cover
   content offsets, output scaling, clamp, and invalid rectangles.
6. Add policy union tests. Initial destructive-priority test exists.
7. Add receipt summary conversion tests.
8. Feed the current fixed-bar detector into `MaskPlan` only as a low-assurance
   source.
9. Add platform observation boundary. Initial `SecureShareProbe` exists in
   `dropsquash-platform` and fails explicitly until real adapters are added.
10. Add observation snapshot to `MaskPlan` composition. Initial
   `MaskPlanDraft` and platform snapshot bridge exist.
11. Add frame-exact observation timing so AX/Vision observations attach to the
   correct frame range instead of the first frame.
12. Add real ScreenCaptureKit/AX/Vision adapters after the schema is stable.

## Acceptance

This spec is accepted when:

- it is linked from `docs/secure-share-beta.md`
- it is linked from `docs/p2-alpha-review.md`
- the first Rust type implementation follows this shape
- tests prove deterministic serialization and union priority
