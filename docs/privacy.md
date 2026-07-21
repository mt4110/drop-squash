# Privacy

DropSquash is local-first.

Phase 0 has no upload code and no telemetry. Successful conversions write a
local sidecar receipt next to the output as
`<output-name>.privacy.json`. Privacy receipts record `uploaded_bytes = 0` and
`metadata_policy = preserve`; they keep file names instead of absolute paths
and do not claim metadata stripping.
The desktop app enables these local receipts by default and lets the user turn
them off in settings.
The CLI can inspect an existing sidecar with
`dropsquash receipt <output.mp4>` without reading or uploading the media file.

The media worker has no network capability. License validation is isolated from
the media path and never receives media contents or media-derived metadata. The
privacy gate rejects network clients outside the license provider, including
browser fetch calls, generic HTTP clients, and Tauri HTTP plugin imports.
The updater is disabled for the first paid beta. If an updater is enabled in a
future release, update checks must be explicit release infrastructure traffic
and must not send media contents, media-derived metadata, or raw license keys.

Evidence mode extends this local-only rule. Any semantic naming analyzer,
segment planner, manifest builder, or verifier must run locally and must not
upload media, transcripts, OCR text, or media-derived metadata. The planned
evidence sidecar model is documented in `docs/evidence-core-architecture.md`.
