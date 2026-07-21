# Product

DropSquash is a small local Drop Zone for turning large screen recordings into
shareable MP4 files on macOS today. Windows and Linux support is planned after
the macOS product path is hardened and verified.

The initial market path is Japan first, with bilingual support:

- Japanese as the primary product language
- English as the secondary support language

For the current paid beta, keep the app and public surface aligned to `JA / EN`
instead of expanding into broader localization early.

v1 does not include cloud upload, timeline editing, screen recording,
automatic privacy masking, or built-in AI transcription.

That said, built-in recording is now an explicitly preserved future direction,
not random scope drift. It remains outside the current paid beta because it
changes the product boundary from post-processing to recording.

In plain terms: the future idea is not "support more conversions." The future
idea is "let DropSquash do the screen recording itself, then optionally shrink
it locally." That deserves its own product validation and QA path.

The product promise is a native OS media pipeline, local-only processing, and no external media executable. It is not a claim that Rust itself makes video codecs faster.

The product direction for evidence workflows is documented separately in
`docs/evidence-core-architecture.md`: deterministic evidence naming, local
semantic hints, safe derivative segment extraction, evidence manifests, and
verification on top of the same local-first media path.

Future capture-plus-masking exploration is documented in
`docs/future-capture-masking.md` and remains outside the current paid beta.
