# Build Week Submission Plan

DropSquash should enter OpenAI Build Week as:

```text
Phase 5 alpha / high-assurance Secure Share R&D prototype
```

This is not a finished enterprise security product and must not be described
as leak-zero. The submission should show the serious technical direction:
local-first screen-recording evidence, native macOS capture signals, local
Vision observations, deterministic `MaskPlan` data, and an honest path toward
Strict Shield fail-closed export.

## Submission Goal

Build a working alpha that judges can understand in under three minutes:

1. open a Japanese/English adversarial screen fixture
2. select its window in DropSquash, start recording, then explicitly stop and
   blacken it before saving
3. show the saved MP4 and its redacted `MaskPlan` sidecar
4. show ScreenCaptureKit frame metadata and redacted Accessibility, focused-input,
   Vision, and changed-region aggregate counts behind that saved result
5. emit a redacted `MaskPlan` JSON containing only the canonical full-frame
   region and aggregate counts, never recognized text or observation geometry
6. prove Strict Shield destructively overwrites every accepted
   ScreenCaptureKit frame with solid black, with a black pixel readback
7. save a final MP4 only after independent final-output verification, then
   show its redacted, locally signed MaskPlan sidecar

## Track

Best fit:

```text
Developer Tools
```

Secondary fit:

```text
Work and Productivity
```

Use Developer Tools if the submission emphasizes deterministic QA evidence,
privacy verification, local-native capture, and security workflow.

## What To Claim

Safe claims:

- local-first prototype
- no media upload
- no `ffmpeg` or `ffprobe`
- native macOS observation path
- ScreenCaptureKit frame metadata foundation
- Accessibility structure foundation
- per-frame focused-input geometry tied to the selected window
- local Apple Vision text/shape observations
- redacted evidence, final-video `MaskPlan`, independent decode verification,
  and a locally signed sidecar
- designed for QA teams sharing screen recordings

Unsafe claims:

- leak-zero
- enterprise audit-ready
- complete automatic PII protection
- finished commercial masking
- production checkout ready
- supported Windows/Linux masking

## Required Artifacts

- working app or harness
- public YouTube demo video under 3 minutes with voiceover
- code repository URL
- README setup instructions
- sample data or fixture instructions
- explanation of how Codex and GPT-5.6 were used
- `/feedback` Codex session ID from the main build thread
- Devpost form draft:
  [docs/build-week-devpost-draft.md](/Users/masakitakemura/_workspace/drop-squash/docs/build-week-devpost-draft.md)

Repository visibility:

- public is acceptable only with a relevant open source license
- private is acceptable if shared with `testing@devpost.com` and
  `build-week-event@openai.com`
- before making the repository public, remove or keep ignored all private keys,
  local receipts, `.codex` state, generated build artifacts, and private docs

Recommended choice for this project:

```text
Keep the repository private for submission, then invite the Devpost/OpenAI
review emails as read-only collaborators.
```

Use GitHub UI near submission time:

1. open `mt4110/drop-squash`
2. go to Settings > Collaborators
3. invite `testing@devpost.com`
4. invite `build-week-event@openai.com`
5. give the lowest permission that lets judges read and test the repository
6. confirm both invitations are sent before submitting the Devpost form

Do not make the repository public just to satisfy Build Week. Public is useful
later, but it adds license, secret-scanning, and claim-surface pressure at the
worst possible time.

Current repository access evidence:

```text
Repository: mt4110/drop-squash
Visibility: private
Invitation 326181116: build-week-event@openai.com email invite, read
Collaborator: devposttesting, read
Expired: false
```

## Deadline Target

External deadline:

```text
Tuesday, July 21, 2026 5:00 PM PT
Wednesday, July 22, 2026 9:00 AM JST
```

Internal target:

```text
Have the demoable alpha and README ready before sleeping on July 21 JST.
```

## Build Order

1. freeze the Build Week submission goal in docs and README
2. add a Japanese/English adversarial browser fixture with annotations
3. run packaged-app SCK/AX/Vision observation against the fixture
4. generate a `MaskPlan` preview from the observation snapshot
5. overwrite planned regions on actual captured pixels and save a final clip
6. independently decode the clip, verify masks/audio/metadata, and sign its
   redacted sidecar
7. add a short demo script
8. add README setup and judging instructions
9. verify tests and policy checks
10. record and upload the demo video
11. submit through Devpost

## Remaining Manual Submission Items

These cannot be fully completed from the repository alone:

- upload the demo video to public YouTube
- run `/feedback` in the main Codex build thread and copy the session ID
- paste the Devpost draft text into the submission form
- paste the repository URL
- confirm Devpost/OpenAI read-only invites are still pending or accepted
- submit before Wednesday, July 22, 2026 9:00 AM JST

## Demo Script

Thirty to sixty seconds is enough:

1. show the adversarial fixture with Japanese and English sensitive text
2. select its window in the alpha UI
3. start blackening verification, then stop and save the verification
4. show the all-black MP4 and signed redacted sidecar
5. show independent local verification, then state the boundary:
   this alpha does not store recognized text and does not claim completed
   privacy protection

Voiceover draft:

```text
DropSquash started as a local Mac screen-recording compressor. For Build Week,
I am showing the Secure Share R&D alpha: a local-first evidence tool for QA
teams that need to share recordings without leaking sensitive UI text.

This fixture contains synthetic Japanese and English private-looking data. The
packaged Mac app captures native ScreenCaptureKit frame metadata, reads
Accessibility structure and focused-input geometry tied to the selected window,
and runs local Apple Vision text and shape detection.
The log shows aggregate counts, a redacted MaskPlan, and a black-fill proof.
Strict Shield records the canonical full-frame region, but never recognized
private text or observation geometry.

The final clip is independently decoded before publication, and a corrupt
signing key fails closed without leaving an output. This is intentionally not
marketed as leak-zero or enterprise audit-ready.
Codex helped build the Rust/Tauri architecture, split files under strict size
rules, add the fixture, and turn the R&D goal into verifiable evidence.
```

## Acceptance Gate

The submission is acceptable when:

- `cargo run -p xtask -- build-week-phase5-check` passes, including the
  Phase 5 documentation honesty lint
- the repository can be checked by a judge
- a fresh user can follow README setup instructions
- the demo video shows the app or harness working
- the fixture and annotation exist
- local observation evidence exists for the fixture
- a local Build Week artifact exists at
  `target/release/bundle/dmg/DropSquash_0.1.0_aarch64.dmg`
  with SHA-256
  `6c0d82d6d11d351cec9fb9886f46529ecc7c3aba3d327a7870643c7c1dd721b0`
- Apple notarization submission `42774ba8-fe01-4439-99d1-6c8565e499aa`
  returned `Accepted`; `xcrun stapler staple` and `xcrun stapler validate`
  succeeded
- the mounted app assessed as `accepted` with
  `source=Notarized Developer ID`
- redacted evidence summaries exist:
  [docs/build-week-phase5-evidence.json](/Users/masakitakemura/_workspace/drop-squash/docs/build-week-phase5-evidence.json)
  as the current signed native baseline, plus historical
  [docs/build-week-evidence.json](/Users/masakitakemura/_workspace/drop-squash/docs/build-week-evidence.json),
  [docs/phase4-strict-shield-evidence.json](/Users/masakitakemura/_workspace/drop-squash/docs/phase4-strict-shield-evidence.json),
  and [docs/build-week-ui-fixture-evidence.json](/Users/masakitakemura/_workspace/drop-squash/docs/build-week-ui-fixture-evidence.json)
- judge runbook exists:
  [docs/build-week-judge-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/build-week-judge-runbook.md)
- no private key or personal data is included
- public claims match the implementation exactly

## Current Evidence

Fixture:

- [NativeAccessibilityFixture.swift](/Users/masakitakemura/_workspace/drop-squash/tests/fixtures/secure-share/NativeAccessibilityFixture.swift)
- [native fixture annotation](/Users/masakitakemura/_workspace/drop-squash/tests/fixtures/secure-share/native-accessibility-fixture.annotation.json)

Packaged macOS Strict Shield run:

```text
Date: Tuesday, July 21, 2026
Fixture window size: 760x652 native Japanese/English accessibility fixture
Frame count: 144
Accessibility observations: 70 aggregate-only
Vision observations: 80 aggregate-only
MaskPlan policy: strict_reveal / Strict Shield
Full-frame destructive regions: 144
Live ScreenCaptureKit frames blackened: 144
Live black pixel readbacks: 144
Final output verification: passed
Output sidecar: redacted, SHA-256-bound, Ed25519 signed
MP4 SHA-256: fb3da3c40c285bf70194eabae611adc0ddd148d0853a05457b88d829c6d834cf
Status: ok
```

Demo capture status:

```text
Recorded locally: signed packaged-app UI with native Japanese/English fixture states
The recording shows fixture content, Strict Shield blackening,
and independent verification. It does not present a selectively redacted result
as shareable. Evidence: docs/phase5-alpha.md
Public upload: pending
```

This proves the packaged GUI-hosted path can observe the native Japanese/English
fixture locally with ScreenCaptureKit frame metadata, Accessibility count, and
Vision observation count, then create a redacted Strict Reveal `MaskPlan`,
destructively blacken the captured buffers, independently decode the final MP4,
and sign the output-bound sidecar. The next manual work is to update and upload
the demo so it visibly shows the Strict Shield boundary. Do not upload the
current all-black export as evidence of usable automatic redaction.
