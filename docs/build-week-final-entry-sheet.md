# Build Week Final Entry Sheet

Use this sheet only while entering the final Devpost form. It has two manual
placeholders: the public demo URL and the Codex `/feedback` session ID.

## Manual Values

```text
Demo video URL: https://youtu.be/6ljSp4eoq0g
Codex feedback session ID: PASTE_CODEX_FEEDBACK_SESSION_ID_HERE
Repository URL: https://github.com/mt4110/drop-squash
```

## Required Pre-Submit Checks

```sh
cargo run -p xtask -- build-week-submit-package
```

This command first runs the local submit gate, then creates a clean `/tmp`
zip after checking the package for forbidden file names and secret-like text.
Expected local-submit gate line:

```text
External still manual: /feedback session ID, Devpost submit
```

That line is correct until the external Devpost form is actually submitted.
Use the printed `zip:` path only if Devpost asks for an uploaded artifact; the
main required links are the repository URL and the uploaded demo video URL.

## Copy Order

1. Project name: `DropSquash Secure Share Alpha`
2. Track: `Developer Tools`
3. Elevator pitch:

```text
Local-first screen-recording evidence for QA teams, with native macOS masking,
independent verification, and redacted signed MaskPlans.
```

4. Built with:

```text
Rust, Tauri, TypeScript, React, AVFoundation, VideoToolbox,
ScreenCaptureKit, Accessibility API, Apple Vision, Ed25519, Nix
```

5. Repository URL:

```text
https://github.com/mt4110/drop-squash
```

6. Demo URL: paste the public demo video URL.
7. Short description:

```text
DropSquash started as a local Mac app for compressing screen recordings without
uploading media. For Build Week, it now has a Secure Share R&D alpha: a native
macOS selected-window recorder using ScreenCaptureKit frame metadata,
Accessibility structure plus focused-input geometry tied to the selected
window, and local Apple Vision text/shape observations.

Its Phase 5 Strict Shield path destroys the full captured frame before
encoding, independently decodes the MP4 to verify that destructive region, and
writes a redacted Ed25519-signed MaskPlan sidecar without storing recognized
text or observation geometry.

This is an R&D alpha, not a leak-zero, enterprise audit-ready, or completed
selective masking claim.
```

8. What it does:

```text
- compresses local Mac screen recordings through native Apple media APIs
- preserves originals and does not upload media
- records a selected macOS window through a local native path
- observes ScreenCaptureKit frame metadata
- observes Accessibility structure and focused-input geometry locally
- observes Apple Vision text/shape regions locally
- destroys the accepted captured frame before encoding in Strict Shield mode
- independently decodes the final MP4 before publication
- writes a redacted MaskPlan sidecar bound to the output SHA-256
- signs and verifies the sidecar locally with Ed25519
- stores no recognized private OCR text or observation geometry
```

9. Why it matters:

```text
QA teams often need to share screen recordings that contain customer names,
emails, tokens, filenames, modals, notifications, or low-contrast private text.
Traditional blur or manual rectangle tools are easy to misuse and hard to
audit. DropSquash is exploring a stricter route: local native observation,
deterministic mask planning, destructive pixel overwrite, and independent
verification before a recording can be shared.
```

10. Demo evidence:

```text
DMG: target/release/bundle/dmg/DropSquash_0.1.0_aarch64.dmg
DMG SHA-256: 6c0d82d6d11d351cec9fb9886f46529ecc7c3aba3d327a7870643c7c1dd721b0
Apple notarization: Accepted
Notary submission: 42774ba8-fe01-4439-99d1-6c8565e499aa
Staple/validate: passed
Mounted app Gatekeeper assessment: accepted, source=Notarized Developer ID

Fixture: tests/fixtures/secure-share/NativeAccessibilityFixture.swift
Frames: 144
Accessibility observations: 70 aggregate-only
Local Vision observations: 80 aggregate-only
MaskPlan policy: strict_reveal / Strict Shield
Full-frame destructive regions: 144
Independent decoded output verification: passed
Audio and metadata: absent
Sidecar signature: Ed25519 verified before publication
Recognized private text or observation geometry stored: no
```

11. How Codex and GPT-5.6 were used:

```text
Codex was used as the primary engineering partner to turn a broad product idea
into a strict Build Week alpha scope, reject unsupported security and privacy
completion claims, split Rust/Tauri work under strict file-size limits, add the
Japanese/English adversarial fixture and annotation sidecar, wire the
packaged-app ScreenCaptureKit, Accessibility, and Vision observation evidence
into a redacted MaskPlan and final-video verification path, and prepare README,
judging runbook, evidence summary, and demo script while preserving no-upload,
no-ffmpeg, original-safety, and local-first constraints.
```

12. What is not finished:

```text
- selective Secure Share redaction that preserves useful non-sensitive pixels
- audio capture, display capture, and any general-purpose recorder workflow
- protection against other local screenshot, clipboard-sync, remote-control,
  or endpoint recording software
- Windows/Linux Secure Share support
- commercial checkout
- do not claim complete privacy protection or enterprise audit readiness
```

13. Feedback session ID: paste the Codex `/feedback` session ID.

## Final Claim Check

- Do not write `leak-zero`.
- Do not write `enterprise audit-ready`.
- Do not write `complete selective masking`.
- Do not imply that all screen-recording leaks are prevented.
- Keep the project framed as a local-first R&D alpha with verified Strict
  Shield output and honest open research gaps.
