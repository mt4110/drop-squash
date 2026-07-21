# Native Media Backends

DropSquash uses a backend-neutral Rust contract and one implementation per operating system.

| Platform | Asset pipeline | Hardware path | Current status |
|---|---|---|---|
| macOS | AVFoundation export session | Apple native media pipeline | MVP conversion implemented |
| Windows | Media Foundation | hardware Media Foundation Transform | not implemented |
| Linux | allowlisted GStreamer through `gstreamer-rs` | approved hardware encoder element | not implemented |

The current macOS path uses AVFoundation's export session and reports H.264 MP4 support for `.mov`, `.mp4`, and `.m4v` inputs. It should not be described as a verified explicit VideoToolbox hardware encoder path until runtime capability reporting proves that path.

For evidence mode, the planned macOS-native core is deeper than the current
export-session path: `AVAssetReader/AVAssetWriter + CoreMedia +
VTCompressionSession`. That path is the target for deterministic segment
extraction, richer verification, and analyzer insertion points. See
`docs/evidence-core-architecture.md`.

## Product Invariants

- No external media executable is executed, linked, downloaded, or bundled.
- No shell or `PATH` lookup exists in the media path.
- H.264 in MP4 is the cross-platform sharing baseline.
- Optional formats appear only after runtime capability probing.
- Hardware acceleration is preferred and its absence is visible to the user.
- There is no silent software fallback or runtime plugin download.

The Linux implementation is deliberately stricter because GStreamer can discover dynamic plugins. Production packaging must set an app-owned registry and plugin path, enumerate every permitted element, reject `gst-libav`, and include a license/security review for each shipped plugin.

GStreamer is therefore an optional adapter strategy, not the default macOS
core. On macOS, DropSquash should prefer Apple-native primitives before adding
cross-platform abstraction cost.

## Why Not Pure-Rust Codecs

Rust materially improves the safety and testability of orchestration, but it does not create portable hardware encoding by itself. Reimplementing H.264/HEVC would increase compatibility, performance, patent, and maintenance risk. DropSquash therefore wraps OS media facilities behind a small reviewed boundary.

Primary references:

- [Apple VideoToolbox](https://developer.apple.com/documentation/videotoolbox)
- [Microsoft Media Foundation hardware transforms](https://learn.microsoft.com/windows/win32/medfound/hardware-mfts)
- [GStreamer Rust bindings](https://gstreamer.freedesktop.org/documentation/rust/)
