# Phase 6 Readiness Audit

Updated: 2026-07-22

## Decision

DropSquash is not ready to sell Secure Share as a selective privacy feature.
The macOS native path does have a valid Strict Shield baseline: it writes a
new final-size BGRA pixel buffer filled with black before giving that buffer to
`AVAssetWriter`. The saved MP4 is then decoded and checked. This is useful
research evidence, but a fully black video is not a useful shared QA artifact.

Phase 6 starts from this truth. It does not reopen production commerce or
claim automatic PII removal.

## Current Evidence

| Area | Current result | Customer claim allowed? |
| --- | --- | --- |
| macOS selected-window capture | Native ScreenCaptureKit session observes the selected target and fails on continuity changes | Research only |
| Local observations | Accessibility, local Vision, and temporal observations are copied through the Apple bridge | Research only |
| Strict Shield destruction | Each accepted frame is overwritten to black in a new final-size pixel buffer before encoding | Bounded research evidence |
| Output verification | The saved MP4 is independently decoded and checked against a generated MaskPlan | Bounded research evidence |
| Receipt and plan | Local signed sidecar generation and signature verification exist | Not an audit-ready claim |
| Selective output | The native writer currently uses full-frame black, not a frame-exact selective plan | No |
| Windows Secure Share | No Media Foundation native capture bridge or packaged evidence exists | No |
| Notarized distribution | Developer ID signing exists; notarization evidence is still absent | No public distribution |

## Non-Negotiable Boundary

The observations collected during capture must not be presented as proof that
their proposed rectangles were used by the writer. At present the writer uses
Strict Shield for every accepted frame. A generated selective MaskPlan is an
analysis artifact until the native writer receives the exact plan for each
frame and emits copied, per-frame destruction evidence.

Therefore the UI and public material must say one of these two things:

- `Strict Shield research: the selected output is fully destroyed before encoding.`
- `Selective sharing is unavailable for this target.`

They must not say that text is selectively removed automatically.

## Phase 6 Acceptance Criteria

1. The Apple bridge emits copied per-frame destruction facts: frame index,
   policy, full-frame/region count, output dimensions, and append success.
2. Rust binds these facts to the same recording session without moving Apple
   reference types or pixel-buffer pointers across the boundary.
3. The signed sidecar records the native writer policy and every accepted
   frame count; a missing fact rejects the export.
4. A labeled, public-safe Strict Shield fixture produces an MP4, plan, receipt,
   and independent verification result from a packaged macOS app.
5. Fault injection for a missing writer fact, observation discontinuity, or
   writer append failure produces no final MP4.

## Phase 7 Promotion Gate

Selective masking may begin only after Phase 6. Its initial scope is a
published macOS fixture matrix, not arbitrary customer software. For each
accepted frame, the native writer must receive the exact resolved regions
before append, overwrite them in the final-size buffer, and emit the same
region digest to the receipt path. The independent decoder must then verify
those regions in the saved MP4.

Any unknown, late, conflicting, or missing observation remains Strict Shield
or fail-closed. A useful-looking selective video is not sufficient evidence.

## Commercial Lanes

| Lane | What can advance now | What remains blocked |
| --- | --- | --- |
| C2C Pro | Safe local single-file conversion and the existing macOS product path | Secure Share privacy promise and paid checkout |
| B2B pilot | Research conversations using labeled Strict Shield test artifacts | Enterprise offer, compliance promise, selective-mask pricing |
| Windows B2B | Contract, fixture schema, verifier compatibility, and acceptance tests | Product UI, capture, or sales claim before native Media Foundation evidence |

## Phase 10 Exit

Phase 10 is finite. It ends with an owner decision package containing:

- a versioned macOS fixture matrix and its pass/fail evidence;
- an honest supported-surface statement for C2C and B2B;
- a Windows native-bridge implementation or an explicit no-go decision;
- notarization and clean-machine evidence for any build proposed for testing;
- pricing, pilot terms, support, refund, and license drafts that match the
  actually supported behavior; and
- one explicit decision: `Go`, `Iterate`, or `Stop`.

No missing item is silently converted into a marketing claim.
