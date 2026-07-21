# Secure Share Fixtures

These fixtures are synthetic local QA targets for the high-assurance Secure
Share R&D track.

Rules:

- use only synthetic private data
- keep raw media out of git unless it is tiny and generated
- store expected sensitive regions as machine-readable annotations
- never store recognized OCR text as evidence

`ja-en-browser-form.html` is the first adversarial browser target. Its JSON
sidecar is selector-based today; a later harness should resolve those selectors
into canonical frame coordinates before comparing against `MaskPlan` output.

`NativeAccessibilityFixture.swift` is the first native Japanese/English target.
Its annotation file defines the expected result for every runner scenario:
successful cases may publish only independently verified Strict Shield black
output, while fail-closed cases may not publish an MP4, sidecar, or partial.
Its title-toggle button creates a deterministic title-only change for an
observation fixture. The current native path deliberately does not retain or
compare titles, so title-only continuity remains detected-but-not-covered.
Set `DROP_SQUASH_FIXTURE_AUTORUN_FOREGROUND_SWITCH=1` to bring Finder forward
during a production-recording QA run. The recording must fail closed; this is
a fixture-only test trigger, not DropSquash product behavior.
Set `DROP_SQUASH_FIXTURE_AUTORUN_SAME_APP_WINDOW_FOCUS=1` to open a second
window in the same fixture process and focus its text field. The selected
window does not move, so this verifies that focused-input geometry is tied to
the selected AX window rather than merely to the owning application PID.
It includes an AppKit sheet with synthetic text to exercise a native modal
state. Its annotation records only synthetic sensitive-label categories, never
recognized text or user data. The packaged-app Strict Shield path has been
exercised against its sheet; that does not establish selective masking. The
same fixture includes a transient AppKit popover, also exercised by Strict
Shield; neither result establishes selective masking. Its editable field makes
native Japanese IME candidate testing possible; one local configuration has
been exercised by Strict Shield, not every input method.
Its rapid-burst button changes synthetic name, code, token, and file-path text
every 50ms. It is a stress fixture for short-lived states, not proof that each
state is separately observed by macOS or Vision.
Its rapid-input button updates an AppKit editable field every 50ms while that
field owns focus. It is a deterministic typed-input stress fixture, not an IME
composition simulator and not evidence for private password-manager surfaces.
Its hide-window autorun removes the selected window from the screen during
capture. This is a deterministic continuity fixture: a high-assurance session
must discard rather than publish when the target can no longer be revalidated.
Its resize-window autorun changes the selected window geometry during capture.
Its move-window autorun changes only the selected window position. Its
move-return autorun moves the window for 100ms and restores it, specifically to
test the boundary that sampled CoreGraphics attestation cannot prove.
This must also discard rather than emit video with a stale coordinate transform.
Its custom-drawn canvas token is intentionally not an AppKit text element. AX
may still expose a coarse parent client-area rectangle, so it is not claimed as
Vision-only; Strict Shield must remain independent of that structural detail.

`transient-input-overlay.html` exercises synthetic IME-candidate, autofill,
paste-notice, spelling-popover, and confirmation-dialog overlays. It is not a
substitute for the real macOS IME or a browser's private autofill UI. It gives
the capture pipeline a deterministic short-lived overlay sequence while those
platform-specific paths remain under separate investigation.
