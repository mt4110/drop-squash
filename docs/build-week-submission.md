# Build Week Submission Plan

DropSquash should enter OpenAI Build Week as:

```text
Phase 3 alpha / high-assurance Secure Share R&D prototype
```

This is not a finished enterprise security product and must not be described
as leak-zero. The submission should show the serious technical direction:
local-first screen-recording evidence, native macOS capture signals, local
Vision observations, deterministic `MaskPlan` data, and an honest path toward
Strict Reveal fail-closed export.

## Submission Goal

Build a working alpha that judges can understand in under three minutes:

1. open a Japanese/English adversarial screen fixture
2. run DropSquash's packaged macOS observation harness
3. show ScreenCaptureKit frame metadata, Accessibility count, and local Vision
   text/shape observation count
4. emit a redacted `MaskPlan` JSON that contains geometry, source, reason,
   confidence, and timing, but no recognized private text
5. prove local Vision candidates drive solid-black destructive overwrite on
   the same ScreenCaptureKit frame, with a black pixel readback
6. explain that final-video destructive export and independent final-output
   verification are the next fail-closed gates before any commercial claim

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
- local Apple Vision text/shape observations
- redacted evidence, `MaskPlan` prototype, and live-frame black-fill proof
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
Invitation 326181039: devposttesting, read
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
5. overwrite local Vision regions on actual captured pixels and read back black
6. add a short demo script
7. add README setup and judging instructions
8. verify tests and policy checks
9. decide repository visibility
10. record and upload the demo video
11. submit through Devpost

## Remaining Manual Submission Items

These cannot be fully completed from the repository alone:

- record the 30-60 second demo video
- upload the demo video to public YouTube
- run `/feedback` in the main Codex build thread and copy the session ID
- paste the Devpost draft text into the submission form
- paste the repository URL
- confirm Devpost/OpenAI read-only invites are still pending or accepted
- submit before Wednesday, July 22, 2026 9:00 AM JST

## Demo Script

Thirty to sixty seconds is enough:

1. show a normal screen recording is compressed locally
2. show the adversarial fixture with Japanese and English sensitive text
3. run the observation harness
4. show the alpha UI, redacted counts, `MaskPlan` preview, and live-frame proof
5. state the boundary: this alpha does not store recognized text and does not
   claim completed privacy protection until final-video destructive export and
   independent verification pass

Voiceover draft:

```text
DropSquash started as a local Mac screen-recording compressor. For Build Week,
I am showing the Secure Share R&D alpha: a local-first evidence tool for QA
teams that need to share recordings without leaking sensitive UI text.

This fixture contains synthetic Japanese and English private-looking data. The
packaged Mac app captures native ScreenCaptureKit frame metadata, reads
Accessibility structure, and runs local Apple Vision text and shape detection.
The log shows counts, a redacted MaskPlan preview, and a black-fill proof:
geometry, reason, source, confidence, timing, and `firstSampleBlackened=true`,
but no recognized private text.

This is intentionally not marketed as leak-zero. The remaining gates are
final-video Strict Reveal destructive pixel overwrite, independent final-output
verification, and fail-closed deletion before any commercial security claim.
Codex helped build the Rust/Tauri architecture, split files under strict size
rules, add the fixture, and turn the R&D goal into verifiable evidence.
```

## Acceptance Gate

The submission is acceptable when:

- the repository can be checked by a judge
- a fresh user can follow README setup instructions
- the demo video shows the app or harness working
- the fixture and annotation exist
- local observation evidence exists for the fixture
- redacted evidence summary exists:
  [docs/build-week-evidence.json](/Users/masakitakemura/_workspace/drop-squash/docs/build-week-evidence.json)
- judge runbook exists:
  [docs/build-week-judge-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/build-week-judge-runbook.md)
- no private key or personal data is included
- public claims match the implementation exactly

## Current Evidence

Fixture:

- [tests/fixtures/secure-share/ja-en-browser-form.html](/Users/masakitakemura/_workspace/drop-squash/tests/fixtures/secure-share/ja-en-browser-form.html)
- [tests/fixtures/secure-share/ja-en-browser-form.annotation.json](/Users/masakitakemura/_workspace/drop-squash/tests/fixtures/secure-share/ja-en-browser-form.annotation.json)

Packaged macOS app observation run:

```text
Date: Sunday, July 19, 2026
Window ID: 3301
Frame size: 1440x900
Frame count: 2
Accessibility observations: 2
Vision observations: 236
MaskPlan policy: strict_reveal
MaskPlan frames: 2
First frame regions after coalescing: 42
MaskPlan audit unmatched observations: 0
MaskPlan verification-required frames: 0
Live ScreenCaptureKit frames blackened: 2
Live text regions blackened: 236
Live black pixel readbacks: 2
First black-fill sample blackened: true
Event log: /tmp/dsq-build-week-live-mask-1784462500.jsonl
Status: ok
```

This proves the packaged GUI-hosted path can observe a Japanese/English browser
fixture locally with ScreenCaptureKit frame metadata, Accessibility count, and
Vision observation count, then create a redacted Strict Reveal `MaskPlan`
preview, then destructively blacken local Vision regions on the actual
ScreenCaptureKit buffers and read back black pixels. It still does not prove
final-video destructive export or final-output residual verification.

Next required implementation:

```text
Connect the MaskPlan to final-video destructive pixel overwrite and independent
residual verification.
```
