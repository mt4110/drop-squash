# Secure Share Threat Model

This document defines what high-assurance Secure Share is trying to protect.

It is intentionally strict. DropSquash must not sell a privacy claim that is
stronger than the evidence it can produce.

## Product Claim Target

Target claim after the R&D and verification gates pass:

```text
DropSquash can locally create a screen-recording export where sensitive and
unknown visual regions are destructively removed before encode, the final
output is independently checked, and the receipt explains the result.
```

Do not claim leak-zero yet. "Leak-zero" is the direction, not the current
state.

## Assets To Protect

High-assurance Secure Share protects visual information that appears in a Mac
screen recording:

- customer names
- user names
- email addresses
- phone numbers
- account IDs
- ticket titles
- file names
- URLs
- auth codes
- tokens
- notification text
- form values
- chat messages
- document text
- browser content
- modal and sheet content
- sidebar labels when they may contain workspace, project, or customer names

Audio is out of scope until a separate audio redaction policy exists. Secure
mode must disable audio by default.

## Primary Users

- QA company operators preparing defect evidence
- contractors sharing reproduction videos with clients
- support engineers preparing customer-safe recordings
- small teams that need local-only handling without a cloud privacy pipeline

## Adversary Model

Assume the recipient may:

- pause on any frame
- zoom the video
- extract frames from the video
- compare neighboring frames
- apply contrast or brightness enhancement before inspection
- use OCR on the final output
- inspect video metadata
- inspect receipt fields
- inspect file names shared with the video

Assume the recipient cannot:

- read the user's original local recording after export
- access DropSquash process memory
- compromise macOS permissions
- access private local app data unless the user shares it

This model also does not treat DropSquash as a macOS endpoint-security product.
It cannot prevent another locally authorized process from taking screenshots,
recording the display, reading the clipboard, synchronizing files, or sharing
the screen while a user is working. Those channels are outside the exported
MP4/sidecar boundary and require OS policy or endpoint controls.

If the stronger adversary is required, the feature must be scoped as an
enterprise security review before release.

## Supported Input Boundary

High-assurance mode requires DropSquash built-in capture because already
recorded `.mov` or `.mp4` files do not contain trusted capture-time context.

Existing files may be sanitized, but they cannot prove:

- exact capture window identity
- frame status
- content scale
- Accessibility timeline
- transient overlay state
- whether audio or metadata already leaked

Existing-file sanitization is therefore assistive, not high assurance.

## Leakage Channels

Treat these as first-class leakage channels:

- unmasked pixels in the final video
- edge pixels left by scaling, chroma, or filtering
- one-frame secrets
- typed and composing text, including IME candidate panels and conversion UI
- autofill, password-manager, paste, spelling, and accessibility overlays
- display configuration changes during capture
- screen lock, sleep, wake, and screen-saver transitions
- notifications and popovers
- raw window titles
- file names in receipts
- video metadata and non-video tracks
- audio
- subtitles and captions
- thumbnails and preview images
- partially written failed outputs
- unmasked temporary files
- unsupported cross-thread transfer of ScreenCaptureKit objects

## Security Invariants

High-assurance Secure Share must preserve these invariants:

1. Sensitive regions are destructively overwritten before encode.
2. Unknown regions are destroyed in Strict Reveal.
3. One-frame detections are masked immediately.
4. Missing Accessibility permission blocks high-assurance export.
5. Incomplete frame metadata marks affected frames untrusted.
6. Audio is disabled by default.
7. Plaintext unmasked raw capture is not written as a durable artifact.
8. Verification failure blocks, deletes, or quarantines the output.
9. Receipts are tamper-evident descriptions, not proof of perfect detection.
10. Public copy never claims more than the supported matrix proves.
11. Stop completion drains the serial capture-output queue before final evidence
    is read; an incomplete or discontinuous frame still fails closed.
12. Target and display continuity checks remain active until native capture
    stop completion, not merely until the user requested stop.

## Policy Classes

### Safe

Safe regions may remain visible.

A region is safe only when it is:

- explicitly allowlisted as static product chrome
- synthetically re-rendered from trusted app identity
- verified by policy as non-user-content

Raw title bars are not safe by default.

### Sensitive

Sensitive regions must be destroyed.

Examples:

- text fields
- tables
- document bodies
- chat content
- notifications
- modal bodies
- browser content
- URLs and file names

### Unknown

Unknown regions are areas where the app cannot confidently prove safety.

Strict Reveal destroys unknown regions. Smart Mask may flag unknown regions,
but Smart Mask must not be sold as high assurance.

## Transient Input And Overlay Rule

Typing and system overlays cannot be treated as reliably observable just
because Accessibility or Vision reported them once. A 50ms composition update,
private password-manager surface, browser-owned autofill panel, or external
notification can be absent from either source while still being visible to a
viewer.

Therefore the high-assurance Phase 5 path does not use observations to decide
which pixels survive: each accepted ScreenCaptureKit frame receives an exact
full-frame destructive region before encoding. AX and Vision are retained only
as aggregate, redacted audit counts. If a frame, its continuity metadata, the
destructive write, final decode, audio check, metadata check, or signed plan
cannot be verified, publication fails closed. This reduces visual disclosure;
it does not prove that every OS-owned surface was observed.

## Fail-Closed Rules

High-assurance export must fail closed when:

- screen capture permission is missing
- Accessibility permission is missing
- the selected capture scope cannot be identified
- frame metadata is missing or inconsistent
- coordinate transforms cannot be verified
- destructive pixel writing fails
- independent verification cannot run
- independent verification finds a non-black pixel inside a required
  full-frame destructive region
- receipt writing fails
- a capture-time change prevents the target or display continuity from being
  revalidated before stop completion

The app may offer a lower-assurance export only when it clearly changes mode
and removes high-assurance claims from the result.

## Out Of Scope For First High-Assurance Gate

Target-continuity evidence is limited by the signals macOS exposes. The current
native bridge compares CoreGraphics and Accessibility window geometry before
every accepted frame and again before writer completion. It keeps the matching
AX window inside the native session and subscribes to `AXMoved`, `AXResized`,
and destruction events; an event fails the capture. Missing permission or a
geometry mismatch also fails before publication. This is not a claim that every
macOS application reliably exposes target-continuity signals; packaged-app
adversarial QA remains required.

Successful current-path evidence also signs a bounded continuity attestation
and a redacted required-watch list. It states that the display, CoreGraphics
window, Accessibility geometry and window-event, macOS lifecycle, and
foreground-activation watches completed without a failure; it does not expose a target title, window
identifier, geometry, app name, notification payload, AX payload, or
user-visible text.

These are not protected yet:

- spoken audio
- faces
- logos that imply private context
- diagrams with embedded secrets not detected as text
- QR codes and barcodes
- handwritten text
- non-Japanese and non-English text
- malicious applications that intentionally evade capture or AX reporting
- separate screen-recording, screenshot, remote-control, clipboard-sync, or
  file-sync applications running on the same Mac
- screen recordings captured outside DropSquash without sidecar metadata
- any capture path that relies on a Rust binding's unproven `Send` contract

Each item needs a separate support-matrix row before public claims expand.

Screen lock and sleep are separately tracked in the Phase 5 matrix. A
`Suspended` or other non-complete ScreenCaptureKit frame, a resumed
presentation-time gap, session resignation, screen sleep, workspace sleep,
power-off, or active-Space change currently fails closed. That is not evidence
that every macOS lock, sleep, wake, or screen-saver transition has been
observed in a packaged-app run.

## Acceptance Evidence

This threat model is accepted only when these files exist and cross-reference
it:

- `docs/secure-share-support-matrix.md`
- `docs/secure-share-adversarial-corpus.md`
- `docs/secure-share-maskplan.md`
- `docs/p2-alpha-review.md`

Implementation evidence must later include packaged-app manual QA and final
output verification, not only unit tests.

Phase 5 adds [capture continuity and exposure coverage](phase5-alpha.md). It
does not change the current Strict Shield claim boundary.
