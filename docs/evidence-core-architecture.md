# Evidence Core Architecture

DropSquash evidence mode is a derivative-safe local workflow for turning raw
recorded videos into smaller, verifiable evidence artifacts without breaking
the product's media-path rules.

This document is the formal plan for:

- deterministic evidence naming
- local semantic naming hints
- safe segment extraction
- evidence manifests and verification
- throughput and capacity modeling on macOS
- the practical macOS-native media stack boundary

It is not the plan for the later enterprise masking product boundary.

## Scope

This plan is for evidence-video processing, not general video editing.
It also does not define destructive masking, signed audit receipts, or
enterprise fail-closed export policy.

The target job is:

```text
accept recorded evidence video
  -> derive explicit evidence metadata
  -> optionally derive local semantic hints
  -> compress or extract a derivative
  -> verify the derivative
  -> emit sidecars
  -> keep the original safe
```

## Boundary Against Enterprise Masking

Evidence mode and enterprise audit-ready masking are related, but they are not
the same track.

Evidence mode owns:

- deterministic derivative creation
- explicit verification
- evidence sidecars and manifests
- source/output hashes
- safe naming and range recording

Enterprise audit-ready masking owns:

- destructive pixel overwrite before encode
- required audit identity metadata
- signed receipt generation
- strict fail-closed export for enterprise proof

If enterprise masking is implemented later, it should reuse the evidence-core
spine rather than replace it. The enterprise track should add stronger policy
and stronger proof on top of the evidence-core derivative workflow.

## Product Invariants

- No `ffmpeg`, `ffprobe`, shell execution, or `PATH` lookup in the media path.
- No media upload, no telemetry by default, and no network capability in the
  worker that handles media.
- No source movement or deletion before verified output exists and product
  policy explicitly allows it.
- No silent fallback to a different trust model.
- No semantic rename that invents certainty it does not have.
- No broad codec/container claim without runtime capability proof.

## Current Backbone

The repository already has the right backbone for this work:

- thin app shells
- Rust crate ownership for business logic
- `EncoderBackend` as a narrow backend contract
- verification after encode
- history counting only successful smaller outputs
- source postprocess separated from encoding

The evidence architecture should extend that spine rather than replace it.

## Executive Decision

DropSquash should keep a macOS-native evidence core.
It should not pivot its macOS core media path around GStreamer.

The preferred stack is:

```text
Rust orchestration and policy
  -> AVAssetReader / AVAssetWriter
  -> CoreMedia timing primitives
  -> VideoToolbox VTCompressionSession
  -> MP4 finalization and verification
```

`AVAssetExportSession` remains an allowed whole-file compatibility path for the
current shipping workflow, but new evidence-only workflows should target the
lower-level Apple-native session path.

## Why This Stack

### AVAssetExportSession

Good for:

- whole-file export MVP
- smaller implementation surface
- current paid-beta hardening path

Weak for:

- sample-aware verification
- explicit capability reporting
- segment provenance
- future analyzer insertion points

### AVAssetReader / AVAssetWriter

Good for:

- explicit track handling
- sample/time visibility
- deterministic segment extraction
- clean insertion points for local analyzers

This is the recommended ingest/egress foundation for evidence mode.

### CoreMedia

Use CoreMedia as the internal time truth:

- source duration
- requested range
- actual encoded range
- sample boundary normalization
- verification tolerances

Raw CoreMedia complexity should stay inside Rust backend modules, not leak into
Tauri commands or React state.

### VideoToolbox / `VTCompressionSession`

This is the right practical encoder depth for macOS evidence mode:

- explicit session control
- capability probing
- Apple-native hardware-assisted encode path when available
- better fit for benchmarkable, deterministic transcode behavior

### GStreamer

GStreamer is valuable as a future optional adapter, especially on Linux, but it
is not the cleanest macOS core for this product.

Reasons:

- dynamic plugin discovery
- larger audit and packaging surface
- registry and plugin-path management burden
- weaker fit with the product promise of a small, deterministic local utility

If GStreamer is used later, it must stay behind an adapter boundary with:

- explicit plugin allowlist
- app-owned registry and plugin path
- `gst-libav` rejection
- no external media fallback

## Native Core Versus Optional Adapters

### Belongs in the macOS-native core

- source probe for MOV/MP4/M4V timing and track facts
- deterministic whole-file transcode
- deterministic evidence segment extraction
- explicit capability probing
- local frame sampling for semantic hints
- local transcript or speech hints if implemented later
- evidence manifest generation
- derivative verification

### Belongs behind optional adapters

- GStreamer backends
- Windows Media Foundation backend
- future Linux backend
- platform-specific analyzers that are not needed for the base evidence flow

### Does not belong in this core plan

- timeline editing
- YouTube downloading
- ad SDK integration
- cloud inference
- broad codec-zoo expansion before validated demand exists

## Evidence Naming

### Rule

Naming must be deterministic first and semantic second.

The default filename must come from explicit fields, not guessed content.
Content-derived hints may help, but they must not silently become truth.

### Tier 0: deterministic naming

Primary fields:

- project
- test case id
- suite or feature
- device
- OS
- app/build
- result
- recorded timestamp
- original stem

Example:

```text
ECAPP_TC1042_iPhone17Pro_iOS27_failed_20260716-083012.mp4
```

### Tier 1: local semantic hints

Optional local-only hint sources:

- OCR on sampled frames
- local transcript keywords
- visible error-banner text
- visible page or screen title hints

Each hint must carry:

- analyzer type
- value
- confidence
- provenance such as frame time or transcript window

### Tier 2: reviewed semantic alias

Only reviewed or explicitly accepted hints may change the final filename.

That keeps evidence naming useful without turning it into opaque AI fiction.

## Safe Segment Extraction

### Rule

Segment extraction creates a derivative.
It never replaces the original evidence.

### Preferred first implementation

Ship bounded re-encode first:

- normalize requested source-relative range
- read the relevant source samples
- re-encode into a fresh MP4 derivative
- verify before rename
- emit manifest with requested and actual ranges

Do not prioritize passthrough clipping early.
It is attractive for speed, but it is harder to keep deterministic and easier
to make misleading around exact boundaries.

### Range policy

Support explicit boundary policies:

- `exact-if-possible`
- `snap-to-sample`
- `snap-to-keyframe-safe`

Default evidence mode policy:

- `snap-to-sample`

That is the best balance between honesty and practicality.

## Evidence Verification

Verification must prove more than file existence.

Required verification layers:

### Filesystem

- output exists
- output is non-zero
- final path matches plan
- temp artifacts are gone

### Container

- MP4 or MOV structure is readable
- duration metadata is readable when expected
- at least one video track exists

### Transform

Whole-file conversion:

- output is smaller when the workflow promises compression
- duration stays within tolerance

Segment extraction:

- output duration matches the actual encoded range within tolerance
- manifest preserves source-relative range

### Policy

- source action is still allowed
- history and trial counting happen only after success
- semantic hints do not override deterministic evidence fields silently

## Evidence Sidecars

The existing privacy receipt remains intentionally narrow.
Evidence mode adds a richer manifest alongside it.

Recommended layout:

```text
<output>.mp4
<output>.privacy.json
<output>.evidence.json
```

The evidence manifest should record:

- source file name
- source hash
- source bytes and duration
- output file name
- output hash
- output bytes and duration
- derivative kind
- backend identity
- transform mode
- requested and actual ranges when relevant
- naming policy and accepted hints
- verification results

Manifest persistence should use the same temp-file-plus-rename discipline used
elsewhere in the repository.

## Throughput And Capacity Modeling

Use the same native backend path as the product for benchmarks.

Track these metrics consistently:

- `elapsed_s`
- `duration_s`
- `speed_ratio`
- `throughput_mib_s`
- `saved_percent`
- `temp_peak_bytes`

Default concurrency for evidence encoding on macOS should remain:

- one active encode job

Parallelize only cheap or safe side work such as:

- file stability waiting
- metadata probe
- bounded hash computation
- bounded semantic frame sampling

Do not default to multiple simultaneous encode jobs. Queue determinism and app
responsiveness matter more than headline throughput.

Recommended practical capacity tiers:

- single-file interactive
- small-batch evidence
- overnight backfill

## Implementation Ownership

Keep business logic in crates and split by responsibility.

Recommended ownership:

- `dropsquash-core`
  - evidence DTOs and plans
- `dropsquash-media`
  - probe, ranges, hashes, lightweight media facts
- `dropsquash-encoder`
  - whole-file encode, segment extract, backend capability probes
- `dropsquash-privacy`
  - privacy receipts and possibly shared sidecar persistence helpers
- `dropsquash-history`
  - successful evidence operation summaries
- `dropsquash-postprocess`
  - unchanged source safety gates

If the evidence domain grows too large, add narrow crates rather than inflating
existing files past the repository limits.

## Recommended Phase Order

### Phase A

- evidence manifest DTOs
- verification report DTOs
- derivative kind and segment plan types

### Phase B

- whole-file evidence sidecar emission
- source/output hashes
- verification persistence

### Phase C

- deterministic naming engine
- user-supplied evidence fields

### Phase D

- bounded segment extraction
- requested versus actual range recording

### Phase E

- lower-level Apple-native session path
- benchmark comparison against export-session path

### Phase F

- optional local semantic analyzers
- review-before-rename flow

## Planning Text For Product Decisions

Use these planning decisions consistently across docs and implementation:

```text
DropSquash evidence mode uses a macOS-native media core based on
AVAssetReader/AVAssetWriter, CoreMedia timing primitives, and
VideoToolbox VTCompressionSession.

AVAssetExportSession remains an allowed whole-file compatibility path, but new
evidence-only workflows target the lower-level Apple-native session path.

Semantic naming is deterministic first and analyzer-assisted second.
Content-derived text is stored as a local semantic hint with confidence and
provenance and does not silently become the filename without review.

Segment extraction always creates a derivative, never replaces the source, and
records requested and actual time ranges plus source/output hashes in an
evidence manifest.

GStreamer is not the macOS-native core. If used later, it belongs behind an
optional adapter boundary with an explicit plugin allowlist and no external
media fallback.
```

## Related Documents

- `docs/architecture.md`
- `docs/native-backends.md`
- `docs/product.md`
- `docs/productization.md`
- `docs/qa-evidence.md`
- `docs/benchmarking.md`
- `docs/privacy.md`
- `docs/enterprise-audit-masking.md`
