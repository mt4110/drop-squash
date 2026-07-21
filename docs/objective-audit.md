# Objective Audit

Use this as the shortest current-state audit for the active productization
goal. The Build Week submission is complete; the active goal is now the
Phase 6-10 commercialization design in
[docs/phase6-10-commercialization.md](/Users/masakitakemura/_workspace/drop-squash/docs/phase6-10-commercialization.md).

For the active internal review gates before any public-commercial reopening,
also see
[docs/p1-review.md](/Users/masakitakemura/_workspace/drop-squash/docs/p1-review.md)
and
[docs/p2-alpha-review.md](/Users/masakitakemura/_workspace/drop-squash/docs/p2-alpha-review.md).

Date baseline:

```text
Sunday, July 19, 2026
```

Primary product hypothesis:

```text
DropSquash becomes sellable when it can locally produce high-assurance
screen-recording exports that destroy sensitive and unknown pixels, verify the
final output, and explain the result honestly.
```

## Build Week Record

The OpenAI Build Week submission produced a demoable alpha:

```text
Phase 3 alpha / high-assurance Secure Share R&D prototype
```

This does not redefine commercial completion. It created a judged working
artifact that honestly shows the R&D direction: fixture, local macOS
observations, redacted `MaskPlan` preview, README, demo video, and no leak-zero
claim. The plan is tracked in
[docs/build-week-submission.md](/Users/masakitakemura/_workspace/drop-squash/docs/build-week-submission.md).

## Scope Boundary

The active internal completion target is:

- preserve current macOS conversion, trial, license, and original safety
- demote weak manual/fixed-bar masking from product-facing claims
- write the high-assurance Secure Share threat model
- define the supported matrix and adversarial corpus
- prototype the macOS observation pipeline
- produce deterministic frame-exact `MaskPlan` data
- move toward Strict Reveal fail-closed export
- add independent final-output verification

Public-commercial work remains gated in this goal:

- public website reopening
- Lemon Squeezy production
- Stripe production
- general public distribution

Do not treat these as complete yet:

- production payment onboarding
- broad formats
- editing
- general built-in capture outside high-assurance Secure Share
- broad automatic privacy masking without fail-closed verification
- advertising SDKs
- YouTube downloading
- media-library expansion

## Commercial Design Record

The next planning horizon is Phase 6 through Phase 10. It separates a CtoC
local-conversion offer from a BtoB Secure Share pilot while keeping one shared
local media/evidence core. Windows is a formal BtoB target; Linux is out of
scope. The phase exits, architecture boundary, measurable scorecard, purchase
hypotheses, legal/distribution surface, and stop conditions are authoritative
in [docs/phase6-10-commercialization.md](/Users/masakitakemura/_workspace/drop-squash/docs/phase6-10-commercialization.md).

## Track Audit

| Track | Current state | Strongest evidence now | Remaining proof |
|---|---|---|---|
| P1 conversion and license groundwork | Keep intact | [docs/p1-review.md](/Users/masakitakemura/_workspace/drop-squash/docs/p1-review.md), existing desktop/CLI tests | Regression checks after privacy-engine work |
| Manual/fixed-bar Secure Share alpha | Demoted to groundwork | [docs/secure-share-beta.md](/Users/masakitakemura/_workspace/drop-squash/docs/secure-share-beta.md), existing destructive-mask tests and receipts | Do not sell or claim high assurance from this alone |
| High-assurance Secure Share R&D | Active | [docs/phase5-native-capture-bridge.md](/Users/masakitakemura/_workspace/drop-squash/docs/phase5-native-capture-bridge.md), [docs/secure-share-beta.md](/Users/masakitakemura/_workspace/drop-squash/docs/secure-share-beta.md), [docs/enterprise-audit-masking.md](/Users/masakitakemura/_workspace/drop-squash/docs/enterprise-audit-masking.md), [docs/p2-alpha-review.md](/Users/masakitakemura/_workspace/drop-squash/docs/p2-alpha-review.md), [docs/secure-share-threat-model.md](/Users/masakitakemura/_workspace/drop-squash/docs/secure-share-threat-model.md), [docs/secure-share-support-matrix.md](/Users/masakitakemura/_workspace/drop-squash/docs/secure-share-support-matrix.md), [docs/secure-share-adversarial-corpus.md](/Users/masakitakemura/_workspace/drop-squash/docs/secure-share-adversarial-corpus.md), [docs/secure-share-maskplan.md](/Users/masakitakemura/_workspace/drop-squash/docs/secure-share-maskplan.md), `crates/dropsquash-core/src/secure_share`, `crates/dropsquash-platform/src/secure_share.rs`, desktop/CLI receipt save paths | Native Apple-owned SCK/AX/Vision capture, Strict Reveal export, independent verification, real export path that creates and passes `MaskPlan`, output deletion when verification fails |
| Public-commercial track | Intentionally paused | [docs/productization.md](/Users/masakitakemura/_workspace/drop-squash/docs/productization.md), [docs/release-blockers.md](/Users/masakitakemura/_workspace/drop-squash/docs/release-blockers.md) | Reopen only after high-assurance Secure Share is commercially convincing |
| Market validation | Framework prepared, evidence thin | [docs/market-validation.md](/Users/masakitakemura/_workspace/drop-squash/docs/market-validation.md), [docs/market-validation-demo.md](/Users/masakitakemura/_workspace/drop-squash/docs/market-validation-demo.md), [docs/manual-beta-license-issuance.md](/Users/masakitakemura/_workspace/drop-squash/docs/manual-beta-license-issuance.md), [docs/market-decision-memo.md](/Users/masakitakemura/_workspace/drop-squash/docs/market-decision-memo.md) | Real cohort data for repeat usage, three-file completion, willingness to pay, and actual paid beta purchases |

## Phase 5 Current Evidence

The current Developer ID-signed macOS app has a local selected-window path that
uses ScreenCaptureKit frames, full-frame Strict Shield destruction, independent
decode verification, and a signed evidence sidecar. It also requires display,
CoreGraphics target, AX geometry, macOS lifecycle, and foreground-activation
watches to finish cleanly before the schema-2 MaskPlan is published with
`captureContinuityAttested: true` and the redacted required-watch list.

Strongest current checks:

- A deterministic 100ms native window move-and-return is rejected by the AX
  event watch with no MP4, MaskPlan, or partial artifact.
- A stationary signed-app run produced 15 full-frame-destroyed frames; the
  independent verifier accepted its schema-2 sidecar and output binding.
- Schema-2 Strict Shield evidence lacking the continuity attestation or its
  required-watch list is rejected even if it has a valid signature.
- Schema-2 Strict Shield evidence now also requires redacted exposure coverage
  for typed text, IME composition candidates, transient UI, notifications,
  browser autofill/password-manager surfaces, focus/foreground changes,
  display geometry/scale changes, and frame drops/discontinuities. The current
  accepted mitigation is only full-frame Strict Shield destruction.
- macOS lifecycle notifications for session resignation, screen sleep,
  workspace sleep, power-off, and active-Space change now block publication
  during the recording boundary; a deterministic test posts the workspace
  screen-sleep notification through the same observer path.
- Foreground application activation is now watched with a PID allowlist:
  DropSquash and the selected target process are allowed, while a third-party
  activation blocks publication.

This is not a claim of complete z-order focus proof, fully exercised lock/sleep handling,
notification compositing, all IME coverage, or complete selective masking. Those remain
explicitly classified in [docs/phase5-alpha.md](/Users/masakitakemura/_workspace/drop-squash/docs/phase5-alpha.md).

Native frame metadata note:

- `NativeFrameMetadataProvider` now exists as the fail-closed insertion point
  for ScreenCaptureKit work
- `objc2-screen-capture-kit` is selected as the macOS binding path and is
  referenced by the platform crate
- `objc2-core-media` is selected for the `CMSampleBuffer` attachment boundary
- `RawSckFrameInfo` validates and maps raw SCK-like values into
  `CaptureFrameMetadata`
- required SCK attachment presence is checked before raw value conversion
- SCK `CMSampleBuffer` attachment value extraction is implemented for status,
  display time, scale factor, content scale, content rect, and bounding rect
- `NativeSampleBufferProvider` now defines the macOS-only boundary for observed
  sample-buffer streams
- `SampleBufferFrameMetadataProvider` adapts observed sample buffers into the
  generic `FrameMetadataProvider` timeline
- empty sample-buffer streams fail closed and cannot be treated as a successful
  trusted capture
- `SckStreamFrameMetadataOutput` now compiles as an `SCStreamOutput` receiver
  and extracts metadata from screen sample-buffer callbacks without retaining
  raw frames
- non-screen callback types are ignored
- `SckFrameMetadataStreamRegistration` creates a real `SCStream` from
  caller-supplied filter/configuration values, registers the screen output
  receiver, and keeps a serial callback queue alive
- redacted `SCShareableContent` candidate extraction now exists for
  caller-supplied content objects
- shareable candidate snapshots intentionally omit window title text and
  application names
- Strict Reveal window target selection now fails closed unless exactly one
  eligible active, on-screen, app-owned, normal-layer window is present
- explicit display target selection now fails closed unless the requested
  display ID exists in the redacted snapshot
- asynchronous `SCShareableContent` discovery now converts completion
  callbacks into redacted snapshots inside `dropsquash-platform`
- empty, failed, disconnected, and timed-out shareable-content discovery fails
  closed before target selection
- `SCStream` start/stop lifecycle helpers now wait for native completion and
  fail closed on error, disconnection, or timeout
- `SckFrameMetadataStreamRegistration` can now run a registered stream for a
  caller-supplied duration and read frame metadata only after stopping it
- redacted Strict Reveal targets can now be resolved against the source
  `SCShareableContent` object to create a real `SCContentFilter`
- `SckStreamCapturePlan` now carries a target-derived
  `SCStreamConfiguration`, frame size, and filter into stream registration
- it still does not prove real SCK metadata capture
- closing this gap requires target selection UI, permission handling, and a
  manual or automated capture observation proving frame size, presentation
  time, and frame status

## Current Command Snapshot

Paid beta technical proof:

```text
cargo run -p xtask -- paid-beta-check
-> 7 required, 1 verified, 6 remaining
```

```text
cargo run -p xtask -- productization-status --track "Paid beta"
-> 13 total, 7 verified, 6 blocked
```

Public web proof:

```text
cargo run -p xtask -- productization-status --track "Public web proof"
-> docs still need refresh after canonical-host activation; expect Public website deployment to move to verified
```

```text
cargo run -p xtask -- public-web-ready
-> local website content and deployable site packaging passed
```

```text
cargo run -p xtask -- public-web-probe
-> owner-only HTTP 200; canonical HTTP 200; pricing HTTP 200; refund HTTP 200
```

Sites hosting snapshot:

```text
project status: active
current live URL: https://dropsquash.app
access mode: public
available access modes: custom, public
latest saved version: 3
latest saved version commit: 1017927b9c1ab26e4fd251ad81ffbd85fd0c88b5
custom domain refresh: dropsquash.app -> status active, provider_status active, ssl_status active
custom domain updated_at: 2026-07-18T06:23:16.868939+00:00
```

Market validation:

```text
deliverables are documented; real cohort evidence is not yet deep enough to justify expansion
```

Current repo state note:

- the everyday worktree is currently dirty
- `cargo run -p xtask -- public-web-rerun` currently reports source head
  `5a69fea9240adfddff77425c83faf72ea3b61751` and `303 more` dirty paths after
  the initial preview
- use snapshot or detached-worktree handoff before packaged-app, sandbox,
  signing, or public-web evidence collection
- shortest snapshot helpers:
  - `scripts/manual-qa-license-sandbox-handoff.sh /tmp/dropsquash-qa-snapshot-$(git rev-parse --short HEAD)`
  - `scripts/manual-qa-distribution-handoff.sh /tmp/dropsquash-qa-snapshot-$(git rev-parse --short HEAD)`
  - `scripts/public-web-handoff.sh /tmp/dropsquash-public-web-$(git rev-parse --short HEAD)`

Checker note from Saturday, July 18, 2026:

- `cargo run -p xtask -- release-check` passes
- `cargo run -p xtask -- file-size-check` still fails on pre-existing `xtask`
  file-length overruns outside the current P1 desktop app surface:
  - `xtask/src/productization_status/render/lines.rs` (`151`)
  - `xtask/src/productization_status/preflight/commands.rs` (`136`)
  - `xtask/src/manual_qa_pending/distribution/reminders.rs` (`131`)
  - `xtask/src/manual_qa_ready_distribution.rs` (`134`)
  - `xtask/src/manual_qa_window_capture.rs` (`139`)
  - `xtask/src/manual_qa_pending.rs` (`134`)
- `cargo run -p xtask -- manual-qa-check docs/manual-qa.md --section license`
  passes
- `cargo run -p xtask -- manual-qa-check docs/manual-qa.md --section distribution`
  passes
- but `cargo run -p xtask -- paid-beta-check` still reports 6 remaining
  blockers

Interpret that carefully:

- release/doc structure checks are healthy
- repo file-size hygiene still needs follow-up, but the current red items are
  not evidence against the P1 packaged-app quality verdict
- manual-QA section structure is healthy
- external paid-beta proof is still incomplete
- therefore these passing checks do not override the `paid-beta-check` blocker
  state

## What Is Actually Blocking Now

These are the real blockers as of Sunday, July 19, 2026:

1. Manual/fixed-bar masking is not commercially strong enough.
2. High-assurance Secure Share needs a real mask-candidate engine.
3. Built-in capture is needed for trusted frame metadata, but only inside the
   secure-sharing boundary.
4. Public-commercial blockers remain paused on purpose and should not drive the
   current implementation order.

## Blocker Types

Use this table to avoid mixing product-code work with external proof work.

| Blocker | Type | Why it is still open | What would close it |
|---|---|---|---|
| Lemon Squeezy `product setup` | External service state | The sandbox product/dashboard observation is not yet recorded | Real Lemon Squeezy dashboard observation recorded in `docs/manual-qa.md` |
| Lemon Squeezy `sandbox purchase` | External service state | No real sandbox buyer order has been recorded yet | One concrete sandbox checkout with non-secret order evidence |
| Lemon Squeezy `valid sandbox activation` | External service state + app observation | The app-side `Pro` activation plus cache delta has not yet been recorded | Real sandbox activation, `Pro`, fingerprint, `instance_id`, raw-key absence |
| `Signed DMG` | Release credentials | Signed artifact evidence is now recorded; keep it current for the final public artifact | `macos-signing-check` passes and signed artifact evidence is recorded |
| `Notarized and stapled DMG` | Release credentials | Notary, stapler, and mounted-app `spctl` evidence are now recorded; keep them tied to the final public artifact | Notary acceptance, stapler, and `spctl` evidence for the public artifact |
| `Gatekeeper clean-machine open` | Release credentials + clean-machine observation | No signed/notarized public artifact has been opened on a clean account yet | Clean-machine or fresh-account open evidence recorded in `docs/manual-qa.md` |
| `Public website deployment` | DNS / SSL / deployment | Canonical host was proven deployable, but it is intentionally back to owner-only access so the business surface is not public yet | Make `https://dropsquash.app/release-status` public again only when release truth is ready |
| `Pricing finalized` | Production copy alignment | Pricing copy is internally aligned, but the canonical host is intentionally private and returns `401` to non-owners | Publish `https://dropsquash.app/pricing` only when public release work is reopened |
| `Refund policy finalized` | Production copy alignment | Refund copy is internally aligned, but the canonical host is intentionally private and returns `401` to non-owners | Publish `https://dropsquash.app/refund` only when public release work is reopened |
| `Live checkout link` | Production commerce state | No tested live checkout URL is published yet | Production pricing page opens the real `store.lemonsqueezy.com/checkout/buy/<id>` URL |
| `Published checksum` | Release publication state | No public GitHub Release with attached `SHA256SUMS` is recorded yet | Public GitHub Release URL plus checksum proof |
| `Homebrew cask install` | Distribution publication state | No public tap PR with install/uninstall proof is recorded yet | Public tap PR URL plus matching artifact/SHA/install evidence |
| Market-validation gates | Cohort evidence | The framework exists, but real user data is still thin | Real cohort rows and rollups for repeat usage, 3-file completion, payment intent, paid conversion |

## Reentry Commands

Use these as the shortest restart points for each blocker family.

| Blocker family | Start here | Then use |
|---|---|---|
| High-assurance Secure Share R&D | Start with threat model and prototype plan | [docs/secure-share-beta.md](/Users/masakitakemura/_workspace/drop-squash/docs/secure-share-beta.md), [docs/p2-alpha-review.md](/Users/masakitakemura/_workspace/drop-squash/docs/p2-alpha-review.md) |
| Lemon Squeezy sandbox proof | `cargo run -p xtask -- manual-qa-license-rerun` | [docs/license-sandbox-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/license-sandbox-runbook.md) |
| Signed/notarized distribution proof | `cargo run -p xtask -- manual-qa-distribution-rerun` | [docs/signed-dmg-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/signed-dmg-runbook.md) |
| Paid beta technical snapshot | `cargo run -p xtask -- paid-beta-check` | [docs/paid-beta-readiness.md](/Users/masakitakemura/_workspace/drop-squash/docs/paid-beta-readiness.md) |
| Public web proof | `cargo run -p xtask -- public-web-ready` | [docs/public-beta-operator-checklist.md](/Users/masakitakemura/_workspace/drop-squash/docs/public-beta-operator-checklist.md) |
| Canonical host / DNS state | `cargo run -p xtask -- public-web-probe` | [docs/website-deployment-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/website-deployment-runbook.md) |
| Market-validation operations | Start with manual issuance and tracker updates | [docs/manual-beta-license-issuance.md](/Users/masakitakemura/_workspace/drop-squash/docs/manual-beta-license-issuance.md), [docs/market-validation.md](/Users/masakitakemura/_workspace/drop-squash/docs/market-validation.md) |
| Full objective snapshot | Read this file first | [docs/productization.md](/Users/masakitakemura/_workspace/drop-squash/docs/productization.md) |

## Current Execution Order

Keep this order. It prevents public-commercial pressure from getting ahead of
product truth and keeps the masking work security-shaped.

1. Preserve current conversion/license/trial safety.
2. Remove or demote weak manual/fixed-bar masking surfaces.
3. Write the high-assurance threat model.
4. Write supported matrix and adversarial corpus.
5. Prototype ScreenCaptureKit metadata capture.
6. Prototype Accessibility structure capture.
7. Prototype local Vision observation.
8. Design deterministic frame-exact `MaskPlan`.
9. Implement Strict Reveal fail-closed export.
10. Implement independent verification.
11. Only then decide whether to resume public-commercial work.

## Completion Gate

Do not mark the current goal complete until all of these are true at the same
time:

1. The high-assurance threat model exists.
2. Supported matrix and adversarial corpus exist.
3. SCK/AX/Vision observation prototypes exist.
4. Deterministic `MaskPlan` exists.
5. Strict Reveal fail-closed export exists.
6. Independent final-output verification exists.
7. The packaged-app review records a clear continue/hold/stop decision.

If any one of those is still missing, the objective is still in progress.

## Evidence Precedence

When two signals seem to disagree, trust them in this order:

1. The blocker-specific gate for the exact scope:
   - `cargo run -p xtask -- paid-beta-check`
   - `cargo run -p xtask -- public-web-probe`
   - Sites custom-domain refresh/status
2. The source-of-truth records:
   - [docs/release-blockers.md](/Users/masakitakemura/_workspace/drop-squash/docs/release-blockers.md)
   - [docs/manual-qa.md](/Users/masakitakemura/_workspace/drop-squash/docs/manual-qa.md)
3. Section or structure checks:
   - `cargo run -p xtask -- manual-qa-check ...`
   - `cargo run -p xtask -- release-check`
4. Planning and routing docs:
   - runbooks
   - operator checklists
   - this audit document

Practical rule:

- if `manual-qa-check` passes but `paid-beta-check` still shows blockers,
  the blockers still win
- if the canonical host is live but the page copy still shows stale draft-era
  content, the production copy blocker still wins
- if an evidence URL is owner-only or access-controlled, it does not satisfy
  the public website, pricing, or refund blockers
- if a doc says something is ready but the current command or hosting state says
  otherwise, refresh the doc from the command or connector result

## Natural Next Actions

1. Write the high-assurance Secure Share threat model.
2. Define the supported matrix and adversarial corpus for Japanese and English
   screen recordings.
3. Prototype ScreenCaptureKit frame metadata capture, then Accessibility
   structure capture, then local Vision text/shape observation.
4. Design deterministic `MaskPlan` before expanding UI.
5. Keep public-commerce work paused until Strict Reveal and independent
   verification are reviewable.

## Owner Checklist

Use this when the repo is ready but external state still is not.

Short version:

- [docs/external-unblock-checklist.md](/Users/masakitakemura/_workspace/drop-squash/docs/external-unblock-checklist.md)

### 1. Public website proof

- deploy the saved Sites version `3`
- verify:
  - `https://dropsquash.app/release-status`
  - `https://dropsquash.app/pricing`
  - `https://dropsquash.app/refund`
- confirm pricing and refund no longer show stale draft-era copy
- keep checkout blocked until a real live checkout URL exists

### 2. Lemon Squeezy sandbox proof

- sign in to the Lemon Squeezy dashboard
- confirm sandbox mode is visible
- confirm the intended product is `DropSquash`
- confirm sandbox license keys are enabled
- complete one sandbox checkout and record a non-secret order id or order number
- activate one real sandbox key in the app and record:
  - disabled submit while `Activating`
  - `Pro` state
  - fingerprint present
  - `instance_id` present
  - raw key absent

### 3. Apple signing / notarization proof

- load one signing credential group:
  - `APPLE_SIGNING_IDENTITY`
  - or `APPLE_CERTIFICATE` with `APPLE_CERTIFICATE_PASSWORD`
- load one notarization credential group:
  - `APPLE_API_KEY`, `APPLE_API_ISSUER`, `APPLE_API_KEY_PATH`
  - or `APPLE_ID`, `APPLE_PASSWORD`, `APPLE_TEAM_ID`
- rerun `cargo run -p xtask -- macos-signing-check`
- only after it passes, continue to codesign, notarize, staple, and Gatekeeper observation

### 4. Market-validation proof

- issue real private beta seats
- collect one real 3-file session result
- collect repeat-usage and willingness-to-pay follow-up
- update the tracker rollup before changing the market decision memo
