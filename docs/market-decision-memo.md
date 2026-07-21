# Market Decision Memo

## Current Decision

- Date: 2026-07-22
- Status: no sales decision yet
- Recommendation: finish the bounded macOS Secure Share evidence path before
  choosing a commercial lead lane

## Product Truth

- C2C Pro: local Mac recording conversion, original safety, and no default
  upload are the only current product claims.
- B2B Secure Share: native selected-window capture, full-frame Strict Shield,
  redacted observations, and independent verification are research evidence.
- Selective privacy masking, Windows Secure Share, notarized distribution, and
  paid checkout are not currently offered.

## Evidence Status

- CI: macOS, Windows, Ubuntu, and Nix checks passed for the current branch.
- macOS Strict Shield: native per-frame destruction evidence and a signed
  redacted plan are implemented; latest packaged-fixture evidence is pending.
- Selective output: unavailable. A black-only recording is not a useful
  customer deliverable and is not represented as one.
- Demand: no private cohort, willingness-to-pay study, or paid purchase result
  is yet sufficient for a commercial decision.

## Next Move

1. Finish Phase 6 packaged macOS fixtures and fail-closed evidence.
2. Run Phase 7 only on a published fixture matrix; promote selective output
   only when decoded residual checks pass.
3. Run separate C2C and B2B private validation cohorts in Phase 8.
4. Use the finite Phase 10 package to choose `Go`, `Iterate`, or `Stop`.

The decision conditions and evidence ledger are in
[docs/phase10-decision-package.md](/Users/masakitakemura/_workspace/drop-squash/docs/phase10-decision-package.md).
