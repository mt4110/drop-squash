# Privacy

DropSquash is local-first.

Phase 0 has no upload code and no telemetry. Privacy receipts record `uploaded_bytes = 0`.

The media worker has no network capability. License validation is isolated from
the media path and never receives media contents or media-derived metadata. The
privacy gate rejects network clients outside the license provider.
