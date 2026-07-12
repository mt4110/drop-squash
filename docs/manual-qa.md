# Manual QA

Manual QA records real packaged-app behavior that automated tests cannot prove.
Do not mark a productization phase complete from this file until the result and
environment are filled in.

Automated coverage that supports this checklist is tracked separately in
`docs/qa-evidence.md`. Do not copy automated pass results into this file unless
the row explicitly asks for a command result.

Run this gate after filling every result:

```sh
cargo run -p xtask -- manual-qa-check
```

For manual observation rows, do not write only `Pass`, `OK`, `Done`, `Works`,
`Verified`, or `Observed expected behavior` in the result. Record the concrete
evidence you saw, such as the generated file name, trial count, Finder
selection, or cache state.
Do not use placeholders such as `TBD`, `N/A`, `None`, `Blocked`, or `Skipped`
as field values or results; leave unfinished rows empty until real evidence is
available.
`App artifact` must point to the existing local artifact used for the run: a
`DropSquash.app` bundle directory or a UDIF `DropSquash.dmg` file. `Date` must
use a real `YYYY-MM-DD` calendar date in year 2000 or later.
`App build` must include both the tested app version and the git commit, for
example `DropSquash 0.1.0 git abc1234`.
`macOS version` must look like `macOS 15.5`, `Machine` must include the CPU
architecture, and `Output folder` must point to an existing absolute directory.
`Input sample set` must mention the short, medium, and large local recordings
used for the packaged-app run.
State path fields must point to the DropSquash app support files shown in the
table. `Tester` must name the tester, not a generic placeholder.

Before starting packaged-app QA, preserve the current local app state instead
of deleting it:

```sh
cargo run -p xtask -- manual-qa-prepare
```

This creates `/tmp/dropsquash-qa-state`, copies any existing config, history,
and license cache there, and creates `/tmp/dropsquash-manual-qa-output` for the
run. It fails if required environment fields cannot be detected, so missing
manual QA metadata cannot be mistaken for a prepared run. Copy the printed
`manual QA App build` value into the `App build` field.
Copy the printed `manual QA App artifact` value into the `App artifact` field,
or pass `--app-artifact <path>` when testing a DMG or a non-default artifact
location.
The `App artifact` value must be the printed absolute path.
Pass `--input-sample-set <text>` and copy the printed value into the `Input
sample set` field.
Copy the printed macOS version, Machine, Output folder, and Date values into
the matching fields before starting observations. Copy the printed Config path,
History path, and License cache path values into the matching state path fields.
Copy the printed Tester value into the matching field.
The command also prints `manual QA Markdown fields:` followed by table rows
that can be pasted into the macOS Packaged App field table.
When the selected app artifact is `DropSquash.dmg`, it also prints
`manual QA Release Candidate rows:` for the artifact-check and checksum rows.
Pass `--markdown-output /tmp/dropsquash-manual-qa-prepared.md` to write those
generated field and release-candidate rows to a temporary Markdown file for
copying. Keep that file outside the repository; it is a preparation aid, not a
substitute for concrete manual observations.
If you override `--app-state-dir`, `--state-dir`, or `--output-dir`, keep those
paths outside the repository so private app state, generated videos, and QA
evidence cannot be committed or deleted by accident.

Then start from a known trial state if the run is meant to verify trial counts.
The reset command requires the sample-set description so the run cannot begin
without naming the short, medium, and large recordings. It also requires an
existing `DropSquash.app` or `DropSquash.dmg` artifact, either at the default
packaged-app path or through `--app-artifact <path>`:

```sh
cargo run -p xtask -- manual-qa-prepare --reset-trial --app-artifact target/release/bundle/dmg/DropSquash.dmg --input-sample-set "short, medium, and large local recordings"
```

`--reset-trial` removes only `history.jsonl` and `license.json`, and only after
copying any existing state into the backup directory. The command prints the
app state source and reset path; confirm they point to the DropSquash app
support directory before starting observations.

After QA, restore the backed up local state when needed:

```sh
cargo run -p xtask -- manual-qa-prepare --restore-state
```

`--restore-state` copies only backed up `config.json`, `history.jsonl`, and
`license.json` files back into the app state directory. It fails if the backup
directory does not exist or contains no restorable state files, so an empty
restore cannot be mistaken for success.

## macOS Packaged App

| Field | Value |
|---|---|
| App build |  |
| App artifact |  |
| macOS version |  |
| Machine |  |
| Input sample set |  |
| Output folder |  |
| Config path |  |
| History path |  |
| License cache path |  |
| Tester |  |
| Date |  |

Packaged-app results must include the concrete thing observed, not only that
the row passed. Use output file names such as `.squashed.mp4`, smaller-output
observations, Finder selection targets, queue counts, trial/history observations,
or Trash/source state as appropriate for the row.
For receipt and reveal rows, record the exact privacy values and Finder
selection state: `uploaded_bytes = 0`, `metadata_policy = preserve`, file names
instead of absolute paths, and `selected`. For duplicate output naming, record
that the second output used a numbered file name such as `.squashed-2.mp4`.
For queue rows, record concrete counts such as `3 recordings`, `1 active`,
finished count, saved bytes, and failed/cancelled/blocked counts. For failed
conversion rows, record the friendly error plus the unchanged original and
trial count.

| Check | Input | Expected | Result |
|---|---|---|---|
| Choose recording conversion | Small `.mov` screen recording | Creates smaller `.squashed.mp4`; original remains |  |
| Drag-and-drop conversion | Small `.mov` screen recording | Creates smaller `.squashed.mp4`; original remains |  |
| Privacy receipt sidecar | Successful conversion | Creates matching `.privacy.json` with file names, `uploaded_bytes = 0`, and `metadata_policy = preserve` |  |
| Reveal privacy receipt | Successful conversion with receipts enabled | Finder opens with generated `.privacy.json` selected |  |
| Duplicate output naming | Same recording twice | Second output uses `.squashed-2.mp4` style numbered suffix |  |
| Cancellation | Large recording | App returns to ready; no success history; no trial count |  |
| Multi-file queue | Three recordings | 3 recordings queue with 1 active sequential conversion; unrelated failures do not block finished jobs |  |
| Queued job cancellation | Three recordings | Cancelling a waiting row marks it cancelled, it never starts, and trial/history shows no new success |  |
| Batch summary | Three recordings with at least one mixed outcome | Queue summary shows numeric finished count, saved bytes, failed count, cancelled count, and blocked count |  |
| Ask source policy | Successful conversion | User can choose Trash or Keep while original remains unchanged |  |
| Trash source policy | Successful conversion | Trash button shows moving/disabled state; original moves to Trash only after verified smaller output |  |
| Failed conversion | Unsupported or intentionally bad input | Friendly error appears; original remains; trial count unchanged |  |
| Larger output | Input that cannot be made smaller | Larger result is treated as failure; original remains; trial count unchanged |  |
| Reveal output | Completed output link | Finder opens with generated `.squashed.mp4` selected |  |

## License Sandbox

Record concrete results. The checker requires:

- Sandbox product setup: mention DropSquash, the intended product, and license keys enabled.
- Sandbox purchase: mention the intended product, test buyer, and order.
- Empty key activation: mention the disabled Activate state and that `license.json` or the license cache has no raw key.
- Invalid key activation: mention the Activating/disabled state, a friendly error, and that `license.json` or the license cache has no raw key.
- Valid sandbox activation: mention the Activating/disabled state, Pro state, and that `license.json` or the license cache has no raw key.
- License network failure: mention a friendly network error, preserved existing valid cache, and no raw key in `license.json` or the license cache.
- Forget license on this Mac: mention the Forgetting/disabled state, cache removal, and the resulting app state.

Inspect the license cache without recording the sandbox key itself. A good
result says the cache path was checked, that the raw key was absent, and whether
only the fingerprint/instance fields were present.

| Check | Expected | Result |
|---|---|---|
| Sandbox product setup | DropSquash sandbox product exists with license keys enabled |  |
| Sandbox purchase | Checkout completes for the intended product and test buyer order |  |
| Empty key activation | Empty key leaves Activate disabled; no raw key persisted |  |
| Invalid key activation | Activating state disables submit; friendly license error; no raw key persisted |  |
| Valid sandbox activation | Activating state disables submit; Pro state; raw key absent from cache |  |
| License network failure | Friendly network error; existing valid cache remains intact |  |
| Forget license on this Mac | Forgetting state disables action; local cache clears; app returns to trial or locked state |  |

## Release Candidate

Release candidate results must name the artifact or command evidence. Record
the `DropSquash.dmg` path/name, SHA-256 line, Developer ID codesign result,
notary/staple assessment, and Gatekeeper clean/fresh-machine observation where
the row asks for them. When `App artifact` is a `.dmg`, artifact-check,
checksum, codesign, and notary/staple result rows must name the same `.dmg`
file. Artifact-check and checksum rows must include the `App artifact` absolute
path.

| Check | Expected | Result |
|---|---|---|
| `cargo run -p xtask -- release-check` | Passes |  |
| `cargo run -p xtask -- file-size-check` | Passes |  |
| `cargo run -p xtask -- media-policy-check` | Passes |  |
| `cargo run -p xtask -- privacy-policy-check` | Passes |  |
| `cargo run -p xtask -- website-check` | Passes |  |
| `cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp>` | `.csv` path recorded outside repo for three local samples; outputs are smaller |  |
| Benchmark sample set | Short, medium, and large private local recordings produce smaller outputs and are recorded with machine and OS context |  |
| Benchmark regression threshold | Throughput does not regress by more than 20% on two or more samples without a documented reason |  |
| `cargo run -p xtask -- manual-qa-check` | Passes after every manual QA result is recorded |  |
| `cargo run -p xtask -- artifact-check path/to/DropSquash.dmg` | Passes |  |
| `cargo run -p xtask -- checksum path/to/DropSquash.dmg` | SHA-256 line with 64-character digest and `DropSquash.dmg` recorded |  |
| `cargo run -p xtask -- macos-signing-check` | Passes in release environment |  |
| Codesign verification | Public DMG/app artifact verifies with Developer ID signature |  |
| Notarization staple verification | Public DMG/app artifact passes notary/staple assessment |  |
| Gatekeeper open test | Signed, notarized, stapled app opens cleanly without Gatekeeper warning |  |
