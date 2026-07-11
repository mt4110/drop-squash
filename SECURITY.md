# Security

DropSquash is local-first and does not upload media by default.

Media files, metadata, file names, and container structures are untrusted input. The media pipeline follows these rules:

- No external media executable, shell, or `PATH` lookup.
- Native backend capability is checked at runtime.
- FFI and `unsafe` code stay inside reviewed platform modules.
- Linux loads only the GStreamer elements shipped and approved for DropSquash; `gst-libav` is prohibited.
- Output is written to a temporary path, probed, size-checked, and atomically renamed before an original can be moved.
- A failed, missing, empty, or larger output never permits original-file movement.
- The Tauri WebView receives only narrow, typed commands and no shell permission.

The production encoder will run outside the WebView/UI process with no network capability and access limited to the user-selected input and output location. The current macOS MVP performs native local conversion in-process; worker isolation remains a release hardening requirement.

Avoid describing the implementation as secure merely because it is written in Rust. OS media frameworks, drivers, FFI, and Linux plugins remain native attack surfaces and must be patched, isolated, and tested.

Do not commit API secrets, signing keys, license keys, private certificates, or updater private keys to this repository.

Report security issues privately to the repository owner.
