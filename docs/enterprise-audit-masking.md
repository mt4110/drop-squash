# High-Assurance Masking And Evidence

This document defines the commercial security direction for DropSquash.

Earlier docs treated enterprise masking as a later optional track. That is no
longer the product decision. Secure sharing is now the core value hypothesis:
the sales route should wait until DropSquash can get very close to leak-zero
behavior with honest fail-closed guarantees.

## Objective

Allow QA teams, contractors, and enterprise users to share screen recordings
without exposing confidential text, personal data, or accidental UI overlays.

The product should locally produce:

- a destructively masked video
- a receipt that explains what was masked and why
- verification evidence for the final output
- a strict failure when the app cannot preserve the privacy claim

## Claim Discipline

"Leak-zero" is an aspiration, not a current claim.

DropSquash may move toward that claim only by narrowing the supported scope,
destroying unknown regions, and proving the final encoded output. If a scenario
cannot be verified, the export must fail or the affected pixels must be
destroyed.

## Commercial Entry Rule

Do not establish the production paid sales route until all of these are true:

1. Strict Reveal mode works in the packaged macOS app.
2. Built-in capture records frame-level metadata.
3. Accessibility structure is collected or the export fails closed.
4. Local Vision text/shape observation is integrated.
5. Policy resolution is deterministic and inspectable.
6. Destructive masking is applied after final sizing and before encode.
7. Independent decode verification passes on the final output.
8. Receipts include mask reasons, sources, confidence, and verification result.
9. Manual QA includes adversarial Japanese and English cases.
10. The user accepts the result as commercially convincing.

Stripe, Lemon Squeezy, public checkout, and broad public claims remain paused
until this rule is satisfied.

## Architecture

### Capture Layer

High-assurance mode must use built-in macOS capture rather than relying only on
already-recorded videos.

Use ScreenCaptureKit to collect:

- frames
- frame status
- display time
- content scale
- content rect
- bounding rect
- selected window or display scope

Rules:

- incomplete, blank, or missing metadata frames are not trusted
- high-assurance capture must not write plaintext unmasked raw video as a
  durable artifact
- audio is disabled by default unless a separate audio redaction policy exists

### Structure Layer

Use Accessibility to collect the UI structure at capture time.

Collect:

- window and modal bounds
- sheets and dialogs
- text element bounds
- text-range bounds when available
- process identity hints

Rules:

- missing Accessibility permission blocks high-assurance export
- partial structure marks affected regions unknown
- unknown regions are destroyed in Strict Reveal

### Vision Layer

Use local Vision analysis for text and text-shape observations.

Collect:

- Japanese text regions
- English text regions
- text rectangles when recognition is uncertain
- confidence values

Rules:

- OCR does not need to read the string to mask the region
- one-frame observations are masked immediately
- temporal tracking may stabilize masks but may not delay first masking

### Policy Resolver

Merge capture, structure, and Vision observations into a frame-exact mask plan.

Policy classes:

- `safe`
- `sensitive`
- `unknown`

Strict Reveal default:

- safe regions remain only when explicitly allowed
- sensitive regions are destroyed
- unknown regions are destroyed

Smart Mask default:

- sensitive regions are destroyed
- unknown regions are flagged as risk unless the user chooses stricter output

### Pixel Destruction

Masking must happen on the final-size frame buffer immediately before encode.

Allowed secure modes:

- solid black
- CSPRNG-backed black noise
- CSPRNG-backed white noise

Blur is not allowed for secure exports.

### Verification

After encoding, decode the exported video and run a residual-leak verification
pass.

Verification must check:

- expected mask regions remain destroyed
- residual text/shape observations are absent or classified as safe
- output metadata and tracks do not leak unsupported data
- receipt hash matches the output

Failure deletes or quarantines the output and records a strict error.

### Receipt

Receipts are tamper-evident, not magical proof that detection was perfect.

Include:

- output SHA-256
- export timestamp UTC
- capture scope
- mask plan summary
- observation sources
- mask reasons
- confidence summary
- verification result
- implementation version
- supported-scope declaration

Signing should prefer a device or customer-owned key. Do not rely on an
extractable app-embedded private key for serious enterprise trust.

## Research Gates

Before implementation expands beyond prototype quality, write and maintain:

- threat model
- supported app and display matrix
- Japanese and English adversarial corpus
- false-negative taxonomy
- failure and deletion policy
- final-output verification procedure
- receipt schema
- manual QA script
- public claim wording

## Explicit Non-Goals

Do not build these as part of this security track:

- general video editor
- media library
- broad format converter
- YouTube downloader
- advertising integration
- cloud upload flow

## Local-First Rules

Always preserve:

- no media upload by default
- no `ffmpeg` or `ffprobe`
- original safety
- deterministic evidence
- inspectable local receipts
