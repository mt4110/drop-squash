# Manual Beta License Issuance

Use manual issuance during the paid beta until checkout, support, and refund
flows are trustworthy enough to automate.

For the shortest cross-track status and blocker-type summary, also see
[docs/objective-audit.md](/Users/masakitakemura/_workspace/drop-squash/docs/objective-audit.md).
For the shortest owner-side external action list, also see
[docs/external-unblock-checklist.md](/Users/masakitakemura/_workspace/drop-squash/docs/external-unblock-checklist.md).

This document is for private/manual beta operations only.
It does not replace:

- `paid-beta-check` for the technical paid-beta blockers
- the public-proof work in `docs/public-beta-operator-checklist.md`
- Stripe or Lemon Squeezy production onboarding

For Lemon Squeezy sandbox setup, checkout, and activation proof recording, use
`docs/license-sandbox-runbook.md` as the shorter operator checklist.
If that flow still shows `Sign in to Lemon Squeezy` or
`auth.lemonsqueezy.com/login`, stop there and sign in before recording the
sandbox product setup, purchase, or activation rows.

Quick routes:

- technical paid-beta blocker status:
  `cargo run -p xtask -- paid-beta-check`
- sandbox 3-row proof:
  `docs/license-sandbox-runbook.md` -> `Short Execution Memo`
- deferred public-proof rows:
  `cargo run -p xtask -- public-web-ready`, then
  `docs/public-beta-operator-checklist.md` -> `Short Execution Memo`

Current consistency guard:

- keep the promise aligned with the app and public docs:
  Mac screen recordings in, smaller local MP4 out
- keep trial and paid language aligned with the current beta rule:
  20 successful smaller conversions free; failed, cancelled, and larger-result
  conversions do not count
- keep support aligned with the current public support path:
  GitHub Issues
- do not imply that public checkout, signed release, or production onboarding
  is already complete

## Short Execution Memo

Use this when the goal is to issue one private/manual beta seat cleanly without
pretending that public checkout or production onboarding is already ready.

1. Confirm the tester belongs to the current private beta cohort.
2. Pick the positioning variant:
   `Work Screen Recordings`, `Private Media Library`, or
   `Creator Upload Preparation`.
3. Confirm the exact build under test and the install path you are handing off.
4. Generate or retrieve the beta license key outside this repository.
5. Send one short message with:
   - build/install handoff
   - beta license key
   - local-first reminder
   - current known limits
   - support contact path
6. Record the issuance outside this repository using the tracking template
   below.
7. Update
   [docs/market-validation-tracker.md](/Users/masakitakemura/_workspace/drop-squash/docs/market-validation-tracker.md)
   when the tester completes a 3-file session, comes back for repeat usage, or
   pays.

Do not imply that Stripe or Lemon Squeezy production onboarding is complete
from this flow alone.

## Workflow

1. Confirm the tester is inside the paid beta cohort.
2. Record the intended positioning variant:
   `Work Screen Recordings`, `Private Media Library`, or
   `Creator Upload Preparation`.
3. Generate or retrieve the beta license key outside this repository.
4. Send the key manually with:
   - version being tested
   - install instructions
   - reminder that recordings stay local
   - reminder that the raw key should not be pasted back into support threads
5. Record:
   - issued date
   - user identifier
   - variant
   - app build or Git commit under test
   - whether they completed the first 3-file session
   - whether they returned for repeat usage
   - whether they paid

## Send Template

Keep the message short and operational:

- thank them for joining the paid beta
- name the exact build under test
- include install steps for the current macOS app
- include the manual beta license key outside this repository
- remind them that recordings stay local
- remind them not to paste the raw key back into support threads
- ask them to report whether they completed one real 3-file session
- ask them to report whether they came back for repeat usage

Suggested structure:

1. Build and install link or artifact handoff
2. Beta license key
3. Current known limits
4. Support contact path
5. What feedback to send after first use

## Variant-Specific Message Angle

Pick one primary angle per tester. Do not mix all three in the same message.

### Work Screen Recordings

- one-line promise:
  Make a Mac screen recording smaller before you send it in chat, docs, or bug reports.
- best for:
  demos, async walkthroughs, bug reproduction clips, status updates
- ask for:
  whether the smaller MP4 was actually sent or attached after conversion

### Private Media Library

- one-line promise:
  Shrink personal Mac recordings locally without uploading them anywhere.
- best for:
  people who care more about storage and privacy than collaboration tools
- ask for:
  whether local-only handling increased trust enough to convert multiple files

### Creator Upload Preparation

- one-line promise:
  Turn a Mac screen capture into a smaller upload-ready MP4 before posting it.
- best for:
  people preparing uploads for Slack, Discord, LMS, or simple upload surfaces
- ask for:
  whether DropSquash became part of a repeat upload-prep routine

## Copy-Ready Send Template

Use this as a plain-text base and fill the placeholders outside the repository.

```text
Thanks for joining the DropSquash paid beta.

Build under test:
<build or app artifact>

Positioning:
<Work Screen Recordings | Private Media Library | Creator Upload Preparation>

Install:
<handoff link or artifact path plus simple install note>

Beta license key:
<send outside this repository>

What DropSquash does right now:
- takes an existing Mac screen recording
- makes a smaller local MP4
- keeps the original safe by default

Current limits:
- macOS only
- current input path is MOV / MP4 / M4V
- no built-in screen recording
- no public checkout yet

Trial rule:
20 successful smaller conversions are free.
Failed, cancelled, and larger-result conversions do not count.

Support:
GitHub Issues

Please tell us:
1. whether you completed one real 3-file session
2. whether you came back and used it again on another day
3. whether you would pay for this once it feels reliable enough

Please do not paste the raw license key into support threads.
```

## Tracking Template

Record each issued beta license outside the repository with a compact row:

```text
issued_at:
user_id:
variant:
build_under_test:
delivery_channel:
first_3_file_session: unknown|yes|no
repeat_usage: unknown|yes|no
paid: unknown|yes|no
refund_requested: no|yes
reissue_count:
notes:
```

For the product hypothesis, the most important updates are:

- first successful 3-file session
- repeat usage on a second day
- willingness to pay or actual payment
- why they stopped if they did not return

Use
[docs/market-validation-tracker.md](/Users/masakitakemura/_workspace/drop-squash/docs/market-validation-tracker.md)
as the shared schema when you roll those rows up into gate metrics.
Before updating
[docs/market-decision-memo.md](/Users/masakitakemura/_workspace/drop-squash/docs/market-decision-memo.md),
also refresh the tracker batch note so the variant comparison and biggest
blocker are explicit.

## After Delivery

Once a tester has the build and beta key, use this follow-up order:

1. After the first real session, send
   [docs/market-validation-feedback-form.md](/Users/masakitakemura/_workspace/drop-squash/docs/market-validation-feedback-form.md).
2. If the tester completes a real 3-file session or gives unusually strong
   value/trust feedback, run the
   [docs/market-validation-interview-script.md](/Users/masakitakemura/_workspace/drop-squash/docs/market-validation-interview-script.md)
   follow-up.
3. Roll the answers into
   [docs/market-validation-tracker.md](/Users/masakitakemura/_workspace/drop-squash/docs/market-validation-tracker.md)
   using one activated-tester row.
4. When a small cohort is complete, refresh the batch note and then update
   [docs/market-decision-memo.md](/Users/masakitakemura/_workspace/drop-squash/docs/market-decision-memo.md).

Do not wait for a large public launch before collecting these signals. The
point of manual issuance is to learn early whether the current Mac
screen-recording job earns repeat usage and willingness to pay.

## Support Rules

- Reissue manually while beta volume is low.
- If a tester loses access on one Mac, verify the situation before issuing a new key.
- Handle refunds and exceptions manually until checkout and support flows are stable.
- Keep pricing or refund promises consistent with the current public docs.

## Rules

- Do not store raw beta license keys in this repository.
- Do not promise multi-device automation yet.
- Use manual follow-up for refunds, re-issuance, and support while volumes are low.
