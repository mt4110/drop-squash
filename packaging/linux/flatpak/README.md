# Flatpak Packaging

Flatpak is the primary Linux desktop packaging target because it gives DropSquash a defined runtime and permission boundary.

The production manifest must expose only user-selected media paths, deny network access to the media worker, and ship an explicit GStreamer plugin allowlist. `gst-libav` is not permitted.
