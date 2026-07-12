# QA Evidence

This file separates automated evidence from packaged-app manual QA. Automated
checks reduce the manual surface, but they do not replace real macOS packaged
app testing in `docs/manual-qa.md`.
Public paid beta blockers and their evidence references are tracked in
`docs/release-blockers.md`.

## Automated Coverage

| Area | Evidence | Command |
|---|---|---|
| File-size architecture | Rust production and TS/TSX limits are enforced | `cargo run -p xtask -- file-size-check` |
| External media process ban | Product sources reject `ffmpeg`, `ffprobe`, common JS media process packages, and process command construction | `cargo run -p xtask -- media-policy-check` |
| Privacy guardrails | Product sources reject default telemetry, browser analytics markers, and network clients outside the license provider | `cargo run -p xtask -- privacy-policy-check` |
| Release workflow shape | Release workflow gates, unsigned DMG QA artifact upload, Tauri distribution metadata, CSP, updater, media, privacy, static site policy, and secret-like files are checked | `cargo run -p xtask -- release-check` |
| Release template synchronization | Manual QA, release blockers, evidence classes, release notes, release docs, workflows, and the actual website directory stay covered by xtask tests | `cargo test -p xtask` |
| CI workflow shape | Pull-request CI runs format, file-size, website, release readiness, and Nix dev shell evaluation gates | `cargo run -p xtask -- release-check` |
| Security workflow shape | Security CI runs advisory, license/source, media, and privacy gates | `cargo run -p xtask -- release-check` |
| Manual QA completeness | Packaged-app evidence fields, concrete identity fields, concrete results, malformed table rows, UDIF `.dmg` artifacts, same `.dmg` file evidence for artifact/checksum/signing rows, and Gatekeeper no-warning evidence are checked | `cargo run -p xtask -- manual-qa-check` |
| Manual QA preparation | App state backup, trial reset, and `--restore-state` are tested without deleting config by default | `cargo test -p xtask manual_qa_prepare` |
| Desktop capability policy | Main window permissions are limited to file open and Finder reveal | `cargo run -p xtask -- release-check` |
| Static site | Required pages, local links/resources, approved external links, placeholders, unsupported platform availability claims, release-status, privacy, license, refund, support contact copy, and pre-release download/checkout links or form actions are checked | `cargo run -p xtask -- website-check` |
| Trial counting | History accepts only successful smaller conversions | `cargo test -p dropsquash-history` |
| CLI history writes | CLI conversion uses the same success-only history guard | `cargo test -p dropsquash && cargo test -p dropsquash-history` |
| CLI license commands | CLI reports local trial/license state and forgets the local cache without a raw key or network call | `cargo test -p dropsquash license && cargo run -p dropsquash -- license status --history /tmp/dropsquash-empty-history.jsonl` |
| License cache safety | Raw key persistence, Pro identity requirements, temp-file cache writes, error redaction, trimmed activation, failed activation partial writes, and existing-cache preservation are tested | `cargo test -p dropsquash-license && cargo test -p dropsquash-desktop license` |
| Settings persistence | Output folder, profile, size, source policy, and privacy receipt preference persist through AppConfig | `cargo test -p dropsquash-core -p dropsquash-desktop && pnpm --dir apps/desktop/web lint` |
| Privacy receipt generation | Successful conversions create a local sidecar receipt with file names, `uploaded_bytes = 0`, and `metadata_policy = preserve`; desktop summaries expose the saved receipt path; CLI can inspect the sidecar locally | `cargo test -p dropsquash-privacy -p dropsquash -p dropsquash-desktop` |
| Source movement safety | Postprocess gates, equal-or-larger output boundaries, and desktop command revalidation are tested | `cargo test -p dropsquash-postprocess && cargo test -p dropsquash-desktop source` |
| Queue order | Sequential queue state transitions, QueueWorker events, queued cancellation, Rust/UI batch summary, and trial-lock pending-job blocking are tested | `cargo test -p dropsquash-queue && pnpm --dir apps/desktop/web lint` |
| Cancellation token path | File stability, desktop active-conversion cancellation, and post-encode postprocess/history guard are tested | `cargo test -p dropsquash-fileguard && cargo test -p dropsquash-desktop state && cargo test -p dropsquash-desktop conversion` |
| Unsupported OS backends | Windows/Linux placeholders report unavailable and reject encode instead of falling back | `cargo test -p dropsquash-encoder unimplemented_platform_backends` |
| Benchmark harness | Local encoder benchmark argument parsing, release-set sample count, output acceptance checks, and release blocker/manual QA linkage are tested | `cargo test -p xtask benchmark` |
| Release artifact naming | Tauri DMG output is normalized to the canonical `DropSquash.dmg` name before artifact checks, checksum generation, upload, manual QA evidence, and release notes | `cargo test -p xtask normalize_dmg` |
| Release artifact hygiene | DMG artifacts must use the canonical `DropSquash.dmg` file name and can be checked for emptiness, UDIF trailer, and `/nix/store` references | `cargo run -p xtask -- artifact-check path/to/DropSquash.dmg` |
| Release checksum | Non-empty UDIF `DropSquash.dmg` checksums can be generated deterministically, SHA256SUMS lines use the artifact file name instead of local parent directories, non-DMG targets are rejected, and wrong-name targets are rejected | `cargo run -p xtask -- checksum path/to/DropSquash.dmg` |
| Release notes evidence | Filled public release notes reject duplicate fields, placeholder URLs, nested artifact URLs, checkout/website/refund mix-up, missing public `DropSquash.dmg` signing/notary/Gatekeeper no-warning evidence, missing conversion/queue/Trash/license action-state evidence, generic verification results, vague limitations, and vague support contact | `cargo run -p xtask -- release-notes-check path/to/release-notes.md` |
| Publish gate | Public beta publication requires release readiness, valid static website pages, complete manual QA evidence, filled release notes evidence, every release blocker marked Verified with a traceable Evidence reference, release blocker URLs matching the release notes URLs, and public publish references that point to the release notes URLs | `cargo run -p xtask -- publish-check path/to/release-notes.md` |
| Homebrew cask generation | Cask generation rejects placeholders, non-semver versions, non-GitHub release URLs, nested artifact URLs, mismatched release versions, non-HTTPS URLs, non-DMG URLs, wrong DMG names, non-canonical homepages, invalid checksums, declares no in-app auto-update, and includes the DropSquash app-state `zap` path | `cargo test -p xtask homebrew_cask` |
| macOS signing preflight | Signing preflight rejects local-only CI identities, placeholder certificates, malformed App Store Connect key ids, malformed issuer UUIDs, missing `.p8` files, and malformed Apple team IDs | `cargo test -p xtask macos_signing_check && cargo run -p xtask -- macos-signing-check` |

## Manual-Only Evidence

These checks still require a packaged macOS app or external service state:

| Area | Why automated tests are insufficient |
|---|---|
| Choose recording conversion | Native file picker and packaged app entitlements must be exercised on macOS |
| Drag-and-drop conversion | Packaged WebView drag/drop behavior can differ from dev mode |
| Privacy receipt sidecar | Packaged app file permissions must create the receipt next to the generated output and reveal it in Finder |
| Privacy receipt Finder reveal | Finder selection behavior for generated receipt sidecars is macOS integration, not core logic |
| Duplicate output naming | Repeated packaged-app conversions must show the user-facing numbered suffix |
| Finder reveal | Finder selection behavior is macOS integration, not core logic |
| Ask source policy | The post-conversion prompt and user choice need packaged UI observation |
| Trash source policy | NSFileManager Trash behavior must be verified on the target macOS version |
| Failed conversion | Packaged-app failure state must preserve the original and trial count |
| Larger output | Packaged-app larger-output handling must show failure without trial count |
| Large-recording cancellation | User-visible timing, progress, and cleanup need real media and packaged app behavior |
| Benchmark sample results | Compression ratio and throughput depend on private local media and target Mac hardware |
| Benchmark sample set | Release coverage needs private short, medium, and large local recordings |
| Benchmark regression threshold | Throughput regression needs same-machine comparison against the release candidate baseline |
| Multi-file queue | UI ergonomics and repeated drops need packaged app observation |
| Queued job cancellation | User-visible queue cancellation needs packaged app observation with repeated drops |
| Batch summary | User-visible queue totals need packaged app observation after mixed queue outcomes |
| Lemon Squeezy product setup | Requires sandbox product and license-key configuration evidence outside the repository |
| Lemon Squeezy sandbox purchase | Requires sandbox checkout, intended product, test buyer, and order evidence outside the repository |
| Lemon Squeezy sandbox activation | Requires sandbox product, keys, and server response outside the repository |
| Empty key activation | Requires packaged-app disabled Activate state and local cache inspection after an empty key |
| Invalid license key handling | Requires packaged-app activating/disabled submit state and local cache inspection after an invalid key |
| License network failure | Requires packaged-app UI and local cache inspection after a failed activation request |
| Local license forget | Requires packaged-app forgetting/disabled action state and local cache inspection after forgetting the license |
| Public website deployment | Requires the production website URL and public release pages |
| Refund policy finalized | Requires the production refund policy URL before checkout goes live |
| Live checkout link | Requires the live Lemon Squeezy checkout URL for the intended product |
| Signed DMG verification | Requires the public DMG/app artifact and Developer ID signature state for the same `.dmg` file |
| Notarized/stapled DMG verification | Requires the public DMG/app artifact and Apple notary/staple assessment for the same `.dmg` file |
| Signed/notarized Gatekeeper open | Requires Developer ID signing, notarization, stapling, a clean machine, and no Gatekeeper warning |
| Published checksum | Requires SHA256SUMS attached to the public GitHub Release |
| Homebrew cask install | Requires the Homebrew tap PR and install evidence for the versioned `DropSquash.dmg` artifact, `auto_updates false`, and `zap` cleanup |

Only `docs/manual-qa.md` should be used to record those manual results. Record
the exact app artifact, input sample set, and output folder so the evidence can
be reproduced later.
