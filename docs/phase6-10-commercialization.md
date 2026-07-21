# Phase 6-10 Commercialization Design

## Purpose

DropSquash moves from a Build Week Secure Share alpha toward a product that can
be sold honestly to individuals and teams. This is a design and evidence plan,
not permission to claim leak-zero, enterprise audit readiness, or complete
automatic masking.

The shared promise stays narrow:

```text
Process recordings locally. Keep originals safe. Publish only verified output.
```

## Product Lanes

| Lane | Customer job | Product promise | Never promise yet |
| --- | --- | --- | --- |
| CtoC Pro | Make a recording smaller before sharing | Local conversion with safe originals and clear results | Automatic PII removal |
| BtoB Secure Share | Share selected-window QA evidence with bounded privacy controls | Local capture, destructive masking, verification, and redacted evidence on macOS and Windows | Leak-zero or universal enterprise compliance |

The codebase has one shared local media and evidence core. Capture policy,
evidence requirements, support commitments, and commercial terms are separate
per lane. Do not build a general editor, cloud media library, or broad recorder
to serve either lane.

## Architecture Boundary

```text
Shared Rust core
  policy, MaskPlan schema, destructive-mask semantics, verification,
  evidence signing, original safety, trial/license state
        |
Native capture bridge per operating system
  macOS: ScreenCaptureKit + Accessibility + Vision + AVFoundation
  Windows: Windows Graphics Capture + UI Automation + Media Foundation
        |
Desktop adapters
  target selection, permissions, local settings, JA/EN presentation
```

The native bridge owns all platform reference types, callback queues, capture
objects, pixel buffers, and writer objects. It exposes copied scalar values,
redacted rectangles, frame timestamps, status codes, and owned output bytes to
Rust. A bridge never exposes a platform object pointer across a Rust thread.

The shared core defines the only publish decision:

```text
complete trusted observations + deterministic MaskPlan + verified output
  -> signed receipt and save
otherwise
  -> discard partial output and explain the blocked condition
```

Windows cannot be called feature-compatible merely because it has similar UI.
Each supported Windows path must independently prove capture continuity,
coordinate transforms, destructive overwrite, final decode, and evidence
binding against the same schema.

## Lane Requirements

| Area | CtoC Pro | BtoB Secure Share |
| --- | --- | --- |
| Input | Existing recordings chosen by the person | Built-in selected-window capture and bounded existing-recording conversion |
| Core outcome | Smaller local MP4 with original retained | Verified local evidence export or explicit fail-closed rejection |
| Privacy UI | Plain local-processing explanation; no masking promise | Target choice, coverage state, blocked reason, and redacted receipt summary |
| Support | Self-service help and best-effort support | Named pilot contact, reproducible QA protocol, and defined escalation path |
| Commercial unit | One person, one-time Pro license | Time-bounded pilot or per-seat agreement; no implied compliance warranty |
| Data policy | No upload and no default telemetry | Same, plus receipt minimization and documented retention responsibility |

An individual may use Secure Share research features during a private beta, but
the CtoC SKU must not be marketed as privacy protection until its own approved
claim matrix exists. Conversely, BtoB must not inherit an unlimited consumer
license or a vague "Pro" promise.

## Validation Scorecard

Record these values by lane, platform, app version, and fixture version. Do
not aggregate away a platform or safety failure.

| Metric | Definition | Gate |
| --- | --- | --- |
| Verified completion | Successful verified workflows / started workflows | Report, never hide rejects |
| Unsafe publication | Published workflow with a required check missing | Must be zero |
| Fail-closed clarity | Rejected workflows with a user-actionable reason | 100% of rejects |
| Selective residual finding | Synthetic sensitive region found after decode | Must be zero on supported fixtures |
| False destruction | Deliberately safe fixture pixels destroyed | Measure before any selective claim |
| Three-workflow completion | Testers completing three real workflows | Phase 8 cohort gate |
| 14/30-day repeat use | Testers returning after first workflow | Lane demand signal |
| Willingness to pay | Interviewees accepting a stated price range | Record exact segment and price |
| Support load | Human support hours / active tester | Must fit the intended business model |

## Pricing And Purchase Hypotheses

Prices are hypotheses to test, not published commitments.

| Lane | First offer | Why this shape | Evidence required before live sale |
| --- | --- | --- | --- |
| CtoC Pro | One-time personal license, with 20 verified free conversions | Matches occasional recording workflows and avoids subscription distrust | Repeat conversion use, stated price acceptance, reliable trial accounting |
| BtoB Secure Share | Fixed-scope paid pilot with named seats and end date | Lets support and threat boundaries stay explicit while evidence matures | QA-lead workflow completion, pilot success criteria, support capacity, security boundary review |

Do not publish exact prices until Phase 8 interviews compare at least two price
anchors per lane and the owner selects a value proposition. BtoB should not use
self-serve checkout until the pilot contract, refund/termination terms,
support promise, and supported-environment matrix are real.

## Legal, Distribution, And Support Surface

Before G4, each chosen lane needs a reviewable version of these artifacts:

- Privacy Policy: local processing, no default telemetry, what receipts do and
  do not contain, and the user's responsibility for retained files.
- Terms of Service: product limits, no compliance guarantee, acceptable use,
  and the service boundary.
- License Policy: device/seat entitlement, transfer, revocation, and beta
  treatment. It remains separate from Terms.
- Refund Policy: CtoC refund window and BtoB pilot termination/refund rule.
- Support Policy: contact path, response target, supported OS/builds, and how
  to provide a redacted diagnostic report.
- Release Status: exact build, signatures, notarization or Windows signing,
  known limits, and every currently supported claim.

No public page may use "secure", "private", "enterprise", or "automatic
masking" more broadly than the current supported matrix and evidence support.
The detailed enterprise sale gate is
[docs/secure-share-enterprise-sale-gate.md](/Users/masakitakemura/_workspace/drop-squash/docs/secure-share-enterprise-sale-gate.md).

## Go-To-Market Sequence

1. Recruit a private CtoC conversion cohort through direct outreach, not broad
   advertising. Learn whether local size reduction alone earns repeat use.
2. Recruit a separate BtoB design-partner cohort of QA leads. Sell discovery
   and a bounded pilot only after the Secure Share matrix is reviewable.
3. Publish a factual technical explainer and demo only after the matching
   artifact, claim matrix, and support route exist.
4. Choose one lead lane after Phase 8. The other remains a documented beta,
   not a simultaneous marketing campaign.
5. Open the smallest paid offer that can be supported well, then review weekly
   evidence before raising price, expanding platforms, or adding checkout.

## Phase Owners And Stop Conditions

| Phase | Owner decision | Stop immediately when |
| --- | --- | --- |
| 6 | Which capabilities are actually supported | Evidence contradicts a product claim |
| 7 | Whether selective masking can leave research | Any residual fixture leak or unbounded unknown region appears |
| 8 | Which lane has real demand | Cohort signals are absent, contradictory, or unsafe |
| 9 | Whether the artifact may be offered | Legal, signing, support, or public copy disagrees with the build |
| 10 | Whether to continue paid beta | Safety incident, unsustainable support load, or weak retention persists |

## Phase Plan

### Phase 6: Product Truth And Core Hardening

Goal: turn the Phase 5 alpha into a reproducible engineering baseline.

- Freeze the supported macOS, hardware, display, and selected-window matrix.
- Finish the native Apple Capture Bridge boundary; Apple reference types remain
  in Objective-C/Swift-native code and Rust receives copied values only.
- Define the Windows Media Foundation / Windows Graphics Capture bridge with
  the same copied-value ABI boundary, capture-time metadata, and fail-closed
  rules. It is a first-class BtoB target, not a later portability cleanup.
- Re-run the adversarial corpus with packaged-app artifacts and record pass,
  fail-closed, or out-of-scope status per threat.
- Preserve conversion, trial, license, original handling, JA/EN UI, and
  independent verification regression coverage.

Exit: every macOS claim has a fixture or manual QA record; the Windows bridge
contract, support matrix, and threat-model delta are reviewed before feature
implementation. Any missing required signal discards output. No sale or public
privacy claim reopens here.

### Phase 7: Selective Share Research Gate

Goal: establish whether useful selective sharing is technically defensible.

- Implement native per-frame policy evaluation for Japanese/English text-shape,
  AX geometry, and changed/unknown regions.
- Implement the Windows equivalent only after its bridge can enforce the same
  capture, overwrite, and final-decode boundaries; do not port UI ahead of the
  evidence path.
- Keep a strict fallback: unknown coverage means full-frame destruction or no
  output, never a silently permissive export.
- Independently decode final MP4s and test residual regions against synthetic
  fixtures before publication.
- Measure false disclosure, false destruction, latency, dropped-frame rate,
  and fail-closed rate separately.

Exit: selective output is permitted only for a published, bounded fixture and
support matrix. If usefulness or safety is not demonstrated, retain Strict
Shield as research only and do not sell selective masking.

### Phase 8: Two Private Validation Cohorts

Goal: learn whether either lane solves a paid problem before commerce work.

- CtoC cohort: Mac creators and developers using screen-recording conversion.
- BtoB cohort: QA leads and testers on macOS or Windows, with the operating
  system recorded separately for every workflow result.
- Give each cohort a distinct onboarding, feedback form, and success metric;
  do not pool their results.
- Use manually issued beta licenses and private distribution only.

Required signals:

| Lane | Minimum signal |
| --- | --- |
| CtoC | Three verified conversions per active tester and repeat use within 14 days |
| BtoB | One representative evidence workflow completed and reviewed by a QA lead |
| Both | Clear willingness-to-pay interviews and no unresolved safety-critical failure |

Exit: a written market decision memo chooses one lead lane. Failure to obtain
evidence is a stop/go decision, not a reason to add features.

### Phase 9: Release And Commercial Readiness

Goal: make the chosen lane distributable without product-truth gaps.

- Complete signed, notarized, stapled macOS release evidence and clean-machine
  Gatekeeper QA for the exact artifact. Complete Windows code-signing,
  SmartScreen/reputation, and clean-machine QA evidence before any Windows
  download is offered.
- Publish only the pages and claims supported by the chosen lane: download,
  release status, privacy, terms, license, support, refund, and pricing.
- Finalize accessible JA primary / EN secondary product copy and support path.
- Run sandbox license and purchase tests only after the public information and
  release artifact match.

Exit: production URL, artifact, policies, trial, license behavior, and support
path are coherent. No checkout opens until this gate is explicitly approved.

### Phase 10: Controlled Sales Launch

Goal: open one sales lane, measure it, and protect trust.

- Start with a limited paid beta, a defined cohort, and a clear support SLA.
- CtoC can use one-time Pro pricing only after conversion reliability and trial
  metrics pass.
- BtoB can use a manual quote or pilot agreement first; seats, retention,
  evidence export, and support obligations must be explicit before self-serve
  team checkout exists.
- Track activation, three-workflow completion, 30-day retention, support
  burden, refunds, and safety failures. Review weekly.

Exit: continue, pause, or expand based on recorded economics and quality. A
single sale is validation of interest, not proof of product-market fit.

## Commercial Gates

| Gate | Required decision |
| --- | --- |
| G1 | Phase 6 evidence supports every stated capability |
| G2 | Phase 7 selective output is useful and bounded, or it remains unavailable |
| G3 | Phase 8 identifies a lead lane from real cohort evidence |
| G4 | Phase 9 distribution and public information match the artifact |
| G5 | Phase 10 paid beta is explicitly approved by the owner |

## Non-Negotiables

- No media upload, default telemetry, `ffmpeg`, `ffprobe`, shell media path,
  or automatic original deletion.
- No claim of leak-zero, audit-ready, PII-safe, or enterprise-safe without
  evidence that directly supports that exact claim.
- No production payment onboarding before G4 and explicit owner approval.
- Windows is a formal BtoB target from Phase 6 onward. macOS remains the first
  shipping baseline; Windows ships only when its independent evidence reaches
  the same selected capabilities and G4 release bar. Linux is out of scope.
