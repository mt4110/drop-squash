# P2 Alpha Multi-Rect Runbook

Use this only for the last remaining P2 alpha packaged-app proof:

```text
prove one packaged-app export with 2 or more secure-share rectangles
```

Date baseline:

```text
Saturday, July 18, 2026
```

## Goal

Close the last weak point between:

- multi-rect code proof
- CLI multi-rect real export proof
- packaged-app one-rect proof

What must be added now is only:

- one packaged-app export
- with 2 or more rectangles
- with receipt confirmation
- with original-safety confirmation

## Artifact

Use this exact local artifact:

```text
/Users/masakitakemura/_workspace/drop-squash/target/release/bundle/macos/DropSquash.app
```

The current packaged build was refreshed by:

```sh
pnpm --dir apps/desktop tauri build
```

## Sample

Recommended sample:

```text
/Users/masakitakemura/Movies/iMovieライブラリ.imovielibrary/マイムービー 1/Original Media/take0.mov
```

Recommended rectangle set for the proof:

```text
rect 1: x=100 y=100 width=900 height=180
rect 2: x=120 y=320 width=700 height=140
mode: solid_black
size: 480p
```

These values already have matching CLI proof in
[docs/p2-alpha-review.md](/Users/masakitakemura/_workspace/drop-squash/docs/p2-alpha-review.md).

## Short Execution Memo

Optional: if you want a machine-readable QA event log during the proof, start
the packaged binary with:

```sh
export DROP_SQUASH_QA_SECURE_SHARE='{"maskMode":"solid_black","maskRects":[{"x":100,"y":100,"width":900,"height":180},{"x":120,"y":320,"width":700,"height":140}]}'
DROP_SQUASH_MANUAL_QA_EVENT_LOG=/tmp/dsq-p2-alpha-multirect-events.jsonl \
'/Users/masakitakemura/_workspace/drop-squash/target/release/bundle/macos/DropSquash.app/Contents/MacOS/dropsquash-desktop'
```

That QA preset makes the packaged app start with secure-share already enabled
and the two proof rectangles already loaded.

Expected secure-share QA events include:

- `secure-share-toggle`
- `secure-share-mode`
- `secure-share-rect-add`
- `secure-share-result`
- `receipt-open`

Otherwise, open the packaged app normally:

```sh
open /Users/masakitakemura/_workspace/drop-squash/target/release/bundle/macos/DropSquash.app
```

1. If you did not use the QA preset, turn on `共有マスク / Secure Share alpha`.
2. If you did not use the QA preset, keep `solid_black`.
3. If you did not use the QA preset, add the second rectangle and enter the
   two rectangle values from the sample above.
4. Choose `take0.mov`.
5. Wait for export to finish.
6. Open the receipt from `共有記録 / Share receipt`.

## What Must Be Observed

The proof is good only if all are true:

1. The packaged app accepts 2 rectangles without clipping or breaking layout.
2. Export finishes successfully.
3. The receipt opens from the result surface.
4. The receipt shows:
   - `mask_mode = solid_black`
   - `mask_rect_count = 2`
5. The output file is a new `.mp4`.
6. The original `.mov` remains unchanged.
7. If the optional QA event log was enabled, it contains the secure-share
   events plus `secure-share-result` and `receipt-open`.

After the export, you can summarize the receipt plus event-log proof with:

```sh
cargo run -p xtask -- manual-qa-secure-share-report \
  /absolute/path/to/take0.squashed.secure-share.json \
  /tmp/dsq-p2-alpha-multirect-events.jsonl
```

This should print:

- `mask_rect_count: 2`
- `secure-share-toggle: true`
- `secure-share-mode: true`
- `secure-share-rect-add: true`
- `secure-share-result: true`
- `receipt-open: true`

## Expected Output Location

If the app is still using the normal default:

```text
~/Movies/DropSquash/
```

Expected files:

```text
take0.squashed.mp4
take0.squashed.secure-share.json
```

## Ready-To-Paste Evidence Block

Copy this after the packaged-app pass succeeds:

```text
Date:
Artifact:
Input:

Secure Share:
- mode: solid_black
- rect count: 2

Result:
- output:
- receipt:
- original safety:

Receipt check:
- mask_mode:
- mask_rect_count:
- output_sha256:

UI check:
- toggle:
- second rect:
- receipt open:

Verdict:
- packaged multi-rect proof recorded
```

## Completion Meaning

If this runbook is completed with real evidence, the remaining P2 alpha gap is
no longer "can multi-rect really export?".

At that point, the remaining work becomes:

- better rectangle authoring feel
- more polished small-window interaction
- broader Phase 2 depth only if we still want it
