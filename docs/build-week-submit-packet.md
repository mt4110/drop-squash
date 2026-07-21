# Build Week Submit Packet

Use this file as the final copy desk while filling Devpost. It is an evidence
packet, not a public product release note.

## Deadline

- Devpost deadline: Tuesday, July 21, 2026 5:00 PM PT
- Japan time: Wednesday, July 22, 2026 9:00 AM JST

## Track

Developer Tools

## Project Title

DropSquash Secure Share Alpha

## Tagline

Local-first screen-recording evidence for QA teams, with native macOS masking,
independent verification, and redacted signed MaskPlans.

## Short Description

DropSquash started as a local Mac app for compressing screen recordings without
uploading media. For Build Week, it now has a Secure Share R&D alpha: a native
macOS window recorder using ScreenCaptureKit frame metadata, Accessibility
structure plus focused-input geometry tied to the selected window, and local
Apple Vision text/shape observations.

Its Phase 5 Strict Shield path destroys the full captured frame before
encoding, independently decodes the MP4 to verify that destructive region, and
writes a redacted Ed25519-signed MaskPlan sidecar without storing recognized
text or observation geometry.

This is not a leak-zero, enterprise audit-ready, or completed selective masking
claim.

## Demo Artifact

- DMG:
  `target/release/bundle/dmg/DropSquash_0.1.0_aarch64.dmg`
- SHA-256:
  `6c0d82d6d11d351cec9fb9886f46529ecc7c3aba3d327a7870643c7c1dd721b0`
- Apple notarization:
  `Accepted`
- Notary submission:
  `42774ba8-fe01-4439-99d1-6c8565e499aa`
- Staple / validate:
  passed
- Mounted app Gatekeeper:
  `accepted`, `source=Notarized Developer ID`

## Evidence Summary

- Fixture:
  `tests/fixtures/secure-share/NativeAccessibilityFixture.swift`
- Frame count:
  144
- Accessibility observations:
  70 aggregate-only
- Local Vision observations:
  80 aggregate-only
- MaskPlan policy:
  `strict_reveal`
- Full-frame destructive regions:
  144
- Independent decoded output verification:
  passed
- Audio and metadata:
  absent
- Sidecar signature:
  Ed25519 verified before publication
- Recognized private text or observation geometry stored:
  no

## Recommended Local Zip Contents

Generate the latest local zip with:

```bash
cargo run -p xtask -- build-week-submit-package
```

Include only shareable evidence files. Do not include private docs, API keys,
`.p8` files, local app state, or raw recording fixtures with real data.

- `DropSquash_0.1.0_aarch64.dmg`
- `DropSquash_0.1.0_aarch64.dmg.sha256`
- `README.md`
- `docs/build-week-submit-packet.md`
- `docs/build-week-devpost-draft.md`
- `docs/build-week-devpost-fields.md`
- `docs/build-week-devpost-checklist.md`
- `docs/build-week-final-entry-sheet.md`
- `docs/build-week-judge-runbook.md`
- `docs/build-week-phase5-evidence.json`
- `docs/phase5-alpha.md`
- `docs/phase5-native-bridge-qa.md`
- `docs/phase5-native-selective-pipeline.md`
- `docs/secure-share-support-matrix.md`

## Repository Access

Keep the repository private for submission. Confirm the pending or accepted
read-only invitations before submitting:

- Repository:
  `https://github.com/mt4110/drop-squash`
- Visibility:
  private
- `devposttesting`:
  accepted collaborator, read permission
- `build-week-event@openai.com`:
  pending email invitation `326181116`, read permission, `expired=false`

## Video Description

```text
DropSquash Secure Share Alpha is a local-first macOS prototype for QA teams
that need safer screen-recording evidence. The demo shows a synthetic
Japanese/English sensitive-data fixture, packaged-app ScreenCaptureKit,
Accessibility, and local Vision observation, Strict Shield full-frame
destructive black masking, and a redacted Strict Reveal MaskPlan sidecar that
stores the canonical full-frame region plus aggregate counts, but no
recognized private OCR text or observation geometry. The final MP4 is
independently decoded before publication. This is an R&D alpha, not a
leak-zero claim.
```

## External Steps Left

- Upload the demo video to YouTube or provide another Devpost-acceptable public
  demo link.
- Run `/feedback` in the main Codex build thread and copy the session ID.
- Paste the Devpost draft from
  `docs/build-week-devpost-draft.md`.
- For field-by-field copy, use
  `docs/build-week-devpost-fields.md`.
- For input order and claim-boundary checks, use
  `docs/build-week-devpost-checklist.md`.
- Paste the repository URL.
- Confirm repository read access for the Devpost/OpenAI accounts.
- Submit before the deadline.

## Deadline Triage

As of the local submit gate, the signed/notarized app evidence is ready enough
for a Build Week R&D alpha submission. Do not switch to sales, payments, or
new product claims before the Devpost form is submitted. If the deadline is
missed, move to a separate sales/productization fallback phase after preserving
the current evidence packet and recording exactly which external step failed.

## Final Local Checks

```bash
cargo run -p xtask -- build-week-local-submit-check
cargo run -p xtask -- build-week-phase5-check
cargo run -p xtask -- file-size-check
cargo run -p xtask -- privacy-policy-check
git diff --check
```

`build-week-local-submit-check` verifies the DMG checksum, code signature,
stapled notarization ticket, mounted-app Gatekeeper acceptance, and required
Devpost packet fields. It intentionally leaves the demo video URL,
`/feedback` session ID, and final Devpost submission as manual external steps.
