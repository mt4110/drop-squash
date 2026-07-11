# Native Media Backends

DropSquash uses a backend-neutral Rust contract and one implementation per operating system.

| Platform | Asset pipeline | Hardware path | Phase 0 status |
|---|---|---|---|
| macOS | AVFoundation | VideoToolbox | capability stub |
| Windows | Media Foundation | hardware Media Foundation Transform | capability stub |
| Linux | allowlisted GStreamer through `gstreamer-rs` | approved hardware encoder element | capability stub |

## Product Invariants

- No external media executable is executed, linked, downloaded, or bundled.
- No shell or `PATH` lookup exists in the media path.
- H.264 in MP4 is the cross-platform sharing baseline.
- Optional formats appear only after runtime capability probing.
- Hardware acceleration is preferred and its absence is visible to the user.
- There is no silent software fallback or runtime plugin download.

The Linux implementation is deliberately stricter because GStreamer can discover dynamic plugins. Production packaging must set an app-owned registry and plugin path, enumerate every permitted element, reject `gst-libav`, and include a license/security review for each shipped plugin.

## Why Not Pure-Rust Codecs

Rust materially improves the safety and testability of orchestration, but it does not create portable hardware encoding by itself. Reimplementing H.264/HEVC would increase compatibility, performance, patent, and maintenance risk. DropSquash therefore wraps OS media facilities behind a small reviewed boundary.

Primary references:

- [Apple VideoToolbox](https://developer.apple.com/documentation/videotoolbox)
- [Microsoft Media Foundation hardware transforms](https://learn.microsoft.com/windows/win32/medfound/hardware-mfts)
- [GStreamer Rust bindings](https://gstreamer.freedesktop.org/documentation/rust/)
