# Build Week Devpost Fields

Use these fields while filling Devpost. Keep the demo URL and `/feedback`
session ID as the only placeholders.

## Project Name

DropSquash Secure Share Alpha

## Elevator Pitch

Local-first screen-recording evidence for QA teams, with native macOS masking,
independent verification, and redacted signed MaskPlans.

## Built With

Rust, Tauri, TypeScript, React, AVFoundation, VideoToolbox,
ScreenCaptureKit, Accessibility API, Apple Vision, Ed25519, Nix

## Repo URL

```text
https://github.com/mt4110/drop-squash
```

The repository is private. Devpost/OpenAI judging access is handled through
read-only repository access:

- `devposttesting`: accepted read collaborator
- `build-week-event@openai.com`: pending read email invitation

## Demo URL

```text
https://youtu.be/6ljSp4eoq0g
```

## Short Description

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

## What It Does

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

## Why It Matters

QA teams often need to share screen recordings that contain customer names,
emails, tokens, filenames, modals, notifications, or low-contrast private text.
Traditional blur or manual rectangle tools are easy to misuse and hard to
audit. DropSquash is exploring a stricter route: local native observation,
deterministic mask planning, destructive pixel overwrite, and independent
verification before a recording can be shared.

## Demo Evidence

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

## How Codex And GPT-5.6 Were Used

Codex was used as the primary engineering partner to turn a broad product idea
into a strict Build Week alpha scope, reject unsupported security and privacy
completion claims, split Rust/Tauri work under strict file-size limits, add the
Japanese/English adversarial fixture and annotation sidecar, wire the
packaged-app ScreenCaptureKit, Accessibility, and Vision observation evidence
into a redacted MaskPlan and final-video verification path, and prepare README,
judging runbook, evidence summary, and demo script while preserving no-upload,
no-ffmpeg, original-safety, and local-first constraints.

## What Is Not Finished

- selective Secure Share redaction that preserves useful non-sensitive pixels
- audio capture, display capture, and any general-purpose recorder workflow
- protection against other local screenshot, clipboard-sync, remote-control,
  or endpoint recording software
- Windows/Linux Secure Share support
- commercial checkout
- do not claim complete privacy protection or enterprise audit readiness

## Feedback Session ID

```text
PASTE_CODEX_FEEDBACK_SESSION_ID_HERE
```
