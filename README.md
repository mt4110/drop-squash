# DropSquash

Drop huge screen recordings. Squash them locally.

DropSquash turns large screen recordings into small, shareable MP4 files on macOS, Windows, and Linux without uploading videos to the cloud.

## Status

Phase 0 scaffold. The repository contains the Rust workspace, core domain types, deterministic safety logic, JSONL history, trial counting, and placeholder CLI/desktop entry points.

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

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Phase 0 defines the native backend contracts but does not encode video yet. The command exits with a clear backend-not-implemented error and never starts an external media process:

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

The Nix shell supports Apple Silicon macOS and Linux development. Native Windows builds and media tests run on Windows.

## Architecture

Business logic lives in `crates/*`. UI and command surfaces in `apps/*` are intentionally thin.

See [native backends](docs/native-backends.md) and the [security model](SECURITY.md) before changing the media pipeline.
