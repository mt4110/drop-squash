# P1 Review

Use this file for the internal nonpublic review gate before Phase 2 alpha.

Date baseline:

```text
Saturday, July 18, 2026
```

Current internal objective:

```text
Finish P1 product-quality improvements in nonpublic development, then reach the
minimum implementation and verification needed to review P2 alpha. Do not
reopen the public site, Lemon Squeezy production, Stripe production, or
general public distribution as part of this gate.
```

## P1 Review Scope

P1 is about product feel and trust, not public commerce.

- the app should behave like one calm Mac utility
- the small main window should not clip important controls
- trial and license states should stay understandable
- queue, cancellation, and original-safety behavior should stay deterministic
- the public business surface should stay private until we choose otherwise

## Current P1 Audit

| Area | Current state | Strongest evidence now | Review status |
|---|---|---|---|
| Single-instance behavior | Improved in code: launch now keeps a held lock for app lifetime instead of only at startup | [apps/desktop/src-tauri/src/single_instance/unix.rs](/Users/masakitakemura/_workspace/drop-squash/apps/desktop/src-tauri/src/single_instance/unix.rs), `cargo test -p dropsquash-desktop single_instance -- --nocapture` | Ready for user review |
| License panel visibility | Improved in code: the 480px window no longer forces the license input and action into a cramped two-column row | [apps/desktop/web/src/components/LicensePanel.tsx](/Users/masakitakemura/_workspace/drop-squash/apps/desktop/web/src/components/LicensePanel.tsx), [apps/desktop/web/src/styles.css](/Users/masakitakemura/_workspace/drop-squash/apps/desktop/web/src/styles.css), `pnpm --dir apps/desktop/web test` | Ready for user review |
| Window auto-height | Improved in code: window-height padding is larger and reactive resizing remains wired to content changes | [apps/desktop/web/src/hooks/useWindowHeight.ts](/Users/masakitakemura/_workspace/drop-squash/apps/desktop/web/src/hooks/useWindowHeight.ts), [apps/desktop/web/src/lib/windowHeight.ts](/Users/masakitakemura/_workspace/drop-squash/apps/desktop/web/src/lib/windowHeight.ts), [apps/desktop/web/src/lib/windowHeight.test.ts](/Users/masakitakemura/_workspace/drop-squash/apps/desktop/web/src/lib/windowHeight.test.ts) | Ready for user review |
| Lower-panel layout | Improved in code: Settings, Queue, and Install notice now collapse more naturally for the fixed-width main window | [apps/desktop/web/src/styles.panels.css](/Users/masakitakemura/_workspace/drop-squash/apps/desktop/web/src/styles.panels.css), `pnpm --dir apps/desktop/web test` | Ready for user review |
| Queue / cancel / trial safety | Technical proof remains strong and deterministic in the current repo checks | [docs/qa-evidence.md](/Users/masakitakemura/_workspace/drop-squash/docs/qa-evidence.md), `cargo run -p xtask -- release-check` | Technically healthy |
| License friendly failure states | Empty, invalid, network, expired-grace, and forget flows are still verified in the current blocker map | [docs/release-blockers.md](/Users/masakitakemura/_workspace/drop-squash/docs/release-blockers.md), `cargo run -p xtask -- release-check` | Technically healthy |
| Public business surface | Intentionally private again; this is correct for the current internal objective | [docs/release-blockers.md](/Users/masakitakemura/_workspace/drop-squash/docs/release-blockers.md), `cargo run -p xtask -- productization-status --track "Public web proof"` | Intentionally deferred |

## Current Review Snapshot

On Saturday, July 18, 2026, the packaged app review evidence is strong enough
to treat P1 as ready for P2 alpha entry.

- Packaged artifact:
  `/Users/masakitakemura/_workspace/drop-squash/target/release/bundle/macos/DropSquash.app`
- Window balance proof:
  the packaged app at `540 x 980` showed the trial banner, full license field,
  drop zone, settings rows, and the new `共有マスク / Secure Share alpha`
  section without clipping; screenshots:
  `/tmp/dsq-secure-share-ui-refresh.png` and `/tmp/dsq-p1-review.png`
- Single-instance proof:
  before and after reopening the packaged app with `open -a`, the process list
  still showed one packaged-app pid
  `82933 /Users/masakitakemura/_workspace/drop-squash/target/release/bundle/macos/DropSquash.app/Contents/MacOS/dropsquash-desktop`
  and AX window count stayed at `1`
- Automated baseline:
  `pnpm --dir apps/desktop/web test`, `cargo test -p dropsquash-desktop`,
  `pnpm --dir apps/desktop tauri build`, and
  `cargo run -p xtask -- release-check` all passed on Saturday, July 18, 2026

## What Still Needs Human Review

These are the remaining P1 items that code/tests alone do not prove:

1. A later polish pass may still tighten typography and density further.
2. Queue-heavy and install-notice-heavy compositions may still deserve another
   design pass after Phase 2 settles.

## Short Execution Memo

When running the P1 review, keep it small and repeatable.

1. Reconfirm the current automated baseline:
   - `pnpm --dir apps/desktop/web test`
   - `cargo test -p dropsquash-desktop single_instance -- --nocapture`
   - `cargo run -p xtask -- release-check`
   - `cargo run -p xtask -- file-size-check`
2. If you are reviewing the packaged DMG flow, start with:
   - `cargo run -p xtask -- manual-qa-p1-review`
   - `cargo run -p xtask -- manual-qa-packaged-rerun`
3. If you need the mounted-DMG process/window snapshot before and after
   relaunch:
   - `cargo run -p xtask -- manual-qa-window-probe`
4. Review these concrete app states:
   - trial state
   - locked state
   - install notice visible
   - queue visible
   - settings visible
5. Record the result in plain language, not just `OK`.

Recommended review prompts:

- The window feels balanced, not cramped.
- The license input and action stay fully visible.
- Settings and queue still look deliberate in the small window.
- Relaunch focuses the existing app instead of feeling like a second copy.

## Review Notes Template

Copy this block when you want one short P1 review record:

```text
Date:
App build:
Artifact:

State 1: trial
- result:

State 2: locked
- result:

State 3: install notice
- result:

State 4: queue + settings
- result:

State 5: relaunch / single-instance
- result:

P1 review verdict:
- ready for P2 alpha entry / needs more polish
- notes:
```

## P1 Review Gate

Treat P1 as review-ready when all of these are true:

1. The user confirms the current window no longer feels clipped in the main
   common states.
2. The user confirms multiple launches now behave like one Mac app.
3. The current automated checks still pass:
   - `pnpm --dir apps/desktop/web test`
   - `cargo test -p dropsquash-desktop single_instance -- --nocapture`
   - `cargo run -p xtask -- release-check`
4. `cargo run -p xtask -- file-size-check` is either green or its failures are
   clearly outside the P1 app-surface scope and recorded separately as repo
   hygiene follow-up.

Until then, P1 is still in progress even if the code changes are directionally
correct.

## Current Verdict

Verdict on Saturday, July 18, 2026:

- `ready for P2 alpha entry`
- Notes:
  - the packaged app now behaves like one Mac app in the relaunch check
  - the small main window no longer clips the important controls in the tested
    packaged state
  - `cargo run -p xtask -- release-check` now passes on the current tree
  - `cargo run -p xtask -- file-size-check` is still red, but the current
    failures are pre-existing `xtask` files outside the P1 desktop app surface
  - remaining work is Phase 2 product strength, not another P1 blocker

## P2 Alpha Entry

Do not start broad P2 work from a blank slate. P2 alpha should begin only after
P1 review with a narrow macOS-only scope:

- destructive masking before export
- local receipt sidecar
- bounded audit metadata
- no built-in recorder
- no public commerce reopening

The implementation boundary remains
[docs/secure-share-beta.md](/Users/masakitakemura/_workspace/drop-squash/docs/secure-share-beta.md).
