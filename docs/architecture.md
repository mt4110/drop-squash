# Architecture

DropSquash is a Rust workspace with thin app shells and testable domain crates.

Tauri provides one UI across macOS, Windows, and Linux. Rust owns orchestration and safety policy. Encoding is delegated through `EncoderBackend` to the native platform stack without starting an external media executable.

The intended flow is:

```text
Drop / Watch event
  -> fileguard::wait_until_stable
  -> media::probe
  -> profiles::resolve
  -> queue::enqueue
  -> encoder::encode
  -> verifier::verify_output
  -> history::record
  -> postprocess::decide_source_action
  -> license::record_successful_conversion
  -> platform::notify
```

Original movement is never decided by encoder code.

## Backend Boundary

```text
Tauri command / CLI
  -> queue and policy crates (safe Rust)
  -> EncoderBackend (backend-neutral settings)
  -> platform module (reviewed FFI boundary)
  -> OS hardware media stack
```

| Platform | Backend |
|---|---|
| macOS | AVFoundation export MVP now; lower-level Apple hardware codec path may follow |
| Windows | Media Foundation Source Reader, Sink Writer, and hardware MFTs |
| Linux | GStreamer through `gstreamer-rs`, using an explicit plugin allowlist and excluding `gst-libav` |

`probe_capabilities` reports what the current machine can actually do. Compile-time OS detection is not proof that a codec or hardware path exists.

## Isolation

The release architecture separates media work from the WebView/UI process. The worker protocol is typed and versioned, does not invoke a shell, carries no license secrets, and has no network capability. Platform packaging must restrict the worker to user-selected input and output locations.

## Build Boundary

Nix pins the Apple Silicon macOS/Linux development environment. It is not a runtime dependency and is not placed in installers. Windows native behavior is built and tested on Windows; WSL or cross-compilation does not replace that gate.
