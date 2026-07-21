# Phase 5 Alpha: Capture Continuity And Exposure Coverage

Phase 5 expands the Phase 4 Strict Shield research path. It does not claim leak-zero behavior, enterprise readiness, or completed selective redaction.

## Normalized-Time Packaged-App Baseline

On 2026-07-21, the implementation corrected ScreenCaptureKit display time from
raw `mach_absolute_time` ticks to nanoseconds using `mach_timebase_info`.
The Developer ID-signed packaged app then completed a native-fixture recording
with normalized timestamps: 144 frames were blackened, independently decoded,
and verified before publication. The signed sidecar and final MP4 passed the
independent evidence verifier (`sha256`
`fb3da3c40c285bf70194eabae611adc0ddd148d0853a05457b88d829c6d834cf`).

Earlier tick-as-nanosecond runs remain historical destructive-output evidence
only. This baseline proves only the stated Strict Shield path on this fixture;
it is not selective-masking, multi-display, IME, overlay, or leak-zero evidence.

## OCR Analysis Budget

The local QA observation report records only aggregate Vision work: analyzed
frame count, total elapsed nanoseconds, and maximum elapsed nanoseconds. It
never records recognized text or image bytes. These values exist to decide
whether a bounded capture-to-analysis pipeline can preserve each frame without
relaxing continuity; they are not a performance claim. A capture that cannot
retain, analyze, destructively overwrite, and encode every required frame must
discard its partial output.

The native bridge admits ScreenCaptureKit metadata under a short lock, copies
only sampled owned BGRA bytes for local Vision, then serializes the destructive
pixel overwrite and writer append. The capture callback never transfers a
`CMSampleBuffer` or `CVPixelBuffer` to Rust. Vision samples the first accepted
frame and every 15th frame thereafter; at most three owned byte samples may
wait for the native worker. Sample-queue exhaustion, copy failure, Vision
failure, or a pending worker at stop rejects and discards the output.

### Current Measurement

On the signed native-fixture QA run, the first-frame-only experiment still
failed closed: one local Vision analysis took `376060750ns`, and the next
ScreenCaptureKit presentation-time gap was `430341875ns` (the limit is
`166666665ns`). This is evidence that even a single Vision request cannot run
inside the ScreenCaptureKit callback. Sampling at two frames per second is the
current strict-shield research compromise; it is not a selective-mask pass.

`DROP_SQUASH_QA_SMART_MASK_VISION_FIRST_FRAME_ONLY=1` enables this experiment
only in the QA route. It is not exposed by the product UI or eligible for a
signed Secure Share receipt.

The native bridge now copies accepted frames into bounded owned BGRA bytes
before Vision runs. It does not move `CMSampleBuffer` or `CVPixelBuffer`
objects across the Rust boundary. A native serial worker reconstructs a local
pixel buffer only for Vision; the capture queue keeps the destructive black
write and writer append in presentation-time order. This supports aggregate
observation evidence for Strict Shield, not selective-mask throughput or a
selective-sharing claim.

### Native Capture Boundary

The legacy Rust callback path transfers `SCShareableContent` into a recording
thread through a retained raw pointer. The active binding does not declare that
object `Send`. It is therefore not an acceptable high-assurance path and must
not publish a new Secure Share artifact.

The required ordering and admission rules for a future native selective
experiment are fixed in [Phase 5 Native Selective Pipeline](phase5-native-selective-pipeline.md).
That document deliberately forbids sending a frame to the writer before the
same frame's native observations and destructive decision have completed.

Phase 5 now has a separate Objective-C bridge that keeps `SCShareableContent`,
`SCStream`, `CMSampleBuffer`, `CVPixelBuffer`, `AVAssetWriter`, and AX element
traversal on the native side. Rust receives only frame index, display-time
ticks, status, scales, and redacted geometry through C ABI callbacks. The AX
callback includes text-element rectangles and a stable kind, never a role name,
AX value, title, or recognized text. The bridge samples this redacted AX
geometry on the first accepted frame and then every 15 accepted frames; each
sample receives the corresponding capture-frame time. An AX element without
usable bounds is omitted from this optional structure observation; callback
backpressure or a malformed callback value fails closed. AX geometry has its
own bounded 2,048-observation bridge queue so a normal complex window does not
consume the much smaller per-frame metadata/Vision queue; exhausting that bound
still fails closed. Before finalization, Rust independently requires the C ABI
metadata sequence to be nonempty, exactly indexed, complete, monotonic, within
the five-frame gap policy, and geometrically identical to its first frame.
Any disagreement discards the partial output. The full-frame Strict Shield
destruction does not depend on
a text element being observable. The
bridge has compiled, linked, and passed its in-memory destructive-blackening
test. The desktop recording command now uses this value-only route and passes
its completed MP4 through the existing verification and signed-evidence
boundary. The Developer ID-signed packaged app has validated this route once
against the native accessibility fixture: 144 frames, 70 local AX geometry
observations, 80 local Vision region observations, destructive blackening,
independent decode verification, and signed-evidence verification all passed.
This is one Strict Shield baseline, not a completed adversarial corpus or a
selective-mask pass.

## Objective

For a selected macOS window, reduce the gap between what the user intended to
record and what the local capture pipeline can prove it destroyed before
publication. When DropSquash cannot prove required continuity or coverage, it
must fail closed instead of saving a video.

Every row uses one or more explicit states: `covered`, `fail-closed`,
`detected-but-not-covered`, or `out-of-scope`. A fixture is `covered` only for
the stated Strict Shield path: every captured frame is destructively blackened
and independently decoded before publication. It never implies selective
masking, every macOS surface, or leak-zero behavior.

The independent verifier reads decoded 32BGRA frames, not compressed H.264
sample order. It rejects an invalid, repeated, reversed, or over-five-interval
decoded presentation-time sequence before accepting the MaskPlan frame order.
This is an output-continuity check; it does not claim that decoded timestamps
alone prove every compositor update was captured.

## Two Evidence Axes

Each exposure path has two deliberately separate classifications in
[`build-week-phase5-evidence.json`](build-week-phase5-evidence.json):

- `outputPixelStatus` answers whether pixels delivered in an accepted
  selected-window frame can remain visible in the published MP4. For this
  Strict Shield path it is `covered`: the entire output frame is overwritten
  before encode and independently decoded as black.
- `sourceSurfaceStatus` answers whether DropSquash can attest that a specific
  macOS or third-party surface was observed. It may remain
  `detected-but-not-covered` even when output pixels are covered. A real IME
  candidate, password-manager panel, or Notification Center compositor is not
  promoted to observed merely because a synthetic fixture exists.

This separation is important. It prevents the evidence from turning
full-frame destruction into an unsupported claim about all operating-system
overlays or selective masking.

The current native-bridge rerun procedure is in
[Phase 5 Native Bridge QA](phase5-native-bridge-qa.md). Historical fixture
notes below do not replace that rerun.

## Phase 5 Coverage Matrix

The rows below distinguish the current native bridge from historical Phase 4
and early Phase 5 fixture notes later in this document. Any later reference to
a `250ms watchdog`, title-only attestation, or raw-pointer capture callback is
historical evidence for that former route, not evidence for the current native
desktop command.

| Exposure path | Required signal | Phase 5 status | Export rule |
| --- | --- | --- | --- |
| Selected wrong window | Explicit UI choice plus native CoreGraphics ID, owner PID, and frame attestation | Fail-closed for an observed mismatch | Reject a missing, different-owner, moved, or resized target; title-only substitution is not separately attested |
| Owner replacement before start | Native CoreGraphics owner PID attestation | Fail-closed | Require the selected owner PID and frame to match before capture |
| Window move or resize before/after capture | Native CoreGraphics and AX frame attestation plus native AX move/resize events | Fail-closed in code; normal packaged-app baseline passed | Reject and discard when either source mismatches or an AX event arrives; the individual adversarial rerun remains separate evidence |
| Selected window hidden during capture | Native CoreGraphics and AX attestation per accepted frame | Fail-closed for an observed disappearance | Reject and discard when either source cannot find the selected target |
| Foreground application change | NSWorkspace activation PID allowlist | Fail-closed for observed third-party app activation | Permit the selected target and recorder to alternate; reject any other observed foreground app. This is not z-order or overlay attestation. |
| Same-app secondary-window focus | Per-frame focused AX element must resolve to the selected AX window | Detected-but-not-covered until packaged-app fixture passes | Code rejects when an input element belongs to another window from the same application; PID equality alone is insufficient. |
| Observable geometry or scale change during capture | Per-frame content rect, bounding rect, and scale | Detected-but-not-covered for current source-surface evidence | Reject and discard when the first complete-frame geometry no longer matches; a fresh signed change fixture is required before restoring a fail-closed claim |
| Title changes or silent geometry change during capture | Per-frame native frame metadata plus CoreGraphics and AX frame attestation | Fail-closed for observed geometry; detected-but-not-covered for title-only change | No window title is retained or compared by the native route; a title-only mutation is not a coverage claim |
| Retina or output-size transform | Capture scale, content scale, final frame size | Covered | Reject invalid or mismatched transform; a per-frame scale change also rejects and discards |
| Negative global desktop coordinates | Signed AX coordinates mapped relative to the selected window | Detected-but-not-covered until multi-display QA passes | The bridge keeps AX positions signed until it validates and maps them into capture pixels. This does not expand the supported multi-display claim. |
| Observed non-complete frame | SCK frame status | Detected-but-not-covered for current source-surface evidence | Reject and discard on Idle, Blank, Started, Suspended, Stopped, or Unknown status; a fresh native harness or packaged-app run is still required |
| Pixel buffer unavailable | Vision and destructive-write input | Covered | Reject and discard; never silently omit the frame |
| Vision sample worker capacity or callback failure | First frame plus every 15th frame, with three owned BGRA copies and a value-only callback | Fail-closed in code; deterministic native failure injection available | Reject and discard instead of omitting a scheduled Vision sample or retaining an unbounded queue. The injected probe has not yet been recorded as packaged-app evidence. |
| Reordered or repeated frame | Monotonic presentation time | Detected-but-not-covered for current source-surface evidence | Reject and discard on repeated or reversed timestamps; adversarial ordering input remains harness-only |
| Silent frame gap | Normalized `mach_absolute_time` presentation timestamps | Detected-but-not-covered for current source-surface evidence | Reject a gap longer than five configured frame intervals. Earlier tick-as-nanosecond evidence is superseded; gap injection remains a separate harness test. |
| Screen lock, sleep, or wake | SCK non-complete status, presentation-time continuity, and NSWorkspace lifecycle notifications | Fail-closed when macOS sends a watched lifecycle notification | Reject `Suspended`/other non-complete frames, a resumed gap, session resignation, screen sleep, workspace sleep, power-off, or active-Space change; physical lock/sleep behavior is not claimed beyond observed events |
| IME composition/candidate window | Per-frame AX focused-text geometry tied to the selected AX window, plus transient-window observations | Detected-but-not-covered | Native focused-text code is integrated; packaged-app evidence and no selective claim remain pending |
| Rapid transient text or focused input (50ms fixture) | Per-frame AX focused-text geometry, native pixel-change candidate, Strict Shield, independent decoded verification | Detected-but-not-covered | Code integration is complete; a fresh signed packaged-app run is required, and this is not an IME or private-input claim |
| Modal, sheet | AX hierarchy plus Vision observations | Detected-but-not-covered | Historical destructive fixture run needs normalized continuity rerun; no selective claim |
| Native popover | AX hierarchy plus Vision observations | Detected-but-not-covered | Historical destructive fixture run needs normalized continuity rerun; no selective claim |
| Notification or overlay | SCK pixels plus transient observation timeline | Covered for Strict Shield output; detected-but-not-covered for OS compositing | Synthetic fixture passed and a native notification was sent during a packaged-app recording; no claim that its banner was composited into the selected-window pixels |
| AX/Vision disagreement | Redacted cross-source overlap audit | Detected-but-not-covered | Record counts only; Strict Shield destroys every frame and selective sharing remains blocked |
| Frame-to-frame pixel change | Native-owned exact BGRA comparison with a redacted changed rectangle | Code-integrated; packaged-app evidence pending | Every detected change becomes an unknown destructive candidate; it never authorizes pixels to survive |
| Audio, captions, metadata | Final-asset inspection | Covered by Phase 4 | Reject audio or metadata |
| Finder or Quick Look preview | macOS-generated derivative artifact | Detected-but-not-covered | Strict Shield MP4 frames are independently black-verified, but macOS-generated previews are not separately enumerated or verified yet |
| Display connection, mode, mirror, or rotation change | CoreGraphics display-configuration callback | Detected-but-not-covered for physical device evidence | Reject and discard if macOS reports any display configuration change during capture; physical attach/detach/mirror/rotation still needs device QA |
| Other displays | Capture scope configuration and per-frame geometry | Fail-closed for observed change; detected-but-not-covered otherwise | Window-only capture. An observed geometry or scale change rejects, but a cross-display move that leaves those signals unchanged is not proven |
| Non-JA/EN, handwriting, QR, faces | Dedicated detector and corpus | Out of scope | Strict Shield only; no selective claim |

## Current Start-Time Target Revalidation

Before the native stream starts, Phase 5 requires the explicitly selected
window to still match its CoreGraphics identity tuple:

```text
same window ID + owner PID + captured frame + on-screen
```

The native route does not retain or compare raw window titles. The title is not
written to the MaskPlan, receipt, logs, or output. This removes title content
from the evidence surface, but it also means title-only mutation is not an
attested failure condition and does not enable a selective-sharing claim.

This prevents a stale selection from silently becoming an off-screen, ownerless,
overlay-layer, zero-size, different-owner, or different-frame capture target.
After stopping, the native writer completion repeats CoreGraphics and AX frame
attestation before it reports success. During capture, every accepted frame
repeats those checks and validates ScreenCaptureKit metadata. This does not
prove an arbitrary transient change that begins and ends between accepted
frames. A legitimate move or resize is not silently accepted for selective
masking: it needs a frame-exact transform proof before support may expand.

After native stop completion, DropSquash synchronously drains the serial
ScreenCaptureKit output queue before it reads frame evidence, finalizes the
writer, or constructs the MaskPlan. This closes the ordinary queued-callback
boundary: a callback already delivered to that queue cannot be omitted merely
because the stop completion arrived first. It does not prove that macOS emitted
every compositor update as a complete capture frame; missing or discontinuous
frames remain fail-closed conditions.
The native target/display continuity checks also remain active until native stop completion,
so the interval between the user's stop request and capture shutdown is still
inside the fail-closed continuity boundary.
During that same boundary, DropSquash subscribes to macOS workspace lifecycle
notifications. `SessionDidResignActive`, `ScreensDidSleep`, `WillSleep`,
`WillPowerOff`, and `ActiveSpaceDidChange` are publication blockers. The
receipt stores no notification payload; the implementation only uses the event
class to reject and discard. This is an event-path guard, not proof that every
physical lock, sleep, screen-saver, or wake transition has already been
exercised in packaged-app QA.
DropSquash also listens for `NSWorkspaceDidActivateApplicationNotification`.
The target application and DropSquash itself are permitted, because starting,
stopping, and interacting with the selected target naturally require those two
applications. Any other foreground application activation blocks publication.
Only the event class is retained as a fail-closed reason; the activated app
name, bundle identifier, and notification payload are not stored. This reduces
the foreground-focus hole, but it does not claim continuous window z-order or
every overlay/compositor state.

## Publication Boundary

The recorder writes video to a dot-prefixed partial path. After independent
decoded-output verification and signed-sidecar preparation, it publishes the
sidecar first and moves the MP4 to its final visible name last. If the final
video move fails, both the final sidecar and all partial paths are removed.
Before a later Secure Share recording starts, DropSquash also removes only its
own abandoned dot-prefixed partial MP4 and partial sidecar names in that output
directory. Normal user files and final recordings are not matched. This is
crash recovery for unpublished artifacts, not an atomic two-file transaction.
On a normal app exit request, the active recording is explicitly cancelled and
its partial paths are removed immediately. A forced process termination still
depends on the next-start recovery path.
Cancellation also sends the stop signal to the capture session before removing
its publication permit. A discovery timeout or start error therefore cannot
leave a reserved background recording running after the UI has returned an
error.
If the stop-control channel disconnects instead of delivering a normal stop,
DropSquash still requests native capture stop, keeps continuity monitoring until
that completion, discards the partial path, and returns a fail-closed error.
If frame processing itself fails, the callback discards the partial recording
immediately; it does not wait for the user to press stop before removing that
unpublished artifact.
The same immediate-discard rule applies to every rejected frame boundary,
including non-complete frame status, malformed frame metadata, repeated time,
geometry/scale change, and a capture-time gap. A later stop action cannot turn
one of those rejected streams into an output.
When macOS reports that Screen Recording permission is missing, the app offers
an explicit path to the matching System Settings pane. It cannot grant the
permission itself; capture remains blocked until macOS reports permission.
This does not make a two-file publication transaction atomic across a process
crash, but it prevents the normal failure path from leaving a final-name MP4
without a signed sidecar.

## Transient Input And Overlay Policy

IME composition text, candidate windows, browser autofill, spelling popovers,
notifications, and transient sheets are hostile inputs. Phase 5 treats them as
unknown unless a fixture proves a stable, frame-exact handling path.

The current export therefore remains Strict Shield: every captured frame is
blackened before encoding, regardless of a detector result. Phase 5 retains
only aggregated event counts for this policy; it must never store composed text,
candidate text, OCR text, or raw AX text in a receipt.

The signed Secure Share evidence verifier rejects every policy except Strict
Shield. AX/Vision agreement counts may support research, but cannot turn a
SmartMask plan into a verifiable Secure Share result.

## Synthetic Selective-Mask Experiment

The QA-only capture command can now run the same local ScreenCaptureKit writer
with `SmartMask` for the native Japanese/English fixture. It blackens AX and
Vision regions per frame, then independently decodes the final MP4 to verify
every declared black region. Its plan is written as
`*.mask-plan.experimental.json`, never as a signed Secure Share receipt.
That explicitly labeled experimental artifact contains the MaskPlan plus only
the aggregate decoded-text and decoded-unmasked-text counts; it never stores
recognized text.
On 2026-07-21, passing that artifact to `secure-share-evidence-check` failed
with the expected signed-evidence schema rejection. Experimental evidence is
therefore not silently accepted as a signed Secure Share receipt.

The next verification gate re-runs local Vision on every decoded final-MP4
frame. A detected text box must be completely contained by a resolved
destructive region, including that region's encoder guard expansion. One
uncontained decoded text observation rejects the experimental output and
removes its partial MP4. The check retains only aggregate counts; it does not
save recognized text. This is a conservative residual detector, not a proof
that Vision finds every remaining text pixel. Its signed-package fixture run
does not change the current claim surface.

On 2026-07-21, the Developer ID-signed packaged app ran this gate against the
native fixture for 20 decoded frames. It observed 220 decoded text candidates
and rejected zero as outside the resolved destructive regions. The output kept
the ordinary visible final MP4 and an experimental MaskPlan sidecar only. This
proves that the implemented rejection path can inspect the final encoded file;
it still cannot prove that Vision detects every possible residual pixel.

The same signed-package experiment ran the fixture's 50ms focused-input update
burst with a QA-only union truth covering all four possible input values. It
captured 20 frames, observed 220 capture-time Vision candidates, and found zero
decoded text candidates outside destructive regions. An earlier raw-plan
measurement reported 975,768ppm coverage and 24,232ppm uncovered, but it did
not include the encoder guard expansion. It is retained only as a historical
research observation; final effective-mask precision must be rerun. The union
is not frame-exact, so this result remains `detected-but-not-covered` for
selective masking. Strict Shield remains the only covered export rule.

The fixture now emits monotonic-clock intervals for each synthetic focused-input
state. The QA evaluator maps those intervals to ScreenCaptureKit frame times
and measures the encoder-expanded destructive rectangles per frame. The
instrumentation is implemented and typechecked; its signed packaged-app rerun
is still pending, so it has no replacement coverage number yet.

This experiment exists to measure coordinates, detector coverage, and encoded
pixel destruction on known synthetic data. It does not prove that pixels outside
the declared regions are safe, so it must not be exposed in the normal product
UI, used with real recordings, or described as selective secure sharing.

### 2026-07-21 Native Fixture Result

A Developer ID-signed packaged-app run captured the Japanese/English native
fixture for 11 frames. It observed 121 local Vision candidates and destroyed
154 declared regions. The experimental verifier decoded the final 760x652 MP4
and confirmed every declared region was black; `fullyMaskedFrameCount` was
zero. The declared destructive union was 227,435 parts per million (22.7435%)
on every captured frame before the live encoder guard expansion. A Quick Look
thumbnail retained the white fixture surface around the destroyed regions, so
this was not another full-frame black output.

This is deliberately narrow evidence. Accessibility exposed multiline text
containers, causing several large black bands. That is acceptable for proving
the pixel-write and encoded-output path, but is not an acceptable preservation
policy for real work. The next experiment must measure region precision against
known fixture truth before any candidate is allowed to preserve surrounding
pixels. It must not weaken Strict Shield, produce a signed receipt, or expand
the supported scope.

The experimental SmartMask resolver now drops an AX text rectangle only when
two or more local Vision text candidates overlap that same AX rectangle. This
distinguishes a multiline AX container from a single text element without
retaining recognized text. It is a fixture-quality refinement, not a safety
rule: a Vision miss can still leave pixels unmasked, so Strict Shield continues
to destroy the complete frame and the experimental output remains unshareable.

A second signed fixture run exercised that rule with 10 decoded frames, 110
Vision candidates, and 120 declared regions. Its declared area was 225,797ppm
(22.5797%). Inspection found one remaining 422x182 AX-only text container:
Vision had not observed the text inside it. The resolver correctly retained
that AX rectangle. Removing it merely to improve the video would create a
known leakage path, so it remains destroyed until a dedicated detector can
cover that fixture region.

### Fixture-Truth Precision Gate

The native fixture can now emit a QA-only JSON sidecar containing only its own
synthetic sensitive pixel rectangles, output frame size, and no text content.
When `DROP_SQUASH_QA_SENSITIVE_TRUTH_PATH` is set, the QA recorder compares
that truth against every SmartMask frame. It reports the minimum sensitive-truth
coverage and maximum destruction outside that truth in parts per million.

On 2026-07-21, a Developer ID-signed packaged-app run decoded 11 frames and
reported 974,162ppm (97.4162%) minimum sensitive-truth coverage and 154,577ppm
(15.4577%) maximum outside-truth destruction before final encoder expansion.
The precision calculation now uses the same expanded rectangles as the encoder,
so this historical raw-plan metric must be rerun before it is compared. The
synthetic truth contains the four static Japanese/English labels and the actual
glyph bounds of the custom canvas token; it intentionally excludes empty
input-field background. SmartMask remains unsuitable for sharing and cannot
produce a receipt.

The same run reported 7 AX observations, 121 Vision observations, 33
cross-source overlaps, 5 AX-only observations, and 88 Vision-only observations.
Those are aggregate counts only: no recognized text is emitted. They explain
why a Vision-only policy is not acceptable even for this small fixture; AX-only
regions still need destructive handling until an independent detector covers
them.

Strict Shield recording-path plans use internal schema 2 and carry
`captureContinuityAttested: true` plus a redacted required-watch list:
`display_configuration`, `core_graphics_window`, `accessibility_geometry`,
`accessibility_window_events`, `macos_lifecycle_notifications`, and
`foreground_activation_pid_allowlist`. These values are added only after the
required watches finish without failure, then covered by the sidecar signature.
They store no window title, ID, coordinates, app name, AX event payload,
notification payload, or recognized text. Schema-2 Strict Shield evidence
without the attestation or the required watch evidence is rejected. Earlier
signed evidence that set this field is not temporal-continuity evidence: its
ScreenCaptureKit display-time ticks were not normalized to nanoseconds. The
current implementation uses `mach_timebase_info` and rejects the observed gap
rather than publishing. A normalized-time packaged-app rerun must pass before
the attestation can be restored.
The QA-only SmartMask artifact deliberately carries a schema-1 research plan
inside its distinct experimental envelope. Both that envelope and a bare
schema-1 plan are rejected by the current Secure Share evidence verifier rather
than silently upgraded.

The same schema-2 audit now requires redacted exposure-coverage evidence for
typed text, IME composition candidates, transient dialogs/popovers,
notification overlays, browser autofill/password-manager surfaces,
focus/foreground changes, display geometry/scale changes, and frame
drops/discontinuities. In Phase 5 alpha, those paths are accepted only when
their mitigation is `strict_shield_full_frame_destruction`. A signed sidecar
that omits a required path or weakens the mitigation is rejected. This is not a
completed selective-masking claim.

## AX Event Continuity Gate

Polling is insufficient for a window change that returns between samples. The
current gate attaches an `AXObserver` to the AX window whose position and size
match the selected ScreenCaptureKit target at start. It registers `AXMoved` and
`AXResized` on a dedicated Core Foundation run-loop thread, then sets a
fail-closed recording flag from the callback. The recorder checks that flag
before publication even if the window has already returned to its initial
frame.

For the high-assurance mode, failure to obtain Accessibility permission, match
one AX window to the selected target, or register either notification is a
start failure, not a polling fallback. This is deliberately stricter than the
current Strict Shield alpha and may exclude applications that do not expose AX
notifications. A Developer ID-signed native-fixture run on 2026-07-21 moved a
selected window for 100ms and restored it during a 12-second capture. The app
returned the AX-event failure and published no artifact. This is one native
fixture result, not a universal macOS event-delivery claim.

The current receipt also omits the raw ScreenCaptureKit window ID. The signed
output hash, rather than a target identifier, binds the evidence to its video.
The verifier additionally accepts only the fixed local Strict Shield capture
identifier; a signed but user-derived capture identifier is rejected.

## Acceptance Evidence

Phase 5 is not complete until each in-progress row has one of:

- a deterministic unit or integration test;
- a synthetic adversarial fixture and a packaged-app run; or
- an explicit fail-closed result with no final or partial artifact.

Every result is recorded as `covered`, `fail_closed`,
`detected_but_not_covered`, or `out_of_scope`. No public copy may compress
those states into a broader privacy claim.

## Evidence Register

| Exposure path | Classification | Strongest current evidence | Boundary that remains |
| --- | --- | --- | --- |
| Focused 50ms typed text | detected_but_not_covered for current source-surface evidence | Historical signed native-fixture run; current native bridge rerun pending | Strict Shield output pixels are black, but current source-surface coverage is not restored yet |
| Japanese IME candidate | detected_but_not_covered | Synthetic/local IME-like evidence only | Other IMEs and private input UI remain unproven |
| Native sheet and popover | detected_but_not_covered | Historical signed packaged-app runs and decoded verification | Current normalized native-bridge rerun is pending; not selective modal masking |
| Synthetic transient overlay | covered for Strict Shield fixture | 2026-07-21 signed browser run: 378 frames, target/recorder foreground changes, independent decode | Not native browser or OS overlay coverage |
| Native notification | detected_but_not_covered | Notification dispatched during a signed run | Banner compositing into selected-window pixels is unproven |
| Window hide | fail_closed | Deterministic native fixture leaves no final or partial artifact | Hide-and-return between watchdog samples is unproven |
| Window resize | fail_closed for sampled changes | A later native resize produced `CoreGraphics selected window bounds changed` and no artifact | Resize-and-return between 250ms samples is unproven |
| Window move-and-return (100ms) | fail_closed for native AX fixture | Current Developer ID-signed run returned the AX-event geometry failure with no artifact | Other applications may lack usable AX events and are rejected at start; universal event delivery is not claimed |
| Frame order, gap, geometry, scale | detected_but_not_covered | Deterministic `stream_output_state` and continuity tests | Current packaged-app rerun is pending; does not prove macOS emits every compositor update |
| Display configuration epoch | detected_but_not_covered | Callback epoch rule plus deterministic finalization tests | Physical attach/detach/mirror/rotation behavior needs device QA before a fail-closed claim |
| AX/Vision disagreement and custom canvas | detected_but_not_covered for selective masking | Aggregate-only audit plus Strict Shield canvas run | Observations never authorize pixels to survive |
| Audio, tracks, metadata, output substitution, and frame count | covered for verified output | Independent decoder, MP4-box verifier, signed SHA-256 binding, and MaskPlan-count tests | Quick Look derivatives are not enumerated |
| Wrong window or changed target | covered/fail_closed | Start/boundary attestation, native hide/resize runs, and redacted-capture-ID verifier tests | Transient identity changes between samples are unproven |
| Foreground-focus loss | fail_closed for observed third-party app activation | PID-allowlist unit tests plus a 2026-07-21 signed Finder-switch run that published no artifact | Not a complete z-order or overlay attestation |
| 100ms move-and-return | fail_closed for native AX fixture | Current signed run returned the AX-event geometry failure with no artifact | This does not prove notification support for every macOS application |
| Synthetic Japanese/English selective mask | experimental only | 2026-07-21 signed native-fixture run: 11 decoded frames, 121 Vision candidates, 154 declared black regions, 227,435ppm declared area, zero fully masked frames | AX multiline containers create broad black bands; pixels outside declared regions have no safety proof |

## Current Deterministic Evidence

- An explicitly selected window that is no longer on screen or has no owner is
  rejected before recording starts.
- An owner PID or frame that differs from the user's selection is rejected
  before recording starts.
- A Developer ID-signed packaged-app QA run on 2026-07-21 supplied the
  non-existent window ID `4294967295`. Target selection returned
  `requested window candidate was not found`; no QA MP4, sidecar, or partial
  artifact was created. This covers the deterministic invalid-selection path,
  not every possible window-ID reuse scenario.
- The explicit target picker retains visible owned windows after the recorder
  receives focus. `active` is not used as a continuity proof because the user
  must legitimately alternate between the selected target and DropSquash.
  Start and boundary attestation still require the same ID, owner PID, frame,
  on-screen state, and layer. The native route intentionally does not retain
  or compare raw titles.
- DropSquash now also watches `NSWorkspaceDidActivateApplicationNotification`.
  If the activated app PID is neither the selected target process nor the
  recorder process, publication is rejected. Unit tests cover target PID
  allowed, recorder PID allowed, third-party PID rejected, and the failure
  staying latched when the target returns to the foreground. On 2026-07-21,
  the signed production recorder was started against the native fixture and
  Finder was activated six seconds later. It returned `third-party application
  became active`; no MP4, sidecar, or partial artifact was published.
- A Developer ID-signed browser run on 2026-07-21 alternated from DropSquash
  to `transient-input-overlay-001`, triggered its synthetic candidate,
  autofill, paste, spelling, and confirmation surfaces, then returned to
  DropSquash and saved 378 destroyed frames. The schema-2 sidecar passed the
  independent verifier with 3 aggregate AX and 4,903 aggregate Vision
  observations; its MP4 SHA-256 was
  `c85a5bb45b2436eb35fc8b62d2eb2cd22e3a168827dcb862ae77b64e6d9f6857`.
  This proves the target/recorder allowlist path, not third-party activation
  or general overlay compositing coverage.
- A non-complete ScreenCaptureKit frame status rejects and discards the
  recording instead of silently omitting that observation.
- The deterministic `Suspended`-status test confirms that this rejection also
  removes a dot-prefixed partial MP4 immediately. It is coverage for the
  received status, not a claim that every lock or sleep transition delivers it.
- During recording, DropSquash now keeps an `NSWorkspace` lifecycle observer
  for session resignation, screen sleep, workspace sleep, power-off, and
  active-Space changes. Any observed event rejects publication and discards
  the recording. Deterministic unit tests prove both the stored fail-closed
  boundary and a posted `NSWorkspaceScreensDidSleepNotification` path; actual
  lock/sleep/manual screen-saver runs remain separate QA evidence before
  claim expansion.
- Repeated or reversed presentation timestamps reject and discard the
  recording.
- Capture requests a 30fps minimum frame interval. A presentation-time gap
  longer than five intervals rejects and discards the recording rather than
  creating continuity evidence it cannot support.
- A signed-app stationary-window run on 2026-07-20 passed the cadence and
  post-capture revalidation gates, then passed independent evidence/output
  verification with 178 full-frame-destroyed frames. Its video SHA-256 was
  `9a060795f56f148980841c7dd2b099aaf57e0ca770d379fb17a51f49f208ff6a`.
- A complete frame whose content rect, bounding rect, or scale differs from the
  first complete frame rejects and discards the recording.
- Final verification rejects an output with no decoded frames or a decoded
  frame count that differs from the signed MaskPlan. The matching and both
  rejection cases have deterministic encoder tests.
- Replacing an MP4 after its signed sidecar was created is rejected because its
  SHA-256 no longer matches the signed evidence. This has a deterministic
  fileguard test; it is not merely a filename or timestamp comparison.
- After the publication step gives both artifacts their final names, the app
  runs the same evidence verifier over that final MP4/sidecar pair. A binding,
  signature, schema, or Strict Shield structure failure returns through the
  publication permit's discard path rather than leaving a final artifact.
- Before capture starts, Secure Share registers a process-local CoreGraphics
  display-configuration callback and snapshots its generation. A connection,
  removal, mode, mirror, rotation, or desktop-shape event during capture makes
  the generation differ and rejects publication. This guards configuration
  changes; it does not prove every cross-display move with unchanged macOS
  signals.
- The display-epoch finalization rule has deterministic unit coverage: an epoch
  change rejects even without a target error, and it takes priority if a target
  error is present too. Physical monitor attach, detach, mirror, and rotation
  events still require separate macOS-hardware evidence before claim expansion.
- Before the stream starts, Secure Share also snapshots the selected window's
  CoreGraphics owner PID and full global frame. The watchdog compares that
  independent snapshot every 250ms and synchronously once more at the stop
  boundary. A missing window, owner replacement, move, or resize discards the
  output. This is intentionally a second signal beside ScreenCaptureKit, after
  a real resize escaped SCK-only revalidation. It remains sampled rather than
  frame-exact: a change-and-return wholly inside one poll interval is not
  claimed as covered.
- After stopping, a fresh ScreenCaptureKit snapshot must still match the
  selected window ID, owner PID, and frame. A signed-app manual run moved and
  resized the selected browser during recording on 2026-07-20; no new MP4 or
  sidecar was published, and the UI reported that the selected window changed.
- A frame whose pixel buffer is unavailable rejects and discards the recording;
  it is not silently omitted.
- A signed-app stationary native-fixture run on 2026-07-20 kept the target
  unchanged while the 250ms watchdog was active. It published 191
  full-frame-destroyed frames, and `secure-share-evidence-check` verified the
  sidecar signature, output binding, decoded blackening, audio, and metadata.
  Its MP4 SHA-256 was
  `fe5e0f58f74cd3ce3be354f3be01d6fa38fbfa0d0f82a6e0bd666fd6154c5214`.
- A signed-app run on 2026-07-20 dragged the selected native fixture window
  during capture and waited beyond the watchdog interval. The watchdog stored
  the failed ScreenCaptureKit revalidation; on stop the UI reported that the
  selected window changed, no final MP4 or sidecar was added, and the new
  dot-prefixed partial path was removed. This proves sampled runtime
  fail-closed geometry detection, not frame-exact continuous title monitoring.
- Earlier title-change failures belonged to the legacy Rust watchdog. They are
  not evidence for the native route, which deliberately does not retain or
  compare a window title. Title-only continuity remains
  `detected-but-not-covered`; geometry, owner, lifecycle, foreground, and AX
  window-event changes are the current native fail-closed boundary.
- A current Developer ID-signed app run on 2026-07-21 used the native fixture's
  deterministic hide-window action during the production continuity session.
  The runtime revalidation returned `requested window candidate was not found`.
  No final MP4, signed MaskPlan, or dot-prefixed partial artifact was created.
  This is fail-closed evidence for a selected window that becomes unavailable;
  it does not prove instantaneous detection of a hide-and-return event between
  watchdog samples.
- After the display-watch startup cleanup change, the rebuilt Developer
  ID-signed app was exercised again with that hide-window fixture on 2026-07-21.
  It returned `selected window changed before capture` and again created no
  final MP4, signed MaskPlan, or dot-prefixed partial artifact.
- A current Developer ID-signed app run on 2026-07-21 found a regression: the
  native fixture's CoreGraphics bounds changed from 760x652 to 840x652, but an
  SCK-only watchdog published a 15-frame output. The correction adds a
  CoreGraphics baseline before capture, 250ms owner/frame co-watching, and a
  synchronous stop-boundary comparison.
- The corrected, Developer ID-signed app was run again on 2026-07-21 with a
  12-second QA capture and a deterministic resize at nine seconds. It returned
  `CoreGraphics selected window bounds changed`; no final MP4, signed MaskPlan,
  or dot-prefixed partial artifact appeared. This is fail-closed evidence for
  a sampled resize, not proof against a resize that reverts within one polling
  interval.
- The same signed app was run with a deterministic 80-point X-axis move at
  nine seconds. It returned the same CoreGraphics bounds-change error and left
  no final MP4, signed MaskPlan, or partial artifact. This separately proves
  sampled position-change rejection; it does not prove a move-and-return
  between samples.
- A separate signed-app adversarial run moved that window 80 points and
  restored it after 100ms during a 12-second recording. The 250ms CoreGraphics
  polling did not observe the transient change and a 37-frame output was
  published. `secure-share-evidence-check` independently verified every frame
  as Strict Shield black, with SHA-256
  `303b335801042babcec744f92f7f410da002c88dd9d3cc635634082b34368769`.
  Therefore this is not a residual-pixel leak, but it is a target-continuity
  attestation gap and remains detected-but-not-covered.
- The rebuilt Developer ID-signed app then installed the AX-event gate before
  capture. The same deterministic 100ms move-and-return fixture on 2026-07-21
  returned `Accessibility reported selected window geometry changed`; no MP4,
  MaskPlan, or partial artifact was published. This closes that fixture's
  polling gap, while preserving a strict start failure for clients that cannot
  expose or register the required AX notifications.
- A separate stationary Developer ID-signed run on 2026-07-21 produced 15
  full-frame-destroyed frames. `secure-share-evidence-check` verified schema 2,
  Strict Shield policy, decoded output, and SHA-256
  `9d34413ef113455d178309e5a24dd7c6d577c96f680b6be3857e2b5080bdf98e`.
  Current schema-2 MaskPlans require `captureContinuityAttested: true` plus
  the redacted required-watch list before publication. The cited historical
  run predates normalized ScreenCaptureKit display time, so it is not current
  evidence for that condition. They retain no target title, ID, coordinate,
  app name, notification payload, AX payload, or recognized text.
- A signed-app run on 2026-07-20 started recording the native fixture and then
  terminated its process. The watchdog reported that the selected target could
  not be found. No final MP4, sidecar, or dot-prefixed partial was added. The
  user-facing error is normalized to the target-change recovery action rather
  than exposing the ScreenCaptureKit implementation error.
- Strict Shield inserts a full-frame destructive region into every MaskPlan
  frame and every live recording frame before encoding.
- Independent final-output verification rejects a Strict Shield plan unless
  every decoded frame has one destructive region that exactly covers the full
  decoded frame. A partial region cannot be published under this policy.
- Decoded Strict Shield pixels must be neutral black: limited-range black is
  accepted, but a dark chromatic residual is rejected rather than being hidden
  inside a broad per-channel threshold.
- The signed plan records only AX/Vision counts and cross-source overlap counts;
  it retains neither recognized text, raw observation payloads, nor observation
  geometry. Each Strict Shield plan frame contains exactly one full-frame
  `UnknownRegion` sourced only from ScreenCaptureKit; the independent verifier
  rejects any other Strict Shield sidecar geometry. It also rejects empty,
  non-contiguous, or non-monotonic frame plans, and refuses a plan that weakens
  required no-audio or metadata-removal verification. It accepts only the
  currently understood Strict Shield MaskPlan schema version.
- A packaged-app native fixture recording on 2026-07-20 produced 184 frames
  under this redacted-sidecar rule. Its MP4 SHA-256 was
  `353168c4effd5ed6acdf22d195610333a2ef69fabd4078f2cbaa6181cd52b1d7`.
  `secure-share-evidence-check` verified its signature, output binding, full
  decoded blackening, and absence of saved observation geometry. The signed
  audit retained only 7 AX observations, 2,391 Vision observations, and their
  aggregate agreement counts.
- On 2026-07-21, macOS Quick Look generated a fresh 640x392 PNG thumbnail from
  the current independently verified 378-frame browser-fixture MP4. Its RGB
  pixels remained neutral black (maximum channel value 1/255); no fixture
  content was visible. This is evidence for one newly generated derivative of
  that verified output, not a claim that Finder, Quick Look, or all cached
  derivatives are exhaustively enumerated.
- Final-output verification requires exactly one visual track. Audio, subtitle,
  caption, or any other non-video track rejects the output before publication.
- Final-output verification also scans the MP4 box hierarchy. It rejects timed
  events, UUID payloads, iTunes-style item lists/keys, copyright tags, and
  user-data payloads; an empty standard metadata container alone is not treated
  as private data.
- `transient-input-overlay-001` supplies a deterministic synthetic sequence
  for IME-like candidates, autofill-like UI, and paste notices. A signed
  packaged-app run recorded the fixture and the independent verifier accepted
  its schema-2, 79-frame output on 2026-07-20. Every plan frame carried a full-frame
  destructive region. The video SHA-256 was
  `01235fb75900145f913a9de44764e860338d1fc86d3eaa7c102affe2fb895195`.
- That fixture run proves the Strict Shield path, not native macOS IME,
  password-manager, or system-notification coverage. Those surfaces remain
  in progress until separately exercised and verified.
- A native macOS notification with a synthetic token was dispatched while the
  packaged app recorded the native fixture on 2026-07-20. The resulting
  167-frame Strict Shield MP4 and schema-2 sidecar passed
  the then-current verifier, including signature, SHA-256 binding, and
  independent decoded-output verification. Its video SHA-256 was
  `23f3a89c20f9339200d152e8c7e0ff1a5cd55b4d4a53c25b26006fdc87c5fccc`.
  This proves the fail-closed blackening/evidence path while macOS delivered a
  notification; it does not prove that the notification banner was composited
  into the selected-window pixels, so selective notification coverage remains
  unproven. That historical sidecar predates the current no-observation-
  geometry rule and is intentionally rejected by the stricter verifier.
- A real macOS Japanese IME candidate window was opened over the native
  fixture's editable field during a signed-app recording on 2026-07-20. The
  output contained 368 full-frame-destroyed frames using `phase5-alpha-v1` and
  passed independent evidence/output verification. Its video SHA-256 was
  `480916583224a585d50e0234829e684eaf42dfbe350ff3c37b92aa89dd1b19b2`.
- That run covers this OS/input-method configuration's Strict Shield path only.
  It does not establish coverage for every IME, private password-manager UI,
  notification center, or selective redaction.
- The same Japanese input source was exercised again under the
  no-observation-geometry rule on 2026-07-20. A live candidate list and its
  explanatory panel were visible over the editable field during capture. The
  resulting 275-frame MP4 SHA-256 was
  `5226d3057edc153e0325224fe1cbf7ea23272f67283f8f227769a1307e02128c`.
  Its sidecar kept exactly one full-frame ScreenCaptureKit `UnknownRegion` per
  frame and passed independent decoded-output and evidence verification. This
  is one local input-source configuration, not a claim about every IME.
- Signed evidence schema version 2 uses canonical JSON before signing. Version
  1 sidecars are rejected as legacy evidence rather than being presented as
  independently verifiable records.
- The expanded synthetic sequence (candidate, autofill, paste notice,
  spelling popover, and confirmation dialog) also passed in the signed app on
  2026-07-20: 240 frames, 240 full-frame destructive regions, schema version
  2, and independent evidence/output verification. Its video SHA-256 was
  `2b3e3b42630bf70b2cda562fcc6475f300e2e27b7ce48111ba28ca23bf0d019f`.
- A current Developer ID-signed app run on 2026-07-21 used the native fixture's
  deterministic focused-input update burst. It captured 13 frames, destroyed
  all 13 full frames, retained only 8 AX and 247 Vision aggregate observations,
  and passed `secure-share-evidence-check` with schema 2. Its MP4 SHA-256 was
  `daec0f235ae7b8013674ecd8694044c0102e36127947140172c3a3ddbdf87640`.
  This exercises short-lived values in one focused AppKit field; it is not a
  claim about IME composition, password managers, or all text-input surfaces.
  Its duration-controlled QA route now uses the production continuity session;
  that stationary run does not itself exercise a target-change failure.
- After the QA route moved onto the production continuity session, the same
  focused-input fixture passed again on 2026-07-21 with 12 destroyed full
  frames, 8 AX and 230 Vision aggregate observations, schema 2 evidence, and
  independent decoded-output verification. Its MP4 SHA-256 was
  `632f8b24249ad9d55e84247067d58be69e375bdae5b62cc3dcff24edea22fbb0`.
- A current Developer ID-signed run on 2026-07-21 recorded the fixture's
  custom-drawn canvas token. It destroyed all 14 captured frames and passed
  schema-2 evidence plus independent decoded-output verification. Its MP4
  SHA-256 was `387e97530cfe6cda4891fcd6371ef86f4ff5d6cd70a950974c66f5f09f1b811a`.
  The audit had 8 AX and 154 Vision observations; because a coarse AX
  client-area rectangle overlapped Vision rectangles, it is not evidence of a
  Vision-only path or a selective-masking claim.
- The publication permit test forces the final video move to fail and confirms
  that the already-published sidecar is removed. A cancelled recording also
  cannot publish a late result.
- A signed-app recording was left active and then terminated through the normal
  app exit request on 2026-07-20. Its fresh dot-prefixed partial MP4 was removed
  before the process exited. A prior abandoned partial from a permission-UI
  interruption was also removed before the next Secure Share capture began.
  Forced process termination is intentionally not presented as immediate
  cleanup; its recovery is verified on the next capture start.
- `native-accessibility-fixture-001` exercised a real AppKit sheet containing
  a synthetic email address and confirmation code while the signed packaged
  app was recording on 2026-07-20. The published output had 383
  full-frame-destroyed frames and passed independent evidence/output
  verification. Its video SHA-256 was
  `f91fc381e53b3c83564064cbe63b3079dcdec8c16b7b63ff369e2aa8343759e5`.
- That native-sheet run proves only the full-frame Strict Shield path over an
  AppKit modal. It does not prove AX/Vision agreement, native IME coverage, or
  that a modal may be selectively retained.
- The native sheet was exercised again under the no-observation-geometry rule
  on 2026-07-20. The confirmation sheet visibly contained a name, email, and
  confirmation code while 382 frames were captured. The resulting MP4 SHA-256
  was `a7dfb654475577150ed4a214dde5a792fa921e0c3b9f4cfb55b4286a769abea1`.
  Its signed sidecar had exactly one full-frame `UnknownRegion` from
  ScreenCaptureKit per frame, and passed independent decode, audio, metadata,
  signature, and binding verification. This proves destructive Strict Shield
  handling for this fixture, not selective masking or universal modal coverage.
- The same native fixture exercised an AppKit transient popover containing a
  synthetic autofill address and one-time code on 2026-07-20. The signed app
  published 318 full-frame-destroyed frames using `phase5-alpha-v1`; both
  independent evidence and decoded-output verification passed. Its video
  SHA-256 was
  `53ca4692b774c6553b38672e16fa6746d57493e17886fa55594b08f25ab5af14`.
- That native-popover run proves the Strict Shield destruction path only. It
  does not establish native password-manager, IME, notification-center, or
  selective-redaction coverage.
- The AppKit popover was exercised again under the no-observation-geometry
  rule on 2026-07-20. It visibly contained a synthetic autofill email and
  one-time code during capture. The 464-frame MP4 SHA-256 was
  `b02d617d6944cda0e9f82a9d3571fd5cbb98bd7c6570054f5fa448adeda9dda6`.
  Its signed sidecar contained exactly one full-frame ScreenCaptureKit
  `UnknownRegion` per frame and passed independent output and evidence
  verification. This proves this fixture's destructive path only.
- A current Developer ID-signed run on 2026-07-21 auto-opened the native AppKit
  popover during the production continuity session. All 16 captured frames were
  destroyed and its schema-2 sidecar passed independent decoded-output
  verification. Its MP4 SHA-256 was
  `b41a54006b349d7ff0ef2ad1ae5490119a2f7e2f43d88b3650724e7264f1a5b9`.
  This remains evidence for destructive Strict Shield over this fixture, not
  selective masking or private browser/password-manager UI coverage.
- A current Developer ID-signed run on 2026-07-21 auto-opened the native AppKit
  confirmation sheet during the same production continuity session. All 12
  captured frames were destroyed; its schema-2 signed MaskPlan and independent
  decoded-output verification passed. Its MP4 SHA-256 was
  `7f2fb78dc4406944645ec783bafd5ac3c6905e47dd8ef8a0e3f8e48173d10625`.
  This covers destructive Strict Shield for this synthetic sheet only. It does
  not prove selective modal masking, every system sheet, or private UI coverage.
- After the display-watch startup cleanup and finalization changes, the rebuilt
  Developer ID-signed app exercised the same auto-opened sheet on 2026-07-21.
  All 12 frames were destroyed; schema-2 evidence and independent decoded-output
  verification passed. Its MP4 SHA-256 was
  `0149026fffb4b92c80c593205ddcd89169384d27b15490d58b9639e16b909de1`.
- After final-pair revalidation was added, a newly rebuilt Developer ID-signed
  app recorded the same synthetic sheet on 2026-07-21. The final visible MP4
  and sidecar passed the app's post-publication verifier and an independent
  `secure-share-evidence-check`: 12 destroyed 760x652 frames, schema 2,
  10 aggregate AX observations, 219 aggregate Vision observations, and no
  unmatched observations. Its MP4 SHA-256 was
  `2a7a29f711ec56d48be90248aedf83333dcd44c05fe571863bb4d37654c24250`.
  This is evidence for the synthetic AppKit sheet's Strict Shield path only;
  it does not establish selective masking or general OS-overlay coverage.
- A synthetic macOS notification containing `SYNTHETIC-TOKEN-9471` was
  dispatched one second into a new Developer ID-signed fixture recording on
  2026-07-21. The final pair passed the app's post-publication verifier and an
  independent `secure-share-evidence-check`: 14 full-frame-destroyed frames,
  schema 2, 8 aggregate AX observations, and 154 aggregate Vision observations.
  Its MP4 SHA-256 was
  `789a9401c8e240a73abd38ba57a369493b0ae222a9b2551c26e83108e6fc66c1`.
  The test proves the latest Strict Shield finalization path while macOS was
  asked to present a notification. It does not prove Notification Center
  composited that banner into the selected-window pixels.
- A historical Finder foreground switch was requested one second into a
  Developer ID-signed fixture recording on 2026-07-21. ScreenCaptureKit
  retained the selected fixture's `active` flag, so the old watchdog did not
  reject; the final 14-frame Strict Shield pair independently verified with MP4 SHA-256
  `64e9213103789a52e4704314f9720f09084a836cd2416b06fee60c029070cf96`.
  This is negative evidence: the output path remained safe because every frame
  is destroyed, but SCK's `active` flag is not proof of foreground focus. The
  current path no longer uses that flag for attestation; it uses the
  NSWorkspace PID allowlist. A subsequent signed production-recording run on
  2026-07-21 activated Finder six seconds after start and failed closed with
  `third-party application became active`; it left no MP4, sidecar, or partial
  artifact. This is hardware QA evidence for the observed activation path, not
  a complete z-order or overlay-attestation claim.
- Historical title-only watchdog runs are retained only as legacy-path research
  notes. They do not support a native capture claim because the native bridge
  intentionally keeps titles out of its capture identity and evidence.
