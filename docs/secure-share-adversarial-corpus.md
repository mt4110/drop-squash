# Secure Share Adversarial Corpus

This corpus plan defines the videos and frame scenarios needed to test
high-assurance Secure Share.

The goal is not to make demo-friendly samples. The goal is to break the masking
engine before customers do.

## Evidence Reset: Normalized Presentation Time

The ScreenCaptureKit display-time conversion was corrected on 2026-07-21:
raw `mach_absolute_time` ticks are now normalized with `mach_timebase_info`.
Earlier signed schema-2 artifacts can still show that captured frames were
destroyed, but cannot prove temporal continuity. References below to earlier
"production continuity" runs are historical destructive-path observations,
not current continuity coverage. A normalized-time packaged-app rerun is
required before any fixture earns `covered` status for capture continuity.

## Corpus Rules

- Keep samples local.
- Use synthetic secrets, never real customer data.
- Include Japanese-first cases.
- Include English secondary cases.
- Include expected sensitive regions as machine-readable annotations.
- Include expected safe regions only when explicitly allowlisted.
- Include one-frame and short-lived overlays.
- Include final-output verification cases, not only pre-encode detection cases.

## Annotation Shape

Each scenario should eventually have a sidecar like:

```json
{
  "scenario_id": "ja-browser-form-001",
  "language": ["ja", "en"],
  "capture_scope": "single_window",
  "frames": [
    {
      "time_ms": 1200,
      "sensitive": [
        {
          "label": "email",
          "rect": { "x": 100, "y": 220, "width": 420, "height": 32 }
        }
      ],
      "safe": [
        {
          "label": "static_app_chrome",
          "rect": { "x": 0, "y": 0, "width": 900, "height": 44 }
        }
      ]
    }
  ]
}
```

Coordinates are canonical frame coordinates after the transform rules in
`docs/secure-share-maskplan.md`.

## Required Scenario Families

### Japanese UI Text

- Japanese form labels and values
- Japanese full-width file names
- Japanese customer names
- Japanese ticket titles
- Japanese modal body text
- Japanese notification text

### English UI Text

- email address in a form
- token in a settings screen
- URL in a browser address or content area
- English chat message
- English table rows
- English modal body

### Mixed Language

- Japanese file name with English extension
- Japanese UI with English IDs
- English dashboard with Japanese customer name
- mixed URL and Japanese path-like text

### Transient Leakage

- one-frame notification
- short-lived auth code
- disappearing modal
- hover tooltip
- copy-paste popover
- browser autofill dropdown

### Layout Stress

- Retina 2x capture
- non-Retina 1x capture
- window resize
- window move
- modal opening over selected window
- sidebar expanding/collapsing
- table scrolling

### Detector Weakness

- tiny text
- low-contrast text
- anti-aliased text
- dark mode
- light mode
- text over image
- text in canvas/custom view
- clipped text
- partially occluded text

### Output Verification

- expected masked region after encode
- expected stripped metadata
- expected no audio track
- expected receipt hash match
- expected verification failure when a synthetic leak remains

## Minimum First Corpus

The first usable corpus must include at least:

| ID | Scenario | Required proof |
|---|---|---|
| `ja-native-form-001` | Native Japanese settings form | AX bounds + Vision observation |
| `ja-en-browser-form-001` | Browser page with Japanese PII fields, English token, mixed filename, modal, and notice | Vision observation |
| `mixed-filename-001` | Japanese/English file names | AX or Vision observation |
| `transient-notice-001` | One-frame notification | Immediate mask plan |
| `modal-sheet-001` | Native modal/sheet | AX structure |
| `resize-window-001` | Window resize during capture | Frame-exact MaskPlan |
| `encoded-residual-001` | Synthetic final-output leak | Verification failure |

## Implemented Fixtures

| ID | Fixture | Annotation | Status |
|---|---|---|---|
| `ja-en-browser-form-001` | [tests/fixtures/secure-share/ja-en-browser-form.html](/Users/masakitakemura/_workspace/drop-squash/tests/fixtures/secure-share/ja-en-browser-form.html) | [tests/fixtures/secure-share/ja-en-browser-form.annotation.json](/Users/masakitakemura/_workspace/drop-squash/tests/fixtures/secure-share/ja-en-browser-form.annotation.json) | selector-based seed ready |
| `ja-en-native-accessibility-001` | [tests/fixtures/secure-share/NativeAccessibilityFixture.swift](/Users/masakitakemura/_workspace/drop-squash/tests/fixtures/secure-share/NativeAccessibilityFixture.swift) | [tests/fixtures/secure-share/native-accessibility-fixture.annotation.json](/Users/masakitakemura/_workspace/drop-squash/tests/fixtures/secure-share/native-accessibility-fixture.annotation.json) | Packaged-app AppKit-sheet Strict Shield pass; selective coverage remains unproven |
| `transient-input-overlay-001` | [tests/fixtures/secure-share/transient-input-overlay.html](/Users/masakitakemura/_workspace/drop-squash/tests/fixtures/secure-share/transient-input-overlay.html) | [tests/fixtures/secure-share/transient-input-overlay.annotation.json](/Users/masakitakemura/_workspace/drop-squash/tests/fixtures/secure-share/transient-input-overlay.annotation.json) | Expanded synthetic sequence passed in signed app; native OS UI remains in progress |

The native fixture also contains an AppKit transient popover with synthetic
autofill and one-time-code text. A signed packaged-app run exercised it with
318 Strict Shield frames and passed both independent verifiers on 2026-07-20.
This does not claim coverage for private browser autofill UI, password managers,
IME, notifications, or selective masking.

The native fixture additionally renders a synthetic token through a custom
`NSView` draw path rather than an AppKit text element. AX may still report a
coarse parent client-area rectangle, so this is not called Vision-only. Strict
Shield must not weaken or omit its full-frame destructive region either way.

For QA only, the fixture can write its synthetic sensitive pixel rectangles to
the path in `DROP_SQUASH_FIXTURE_TRUTH_PATH`. The sidecar has output dimensions
and rectangles only, never recognized or fixture text. The recorder compares it
to the experimental SmartMask plan using the separate
`DROP_SQUASH_QA_SENSITIVE_TRUTH_PATH`; this is a precision measurement, not an
input to masking and not a product feature.

The fixture can auto-open its AppKit popover with
`DROP_SQUASH_FIXTURE_AUTORUN_POPOVER=1`. A current production-continuity run on
2026-07-21 destroyed all 16 frames and passed independent schema-2 evidence
and decoded-output verification; its MP4 SHA-256 was
`b41a54006b349d7ff0ef2ad1ae5490119a2f7e2f43d88b3650724e7264f1a5b9`.

The native fixture's editable field also exercised a real macOS Japanese IME
candidate window in a signed packaged-app run. The 368-frame Strict Shield
output passed both independent verifiers on 2026-07-20. This is one concrete
input-method configuration, not a claim about every IME or selective masking.

The native fixture also contains a 50ms rapid-state burst that rotates
synthetic name, one-time code, token, and local file-path text. It is prepared
for a packaged-app Strict Shield run. Until that run and its independent
decoded-output result are recorded, it remains a fixture rather than coverage
evidence for every short-lived macOS surface.

It also includes a separate 50ms focused AppKit input-update burst. This
changes synthetic name, email, code, and token values inside the editable
field; it is intentionally not called an IME simulation. A current
Developer ID-signed packaged-app run on 2026-07-21 captured 13 frames and
passed independent Strict Shield evidence/output verification; its MP4
SHA-256 was `daec0f235ae7b8013674ecd8694044c0102e36127947140172c3a3ddbdf87640`.
This is limited to this fixture's focused AppKit input path. Its fixed-duration
QA recorder now uses the production continuity session. Earlier title-change
failure evidence belonged to the legacy Rust watchdog and does not apply to the
native route, which deliberately does not retain or compare window titles.

The same focused-input burst also ran through the QA-only SmartMask experiment
on 2026-07-21. The fixture's `DROP_SQUASH_FIXTURE_TRUTH_INCLUDE_INPUT=1` mode
adds glyph bounds for all four possible input values to a conservative union
truth sidecar. The signed app captured 20 frames, observed 220 capture-time
Vision candidates, and the decoded residual gate found zero text candidates
outside destructive regions. The prior 975,768ppm union-truth figure measured
raw plan rectangles before encoder expansion, so it is not final destructive
coverage and must be rerun. The non-frame-exact union means this is not
selective-mask coverage evidence. It is recorded only as a deterministic
residual-gate observation.

For the rerun, the fixture emits a monotonic interval for each input state.
The QA evaluator compares each ScreenCaptureKit frame only with the static
truth plus the state active at that frame time, using the same expansion as the
encoder. Until a signed packaged-app run records that result, this remains
instrumentation rather than coverage evidence.

The continuity-session rerun produced 12 full-frame-destroyed frames with
schema-2 evidence and independent decoded-output verification. Its MP4 SHA-256
was `632f8b24249ad9d55e84247067d58be69e375bdae5b62cc3dcff24edea22fbb0`.

The first browser fixture is intentionally synthetic. It includes Japanese
name, email, phone number, address, low-contrast auth code, English API token,
URL, mixed Japanese/English file name, notification, and modal text.

The annotation is selector-based for now. The next harness step should resolve
selectors into canonical frame coordinates and compare them against observed
`MaskPlan` regions.

Initial packaged-app evidence on Sunday, July 19, 2026:

```text
windowId = 3301
frameWidth = 1440
frameHeight = 900
frameCount = 2
axObservationCount = 2
visionObservationCount = 235
status = ok
eventLog = /tmp/dsq-build-week-fixture-observe-1784457699.jsonl
```

This evidence confirms local observation only. It does not yet satisfy the
commercial gate for destructive masking or independent final-output
verification.

Native modal evidence on Monday, July 20, 2026:

```text
fixture = ja-en-native-accessibility-001
surface = AppKit confirmation sheet
frameCount = 383
fullFrameDestructiveRegions = 383
evidenceVerification = passed
independentOutputVerification = passed
videoSha256 = f91fc381e53b3c83564064cbe63b3079dcdec8c16b7b63ff369e2aa8343759e5
```

This proves Strict Shield destruction over the fixture, not native IME,
notification-center, AX/Vision agreement, or selective-redaction coverage.

The same sheet was auto-opened during the production continuity session on
Tuesday, July 21, 2026. The current Developer ID-signed app destroyed all 12
captured frames; its schema-2 signed MaskPlan and independent decoded-output
verification passed. The MP4 SHA-256 was
`7f2fb78dc4406944645ec783bafd5ac3c6905e47dd8ef8a0e3f8e48173d10625`.
This is a deterministic destructive-path result for the synthetic sheet, not a
claim that every native or system dialog is observed or selectively masked.

Native selected-window disappearance evidence on Tuesday, July 21, 2026:

```text
fixture = ja-en-native-accessibility-001
trigger = DROP_SQUASH_FIXTURE_AUTORUN_HIDE_WINDOW=1
result = requested window candidate was not found
finalMp4 = absent
signedMaskPlan = absent
partialArtifact = absent
```

This is a fail-closed continuity result. It proves the deterministic hidden
window fixture did not publish; it does not establish frame-exact detection of
a window that hides and returns between watchdog samples.

Native resize evidence on Tuesday, July 21, 2026:

```text
fixture = ja-en-native-accessibility-001
trigger = DROP_SQUASH_FIXTURE_AUTORUN_RESIZE_WINDOW=1
result = selected window changed before capture
finalMp4 = absent
signedMaskPlan = absent
partialArtifact = absent
```

This is a fail-closed geometry result. It proves a coordinate-boundary change
did not publish, not that a dynamic window transform can safely be followed.

## Initial QA Flow

1. Open
   [tests/fixtures/secure-share/ja-en-browser-form.html](/Users/masakitakemura/_workspace/drop-squash/tests/fixtures/secure-share/ja-en-browser-form.html)
   in a browser window.
2. Run the packaged-app Secure Share observation harness against that browser
   window.
3. Require `status = ok`, at least one captured frame, and non-zero AX/Vision
   observations.
4. For the Build Week alpha, compare the observation count and `MaskPlan`
   preview against the annotation sidecar.
5. Do not store recognized OCR text in logs, receipts, screenshots, or JSONL
   evidence.

## Acceptance Gates

R&D checkpoint:

- corpus index exists
- annotations exist for the minimum first corpus
- each scenario maps to at least one support-matrix row

Commercial checkpoint:

- packaged app can run the corpus or an equivalent manual QA script
- final encoded outputs are independently verified
- any unsupported case fails closed or is destroyed as unknown

## Storage Rule

Large sample videos should not be committed to the repository unless they are
small synthetic fixtures. Prefer local paths, generated fixtures, or release
artifact storage with checksums.
