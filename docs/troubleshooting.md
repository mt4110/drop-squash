# Troubleshooting

Phase 0 contains native backend stubs and does not encode video. Use `dropsquash doctor` to see which backend is selected for the current OS and whether it is implemented.

The application does not search for or start external media executables. Installing one does not change backend availability.

On Linux, codec availability depends on the approved GStreamer runtime and hardware driver. DropSquash reports a missing capability instead of loading plugins from arbitrary user paths.
