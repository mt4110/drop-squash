# Troubleshooting

Use `dropsquash doctor` to see which backend is selected for the current OS and what capabilities it reports.

On macOS, DropSquash currently converts through Apple's native AVFoundation export pipeline. If conversion fails, keep the original recording and check that the input is a `.mov`, `.mp4`, or `.m4v` file and that the selected output directory is writable.

On Windows and Linux, native encoder implementations are still pending. The commands should fail with `native-encoder-unavailable` rather than starting an external media executable or silently falling back to an unreviewed codec.

The application does not search for or start external media executables. Installing one does not change backend availability.

On Linux, codec availability depends on the approved GStreamer runtime and hardware driver. DropSquash reports a missing capability instead of loading plugins from arbitrary user paths.
