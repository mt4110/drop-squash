# Build Week Demo Script

Target length: 45 to 60 seconds.

## Recording Beats

1. Open `tests/fixtures/secure-share/transient-input-overlay.html` in a dedicated Chrome window.
2. Show its synthetic Japanese/English form fields before recording.
3. In the signed DropSquash app, choose the Chrome window.
4. Start recording with `Start blackening verification`.
5. Return to the fixture, press `Trigger transient states`, then return to DropSquash.
6. Stop with `Stop and save verification`.
7. Show `Blackened verification video saved`, then reveal the all-black MP4 and MaskPlan sidecar.
8. Show the independent local verifier passing for that exact pair.
9. State the boundary plainly: Phase 5 Strict Shield alpha, not a leak-zero claim.

## Voiceover

```text
DropSquash is a local Mac tool for making screen recordings smaller without
uploading media. For Build Week, I added a Secure Share R&D alpha for QA teams.

This synthetic fixture starts as a Japanese and English input form, then emits
short-lived candidate, autofill, paste, spelling, and confirmation overlays. I
choose its window, start recording, trigger those transient states, and stop
when the state is ready to share. The packaged app observes that selected window
locally with ScreenCaptureKit, Accessibility, focused-input geometry, and Apple
Vision. A focused input from another window in the same app stops the recording.
Strict Shield does not trust detection to preserve pixels: it destroys every
captured frame and reads back black pixels. The MaskPlan stores only the
canonical full-frame region and aggregate observation counts. Recognized private
text and observation geometry are not saved.

The recording is finalized only after the MP4 is independently decoded and each
planned region is checked as black. Its sidecar binds the output SHA-256 to a
locally verified Ed25519 signature. The local verification command checks the
binding and decoded output again without network access. This is not sold as
leak-zero or enterprise audit-ready. The value is a native local path with
explicit evidence before any public security claim.
```

## Demo Prep

```bash
"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" --new-window \
  "file://$PWD/tests/fixtures/secure-share/transient-input-overlay.html"
open target/release/bundle/macos/DropSquash.app
```

In the app, choose the fixture window, record it, and choose **Stop and save
verification**. After the app shows its saved state, verify the exact saved
pair on camera:

```bash
cargo run -p xtask -- secure-share-evidence-check \
  /path/to/secure-share-recording-<id>.mp4 \
  /path/to/secure-share-recording-<id>.mask-plan.json
```
