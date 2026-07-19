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
