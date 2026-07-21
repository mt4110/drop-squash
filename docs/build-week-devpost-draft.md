# Build Week Devpost Draft

Use this draft as the source text for the Devpost submission.

## Project Title

DropSquash Secure Share Alpha

## Tagline

Local-first screen-recording evidence for QA teams, with native macOS masking,
independent verification, and redacted signed MaskPlans.

## Track

Developer Tools

## Short Description

DropSquash started as a local Mac app for compressing screen recordings without
uploading media. For Build Week, it now has a Secure Share R&D alpha: a native
macOS window recorder using ScreenCaptureKit frame metadata, Accessibility
structure plus focused-input geometry tied to the selected window, and local
Apple Vision text/shape observations. Its Phase 5 Strict
Shield path destroys the full captured frame before encoding, independently
decodes the MP4 to verify that destructive region, and writes a redacted
Ed25519-signed MaskPlan sidecar without storing recognized text or observation
geometry.

The alpha does not claim completed privacy protection. It produces redacted
evidence, a Strict Reveal `MaskPlan`, and independent final-output verification
without storing recognized private OCR text. Signature, metadata, or pixel
verification failures fail closed before output publication.

## What It Does

- compresses local Mac screen recordings through native Apple media APIs
- preserves originals and does not upload media
- opens a Japanese/English adversarial browser fixture
- observes the selected browser window from the packaged macOS app
- records ScreenCaptureKit frame size and frame count
- records Accessibility observation count
- records focused-input geometry count without input values
- records local Vision text/shape observation count
- emits a redacted `MaskPlan` sidecar bound to the MP4 SHA-256
- records only redacted AX/focused-input/Vision aggregate counts, while Strict Shield destroys
  every captured frame before encoding and independently checks decoded output
- signs and verifies the sidecar locally with Ed25519 before publication
- keeps recognized private text out of evidence JSON

## Why It Matters

QA teams often need to share screen recordings that contain customer names,
emails, tokens, filenames, modals, notifications, or low-contrast private text.
Traditional blur or manual rectangle tools are easy to misuse and hard to
audit. DropSquash is exploring a stricter route: local native observation,
deterministic mask planning, destructive pixel overwrite, and independent
verification before a recording can be shared.

The current Build Week submission is an R&D alpha, not the final security
product or a leak-zero claim.

## Demo Evidence

Recorded local packaged-app evidence:

```text
Fixture: tests/fixtures/secure-share/NativeAccessibilityFixture.swift
Window: 760x652 native Japanese/English accessibility fixture
Frames: 144
Accessibility observations: 70 aggregate-only
Local Vision observations: 80 aggregate-only
MaskPlan policy: strict_reveal / Strict Shield
Full-frame destructive regions: 144
Independent decoded output verification: passed
Audio and metadata: absent
Sidecar signature: Ed25519 verified before publication
Recognized private text or observation geometry stored: no
MP4 SHA-256: fb3da3c40c285bf70194eabae611adc0ddd148d0853a05457b88d829c6d834cf
```

Build Week local artifact:

```text
DMG: target/release/bundle/dmg/DropSquash_0.1.0_aarch64.dmg
SHA-256: 6c0d82d6d11d351cec9fb9886f46529ecc7c3aba3d327a7870643c7c1dd721b0
Apple notarization: Accepted
Notary submission: 42774ba8-fe01-4439-99d1-6c8565e499aa
Staple/validate: passed
Mounted app Gatekeeper assessment: accepted, source=Notarized Developer ID
```

See:

- `docs/phase5-alpha.md`
- `docs/build-week-phase5-evidence.json`
- `docs/build-week-ui-fixture-evidence.json`
- `docs/build-week-judge-runbook.md`
- `docs/build-week-demo-script.md`

## How To Run

```bash
nix develop
pnpm --dir apps/desktop tauri build
open -a "Google Chrome" tests/fixtures/secure-share/ja-en-browser-form.html
open target/release/bundle/macos/DropSquash.app
```

In the app, choose the fixture window, record it, and use **Stop and save
verification**. The saved state provides controls to reveal the all-black MP4 and its
signed MaskPlan sidecar.

For the submitted local artifact, use the notarized DMG above instead of
claiming a public release. It is a Build Week evidence artifact, not a paid
beta distribution.

## How Codex And GPT-5.6 Were Used

Codex was used as the primary engineering partner for:

- converting a broad product idea into a strict Build Week alpha scope
- rejecting overclaims such as leak-zero, audit-ready, or complete PII safety
- splitting Rust/Tauri work under strict file-size limits
- adding the Japanese/English adversarial fixture and annotation sidecar
- wiring the packaged-app ScreenCaptureKit, Accessibility, and Vision
  observation evidence into a redacted MaskPlan, final-video verification, and
  signed sidecar
- preparing README, judging runbook, evidence summary, and demo script
- keeping no-upload, no-ffmpeg, original-safety, and local-first constraints
  visible while the prototype evolved

## What Is Not Finished Yet

- selective Secure Share redaction that preserves useful non-sensitive pixels
- audio capture, display capture, and any general-purpose recorder workflow
- endpoint protection against other recording, screenshot, clipboard-sync, or
  remote-control software on the same Mac
- Windows/Linux Secure Share support
- commercial checkout
- do not claim that the app is leak-zero or enterprise audit-ready

## Repository Access

The repository is private for submission safety. Read-only invitations are
prepared for:

- `testing@devpost.com`
- `build-week-event@openai.com`

This avoids publishing private productization work while still giving judges
access to source, setup instructions, fixtures, and evidence.

## Suggested Video Description

```text
DropSquash Secure Share Alpha is a local-first macOS prototype for QA teams
that need safer screen-recording evidence. The demo shows a synthetic
Japanese/English sensitive-data fixture, packaged-app ScreenCaptureKit +
Accessibility + local Vision observation, Strict Shield full-frame destructive
black masking, and a redacted Strict Reveal MaskPlan sidecar that stores the
canonical full-frame region plus aggregate counts, but no recognized private
OCR text or observation geometry. The final MP4 is independently decoded before
publication. This is an R&D alpha, not a leak-zero claim.
```
