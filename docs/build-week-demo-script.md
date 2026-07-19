# Build Week Demo Script

Target length: 30 to 60 seconds.

## Recording Beats

1. Show DropSquash compressing a local screen recording.
2. Open
   `tests/fixtures/secure-share/ja-en-browser-form.html`.
3. Point out synthetic Japanese and English sensitive-looking UI text.
4. Run the packaged macOS observation harness.
5. Show the JSON summary:
   `frameCount`, `axObservationCount`, `visionObservationCount`,
   `maskPlanPreview.policy`, first-frame region count,
   `liveMaskedFrameCount`, and `liveVerifiedPixelCount`.
6. Say the boundary plainly: this is R&D alpha, not a leak-zero claim.

## Voiceover

```text
DropSquash is a local Mac tool for making screen recordings smaller without
uploading media. For Build Week, I turned the next product bet into a Secure
Share R&D alpha for QA teams.

This synthetic fixture mixes Japanese customer data, English tokens, filenames,
notifications, and modal text. The packaged app observes the selected browser
window locally with ScreenCaptureKit, Accessibility, and Apple Vision. It then
blackens detected text regions on the same captured frames and reads back black
pixels. The redacted MaskPlan stores only geometry, reasons, sources,
confidence, and timing. Recognized private text is not saved.

This is not sold as leak-zero. The next gates are final-video Strict Reveal
destructive pixel overwrite, independent final-output verification, and
fail-closed output deletion. The value is the architecture: native local
observation first, then deterministic masking evidence before any public
security claim.
```

## Commands For Demo Prep

```bash
open -a "Google Chrome" tests/fixtures/secure-share/ja-en-browser-form.html
cargo run -p xtask -- manual-qa-secure-share-observe list
```

Use the browser window ID from the list:

```bash
env \
  DROP_SQUASH_QA_INSTANCE_ID=build_week_maskplan_preview \
  DROP_SQUASH_QA_SCK_OBSERVE=1 \
  DROP_SQUASH_QA_SCK_OBSERVE_WINDOW_ID=3301 \
  DROP_SQUASH_QA_SCK_OBSERVE_CAPTURE_MS=1000 \
  DROP_SQUASH_QA_SCK_OBSERVE_TIMEOUT_MS=15000 \
  DROP_SQUASH_QA_SCK_OBSERVE_QUIT_AFTER=1 \
  DROP_SQUASH_MANUAL_QA_EVENT_LOG=/tmp/dsq-build-week-maskplan-preview.jsonl \
  target/release/bundle/macos/DropSquash.app/Contents/MacOS/dropsquash-desktop
```
