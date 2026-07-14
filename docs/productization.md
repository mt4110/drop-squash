# Productization Plan

DropSquash is currently a macOS MVP: a small local Drop Zone that converts user-selected screen recordings into numbered `.squashed.mp4` outputs through Apple's native AVFoundation export pipeline.

The v1 promise stays intentionally narrow:

```text
Drag a screen recording.
Get a smaller MP4.
Original stays safe.
Nothing uploads.
```

## Operating Rules

| Rule | Decision |
|---|---|
| Media path | No `ffmpeg`, no `ffprobe`, no shell, no `PATH` lookup |
| Privacy | No media upload, no default telemetry |
| Original handling | Keep original unless verified success and user policy allow Trash |
| Trial | Count successful, smaller verified conversions only |
| UI | Keep the main window small, calm, and task-focused |
| Platform order | macOS first; Windows and Linux after macOS product hardening |
| Claims | Say AVFoundation/native macOS pipeline now; do not overclaim explicit VideoToolbox hardware encoding |
| File size | Rust production files <= 128 lines; Rust test files may exceed 1000 lines; TS/TSX files <= 512 lines |

## Plan Table

| Phase | Status | Goal | Scope | Acceptance | Verification |
|---|---|---|---|---|---|
| 0.5 Repo alignment | Done | Make names, docs, and metadata consistent before adding product surface | README/docs current-state wording, product invariants, repository URL decision | Docs match implementation; no unsupported hardware claims; no secrets added | `cargo fmt --all -- --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace` |
| 0.6 File-size architecture cleanup | Done | Bring existing code under the repository's size rules before adding more behavior | Split large Rust production files by responsibility; keep TS/TSX under 512 lines | No Rust production file exceeds 128 lines; no TS/TSX file exceeds 512 lines; behavior unchanged | `cargo run -p xtask -- file-size-check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings` |
| 1 macOS encoder hardening | In progress | Make single-file macOS conversion safe enough for paid beta | Replace hard-link finalization, strengthen output verification, clean temp files, friendly failure states | Existing outputs are never overwritten; failed/larger/cancelled conversions do not count; original remains untouched | Unit tests, desktop/CLI success-only history guards, CLI conversion summary output, `docs/qa-evidence.md`, reproducible `docs/manual-qa.md` packaged-app conversion evidence, and release-set benchmark blocker evidence; `manual-qa-check` before completion |
| 2 Cancellation | In progress | Let users stop an active conversion cleanly | Cancellation token through command/encoder boundary, UI cancel action, temp cleanup, post-encode postprocess/history guard | Cancel returns app to ready state; no success history; no trial count | Unit tests, `docs/qa-evidence.md`, plus reproducible `docs/manual-qa.md` cancellation evidence; `manual-qa-check` before completion |
| 3 Sequential queue | In progress | Handle multiple dropped files deterministically | Queue states, per-job progress, one active conversion at a time, queued job cancellation, batch summary | Multiple drops create rows; one job runs at a time; queued cancellation prevents that job from starting; failures do not block unrelated jobs; summary shows finished count and saved bytes | Queue unit tests, `docs/qa-evidence.md`, plus reproducible `docs/manual-qa.md` multi-file, queued-cancellation, and batch-summary evidence; `manual-qa-check` before completion |
| 4 Source postprocess | In progress | Safely move originals to Trash only after verified success | Source policy setting, macOS Trash adapter, ask-after-success flow | Keep never moves; Ask prompts; Trash moves only after all safety gates | Setting, Ask UI, command revalidation tests, Trash adapter, and reproducible `docs/manual-qa.md` Trash evidence; `manual-qa-check` before completion |
| 5 License and trial UI | In progress | Convert trial usage into Pro unlock without account creation | License cache, instance id, activation/validation provider, local forget action, locked/Pro UI, local CLI status/forget | Raw license key is not persisted; empty/invalid/network/deactivation errors are friendly; valid cache survives grace period | Provider, trial UI, activation shell, local raw-key-free forget action, empty-key, network-failure, expired offline-grace reconnect state, failed-activation cache safety, local CLI status/forget, and safe cache/grace are implemented; Lemon Squeezy sandbox test remains |
| 6 Release pipeline | In progress | Ship a trusted macOS beta | Signed app, notarized DMG, checksums, release checklist, Homebrew cask draft, DMG install cleanup design | Gatekeeper opens without warning; secrets stay in CI; artifact checksum is published for the same DMG | Readiness, media/privacy security, read-only workflow permissions, unsigned DMG QA artifact upload, unsigned release failure gate, clean worktree preflights for manual QA and release notes preparation, UDIF artifact, checksum, benchmark blocker, required release notes URL field publish gate, checkout blocker URL rule, same-DMG manual QA signing evidence, stapled manual QA evidence, Gatekeeper no-warning evidence, numeric queue counts, verified smaller Trash output, Lemon Squeezy sandbox activation identity evidence, release notes prepared field-label synchronization, prepared draft placeholder rejection, prepared release notes/manual QA Markdown draft rejection, release blocker classification/execution-order synchronization, release blocker Next action detail gates, signing/notarization Next action signing preflight, CI codesign execution runner, CI notarization execution runner, CI stapler execution runner, CI Gatekeeper assessment runner, CI signed DMG artifact check, CI signed checksum generation, private CI signed DMG artifact upload, private CI signed checksum upload, GitHub release command plan, macOS signing command plan, macOS keychain argv plan, macOS keychain cleanup argv plan, macOS codesign argv plan, macOS codesign verification argv plan, macOS notarytool argv plan, macOS stapler argv plan, macOS spctl argv plan, signing runner acceptance criteria, signed DMG target preparation, signed DMG copy isolation, signed DMG artifact guard, conversion/queue/Trash/license action-state release notes evidence, final publish gate exact distribution evidence references, publish Artifact URL completion evidence gate, Homebrew install evidence tied to the Homebrew tap PR URL, Homebrew cask release match check, manual QA cask check evidence, complete public web and distribution Execution Order exit evidence, CI signing preflight, DMG bundle target, cask generation gates, macOS verification command/evidence drafts, DMG install cleanup design, and release-doc/workflow/template coverage tests exist; `docs/release-blockers.md` tracks external evidence; signing/notarization remain |
| 7 Sales site | In progress | Let a stranger understand, download, try, and buy | Landing, pricing, privacy, license, download, refund, FAQ, support | Privacy claims match implementation; CTA works; download path works | Static site draft, required-page gate, release-status/privacy/license/refund/support contact copy gates, pricing-finalization blocker, pre-release CTA copy guard, download/checkout link and form action guard, local fragment link guard, Public web proof exit coverage for release-status, privacy, pricing, license, support, download, checkout, and refund, actual website directory coverage, and live checkout/download link guard exist; hosting, final pricing, final refund policy, and live checkout are tracked in `docs/release-blockers.md` |
| 8 Windows/Linux | Later | Expand after macOS signal | Media Foundation and GStreamer allowlist backends | Same core invariants; no silent software fallback | Placeholder backends report unavailable and reject encode; native OS CI and real-device smoke tests remain |

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
| 13 | Design DMG install cleanup | In progress | Finder copy or drag-to-Applications cannot run app code, so DropSquash must not promise automatic deletion of the downloaded `.dmg`; `load_install_location` can detect `/Volumes` launches and `/Applications` installs; the desktop UI warns when running from a disk image; explicit copy to `/Applications` is implemented without overwriting an existing app and keeps a post-copy notice visible; copy results expose mounted-volume eject eligibility while keeping downloaded `.dmg` Trash cleanup disabled until the backing path is proven; native mounted-volume eject command exists; relaunch, eject execution UI after relaunch, and user-triggered downloaded `.dmg` cleanup remain |

## Release Gate

Use this command before choosing the next productization step:

```sh
cargo run -p xtask -- productization-status
```

It summarizes `docs/release-blockers.md`, groups the remaining blockers by
Execution Order, and prints the next unfinished track. It must not be used to
mark manual or external evidence as complete.

Do not start a public paid beta until all of these are true:

```text
macOS single-file conversion is verified
cancel does not count trial
failed conversion does not count trial
larger output is treated as failure
original is never moved without verified success
app is signed, notarized, and opens without Gatekeeper warning
`docs/release-blockers.md` rows are Verified with concrete Completion evidence and traceable Evidence reference
privacy claims match implementation
license secrets are not in the repository
```
