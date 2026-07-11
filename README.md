# DropSquash

Drop huge screen recordings. Squash them locally.

DropSquash turns large screen recordings into small, shareable MP4 files on
macOS today without uploading videos to the cloud.
Windows and Linux support is planned after the macOS product path is hardened
and verified.

## Status

macOS MVP. The desktop app can convert user-selected `.mov`, `.mp4`, and `.m4v` recordings to numbered `.squashed.mp4` outputs through Apple's native AVFoundation export pipeline. The shared Rust workspace also contains deterministic safety logic, cancellation, a sequential queue model, safe source postprocessing, JSONL history, local trial counting, license activation plumbing, and backend contracts for Windows and Linux.

Packaged-app manual QA remains for cancellation, multi-file queueing, Trash handling, and live license activation. The release pipeline can build an unsigned macOS `.app` and `.dmg`, run artifact/checksum gates, and block publication until signing is ready. Windows Media Foundation, Linux GStreamer, signing, notarization, and public release publication are still planned work.

## Principles

- Local-first
- Native OS media pipelines
- No external media executables
- No cloud upload
- Safe original handling
- Screen-recording aware compression
- Small UI
- Scriptable core

## Local Commands

Use the pinned Nix shell when you want the repository's Rust, Node, and pnpm
versions:

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
cargo run -p xtask -- file-size-check
cargo run -p xtask -- release-check
```

On macOS, the CLI uses the native encoder. On Windows and Linux, the backend contracts are present but the encoders still return a clear not-implemented error:

```bash
cargo run -p dropsquash -- convert ./demo.mov --output-dir ./out --profile auto
cargo run -p dropsquash -- receipt ./out/demo.squashed.mp4
```

The target backend matrix is:

| Platform | Probe / pipeline | Encoder |
|---|---|---|
| macOS | AVFoundation | AVFoundation export MVP now; lower-level VideoToolbox may follow |
| Windows | Media Foundation | planned Hardware MFT backend; not implemented |
| Linux | allowlisted GStreamer via `gstreamer-rs` | planned hardware element; not implemented |

Runtime capability detection is authoritative. DropSquash does not silently switch to an unreviewed codec or external media executable.

## Reproducible Development

Nix is optional and is never shipped with the application:

```bash
nix develop
cargo test --workspace
```

The Nix shell supports Apple Silicon macOS and Linux development and pins Rust
1.95.0, Node 24.16.0, and pnpm 10.34.0 for the desktop build. Native Windows
builds and media tests run on Windows. Do not add mise or another version
manager unless Nix cannot express a required tool.

## Architecture

Business logic lives in `crates/*`. UI and command surfaces in `apps/*` are intentionally thin.

See [native backends](docs/native-backends.md) and the [security model](SECURITY.md) before changing the media pipeline.
