# Agent Instructions

## Collaboration

- Work from the productization plan in `docs/productization.md`.
- Proceed in small, verifiable steps instead of large speculative rewrites.
- Before implementing a phase, identify the acceptance criteria and validation commands.
- Keep the user informed in Japanese with concise progress updates.
- Support the user's confidence while still saying clearly when something is wrong.
- Avoid repeating the same point when one clear explanation is enough.
- If a requested direction risks long-term maintainability, say so clearly and propose the safer path.

## Product Principles

- Preserve the local-first promise: no media upload, no default telemetry.
- Productization follows a strict order: first high-assurance product truth,
  then public information and distribution truth, and only after that
  production payment onboarding.
- Do not start Stripe or Lemon Squeezy production onboarding while the public
  site, legal surface, pricing consistency, support path, refund path, trial
  behavior, or distribution path are still incomplete or inconsistent.
- The current completion target is the high-assurance Secure Share R&D
  foundation: ScreenCaptureKit frame metadata, Accessibility structure, local
  Vision text/shape observations, deterministic MaskPlan, Strict Reveal
  fail-closed export, and independent verification.
- Run `cargo run -p xtask -- privacy-policy-check` after touching app, website, or privacy-sensitive code.
- Never add `ffmpeg`, `ffprobe`, shell execution, or `PATH` lookup to the media path.
- Run `cargo run -p xtask -- media-policy-check` after touching media, CLI, or desktop command code.
- Never move or delete originals unless conversion success, output verification, and user policy all permit it.
- Do not overclaim hardware acceleration or metadata removal unless the implementation verifies it.
- Keep the UI small, calm, and focused on dropping recordings and getting smaller MP4 files.
- Design for an Apple-like, hand-fitting feel that improves with repeated use.
- Do not add speculative features without a clear product reason and validation path.

## Productization Order

- Complete the current phase in this order:
  1. Preserve existing macOS conversion, trial, license, and original safety.
  2. Remove or demote weak manual masking surfaces from product-facing claims.
  3. Write the high-assurance Secure Share threat model.
  4. Define the supported matrix and adversarial corpus.
  5. Prototype ScreenCaptureKit frame metadata capture.
  6. Prototype Accessibility structure collection.
  7. Prototype local Vision Japanese/English text and shape observation.
  8. Emit deterministic frame-exact MaskPlan data.
  9. Implement Strict Reveal fail-closed destructive export.
  10. Implement independent final-output verification.
- Keep the public information surface aligned with implementation:
  Top, Pricing, Privacy Policy, Terms of Service, Support, Refund Policy,
  License policy, Release status, and Download.
- Terms of Service and License policy are separate artifacts and must not be
  merged conceptually or operationally.
- All public claims must match the real product. Do not overclaim supported
  OSes, supported formats, privacy behavior, metadata behavior, signing state,
  release readiness, or leak-zero behavior.
- Before broad format support, editing, advertising SDKs, YouTube downloading,
  or media-library features, complete the Market Validation phase and produce
  a market decision memo.

## Quality Bar

- Phase labels such as P0, P1, P2, and P3 are planning tools, not permission to
  ship something half-finished.
- If a phase result is still not good enough for the user, keep iterating on
  the product until the user is satisfied with the quality, clarity, and
  product truth.
- Do not force public release, public web opening, or payment onboarding just
  because a prior checklist partially passed.
- Keep unfinished business surfaces private or owner-only until the user
  explicitly approves public exposure.
- Lemon Squeezy, Stripe, and other production commerce steps stay deferred
  until high-assurance Secure Share passes review and the user explicitly
  reopens them.

## High-Assurance Secure Share Rules

- Manual rectangles and fixed-bar detection are not enough to sell as a
  privacy or enterprise feature.
- Existing `.mov` / `.mp4` files cannot provide high-assurance provenance by
  themselves because they lack capture-time frame metadata and Accessibility
  timeline data.
- Built-in capture is allowed only for high-assurance Secure Share, not as a
  general screen-recorder feature.
- High-assurance mode must combine ScreenCaptureKit frame metadata,
  Accessibility structure, local Vision text/shape observations, deterministic
  MaskPlan generation, final-size CVPixelBuffer destruction, and independent
  output verification.
- Unknown regions are destroyed in Strict Reveal mode.
- Do not claim leak-zero, audit-ready, PII-safe, or enterprise-safe until the
  fail-closed packaged-app path and adversarial manual QA evidence exist.
- Keep ScreenCaptureKit, Vision, CoreVideo, and AVFoundation reference types
  inside the native Apple Capture Bridge. Rust may receive only copied values;
  do not move Apple object pointers through Rust threads.

## Paid Beta Exit

- Paid beta readiness means all of the following are true:
  single-file conversion is safe, failed/cancelled/larger-result conversions do
  not count toward trial usage, original safety rules hold, trial and license
  flows are coherent, public pages match implementation, the production URL
  exists, and signed/notarized distribution is ready for testing.

## Repository Hygiene

- Do not add codex, Codex, or `[codex]` to repository names, branch names, PR titles, Issue titles, commit messages, or labels unless explicitly requested.
- Keep `.codex` as local agent state: it must stay in `.gitignore` and must not be committed.

## Code Size Rules

- Avoid large files. Do not let files drift toward 1000 lines.
- Rust production files must stay at or below 128 lines.
- Rust test files may exceed 1000 lines when the extra length is justified by fixtures or exhaustive cases.
- TypeScript and TSX files must stay at or below 512 lines.
- When a file would exceed its limit, split by responsibility before adding more code.
- Prefer small modules with clear ownership over broad utility files.
- Do not hide complexity by creating vague helpers, catch-all modules, or arbitrary constants.
- Run `cargo run -p xtask -- file-size-check` before committing code changes.

## Architecture Rules

- Keep business logic in `crates/*`, not in React or Tauri command glue.
- Keep Tauri commands as typed adapters around core services.
- Keep React components focused on rendering and user interaction.
- Put platform-specific behavior behind platform crates or backend modules.
- Add tests near the behavior they protect, while respecting production file size limits.
- Each new feature should have a deterministic validation path: unit test, integration test, or documented manual check.
