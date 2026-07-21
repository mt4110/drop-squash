# Agent Rules

## Work Style

- Work from `docs/productization.md` and the productization design pack.
- Move in small, verifiable commits.
- Support the user's confidence while still saying clearly when something is wrong.
- State mistakes clearly, and choose the path that will not collapse later.
- Keep user-facing progress updates concise and in Japanese.
- Avoid repeating the same point when one clear explanation is enough.

## Product Principles

- Preserve the local-first promise: no media upload and no default telemetry.
- Do not add `ffmpeg`, `ffprobe`, shell execution, or `PATH` lookup to the media path.
- Do not move or delete originals unless conversion success, output verification, and user policy all permit it.
- Keep the UI calm, direct, and Apple-like: it should feel better the more it is used.
- Do not overclaim hardware acceleration, privacy behavior, or release readiness without verification.
- Do not add speculative features without a clear product reason and validation path.

## Current Objective

- Complete only the P0 safety, signed release, trial, license, and single-file conversion path required for a testable macOS paid beta.
- Do not start Stripe or Lemon Squeezy production onboarding before the public information surface, production URL, and distribution surface are ready.
- Keep the implementation focus on the single-file Mac screen-recording workflow until paid-beta readiness is established.

## Updated Productization Direction

- The objective has been updated: proceed from public information and product integrity first, not payment setup first.
- First make the public-facing product surface true, stable, and internally consistent.
- Then make distribution, signing, notarization, trial, license, and support flows line up with that public surface.
- Only after that may Stripe or Lemon Squeezy production onboarding resume.
- Avoid inflated claims, speculative scope, or sales setup that outruns the real implementation.

## Execution Order

1. Finish P0 single-file conversion safety.
2. Keep failed, cancelled, and not-smaller results out of trial counts.
3. Keep original-safety rules intact.
4. Finish trial and license flows.
5. Align the public website and public documents with the real product.
6. Establish production URL and signed/notarized distribution readiness.
7. Only then continue Stripe and Lemon Squeezy production setup.

## Public Surface

- Keep these pages aligned with implementation:
  - Top
  - Pricing
  - Privacy Policy
  - Terms of Service
  - Support
  - Refund Policy
  - License policy
  - Release status
  - Download
- Do not treat License policy as a replacement for Terms of Service.
- Keep trial limits, pricing, purchase flow, refund flow, support flow, and release status consistent across app, docs, and website.

## Market Validation Gate

- Before broad format support, editing features, advertising SDKs, YouTube downloading, or media-library features, complete Market Validation.
- Deliverables:
  - Three positioning variants: Work Screen Recordings, Private Media Library, Creator Upload Preparation
  - 30-second before/after demo
  - Beta feedback form and interview script
  - Manual beta license issuance flow
  - Validation gates: repeat usage, three-file completion, willingness to pay, actual paid beta purchases
  - Market decision memo before scope expansion
- Primary product hypothesis: DropSquash is the automatic local post-processor for Mac screen recordings.

## Explicit Do Not

- Do not add third-party advertising SDKs.
- Do not implement YouTube downloading.
- Do not turn the product into a timeline editor.
- Do not add broad formats without validated demand.
- Do not weaken local-first, no-ffmpeg, or original-safety rules.

## Repository Hygiene

- Do not add codex, Codex, or `[codex]` to repository names, branch names, PR titles, Issue titles, commit messages, or labels unless explicitly requested.
- Keep `.codex` as local agent state: it must stay in `.gitignore` and must not be committed.

## Code Size Rules

- Do not let files grow toward 1000 lines.
- Rust production files must stay at or below 128 lines.
- Rust test files may exceed 1000 lines when justified by fixtures or exhaustive cases.
- TypeScript and TSX files must stay at or below 512 lines.
- Split by responsibility before adding code that would exceed a file limit.
- Prefer focused modules over broad utility files.
- Do not invent arbitrary functions, constants, or variables without a clear reason in the existing architecture.

## Architecture Rules

- Keep business logic in `crates/*`, not in React or Tauri command glue.
- Keep Tauri commands as typed adapters around core services.
- Keep React components focused on rendering and user interaction.
- Put platform-specific behavior behind platform crates or backend modules.
- Add deterministic validation for each meaningful change.

## Required Checks

- Run `cargo run -p xtask -- file-size-check` before committing code changes.
- Run `cargo run -p xtask -- media-policy-check` after touching media, CLI conversion, or desktop command code.
- Run `cargo run -p xtask -- privacy-policy-check` after touching app, website, or privacy-sensitive code.
