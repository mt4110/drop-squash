# Secure Share Research Coverage Matrix

This matrix defines the fixture and research scope for high-assurance Secure
Share. It is not a customer support matrix and does not authorize a privacy or
enterprise product claim.

Nothing outside this matrix may be used in public claims until it has matching
implementation and QA evidence.

## Phase 6 Product Truth

The current native macOS writer is `Strict Shield`: every accepted frame is
written as a new all-black final-size buffer before encoding. The saved MP4 is
independently decoded and checked, and a signed redacted MaskPlan sidecar is
verified before publication.

This proves a narrow destructive-output baseline, not a useful selectively
shareable recording. The current packaged fixture rerun and its labeled Phase
6 evidence record remain required before this baseline can leave research.

| Capability | Current status | Customer claim |
| --- | --- | --- |
| Selected-window native capture | Research implementation | None |
| Strict Shield all-frame destruction | Bounded research baseline | None |
| Independent final-output verification | Implemented research gate | None |
| Selective text/shape masking | Not supported | Forbidden |
| Windows Secure Share | Not implemented | Forbidden |
| Notarized distribution | Not evidenced | Forbidden |

## Current Native Bridge Boundary

The historical `phase5-native-bridge-v1` signed-app fixture result is retained
as engineering context. It is not Phase 6 release evidence: the exact current
build and fixture must be rerun, labeled, and independently verified.

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

The QA-only `SmartMask` and residual-Vision paths remain historical experiments.
They are not a supported capture scope, product feature, or proof that any
unmasked pixel is safe to share. Phase 7 may revisit them only after the Phase
6 Strict Shield evidence record is complete.

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
