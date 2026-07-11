# QA Evidence

This file separates automated evidence from packaged-app manual QA. Automated
checks reduce the manual surface, but they do not replace real macOS packaged
app testing in `docs/manual-qa.md`.

## Automated Coverage

| Area | Evidence | Command |
|---|---|---|
| File-size architecture | Rust production and TS/TSX limits are enforced | `cargo run -p xtask -- file-size-check` |
| External media process ban | Product sources reject `ffmpeg`, `ffprobe`, and process command construction | `cargo run -p xtask -- media-policy-check` |
| Privacy guardrails | Product sources reject default telemetry, browser analytics markers, and network clients outside the license provider | `cargo run -p xtask -- privacy-policy-check` |
| Release readiness | Release workflow, CSP, updater, media, privacy, and secret-like files are checked | `cargo run -p xtask -- release-check` |
| Desktop capability policy | Main window permissions are limited to file open and Finder reveal | `cargo run -p xtask -- release-check` |
| Static site | Required pages, local links, and placeholders are checked | `cargo run -p xtask -- website-check` |
| Trial counting | History accepts only successful smaller conversions | `cargo test -p dropsquash-history` |
| License cache safety | Raw key persistence, Pro identity requirements, temp-file cache writes, error redaction, trimmed activation, and failed activation partial writes are tested | `cargo test -p dropsquash-license && cargo test -p dropsquash-desktop license` |
| Source movement safety | Postprocess gates and desktop command revalidation are tested | `cargo test -p dropsquash-postprocess && cargo test -p dropsquash-desktop source` |
| Queue order | Sequential queue state transitions are tested | `cargo test -p dropsquash-queue` |
| Cancellation token path | File stability and desktop active-conversion cancellation are tested | `cargo test -p dropsquash-fileguard && cargo test -p dropsquash-desktop state` |
| Benchmark harness | Local encoder benchmark argument parsing and output acceptance checks are tested | `cargo test -p xtask benchmark` |
| Release artifact hygiene | DMG artifacts can be checked for emptiness and `/nix/store` references | `cargo run -p xtask -- artifact-check path/to/DropSquash.dmg` |
| Release checksum | Non-empty DMG checksums can be generated deterministically | `cargo run -p xtask -- checksum path/to/DropSquash.dmg` |
| Homebrew cask generation | Cask generation rejects placeholders, non-HTTPS URLs, non-DMG URLs, and invalid checksums | `cargo test -p xtask homebrew_cask` |

## Manual-Only Evidence

These checks still require a packaged macOS app or external service state:

| Area | Why automated tests are insufficient |
|---|---|
| Choose recording conversion | Native file picker and packaged app entitlements must be exercised on macOS |
| Drag-and-drop conversion | Packaged WebView drag/drop behavior can differ from dev mode |
| Finder reveal | Finder selection behavior is macOS integration, not core logic |
| Trash source policy | NSFileManager Trash behavior must be verified on the target macOS version |
| Large-recording cancellation | User-visible timing, progress, and cleanup need real media and packaged app behavior |
| Benchmark sample results | Compression ratio and throughput depend on private local media and target Mac hardware |
| Multi-file queue | UI ergonomics and repeated drops need packaged app observation |
| Lemon Squeezy sandbox activation | Requires sandbox product, keys, and server response outside the repository |
| Signed/notarized Gatekeeper open | Requires Developer ID signing, notarization, stapling, and a clean machine |

Only `docs/manual-qa.md` should be used to record those manual results. Record
the exact app artifact, input sample set, and output folder so the evidence can
be reproduced later.
