# Secure Share Support Matrix

This matrix defines the first supported scope for high-assurance Secure Share.

Nothing outside this matrix may be used in public claims until it has matching
implementation and QA evidence.

## Current Native Bridge Boundary

The current `phase5-native-bridge-v1` path has one Developer ID-signed
packaged-app baseline against the synthetic native accessibility fixture. That
run produced 144 accepted frames, full-frame Strict Shield destruction,
independent decoded-output verification, and signed-evidence verification.
This supports only the narrow claim that accepted selected-window pixels in
that fixture were destroyed before publication.

Earlier `phase5-alpha-v1` fixture records demonstrate an older route and must
not be mixed into the native bridge evidence. Historical references below to
AX event callbacks, move-and-return tests, or signed fixture success require a
native-bridge rerun before they support any broader source-surface claim. The
native export claim remains full-frame Strict Shield destruction after its
native continuity gates pass; selective sharing is not supported.

The current native bridge requires macOS 14.0 or later. This is a technical
minimum for the ScreenCaptureKit metadata used by the fail-closed boundary, not
a claim that every macOS 14 configuration has completed QA.

## Language Scope

| Area | First gate | Claim rule |
|---|---|---|
| Japanese printed UI text | Required | Primary market support |
| English printed UI text | Required | Secondary support |
| Mixed Japanese and English | Required | Must cover common Mac/browser workflows |
| Numbers and symbols | Required | Must cover dates, IDs, amounts, URLs, and codes |
| Other languages | Out of scope | Do not claim |
| Handwriting | Out of scope | Do not claim |

## App And UI Scope

| Surface | First gate behavior | Notes |
|---|---|---|
| Native macOS app window | Required | SCK + AX should provide strong structure |
| Native modal/sheet | R&D required | Strict Shield only; selective sharing is not supported |
| Browser content | R&D required | Vision is observed, but Strict Shield remains required |
| Electron app | R&D required | Vision is observed, but Strict Shield remains required |
| Table/list view | Required | Text rows treated as sensitive unless allowlisted |
| Sidebar | Required | Often contains workspace/project/customer names |
| Toolbar | Required | Static allowlist only; dynamic labels are sensitive |
| Raw title bar | Destroy by default | Re-render safe app label later if needed |
| Notification/banner/popover | R&D required | Synthetic coverage exists; OS-native coverage remains in progress |
| Canvas/custom rendering | Best effort first gate | Strict Reveal destroys unknown regions |
| Video inside recording | Out of scope first gate | Treat as unknown unless explicitly supported |

The QA-only `SmartMask` path is limited to the synthetic Japanese/English
fixture. It verifies declared black regions after encoding but has no
safe-preservation proof for the remaining pixels, so it is not a supported
capture scope or product feature.

The experimental recorder also has a decoded-frame local-Vision residual gate:
detected text outside the resolved destructive regions rejects the partial
output. This is a leakage detector, not an assurance claim, because Vision can
itself miss text. A 2026-07-21 signed-package native-fixture run decoded 20
frames, observed 220 text candidates, and found zero outside the destructive
regions. That validates the rejection path only, not selective-sharing safety.

On 2026-07-21, a Developer ID-signed packaged-app fixture run independently
decoded 11 selective-mask frames and verified 154 declared black regions. Its
declared region union was 227,435ppm (22.7435%) before live encoder guard
expansion, and it retained visible synthetic background rather than producing a
full-black video. This validates the experimental pixel-write path only: broad
AX text containers still produce broad black bands, and no real recording may
rely on the unmasked pixels.

## Capture Scope

| Scope | First gate | Rule |
|---|---|---|
| Single selected window | Required | Preferred initial scope |
| Modal/sheet attached to selected window | Required | Track as part of selected workflow |
| Single display | Out of scope | More leakage risk from unrelated windows |
| Multiple displays | Out of scope | Requires transform and overlay proof |
| Existing `.mov` / `.mp4` | Assistive only | Not high-assurance provenance |

## Display And Geometry

| Case | First gate | Rule |
|---|---|---|
| Retina scale 2x | Required | Canonical transform must be tested |
| Non-Retina scale 1x | Required | Canonical transform must be tested |
| Window resize during capture | Phase 5 sampled fail-closed | A real resize escaped SCK revalidation, then the independent CoreGraphics bounds watcher rejected a nine-second in-recording resize with no artifact publication |
| Window move during capture | Phase 5 sampled fail-closed | A signed-app native 80-point X-axis move was rejected by CoreGraphics with no artifact publication |
| Window move-and-return within 100ms | Fail-closed for native AX fixture | A current signed run returned the AX event geometry failure with no artifact; unsupported AX clients fail before capture rather than using polling alone |
| Selected window loses foreground focus | Fail-closed for observed third-party app activation | Permit the selected target and DropSquash to alternate; reject an observed activation from any other app. A 2026-07-21 signed run activated Finder during capture and published no artifact. SCK `active` is not used as a continuity proof; no full z-order claim is made |
| Selected window hidden during capture | Phase 5 fail-closed | A deterministic native fixture caused revalidation failure and no artifact publication; a hide-and-return between samples is not proven |
| Screen lock, sleep, or wake | Fail-closed when watched macOS lifecycle event is observed | Reject received SCK non-complete status, a resumed presentation-time gap, session resignation, screen sleep, workspace sleep, power-off, or active-Space change; physical lifecycle behavior is not claimed beyond observed events |
| Display connection/mode/mirror/rotation change | Phase 5 fail-closed | Reject when the CoreGraphics display-configuration callback changes during capture |
| Display scale change during capture | Phase 5 fail-closed | Reject when ScreenCaptureKit per-frame geometry, point-pixel scale, or content scale changes |
| Cross-display window move | Out of scope | A sampled window-frame change rejects; a move with unchanged observable frame/scale is not claimed |
| Rotated displays | Out of scope | Fail closed if observed; physical rotation behavior is not claimed |

The native bridge preserves signed Accessibility coordinates until it maps them
relative to the selected window. This prevents a left- or above-primary display
from being rejected merely because its global origin is negative. It is a
coordinate-correctness property, not multi-display assurance or device QA.

Phase 5 records the coverage state for title/owner continuity, frame gaps,
transient input, overlays, and AX/Vision disagreement in
[docs/phase5-alpha.md](phase5-alpha.md). An `in progress` or
`detected-but-not-covered` state never expands this matrix's claim surface.

## Privacy Classes

| Data type | First gate behavior |
|---|---|
| Emails | Destroy |
| Phone numbers | Destroy |
| Names | Destroy |
| URLs | Destroy |
| File paths | Destroy |
| Ticket IDs and titles | Destroy |
| Tokens and auth codes | Destroy |
| Chat messages | Destroy |
| Form values | Destroy |
| Static app chrome | Preserve only with explicit allowlist |
| Unknown text-like region | Destroy in Strict Reveal |

## Metadata And Tracks

| Channel | First gate behavior |
|---|---|
| Video pixels | Required protection path |
| Audio | Disabled by default |
| Subtitles/captions | Strip or fail closed |
| Container metadata | Reject AVFoundation metadata and untrusted MP4 user-data/timed-event boxes |
| File names in receipt | Sanitize or minimize |
| Thumbnails/previews | Detected-but-not-covered: a fresh Quick Look thumbnail of the current verified Strict Shield MP4 was visually and pixel-checked as neutral black; macOS-generated derivatives and caches are not exhaustively enumerated |

## Quality Gates

First R&D checkpoint:

- SCK prototype emits frame metadata.
- SCK shareable-content discovery returns redacted candidates and fails closed
  on empty, failed, disconnected, or timed-out discovery.
- SCK stream lifecycle waits for native start/stop completion and fails closed
  on error, disconnection, or timeout.
- After native stop completion, the serial SCK output queue is drained before
  frame evidence, the writer, or the MaskPlan can be finalized.
- SCK target plans bridge selected redacted candidates to real
  `SCContentFilter` / `SCStreamConfiguration` objects with audio disabled.
- CLI QA may enumerate redacted SCK candidates, but CLI capture start is not
  accepted as frame-metadata evidence.
- Real captured frame metadata must come from a GUI-hosted packaged-app or
  signed-helper harness that can fail closed without process abort.
- AX prototype emits window/text bounds.
- Vision prototype emits Japanese and English text/shape observations.
- Canonical coordinate tests exist.
- MaskPlan schema exists.

Commercial checkpoint:

- Strict Reveal packaged-app path works.
- Unknown regions are destroyed.
- Independent verification checks final encoded output.
- Failures block or remove output.
- Manual QA covers every Required row above.

## Claim Wording

Allowed during R&D:

```text
High-assurance Secure Share is under local-first R&D.
```

Allowed after first R&D checkpoint:

```text
DropSquash can prototype local window-aware mask planning for Japanese and
English Mac screen recordings.
```

Allowed only after commercial checkpoint:

```text
DropSquash can produce locally verified Strict Reveal exports for the supported
macOS capture scope.
```

Forbidden until separately proven:

- leak-zero
- protects all PII
- enterprise audit-ready
- works on every app
- works on existing videos with the same assurance as built-in capture
