# Privacy

DropSquash is local-first.

Phase 0 has no upload code and no telemetry. Successful conversions write a
local sidecar receipt next to the output as
`<output-name>.privacy.json`. Privacy receipts record `uploaded_bytes = 0` and
`metadata_policy = preserve`; they do not claim metadata stripping.
The desktop app enables these local receipts by default and lets the user turn
them off in settings.

The media worker has no network capability. License validation is isolated from
the media path and never receives media contents or media-derived metadata. The
privacy gate rejects network clients outside the license provider.
