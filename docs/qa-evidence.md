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
| External media process ban | Product sources reject `ffmpeg`, `ffprobe`, and process command construction | `cargo run -p xtask -- media-policy-check` |
| Privacy guardrails | Product sources reject default telemetry, browser analytics markers, and network clients outside the license provider | `cargo run -p xtask -- privacy-policy-check` |
| Release workflow shape | Release workflow gates, Tauri distribution metadata, CSP, updater, media, privacy, and secret-like files are checked | `cargo run -p xtask -- release-check` |
| Manual QA completeness | Packaged-app evidence fields and results are checked | `cargo run -p xtask -- manual-qa-check` |
| Desktop capability policy | Main window permissions are limited to file open and Finder reveal | `cargo run -p xtask -- release-check` |
| Static site | Required pages, local links, placeholders, release-status, privacy, license, support copy, and pre-release download/checkout links are checked | `cargo run -p xtask -- website-check` |
| Trial counting | History accepts only successful smaller conversions | `cargo test -p dropsquash-history` |
| CLI history writes | CLI conversion uses the same success-only history guard | `cargo test -p dropsquash && cargo test -p dropsquash-history` |
| CLI license status | CLI reports local trial/license state without a raw key or network call | `cargo test -p dropsquash license && cargo run -p dropsquash -- license status --history /tmp/dropsquash-empty-history.jsonl` |
| License cache safety | Raw key persistence, Pro identity requirements, temp-file cache writes, error redaction, trimmed activation, failed activation partial writes, and existing-cache preservation are tested | `cargo test -p dropsquash-license && cargo test -p dropsquash-desktop license` |
| Settings persistence | Output folder, profile, size, source policy, and privacy receipt preference persist through AppConfig | `cargo test -p dropsquash-core -p dropsquash-desktop && pnpm --dir apps/desktop/web lint` |
| Privacy receipt generation | Successful conversions create a local sidecar receipt with `uploaded_bytes = 0` and `metadata_policy = preserve`; desktop summaries expose the saved receipt path | `cargo test -p dropsquash-privacy -p dropsquash -p dropsquash-desktop` |
| Source movement safety | Postprocess gates, equal-or-larger output boundaries, and desktop command revalidation are tested | `cargo test -p dropsquash-postprocess && cargo test -p dropsquash-desktop source` |
| Queue order | Sequential queue state transitions and trial-lock pending-job blocking are tested | `cargo test -p dropsquash-queue && pnpm --dir apps/desktop/web lint` |
| Cancellation token path | File stability and desktop active-conversion cancellation are tested | `cargo test -p dropsquash-fileguard && cargo test -p dropsquash-desktop state` |
| Unsupported OS backends | Windows/Linux placeholders report unavailable and reject encode instead of falling back | `cargo test -p dropsquash-encoder unimplemented_platform_backends` |
| Benchmark harness | Local encoder benchmark argument parsing and output acceptance checks are tested | `cargo test -p xtask benchmark` |
| Release artifact hygiene | DMG artifacts can be checked for emptiness and `/nix/store` references | `cargo run -p xtask -- artifact-check path/to/DropSquash.dmg` |
| Release checksum | Non-empty DMG checksums can be generated deterministically | `cargo run -p xtask -- checksum path/to/DropSquash.dmg` |
| Homebrew cask generation | Cask generation rejects placeholders, non-semver versions, non-GitHub release URLs, mismatched release versions, non-HTTPS URLs, non-DMG URLs, and invalid checksums | `cargo test -p xtask homebrew_cask` |

## Manual-Only Evidence

These checks still require a packaged macOS app or external service state:

| Area | Why automated tests are insufficient |
|---|---|
| Choose recording conversion | Native file picker and packaged app entitlements must be exercised on macOS |
| Drag-and-drop conversion | Packaged WebView drag/drop behavior can differ from dev mode |
| Privacy receipt sidecar | Packaged app file permissions must create the receipt next to the generated output and reveal it in Finder |
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
| Lemon Squeezy sandbox purchase | Requires sandbox checkout, intended product, and test buyer evidence outside the repository |
| Lemon Squeezy sandbox activation | Requires sandbox product, keys, and server response outside the repository |
| Signed DMG verification | Requires the public DMG/app artifact and Developer ID signature state |
| Notarized/stapled DMG verification | Requires the public DMG/app artifact and Apple notary/staple assessment |
| Signed/notarized Gatekeeper open | Requires Developer ID signing, notarization, stapling, and a clean machine |

Only `docs/manual-qa.md` should be used to record those manual results. Record
the exact app artifact, input sample set, and output folder so the evidence can
be reproduced later.
