# DropSquash

Drop huge screen recordings. Squash them locally.

DropSquash turns large screen recordings into small, shareable MP4 files on macOS, Windows, and Linux without uploading videos to the cloud.

## Status

macOS MVP. The desktop app can convert user-selected `.mov`, `.mp4`, and `.m4v` recordings to numbered `.squashed.mp4` outputs through Apple's native AVFoundation export pipeline. The shared Rust workspace also contains the core domain types, deterministic safety logic, JSONL history, local trial counting, and backend contracts for Windows and Linux.

Windows Media Foundation, Linux GStreamer, cancellation, multi-file queueing, license activation, signing, notarization, and release packaging are still planned work.

## Principles

- Local-first
- Native hardware acceleration
- No external media executables
- No cloud upload
- Safe original handling
- Screen-recording aware compression
- Small UI
- Scriptable core

## Local Commands

Use the pinned Nix shell when you want the repository's Node and pnpm versions:

```bash
nix develop
node -v
pnpm -v
```

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
pnpm --dir apps/desktop/web build
```

On macOS, the CLI uses the native encoder. On Windows and Linux, the backend contracts are present but the encoders still return a clear not-implemented error:

```bash
cargo run -p dropsquash -- convert ./demo.mov --output-dir ./out --profile auto
```

The production backend matrix is:

| Platform | Probe / pipeline | Encoder |
|---|---|---|
| macOS | AVFoundation | VideoToolbox |
| Windows | Media Foundation | Hardware MFT |
| Linux | allowlisted GStreamer via `gstreamer-rs` | available hardware element |

Runtime capability detection is authoritative. DropSquash does not silently switch to an unreviewed or unexpectedly slow codec.

## Reproducible Development

Nix is optional and is never shipped with the application:

```bash
nix develop
cargo test --workspace
```

The Nix shell supports Apple Silicon macOS and Linux development and pins Node 24 with pnpm 10 for the desktop web build. Native Windows builds and media tests run on Windows.

## Architecture

Business logic lives in `crates/*`. UI and command surfaces in `apps/*` are intentionally thin.

See [native backends](docs/native-backends.md) and the [security model](SECURITY.md) before changing the media pipeline.
