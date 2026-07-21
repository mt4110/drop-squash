# Phase 4 Alpha Gate

This is an R&D gate for Secure Share. It is not a commercial security claim.

## Objective

Build a local macOS path that derives a redacted Strict Reveal `MaskPlan`,
destructively overwrites its regions in final output frames, independently
decodes the output, and fails closed before publishing an output file.

## Required Invariants

- No recognized text content is stored in `MaskPlan` or its receipt.
- A `MaskPlan` export accepts only `StrictReveal` and `SolidBlack`.
- Every decoded output frame must have a matching `MaskPlan` frame.
- The decoded frame size must exactly match the plan's frame size.
- The capture configuration must use that same final, even-pixel frame size;
  the encoder must not silently round one coordinate system after planning.
- Every planned destructive region must be black after an independent decode.
- The residual-pixel verifier must reject a planned region containing a BGR
  channel above the black threshold; this behavior is covered without a
  native pixel-buffer dependency.
- The independently decoded output must contain no audio track or asset/track metadata.
- A verification failure leaves the temporary output unfinalized.
- A start, stop, or finalization timeout revokes the publication permit and
  removes the partial MP4, final MP4, and temporary/final receipt. A worker
  that finishes late cannot publish after that revocation.

## Current Implementation

- `ScreenCaptureKit`, Accessibility, and local Vision produce a redacted
  preview plan in the packaged-app alpha harness.
- Accessibility rectangles are converted from global macOS coordinates into
  the selected window's capture-pixel coordinates before recording starts;
  an out-of-target rectangle fails closed rather than being guessed.
- The VideoToolbox export path resolves each output frame through the plan and
  overwrites pixels before append.
- A separate `AVAssetReader` reopens the temporary MP4 and checks all pixels in
  each planned region before `finalize_verified_output` may rename it.
- The packaged alpha records only the already-blackened video samples, then
  independently checks video pixels, audio absence, and asset/track metadata
  before moving `.partial.mp4` to its final name.
- The finalized MP4 is paired with a redacted `mask-plan.json` sidecar. It
  contains frame geometry, policy, reasons, and the MP4 SHA-256, but never
  recognized text or window titles. The sidecar payload is signed and verified
  with a locally persisted Ed25519 key before publication. Failure to sign,
  verify, or publish either file removes the finalized video.
- `dropsquash verify-evidence <video> <sidecar>` independently checks the
  output name, SHA-256 binding, Ed25519 signature, and absence of raw text,
  Vision, or Accessibility observation fields in the sidecar.
- `dropsquash verify-secure-share-output <video> <sidecar>` additionally
  re-decodes the final MP4 and checks black regions, audio absence, and
  metadata absence against the signed plan.
- Secure Share preflights Screen Recording permission before discovering
  candidate windows and gives a localized permission recovery message.
- The packaged macOS UI now lists eligible windows, starts a silent
  window-only recording, exposes an explicit stop action, and only then enters
  destructive masking, independent verification, receipt signing, and save.
- The selected `Auto`, `1080p`, `720p`, or `480p` output size is resolved
  before capture. Accessibility, Vision, frame metadata, the `MaskPlan`, and
  destructive writes therefore share one final-size coordinate system.

## Evidence Still Required

The following safety evidence is implemented and verified:

1. A real saved capture whose full frame timeline produces a matching plan.
   Done: see `docs/build-week-evidence.json`.
2. A packaged-app export where independent decode verification succeeds.
   Done: the same evidence records the output SHA-256 and redacted sidecar.
3. A forced mismatch test showing the temporary output is not finalized.
   Done: MaskPlan frame-size mismatch is unit tested, and a packaged-app
   corrupt-signing-key run left no final or partial artifact.
4. JA/EN adversarial fixture results and a redacted signed receipt from that
   export. Done: see `docs/build-week-evidence.json`.
5. A user-controlled start/stop recording proves the production-shaped flow.
   On July 20, 2026, the signed local build selected a 604 by 421 window,
   recorded until explicit stop, normalized the final capture to 604 by 420,
   and saved 42 frames. The UI reported 714 local text candidates and 546
   masked regions. Both `verify-secure-share-output` and `verify-evidence`
   passed against the saved MP4 and signed sidecar.
6. A resize run must prove the plan uses the encoded dimensions. On July 20,
   2026, a 1443 by 950 ChatGPT window was captured with the `480p` setting.
   The signed sidecar recorded a final 640 by 420 plan, and both independent
   verification commands passed for its 19-frame output.
7. A 22-second native macOS end-to-end proof records target selection,
   recording state, explicit stop, and the verified save transition. The
   private QA artifact is `/private/tmp/dropsquash-phase4-end-to-end-1784519602.mov`.
8. The residual-pixel verifier accepts black pixels in padded 32BGRA rows and
   rejects a planned pixel with BGR `33, 0, 0`. This is covered by
   `videotoolbox::pixel_buffer::verify::tests`.
9. On July 20, 2026, the packaged app was switched to English through its
   visible `EN` control. The target selection, start recording, saved-output,
   receipt, local-only, and failure-state copy rendered in English without
   changing the verified recording result.
10. After adding the publication-timeout guard, the re-signed packaged app
    recorded the 604 by 421 fixture until explicit stop and saved
    `secure-share-recording-1784521893722-12664.mp4`. Its English UI reported
    244 frames, 4,148 local text candidates, and 3,172 blackened regions.
    The app published the MP4 and signed sidecar only after its independent
    decode gate. The desktop unit test
    `cancelled_recording_cannot_publish_late_output` proves that a cancelled
    worker cannot publish a late partial MP4 or receipt.
11. A new 16-second proof after that guard is
    `/private/tmp/dropsquash-phase4-guarded-end-to-end-1784522487.mov`. It
    shows target selection, start, recording state, explicit stop, and the
    verified-save result. Its generated MP4
    `secure-share-recording-1784522489855-12664.mp4` independently passed
    both CLI verifiers with 47 masked frames and a 604 by 420 signed plan.
12. The signed app was resized to its configured minimum 480 by 520 window.
    The target selector and start control remained visible; lower settings
    remained reachable through the application scroll area without overlap.
    `secureShareError.test.ts` also fixes the clear Japanese and English
    Screen Recording permission recovery copy in the web test run.

The signature is a local R&D-alpha device identity, not a hardware-backed or
independent audit authority. It is evidence binding, not an enterprise claim.

## Completion Audit

| Requirement | Current evidence |
| --- | --- |
| Target, start, stop, save | Guarded native proof and items 10-11 above |
| Local AX/Vision detection | Packaged runs report local candidate counts; sidecars contain geometry only |
| Frame-exact final-size plan | 604 by 420 and 640 by 420 signed plans; frame-count verifier |
| Destructive blackening | Live writer plus decoded-region verifier and residual-pixel test |
| Selected-size capture | 1443 by 950 target with `480p` produced a 640 by 420 verified plan |
| Fail-closed output | Mismatch, signing, and late-publication cancellation evidence |
| Signed receipt | `verify-evidence` passed for the latest guarded output |
| JA/EN and resizable UI | English native run, 480 by 520 native run, and permission-copy test |
| Local-first/no ffmpeg | `media-policy-check` and `privacy-policy-check` pass |

The audit proves a Strict Shield alpha only. It does not turn the current
all-black output into a selectively shareable product or a leak-zero claim.

## Current Quality Limit

The latest packaged capture selected the native JA/EN Accessibility fixture
and completed the safety gate for 20 frames. It produced five Accessibility
text observations and 200 local Vision observations. A missed text region was
found during independent decode verification. Therefore the shareable alpha
does not trust selective detection alone: it adds an `ax_unknown_client_area`
Strict Shield region over the capture target and destroys that region on every
frame. The output is safe to inspect as a black-frame evidence artifact, but
it is not a useful selectively redacted recording.

This is a high-assurance pipeline pass, not a selective-redaction product
quality pass. Do not present the current output as a useful redacted demo or
reopen checkout or sales work. The next R&D gate is a structured-window
resolver that can prove which non-text pixels are safe to retain while still
treating unresolved pixels as unknown and destructive. Until that gate has
adversarial evidence, do not claim leak-zero, complete PII protection,
shareable automatic redaction, or enterprise audit readiness.
