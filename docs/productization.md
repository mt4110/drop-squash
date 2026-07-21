# Productization Plan

> Current commercial design: after the Build Week submission, the authoritative
> Phase 6-10 plan is
> [docs/phase6-10-commercialization.md](/Users/masakitakemura/_workspace/drop-squash/docs/phase6-10-commercialization.md).
> Historical Build Week sections below record how the R&D alpha was prepared;
> they do not authorize a public release, payment onboarding, or broad claim.

DropSquash is currently a macOS MVP: a small local Drop Zone that converts user-selected screen recordings into numbered `.squashed.mp4` outputs through Apple's native AVFoundation export pipeline.

The v1 promise stays intentionally narrow:

```text
Drag a screen recording.
Get a smaller MP4.
Original stays safe.
Nothing uploads.
```

## Updated Direction

The working goal has moved from a narrow paid-beta checkout path to a stricter
product-quality path.

DropSquash should not establish the paid sales route until the secure-sharing
engine is strong enough to justify a high-trust privacy and evidence claim.
The current fixed-bar detector and the earlier manual-rectangle Secure Share
alpha are not sufficient for that claim.

For the current internal phase, DropSquash must advance in this order:

1. preserve the existing macOS conversion safety and license/trial groundwork
2. remove or demote weak manual masking UI from the product-facing experience
3. write the high-assurance Secure Share threat model and support matrix
4. build the macOS observation foundation through the native Capture Bridge:
   ScreenCaptureKit frame metadata, Accessibility structure, and local Vision
   text/shape observations remain inside Apple-owned native code
5. emit deterministic frame-exact `MaskPlan` data with reason, confidence, and
   source
6. prove Strict Reveal fail-closed destructive masking and independent output
   verification
7. only then decide whether public web, distribution, and payment work may
   reopen

This means public release and payment setup are not the current track. They
stay paused until the app itself is commercially convincing.

## Build Week Submission Override

Until the OpenAI Build Week deadline, the active execution target is narrower
and more time-sensitive:

```text
Ship a Phase 5 alpha / high-assurance Secure Share R&D prototype for Build
Week: preserve existing macOS conversion safety, show a Japanese/English
adversarial fixture, prove local ScreenCaptureKit/Accessibility/Vision
observation in the packaged app, and demonstrate a Strict Shield MaskPlan that
destructively masks final frames and passes independent verification. Do not
claim leak-zero or reopen payments.
```

See
[docs/build-week-submission.md](/Users/masakitakemura/_workspace/drop-squash/docs/build-week-submission.md).

## Productization Execution Goal

DropSquash productization must not lead with payment setup.

The active goal is:

```text
Build the Phase 2 R&D foundation for high-assurance Secure Share: define the
threat model, support matrix, adversarial corpus, and macOS observation
pipeline that combines ScreenCaptureKit frame metadata, Accessibility
structure, and local Vision text/shape observations into a deterministic
frame-exact MaskPlan. Do not reopen public checkout, Lemon Squeezy production,
Stripe production, or broad public claims until Strict Reveal, fail-closed
export, and independent verification pass.
```

The older public paid-beta sequence still matters later, but it is paused.

### Scope Of Completion

For this phase, only the following count as completion targets:

- existing single-file conversion safety stays intact
- original safety stays intact
- trial and license behavior do not regress
- weak manual masking is not presented as a security product
- high-assurance Secure Share threat model exists
- supported app/window/language/display matrix exists
- adversarial Japanese and English corpus plan exists
- macOS observation pipeline prototype exists
- deterministic `MaskPlan` schema exists
- Strict Reveal fail-closed acceptance criteria exist
- independent decode verification plan exists

### Current Execution Order

Follow this order and do not skip ahead:

1. stabilize the current app and remove misleading masking surfaces
2. write the high-assurance threat model
3. write the support matrix and adversarial corpus
4. prototype ScreenCaptureKit frame metadata capture
5. prototype Accessibility window/text-frame collection
6. prototype local Vision Japanese/English text-shape observation
7. design and test deterministic `MaskPlan`
8. implement Strict Reveal destructive export
9. implement independent final-output verification
10. review whether the product is strong enough for sales

Keep the older public-commercial sequence paused until the user explicitly
reopens it after this review.

P1 means:

- finish the single-file conversion path safely
- ensure failed conversions do not count toward trial usage
- ensure cancelled conversions do not count toward trial usage
- ensure larger-result or not-smaller outputs do not count toward trial usage
- ensure originals are never harmed by a failed safety condition
- keep success-only history, verification, and receipt behavior aligned

P1 also includes:

- align the visible trial state with the formal product rule
- keep the trial limit fixed and documented
- finish activation, validation, forget, and unlock flows
- never persist the raw license key
- make the post-purchase unlock path understandable without guesswork

The later public-commercial track still requires the public surface to match
implementation across:

- Top
- Pricing
- Privacy Policy
- Terms of Service
- Support
- Refund Policy
- License policy
- Release status
- Download

That public surface must clearly state:

- supported OS
- supported formats
- trial behavior
- pricing
- support contact
- refund path
- privacy claim
- what DropSquash can do now
- what DropSquash still cannot do

That later public-commercial track also requires:

- the production URL exists
- the signed and notarized DMG matches the public distribution story
- the download page and actual distribution state match
- the public web proof and distribution proof are filled with current evidence

Use
[docs/website-deployment-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/website-deployment-runbook.md)
to move from local `website/` proof to real `https://dropsquash.app/...`
evidence, and use
[docs/public-beta-operator-checklist.md](/Users/masakitakemura/_workspace/drop-squash/docs/public-beta-operator-checklist.md)
to sequence the later public-web and distribution blocker updates.

Phase 2 now means the next product step must be strong enough to justify review
as a security-relevant product. The implementation breakdown lives in
[docs/secure-share-beta.md](/Users/masakitakemura/_workspace/drop-squash/docs/secure-share-beta.md),
and the review gate lives in
[docs/p2-alpha-review.md](/Users/masakitakemura/_workspace/drop-squash/docs/p2-alpha-review.md).

- keep macOS as the only release target for this step
- allow built-in capture only for high-assurance Secure Share, because existing
  videos cannot carry trusted capture-time metadata
- add ScreenCaptureKit frame metadata
- add Accessibility structure collection
- add local Vision text and text-shape observation for Japanese and English
- add deterministic frame-exact `MaskPlan`
- add Strict Reveal fail-closed destructive masking before encode
- add independent final-output verification
- add a local receipt sidecar that explains sources, reasons, confidence, and
  verification result
- keep the feature scoped to secure sharing, not general editing or recording
  convenience
- do not expand into broad formats, editing, or library workflows
- preserve local-first, no-ffmpeg, and original-safety rules

The later public-commercial track means production payment setup is allowed
only after:

- public site completion
- public-information alignment
- production URL readiness
- distribution readiness

Only after those are true may Stripe production onboarding, Lemon Squeezy
production setup, and the real pricing/refund/checkout/license wiring proceed.

## Operating Rules

| Rule | Decision |
|---|---|
| Media path | No `ffmpeg`, no `ffprobe`, no shell, no `PATH` lookup |
| Privacy | No media upload, no default telemetry |
| Original handling | Keep original unless verified success and user policy allow Trash |
| Trial | Count successful, smaller verified conversions only; paid beta uses 20 |
| UI | Keep the main window small, calm, and task-focused |
| Platform order | macOS first; Windows and Linux after macOS product hardening |
| Paid beta scope | Paused until high-assurance Secure Share is commercially convincing |
| Scope gate | Before broad format support, editing, advertising SDKs, YouTube downloading, or media-library features, complete `docs/market-validation.md` and write a market decision memo |
| Recording boundary | Do not build general recording convenience yet; built-in capture is allowed only for high-assurance Secure Share because it needs trusted frame metadata |
| Privacy masking | Manual rectangles and fixed-bar detection are not enough to sell; the active track is native Capture Bridge-owned ScreenCaptureKit + Accessibility + local Vision + Strict Reveal + independent verification |
| Payment order | Do not start Stripe or Lemon Squeezy production onboarding until high-assurance Secure Share passes review and the public information surface, production URL, and distribution surface are ready |
| Public pages | Keep Top, Pricing, Privacy Policy, Terms of Service, Support, Refund Policy, License policy, Release status, and Download aligned with implementation |
| Localization | Initial market validation and paid beta copy must support `JA / EN`; prioritize Japanese as the primary surface and English as the secondary support surface |
| Claims | Say AVFoundation/native macOS pipeline now; do not overclaim explicit VideoToolbox hardware encoding |
| File size | Rust production files <= 128 lines; Rust test files may exceed 1000 lines; TS/TSX files <= 512 lines |

## Current Objective

The current shipping-facing objective is paused. The active internal working
objective takes precedence:

```text
Build the Phase 2 R&D foundation for high-assurance Secure Share. The first
checkpoint is not checkout; it is a documented and prototyped mask-candidate
engine that can combine ScreenCaptureKit, Accessibility, and local Vision into
a deterministic MaskPlan, then move toward Strict Reveal fail-closed export and
independent verification.
```

Use [docs/p1-review.md](/Users/masakitakemura/_workspace/drop-squash/docs/p1-review.md)
as the first review gate and
[docs/p2-alpha-review.md](/Users/masakitakemura/_workspace/drop-squash/docs/p2-alpha-review.md)
as the second review gate for that internal objective.

The older public-paid-beta objective is still relevant as a later release gate:
complete only the P0 safety, signed release, trial, license, and single-file
conversion path required for a testable macOS paid beta.

For the shortest current-state audit across paid beta, public proof, and
market validation, use
[docs/objective-audit.md](/Users/masakitakemura/_workspace/drop-squash/docs/objective-audit.md).

This objective exists inside a stricter commercialization rule:

- public information before production payment onboarding
- public claims only when implementation and evidence support them
- one coherent structure across app, website, docs, release, support, refund,
  pricing, and license behavior

Do not resume Stripe or Lemon Squeezy production onboarding until all of these
are true:

1. High-assurance Secure Share is commercially convincing to the product owner.
2. Strict Reveal fail-closed export works in the packaged macOS app.
3. Independent final-output verification passes on adversarial cases.
4. Trial and license flows are coherent.
5. The public information surface matches the implementation.
6. The production website URL exists.
7. The signed/notarized distribution surface is ready.

For the current objective, follow this order:

1. high-assurance threat model
2. supported matrix and adversarial corpus
3. ScreenCaptureKit metadata prototype
4. Accessibility structure prototype
5. local Vision observation prototype
6. deterministic `MaskPlan`
7. Strict Reveal export
8. independent verification
9. packaged-app review

Source documents for the first R&D checkpoint:

- [docs/secure-share-threat-model.md](/Users/masakitakemura/_workspace/drop-squash/docs/secure-share-threat-model.md)
- [docs/secure-share-support-matrix.md](/Users/masakitakemura/_workspace/drop-squash/docs/secure-share-support-matrix.md)
- [docs/secure-share-adversarial-corpus.md](/Users/masakitakemura/_workspace/drop-squash/docs/secure-share-adversarial-corpus.md)
- [docs/secure-share-maskplan.md](/Users/masakitakemura/_workspace/drop-squash/docs/secure-share-maskplan.md)

Treat these as paused, not active:

10. public website and public documents
11. production URL and distribution readiness
12. Stripe and Lemon Squeezy production setup

## Future Track: Built-In Screen Recording

DropSquash may grow into a built-in screen-recording app for secure capture,
but this is not a general recorder feature.

Treat built-in capture as part of high-assurance Secure Share only when it
serves privacy proof:

1. Capture locally with ScreenCaptureKit.
2. Keep frame metadata tied to each frame.
3. Collect Accessibility and Vision observations at capture time.
4. Build a frame-exact `MaskPlan`.
5. Destroy sensitive and unknown pixels before encode.
6. Verify the final output independently.

Do not implement a general capture app, timeline editor, or convenience
recorder during this phase.

For the preserved future direction and entry criteria, see
`docs/future-capture-masking.md`.
For the stricter enterprise masking and signed-receipt direction, see
`docs/enterprise-audit-masking.md`.

The public information surface for this phase includes:

- Top
- Pricing
- Privacy Policy
- Terms of Service
- Support
- Refund Policy
- License policy
- Release status
- Download

Initial localization rule for this phase:

- support Japanese and English only
- treat Japanese as the primary market-facing language for the first paid beta
- keep English available as the secondary support language across app and site
- avoid adding more locales before the Japanese launch path is verified

License policy is not a substitute for Terms of Service.
Trial limits, pricing, purchase flow, support flow, refund flow, and release
status must also stay consistent across the app, website, docs, and release
artifacts.

## Plan Table

| Phase | Status | Goal | Scope | Acceptance | Verification |
|---|---|---|---|---|---|
| 0.5 Repo alignment | Done | Make names, docs, and metadata consistent before adding product surface | README/docs current-state wording, product invariants, repository URL decision | Docs match implementation; no unsupported hardware claims; no secrets added | `cargo fmt --all -- --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace` |
| 0.6 File-size architecture cleanup | Done | Bring existing code under the repository's size rules before adding more behavior | Split large Rust production files by responsibility; keep TS/TSX under 512 lines | No Rust production file exceeds 128 lines; no TS/TSX file exceeds 512 lines; behavior unchanged | `cargo run -p xtask -- file-size-check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings` |
| 1 macOS encoder hardening | In progress | Make single-file macOS conversion safe enough for paid beta | Replace hard-link finalization, strengthen output verification, clean temp files, friendly failure states | Existing outputs are never overwritten; failed/cancelled conversions do not count; not-smaller results keep the original and do not count; original remains untouched | Unit tests, desktop/CLI success-only history guards, CLI conversion summary output, `docs/qa-evidence.md`, reproducible `docs/manual-qa.md` packaged-app conversion evidence, and release-set benchmark blocker evidence including the sample-selection rule; `manual-qa-check` before completion |
| 2 Cancellation | In progress | Let users stop an active conversion cleanly | Cancellation token through command/encoder boundary, UI cancel action, temp cleanup, post-encode postprocess/history guard | Cancel returns app to ready state; no success history; no trial count | Unit tests, `docs/qa-evidence.md`, plus reproducible `docs/manual-qa.md` cancellation evidence; `manual-qa-check` before completion |
| 3 Sequential queue | In progress | Handle multiple dropped files deterministically | Queue states, per-job progress, one active conversion at a time, queued job cancellation, batch summary | Multiple drops create rows; one job runs at a time; queued cancellation prevents that job from starting; failures do not block unrelated jobs; summary shows finished count and saved bytes | Queue unit tests, `docs/qa-evidence.md`, plus reproducible `docs/manual-qa.md` multi-file, queued-cancellation, and batch-summary evidence; `manual-qa-check` before completion |
| 4 Source postprocess | In progress | Safely move originals to Trash only after verified success | Source policy setting, macOS Trash adapter, ask-after-success flow | Keep never moves; Ask prompts; Trash moves only after all safety gates | Setting, Ask UI, command revalidation tests, Trash adapter, and reproducible `docs/manual-qa.md` Trash evidence; `manual-qa-check` before completion |
| 5 License and trial UI | In progress | Convert trial usage into Pro unlock without account creation | License cache, instance id, activation/validation provider, local forget action, locked/Pro UI, local CLI status/forget | Raw license key is not persisted; empty/invalid/network/deactivation errors are friendly; valid cache survives grace period | Provider, trial UI, activation shell, local raw-key-free forget action, empty-key, network-failure, expired offline-grace reconnect state, failed-activation cache safety, local CLI status/forget, and safe cache/grace are implemented; Lemon Squeezy sandbox test remains |
| 6 Release pipeline | In progress | Ship a trusted macOS beta | Signed app, notarized DMG, checksums, release checklist, Homebrew cask draft, DMG install cleanup design | Gatekeeper opens without warning; secrets stay in CI; artifact checksum is published for the same DMG | Readiness, media/privacy security, read-only workflow permissions, unsigned DMG QA artifact upload, unsigned release failure gate, clean worktree preflights for manual QA and release notes preparation, UDIF artifact, checksum, benchmark blocker, required release notes URL field publish gate, checkout blocker URL rule, same-DMG manual QA signing evidence, stapled manual QA evidence, Gatekeeper no-warning evidence, numeric queue counts, verified smaller Trash output, Lemon Squeezy sandbox activation identity evidence, release notes prepared field-label synchronization, prepared draft placeholder rejection, prepared release notes/manual QA Markdown draft rejection, release blocker classification/execution-order synchronization, release blocker Next action detail gates, signing/notarization Next action signing preflight, CI codesign execution runner, CI notarization execution runner, CI stapler execution runner, CI Gatekeeper assessment runner, CI signed DMG artifact check, CI signed checksum generation, private CI signed DMG artifact upload, private CI signed checksum upload, GitHub release command plan, macOS signing command plan, macOS keychain argv plan, macOS keychain cleanup argv plan, macOS codesign argv plan, macOS codesign verification argv plan, macOS notarytool argv plan, macOS stapler argv plan, macOS spctl argv plan, signing runner acceptance criteria, signed DMG target preparation, signed DMG copy isolation, signed DMG artifact guard, conversion/queue/Trash/license action-state release notes evidence, final publish gate exact distribution evidence references, publish Artifact URL completion evidence gate, Homebrew install evidence tied to the Homebrew tap PR URL, Homebrew cask release match check, manual QA cask check evidence, complete public web and distribution Execution Order exit evidence, CI signing preflight, DMG bundle target, cask generation gates, macOS verification command/evidence drafts, DMG install cleanup design, and release-doc/workflow/template coverage tests exist; `docs/release-blockers.md` tracks external evidence; signing/notarization remain |
| 7 Market validation | In progress | Prove that the paid beta solves a real screen-recording problem before broadening scope | Three positioning variants, 30-second demo, feedback form, interview script, manual beta license issuance, measurable gates, decision memo | A market decision memo exists before any expansion into broad formats, editing, advertising SDKs, YouTube downloading, or media-library features | `docs/market-validation.md`; `docs/market-validation-demo.md`; `docs/market-validation-feedback-form.md`; `docs/market-validation-interview-script.md`; `docs/market-validation-tracker.md`; `docs/manual-beta-license-issuance.md`; `docs/market-decision-memo.md`; `docs/market-decision-memo-template.md` |
| 7.5 Evidence core plan | Planned | Fix the macOS-native evidence architecture before broad workflow expansion | Deterministic naming, safe segment extraction, evidence sidecars, richer verification, throughput model, native-stack decision | A formal evidence architecture exists and keeps local-first, no-ffmpeg, and source-safety rules intact before implementation spreads across CLI and desktop flows | `docs/evidence-core-architecture.md` |
| 8 Phase 2 high-assurance Secure Share R&D | Active | Build the mask-candidate engine that can justify a future sales route | ScreenCaptureKit frame metadata, Accessibility structure, local Vision Japanese/English text-shape observations, canonical coordinates, deterministic `MaskPlan` | Threat model, support matrix, adversarial corpus, and macOS observation prototypes exist | `docs/secure-share-beta.md`; `docs/p2-alpha-review.md` |
| 8.5 Strict Reveal and evidence verification | Planned | Turn the R&D foundation into a fail-closed packaged-app proof | Strict Reveal unknown-region destruction, final-size CVPixelBuffer overwrite, independent decode verification, receipt with sources/reasons/confidence/result | Verification failure blocks or deletes output; packaged-app QA proves pass and fail paths | `docs/enterprise-audit-masking.md` |
| 8 Expansion after validation | Later | Expand only after the paid beta and market memo justify it | Additional formats, platform expansion, workflow expansion | New scope is justified by repeat usage, three-file completion, willingness to pay, and actual paid beta purchases | Market decision memo plus the follow-on plan for the validated scope |

Legacy public-launch website work, including refund/support contact copy gates,
still remains tracked for the later public launch path even though the current
implementation focus is the narrower paid beta.
That later public-launch path also retains the pricing-finalization blocker, final pricing, pre-release CTA copy guard, download/checkout link and form action guard, local fragment link guard, and Public web proof exit coverage for release-status, privacy, pricing, terms, license, support, download, checkout, and refund.

## Immediate Backlog

| Order | Item | Status | Notes |
|---:|---|---|---|
| 1 | Persist output/profile/size settings | Done | Stored in platform app config path, including the privacy receipt preference |
| 2 | Update README/docs from Phase 0 wording | Done | Current macOS path is described as AVFoundation MVP |
| 3 | Decide canonical repository slug | Done | Metadata follows the current GitHub remote, `mt4110/drop-squash` |
| 4 | Replace macOS hard-link finalization | Done | Uses no-clobber atomic rename on macOS |
| 5 | Split oversized production files | Done | All Rust production files are now <= 128 lines; TS/TSX remain <= 512 lines |
| 6 | Add output media validation beyond size | Done | Requires smaller `.mp4`, MP4 file-type box, non-zero `mvhd` duration, source/output duration closeness when readable, and success-only history writes |
| 7 | Add cancellation | In progress | Command/UI/encoder path and post-encode postprocess/history guard are implemented; reproducible `docs/manual-qa.md` packaged-app evidence remains |
| 8 | Add sequential queue | In progress | Queue model, React sequential runner backed by Rust queue lifecycle commands, queued job cancellation, and batch summary are implemented; reproducible `docs/manual-qa.md` multi-file, queued-cancellation, and batch-summary evidence remains |
| 9 | Wire source policy and Trash | In progress | Setting is persisted; Ask has explicit Trash action; Trash uses macOS NSFileManager after command-side output revalidation; reproducible `docs/manual-qa.md` Trash evidence remains |
| 10 | Add license activation UI | In progress | Trial UI, locked activation form, local forget action, provider networking, local CLI status/forget, raw-key-free cache, empty-key, network-failure, expired offline-grace reconnect state, failed-activation cache safety, and deactivation-error redaction exist; sandbox/manual validation remains |
| 11 | Build release pipeline | In progress | Unsigned DMG QA artifact upload, read-only workflow permissions, unsigned release failure gate, release gates, clean worktree preflights for manual QA and release notes preparation, artifact/checksum checks, same-DMG manual QA signing evidence, stapled manual QA evidence, Gatekeeper no-warning evidence, required release notes URL field publish gate, checkout blocker URL rule, numeric queue counts, verified smaller Trash output, Lemon Squeezy sandbox activation identity evidence, release notes prepared field-label synchronization, prepared draft placeholder rejection, prepared release notes/manual QA Markdown draft rejection, conversion/queue/Trash/license action-state release notes evidence, final publish gate exact distribution evidence references, publish Artifact URL completion evidence gate, Homebrew install evidence tied to the Homebrew tap PR URL, Homebrew cask release match check, manual QA cask check evidence, complete public web and distribution Execution Order exit evidence, local fragment link guard, CI signing preflight, signing/notarization Next action signing preflight, CI codesign execution runner, CI notarization execution runner, CI stapler execution runner, CI Gatekeeper assessment runner, CI signed DMG artifact check, CI signed checksum generation, private CI signed DMG artifact upload, private CI signed checksum upload, GitHub release command plan, macOS signing command plan, macOS keychain argv plan, macOS keychain cleanup argv plan, macOS codesign argv plan, macOS codesign verification argv plan, macOS notarytool argv plan, macOS stapler argv plan, macOS spctl argv plan, signing runner acceptance criteria, signed DMG target preparation, signed DMG copy isolation, signed DMG artifact guard, macOS verification command/evidence drafts, workflow coverage tests, and template coverage tests exist; signed/notarized publication remains |
| 12 | Generate privacy receipt | Done | Successful CLI/desktop conversions write a local sidecar receipt with file names, `uploaded_bytes = 0`, and `metadata_policy = preserve`; desktop can reveal the saved receipt in Finder; CLI can inspect an existing sidecar |
| 13 | Design DMG install cleanup | In progress | Finder copy, copy-and-paste, or drag-to-Applications cannot run app code, so DropSquash must not promise automatic deletion of the downloaded `.dmg`; copy-and-paste is not a special case and is unsupported for the same reason as drag-and-drop; the requested "clean up the DMG after install" behavior is therefore out of scope for normal Finder installs and only remains a possible future feature for DropSquash's own in-app install flow; `load_install_location` can detect `/Volumes` launches and `/Applications` installs; the desktop UI warns when running from a disk image; explicit copy to `/Applications` is implemented without overwriting an existing app and keeps a post-copy notice visible; the post-copy notice can open the installed app through native macOS APIs, request mounted-volume eject through native macOS APIs, and quit the disk image copy after the installed app opens; downloaded `.dmg` Trash cleanup remains disabled until the backing path is proven; if shipped later, cleanup is an explicit `Move downloaded DMG to Trash` action, never automatic deletion; implement downloaded-DMG Trash only after signed/notarized DMG manual QA proves deterministic backing-path detection for the same DropSquash distribution image and the UI presents it as an explicit user action; detailed rules live in `docs/dmg-install-cleanup.md` |
| 14 | Simplify desktop profile choices | Done | Internal profiles remain available for CLI and backend behavior, while the desktop UI offers Auto, Slack, Docs, and Archive; older chat-service-specific desktop settings normalize to Slack, and saved Privacy settings normalize to Archive until the dedicated metadata/audio pipeline exists |
| 15 | Run Market Validation before scope expansion | In progress | Keep paid beta scope narrow and measurable before adding adjacent media workflows | Positioning variants, demo, feedback loop, manual license issuance, gates, and decision memo | `docs/market-validation.md` deliverables are prepared and the expansion decision is deferred until the memo exists |
| 16 | Freeze evidence-core decisions before implementation spread | Planned | Keep evidence-mode implementation from fragmenting across UI glue and backend guesses | Native-stack choice, naming policy, segment safety, manifest shape, and throughput model are fixed in one formal plan | `docs/evidence-core-architecture.md` |
| 17 | Create a Sites-compatible public web surface | In progress | Keep the public site deployable through the current Sites project instead of leaving `website/` as a local-only static draft | A deployable hosting surface can emit `dist/server/index.js` and `dist/.openai/hosting.json`, then save and deploy a Sites version that later backs `https://dropsquash.app/...` evidence | [docs/website-deployment-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/website-deployment-runbook.md), `cargo run -p xtask -- website-check`, and the final `cargo run -p xtask -- publish-check path/to/release-notes.md` gate |

## Market Validation Deliverables

Complete these deliverables before broad scope expansion:

1. Create three positioning variants:
   - Work Screen Recordings
   - Private Media Library
   - Creator Upload Preparation
2. Create a 30-second before/after demo.
3. Create a beta feedback form and interview script.
4. Prepare manual beta license issuance.
5. Define measurable validation gates:
   - repeat usage
   - three-file completion
   - willingness to pay
   - actual paid beta purchases
6. Produce a market decision memo before expanding scope.

Future ideas such as broad format support, editing, media-library workflows, or
general screen recording still require Market Validation. The approved near-term
exception is high-assurance Secure Share R&D on macOS because it is now the
core value hypothesis. Built-in capture is allowed only when it serves trusted
frame metadata for Secure Share; it must not become a general recorder or
timeline editor.

Updated product hypothesis:

```text
DropSquash becomes sellable when it can locally produce high-assurance
screen-recording exports that destroy sensitive and unknown pixels, verify the
final output, and explain the result honestly.
```

Do not:

- add third-party advertising SDKs
- implement YouTube downloading
- turn the product into a timeline editor
- add broad formats without validated demand
- weaken local-first, no-ffmpeg, or original safety rules

## Release Gate

Use this command before choosing the next productization step:

```sh
cargo run -p xtask -- productization-status
cargo run -p xtask -- productization-status --track "License sandbox proof"
```

It summarizes `docs/release-blockers.md`, groups the remaining blockers by
Execution Order, prints the next unfinished track, and emits preflight
follow-up lines for the current track. It also prints the remaining blocker
names, record target, owner-target next actions, and one primary command under
each incomplete track. It must not be used to mark manual or external evidence
as complete.

Do not start a public paid beta until all of these are true:

```text
macOS single-file conversion is verified
cancel does not count trial
failed conversion does not count trial
larger output keeps the original without counting trial
original is never moved without verified success
app is signed, notarized, and opens without Gatekeeper warning
`docs/release-blockers.md` rows are Verified with concrete Completion evidence and traceable Evidence reference
privacy claims match implementation
license secrets are not in the repository
```

When the narrower private/manual paid beta technical proof is done but public
proof still remains, use
[docs/public-beta-operator-checklist.md](/Users/masakitakemura/_workspace/drop-squash/docs/public-beta-operator-checklist.md)
to finish the deferred public website, checkout, checksum, and Homebrew work
before production onboarding.

After the paid beta is technically ready, do not broaden the product scope
until the Market Validation deliverables in `docs/market-validation.md` exist
and a market decision memo explains whether DropSquash should stay focused on
Mac screen recordings or expand into another validated job.

Before implementing evidence-specific naming, segment extraction, or analyzer
workflows, use `docs/evidence-core-architecture.md` as the fixed design
surface so those features do not drift into UI glue, silent trust changes, or
cross-platform abstraction by habit.

Use [docs/paid-beta-readiness.md](/Users/masakitakemura/_workspace/drop-squash/docs/paid-beta-readiness.md)
as the narrower decision surface for the current objective. Keep
`docs/release-blockers.md` as the superset that still covers the later public
launch path. Continue with
[docs/public-beta-operator-checklist.md](/Users/masakitakemura/_workspace/drop-squash/docs/public-beta-operator-checklist.md)
when the private/manual paid beta proof is done and the deferred public-proof
phase is next.
