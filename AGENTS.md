# Agent Instructions

## Collaboration

- Work from the productization plan in `docs/productization.md`.
- Proceed in small, verifiable steps instead of large speculative rewrites.
- Before implementing a phase, identify the acceptance criteria and validation commands.
- Keep the user informed in Japanese with concise progress updates.
- If a requested direction risks long-term maintainability, say so clearly and propose the safer path.

## Product Principles

- Preserve the local-first promise: no media upload, no default telemetry.
- Never add `ffmpeg`, `ffprobe`, shell execution, or `PATH` lookup to the media path.
- Never move or delete originals unless conversion success, output verification, and user policy all permit it.
- Do not overclaim hardware acceleration or metadata removal unless the implementation verifies it.
- Keep the UI small, calm, and focused on dropping recordings and getting smaller MP4 files.

## Code Size Rules

- Avoid large files. Do not let files drift toward 1000 lines.
- Rust production files must stay at or below 128 lines.
- Rust test files may exceed 1000 lines when the extra length is justified by fixtures or exhaustive cases.
- TypeScript and TSX files must stay at or below 512 lines.
- When a file would exceed its limit, split by responsibility before adding more code.
- Prefer small modules with clear ownership over broad utility files.
- Do not hide complexity by creating vague helpers, catch-all modules, or arbitrary constants.

## Architecture Rules

- Keep business logic in `crates/*`, not in React or Tauri command glue.
- Keep Tauri commands as typed adapters around core services.
- Keep React components focused on rendering and user interaction.
- Put platform-specific behavior behind platform crates or backend modules.
- Add tests near the behavior they protect, while respecting production file size limits.
- Each new feature should have a deterministic validation path: unit test, integration test, or documented manual check.
