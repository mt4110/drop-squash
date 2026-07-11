# Agent Rules

## Work Style

- Work from `docs/productization.md` and the productization design pack.
- Move in small, verifiable commits.
- State mistakes clearly, and choose the path that will not collapse later.
- Keep user-facing progress updates concise and in Japanese.

## Product Principles

- Preserve the local-first promise: no media upload and no default telemetry.
- Do not add `ffmpeg`, `ffprobe`, shell execution, or `PATH` lookup to the media path.
- Do not move or delete originals unless conversion success, output verification, and user policy all permit it.
- Keep the UI calm, direct, and Apple-like: it should feel better the more it is used.
- Do not overclaim hardware acceleration, privacy behavior, or release readiness without verification.

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
