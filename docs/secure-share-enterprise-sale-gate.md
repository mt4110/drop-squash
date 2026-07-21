# Secure Share Enterprise Sale Gate

## Purpose

This document defines the conditions for offering Secure Share to an
enterprise customer. It applies to a bounded supported matrix only. It does
not authorize a leak-zero, compliance, audit-ready, or universal PII-safety
claim.

The minimum product truth is stronger than "a recording was masked":

```text
The selected capture was locally observed, a deterministic plan destroyed
required pixels before encoding, the final decoded video was verified, and a
redacted receipt identifies exactly which fixture and policy version ran.
```

## Required Output Set

Every accepted Secure Share test run produces one directory:

```text
fixture-id/
  recording.mp4
  recording.receipt.json
  recording.mask-plan.json
  verification.json
  manifest.json
```

No raw captured frame, recognized text, window title, accessibility value, or
application-private content may appear in these artifacts. A failed run must
produce no MP4. It may produce a redacted failure report only when the report
cannot reveal the captured content.

## Test Video Identity

Each test video is identifiable without exposing the test secret. A fixture
uses a visible, non-sensitive test slate and a signed manifest with:

| Field | Meaning |
| --- | --- |
| `fixtureId` | Stable scenario identifier, such as `ja-ime-candidate-001` |
| `fixtureVersion` | Immutable fixture content revision |
| `platform` | `macos` or `windows` |
| `capturePath` | Native bridge and writer path version |
| `policyVersion` | Mask resolver rules used for this run |
| `expectedOutcome` | `selective-pass`, `strict-shield-pass`, or `fail-closed` |
| `verificationStatus` | Independent decoder verdict |
| `artifactSha256` | Binding to the final MP4 |

The visible slate may show `DropSquash Secure Share Test`, the fixture ID, and
an intentionally public nonce. It must not show a production customer name,
credential, personal identifier, or the sensitive test string.

## Fixture Families

Each supported platform needs fixture families below. A fixture is not
"covered" until it has a versioned source, expected policy decision, packaged
app artifact, and independent decoder result.

| Family | Examples | Required acceptance result |
| --- | --- | --- |
| Text input | JA/EN typing, paste, password-like field | Required regions are destroyed or export fails closed |
| IME | Composition and candidate list | Same, including transient candidate movement |
| Browser UI | Autofill, password manager, permission sheet | Same, or explicit out-of-scope rejection |
| Application UI | Modal, popover, tooltip, sidebar, notification | Same across appearance and disappearance |
| Window continuity | Move, resize, hide, owner replacement, focus change | Export fails closed when continuity cannot be proven |
| Display continuity | Scale, rotation, display attach/detach, mirror | Export fails closed when observed; physical QA remains explicit |
| Media integrity | Audio, captions, metadata, frame loss/order | No forbidden stream or metadata; decoder rejects bad sequence |
| Policy boundary | Unknown region and detector disagreement | Strict Shield or fail-closed, never permissive selective output |

## Pixel Destruction Requirement

Masking must overwrite the final-size pixel buffer before the writer receives
the frame. Blur, crop-only metadata, transparent overlays, player-side filters,
and post-delivery rendering are not masking.

For every planned region, verification samples the decoded output over the
region and checks the selected destructive policy. The initial supported policy
is solid black. Noise is not a release policy until it has an independently
testable residual-information definition and decoder verifier.

The verifier must separately prove:

1. The final MP4 hash matches the receipt.
2. Decoded frame order and timestamps meet continuity rules.
3. Required plan regions are black in the decoded frame tolerance model.
4. No audio stream, subtitle stream, or prohibited metadata survives.
5. The receipt signature verifies against the published verification key.

An encoder cannot mark its own output accepted. The verifier is a separate
code path and must read the saved MP4, not writer-side frame memory.

## Selective-Mask Promotion Rules

Selective masking may move from research to a customer pilot only when all of
these hold for one declared fixture matrix:

- Each fixture family has a packaged-app pass or an explicit fail-closed test.
- No decoded sensitive fixture region remains visible in accepted output.
- Unknown, missing, late, or conflicting observations produce Strict Shield or
  a rejected export.
- The exact native bridge, policy, fixture, and verifier versions are recorded.
- A manual adversarial review confirms that the visible test video matches the
  manifest and receipt.
- The customer-facing support matrix names the supported OS version, display
  arrangement, application class, language, and known exclusions.

The result is a bounded claim, for example:

```text
Secure Share pilot supports the published macOS or Windows fixture matrix and
rejects captures that fail its continuity or verification checks.
```

It is not acceptable to shorten this to "automatically removes sensitive data."

## Enterprise Pilot Contract Boundary

Before charging an enterprise pilot, document these facts in the offer and
pilot agreement:

- Supported platform/build and target application classes.
- Which fixture families have evidence and which are fail-closed or excluded.
- That all capture processing remains on the customer's device by default.
- What redacted receipt data is retained locally and who controls it.
- The escalation channel, response target, and diagnostic-data consent flow.
- No compliance certification, no assurance for unsupported surfaces, and no
  promise of zero residual risk.
- Pilot success criteria: a defined representative QA workflow, named users,
  time period, and review meeting.

## Release Checklist

An enterprise Secure Share build is eligible for a private pilot only when:

- The platform-specific signed artifact and clean-machine evidence exist.
- The fixture manifest, expected outcomes, and latest results are versioned.
- Independent verification passes for every advertised fixture row.
- Failed fixture rows are visible in the support matrix and cannot be hidden by
  an aggregate pass percentage.
- JA primary and EN secondary explanations identify the same limitations.
- A rollback path removes the build and revokes pilot access without deleting
  the customer's local evidence files.

Any release that lacks one row is a research build, not an enterprise offer.
