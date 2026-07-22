# Windows Secure Share Entry Gate

## Decision

Windows is a B2B target, not a supported Secure Share product today. Do not
ship a Windows privacy UI, accept pilot money, or claim parity until every gate
in this document has evidence from a Windows test machine.

## Native Boundary

The first bridge is C++/WinRT, owned by the Windows platform module. It keeps
all WinRT, COM, Direct3D, UI Automation, and Media Foundation objects on
Windows-owned threads. Rust receives copied values only: status, monotonic QPC
time, frame dimensions, redacted rectangles, reason codes, and artifact bytes.

The initial capture path is:

```text
GraphicsCaptureItem (one HWND)
  -> Direct3D11CaptureFramePool
  -> D3D11 final-size texture overwrite
  -> Media Foundation encoder
  -> saved MP4 independent decode
  -> shared Rust evidence verifier
```

Windows Graphics Capture exposes a frame surface, content size, and
`SystemRelativeTime`; Microsoft documents `CreateFreeThreaded` for a frame pool
whose events run on its internal worker thread. See [Microsoft screen capture
documentation](https://learn.microsoft.com/en-us/windows/apps/develop/media-authoring-processing/screen-capture)
and [CreateFreeThreaded](https://learn.microsoft.com/en-us/uwp/api/windows.graphics.capture.direct3d11captureframepool.createfreethreaded).

## Entry Gates

| Gate | Required proof | Rejection rule |
| --- | --- | --- |
| W0 OS support | Supported Windows build, GPU driver, DPI, HDR state, and capture permission are recorded | Unknown environment blocks start |
| W1 Target identity | One HWND is selected and its owner, bounds, and DPI transform are copied at start | Target replacement, move, resize, or DPI change rejects output |
| W2 Capture continuity | Every accepted frame has monotonic QPC time, final frame size, and no lost/recreated frame-pool ambiguity | Missing, duplicate, late, or discontinuous frame rejects output |
| W3 Structure input | UI Automation observations and local OCR observations are redacted before crossing the bridge | Raw text, automation values, titles, or object pointers reject the run |
| W4 Destruction | The native bridge overwrites the final-size texture before Media Foundation receives it | Overlay, blur, crop-only, or post-encode masking is rejected |
| W5 Decode | A separate decoder reads the saved MP4 and checks timeline, black regions, audio, streams, and metadata | Any failed check deletes the final output |
| W6 Evidence | Shared Rust schema binds output hash, plan, bridge version, and copied destruction facts | Signature or hash mismatch deletes the final output |
| W7 Packaging | Exact signed build is tested on a clean Windows account or machine | No download or pilot offer before this evidence |

## Mandatory Rejection Conditions

The bridge must reject rather than guess when any of these conditions occurs:

- protected-content, secure-desktop, lock-screen, session switch, remote-session,
  or capture-permission transition;
- an HWND becomes minimized, cloaked, replaced, reparented, or moves to a
  different desktop or monitor configuration;
- a Direct3D device reset, texture sharing/fence failure, frame-pool recreation,
  format change, HDR state change, or unsupported color-space transition;
- a missing UI Automation/OCR observation required by the active policy, or an
  observation that cannot be reduced to copied redacted geometry before it
  reaches Rust; and
- an unsigned native bridge DLL, an unrecorded bridge version, or a mismatch
  between the capture bridge and the evidence receipt.

The first Windows experiment has no permissive fallback. If a condition makes
the source or final-size texture ambiguous, it emits no final MP4. A future
Strict Shield fallback is allowed only after the exact state transition has a
fixture result and independently decoded destructive output evidence.

## First Supported Matrix

The first experiment is intentionally narrow:

- Windows 11 only, one display, SDR, one selected top-level HWND.
- Fixed DPI scale and no HDR, display attach/detach, rotation, or remote desktop.
- Synthetic Japanese/English fixture window only.
- Solid black destructive policy only; no selective customer export.
- No audio, captions, or existing-recording provenance claim.

Anything outside the matrix is Strict Shield research or fail-closed. It is not
silently downgraded into an unverified recording.

## Required Fixture Results

Run every case from a packaged Windows build and record the same shared schema
used on macOS:

1. stationary synthetic text: accepted Strict Shield output and verifier pass;
2. target move, resize, close, owner replacement, and focus loss: no final MP4;
3. DPI change, display change, frame-pool recreation, and timestamp gap: no
   final MP4;
4. Japanese/English input, paste, transient popup, and modal: Strict Shield or
   explicit failure, with no raw text in sidecars;
5. tampered MP4, tampered plan, and missing native destruction fact: verifier
   rejection.

## Promotion Rule

Only after W0 through W7 and the fixture results exist may Phase 7 selective
masking research begin on Windows. It must use the same deterministic MaskPlan
schema and independent final decode as macOS. A visual match with the macOS UI
does not count as feature compatibility.
