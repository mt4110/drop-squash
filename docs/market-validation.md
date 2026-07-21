# Market Validation

Primary product hypothesis:

```text
DropSquash is the automatic local post-processor for Mac screen recordings.
```

For the shortest cross-track snapshot, also see
[docs/objective-audit.md](/Users/masakitakemura/_workspace/drop-squash/docs/objective-audit.md).

Do not expand into broad formats, editing, advertising SDKs, YouTube
downloading, or media-library features before the paid beta is live enough to
test this hypothesis.

Also do not turn DropSquash into a built-in screen-recording app during this
phase. Built-in capture is now a clearly named future direction, but it still
changes the product boundary from post-processing to recording, so it needs its
own validation after the current paid beta. The same rule applies to automatic
privacy masking: treat it as a possible future Pro feature only after the core
local post-processing job proves repeat usage and willingness to pay.
See [docs/future-capture-masking.md](/Users/masakitakemura/_workspace/drop-squash/docs/future-capture-masking.md)
for the preserved future candidate boundary.

## Positioning Variants

| Variant | Core user | Promise | What to measure |
|---|---|---|---|
| Work Screen Recordings | People who send demos, bug reports, status updates, or async walkthroughs | Drop a Mac screen recording and get a smaller MP4 that is easier to send without changing tools | Repeat usage, three-file completion, actual sending/sharing after conversion |
| Private Media Library | People who want to keep personal recordings local and easier to store | Shrink private Mac recordings locally without upload risk or destructive cleanup | Whether users trust local-only enough to convert multiple private files |
| Creator Upload Preparation | People preparing uploads for Slack, Discord, LMS, or simple upload surfaces | Turn screen captures into smaller upload-ready MP4 files before posting | Whether conversion becomes part of a repeated upload-prep workflow |

Recommended default variant for the paid beta:

```text
Work Screen Recordings
```

The other two variants are probes, not expansion commitments.

## Expansion Boundaries

The current validation phase is about whether users want a local tool that
takes existing Mac screen recordings and makes them smaller safely.

Do not use this phase to justify:

- built-in screen recording
- timeline editing
- automatic privacy masking
- broad non-screen-recording format support

Those may become follow-on experiments only after the market decision memo says
the core post-processing job is strong enough to expand.

## Deliverables

1. 30-second before/after demo:
   Show the original screen recording, the DropSquash flow, the smaller output,
   and the unchanged original. Use
   [docs/market-validation-demo.md](/Users/masakitakemura/_workspace/drop-squash/docs/market-validation-demo.md).
2. Beta feedback form:
   Use [docs/market-validation-feedback-form.md](/Users/masakitakemura/_workspace/drop-squash/docs/market-validation-feedback-form.md).
3. Interview script:
   Use [docs/market-validation-interview-script.md](/Users/masakitakemura/_workspace/drop-squash/docs/market-validation-interview-script.md).
4. Manual beta license issuance:
   Use [docs/manual-beta-license-issuance.md](/Users/masakitakemura/_workspace/drop-squash/docs/manual-beta-license-issuance.md).
5. Market decision memo:
   Update [docs/market-decision-memo.md](/Users/masakitakemura/_workspace/drop-squash/docs/market-decision-memo.md)
   before expanding scope. Keep
   [docs/market-decision-memo-template.md](/Users/masakitakemura/_workspace/drop-squash/docs/market-decision-memo-template.md)
   as the blank template.
6. Validation tracker:
   Use [docs/market-validation-tracker.md](/Users/masakitakemura/_workspace/drop-squash/docs/market-validation-tracker.md)
   to collect the four gate metrics consistently.

## Requirement Audit As Of Saturday, July 18, 2026

Use this table when deciding whether scope expansion is justified. This is an
audit view, not a substitute for real cohort evidence.

| Deliverable | Current status | Strongest evidence now | Remaining proof |
|---|---|---|---|
| Positioning variants | Prepared | This document defines `Work Screen Recordings`, `Private Media Library`, and `Creator Upload Preparation` with user/job framing | Run real beta sessions against the variants and record which one drives repeat usage and payment intent |
| 30-second before/after demo | Script prepared | [docs/market-validation-demo.md](/Users/masakitakemura/_workspace/drop-squash/docs/market-validation-demo.md) defines the required shots and claims boundary | Record the actual packaged-app demo artifact and keep it tied to a tested build |
| Beta feedback form | Prepared | [docs/market-validation-feedback-form.md](/Users/masakitakemura/_workspace/drop-squash/docs/market-validation-feedback-form.md) exists and covers outcome, friction, and value | Collect real responses from activated testers |
| Interview script | Prepared | [docs/market-validation-interview-script.md](/Users/masakitakemura/_workspace/drop-squash/docs/market-validation-interview-script.md) exists and covers repetition, fit, and payment | Run real follow-up interviews and roll up results |
| Manual beta license issuance | Prepared | [docs/manual-beta-license-issuance.md](/Users/masakitakemura/_workspace/drop-squash/docs/manual-beta-license-issuance.md) exists and aligns with the current private beta promise | Issue real beta seats and track three-file completion, repeat usage, and paid outcomes |
| Validation tracker | Prepared schema only | [docs/market-validation-tracker.md](/Users/masakitakemura/_workspace/drop-squash/docs/market-validation-tracker.md) defines per-tester rows, rollups, and formulas | Fill it with real cohort data outside the repository if personal data is involved |
| Market decision memo | Provisional only | [docs/market-decision-memo.md](/Users/masakitakemura/_workspace/drop-squash/docs/market-decision-memo.md) already recommends staying focused on Mac screen recordings because evidence is still thin | Refresh the memo after real cohort evidence exists for all four validation gates |

Current interpretation:

- the market-validation framework is prepared
- the cohort evidence is not yet collected deeply enough to justify expansion
- therefore broad formats, editing, advertising SDKs, YouTube downloading,
  media-library features, built-in capture, and automatic privacy masking stay
  out of scope

## Validation Gates

| Signal | Gate for expansion review |
|---|---|
| Repeat usage | At least 40% of activated beta users complete successful conversions on 2 or more separate days within 14 days |
| Three-file completion | At least 70% of observed first sessions complete 3 successful files in one session |
| Willingness to pay | At least 30% of interviewed or surveyed qualified beta users say they would pay and can name a believable price range |
| Actual paid beta purchases | At least 5 independent paid beta purchases or at least 10% paid conversion from qualified beta users |

If these gates are not met, keep the scope narrow and fix the core screen
recording job before adding adjacent features.
