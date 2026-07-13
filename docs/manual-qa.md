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
example `DropSquash 0.1.0 git abc1234`, and the commit must match the current
short `HEAD` when `manual-qa-check` is run.
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

Use an app artifact built from a clean git worktree. If the build reported a
dirty tree, rebuild after committing or intentionally removing the unrelated
local change before recording packaged-app evidence.

This creates `/tmp/dropsquash-qa-state`, copies any existing config, history,
and license cache there, and creates `/tmp/dropsquash-manual-qa-output` for the
run. It fails if required environment fields cannot be detected or the selected
artifact is older than `HEAD`, so missing manual QA metadata cannot be mistaken
for a prepared run. Copy the printed `manual QA App build` value into the `App
build` field.
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
generated field and release-candidate rows to a temporary `.md` Markdown file
for copying. The path must be a new file outside the repository so previous QA
evidence cannot be overwritten. It is a preparation aid, not a substitute for
concrete manual observations.
If you override `--app-state-dir`, `--state-dir`, or `--output-dir`, use
absolute paths outside the repository so private app state, generated videos,
and QA evidence cannot be committed or deleted by accident.

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
finished count, saved bytes, failed/cancelled/blocked counts, and trial or
license lock blocked jobs. For failed conversion rows, record the friendly
error plus the unchanged original and trial count.

| Check | Input | Expected | Result |
|---|---|---|---|
| Choose recording conversion | Small `.mov` screen recording | Creates smaller `.squashed.mp4`; original remains |  |
| Drag-and-drop conversion | Small `.mov` screen recording | Creates smaller `.squashed.mp4`; original remains |  |
| Privacy receipt sidecar | Successful conversion | Creates matching `.privacy.json` with file names, `uploaded_bytes = 0`, and `metadata_policy = preserve` |  |
| Reveal privacy receipt | Successful conversion with receipts enabled | Finder opens with generated `.privacy.json` selected |  |
| Duplicate output naming | Same recording twice | Second output uses `.squashed-2.mp4` style numbered suffix |  |
| Cancellation | Large recording | App returns to ready after temp cleanup; no success history; trial count unchanged |  |
| Multi-file queue | Three recordings | 3 recordings queue with 1 active sequential conversion; unrelated failures do not block finished jobs |  |
| Queued job cancellation | Three recordings | Cancelling a waiting row marks it cancelled, it never starts, trial count is unchanged, and history shows no new success |  |
| Batch summary | Three recordings with at least one mixed outcome | Queue summary shows trial or license lock blocked jobs plus numeric finished count, saved bytes, failed count, cancelled count, and blocked count |  |
| Ask source policy | Successful conversion | User can choose Trash or Keep while original remains unchanged |  |
| Trash source policy | Successful conversion | Trash button shows moving/disabled state; original moves to Trash only after verified smaller output |  |
| Failed conversion | Unsupported or intentionally bad input | Friendly error appears; original remains; trial count unchanged |  |
| Larger output | Input that cannot be made smaller | Larger/not-smaller result is treated as failure; original remains; trial count unchanged |  |
| Reveal output | Completed output link | Finder opens with generated `.squashed.mp4` selected |  |

## License Sandbox

Record concrete results. The checker requires:

- Sandbox product setup: mention the sandbox product, DropSquash, the intended product, license keys enabled, and private store IDs not recorded.
- Sandbox purchase: mention the sandbox checkout, intended product, test buyer, and concrete order id or order number.
- Empty key activation: mention the disabled Activate state and that `license.json` or the license cache was checked and has no raw key, no fingerprint, and no instance.
- Invalid key activation: mention the Activating/disabled state, a friendly error, and that `license.json` or the license cache was inspected and has no raw key, no fingerprint, and no instance.
- Valid sandbox activation: mention the Lemon Squeezy sandbox activation request, Activating/disabled state, Pro state, that `license.json` or the license cache was checked, the 64-character lowercase hex fingerprint, the `instance_id` field, and that it has no raw key.
- License network failure: mention a friendly network error, checked or inspected preserved existing valid cache, the 64-character lowercase hex fingerprint, the `instance_id` field, and no raw key in `license.json` or the license cache.
- Expired license refresh: mention an attempted conversion with an expired offline grace cache, the reconnect prompt, conversion blocked before starting, checked `license.json` or license cache, and no raw key.
- Forget license on this Mac: mention the Forgetting/disabled state, confirmed cache removal, and the observed resulting app state.

Inspect the license cache without recording the sandbox key itself or private store IDs.
A good result says the cache path was checked, that the raw key was absent, and
whether only the fingerprint and `instance_id` fields were present.

| Check | Expected | Result |
|---|---|---|
| Sandbox product setup | Intended product is DropSquash, sandbox license keys are enabled, and private store IDs are not recorded |  |
| Sandbox purchase | Sandbox checkout completes for the intended product and test buyer order |  |
| Empty key activation | Empty key leaves Activate disabled; checked license cache has no raw key, no fingerprint, and no instance |  |
| Invalid key activation | Activating state disables submit; friendly license error; inspected license cache has no raw key, no fingerprint, and no instance |  |
| Valid sandbox activation | Lemon Squeezy sandbox activation request disables submit while Activating; Pro state; checked license cache has 64-character lowercase hex fingerprint and `instance_id` fields with no raw key |  |
| License network failure | Friendly network error; checked existing valid cache with 64-character lowercase hex fingerprint and `instance_id` fields remains intact and has no raw key |  |
| Expired license refresh | Attempted conversion with expired offline grace cache shows reconnect prompt; conversion is blocked before starting; checked license cache has no raw key |  |
| Forget license on this Mac | Forgetting state disables action; confirmed license cache removed; observed app returns to trial or locked state |  |

## Release Candidate

Release candidate results must name the artifact or command evidence. Record
the `DropSquash.dmg` path/name, SHA-256 line, Developer ID codesign result,
notary, stapler, `spctl` assessment, and Gatekeeper clean/fresh-machine
observation where the row asks for them. When `App artifact` is a `.dmg`,
artifact-check, checksum, codesign, notary/stapler/`spctl`, and Gatekeeper result rows must name the
same public `DropSquash.dmg` file. Artifact-check and checksum rows must
include the `App artifact` absolute path. Codesign, notary/stapler/`spctl`,
and Gatekeeper rows must also include that same absolute path before marking
the release evidence complete. The final public release notes must tie those
rows to the exact Artifact URL for the same public `DropSquash.dmg`.
Manual QA does not replace the required release notes URL fields; `publish-check`
still requires Artifact URL, Public website URL, Pricing URL, Refund policy URL,
Live checkout URL, GitHub Release URL, and Homebrew tap PR URL before publish.

| Check | Expected | Result |
|---|---|---|
| `cargo run -p xtask -- release-check` | Passes |  |
| `cargo run -p xtask -- file-size-check` | Passes |  |
| `cargo run -p xtask -- media-policy-check` | Passes |  |
| `cargo run -p xtask -- privacy-policy-check` | Passes |  |
| `cargo run -p xtask -- website-check` | Passes |  |
| `cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp> --csv-output <tmp/results.csv>` | Existing absolute `.csv` path recorded outside repo for three local samples; outputs are smaller |  |
| Benchmark sample set | Three short, medium, and large private local recordings produce smaller outputs and are recorded with backend, saved percent, duration, speed ratio, existing absolute CSV path outside repo, machine, and OS context |  |
| Benchmark regression threshold | Throughput does not regress by more than 20% on two or more samples against the same-machine release candidate baseline without a documented reason |  |
| `cargo run -p xtask -- manual-qa-check` | Passes after every manual QA result is recorded |  |
| `cargo run -p xtask -- artifact-check path/to/DropSquash.dmg` | Public UDIF `DropSquash.dmg` artifact check passes |  |
| `cargo run -p xtask -- checksum path/to/DropSquash.dmg --output SHA256SUMS` | `SHA256SUMS` created with 64-character lowercase SHA-256 digest and `DropSquash.dmg` recorded |  |
| `cargo run -p xtask -- macos-signing-check` | Passes in release environment |  |
| Codesign verification | Public DMG/app artifact verifies with Developer ID signature |  |
| Notarization staple verification | Public DMG/app artifact passes notary, stapler validate or stapled status, and `spctl` assessment |  |
| Gatekeeper open test | Signed, notarized, stapled app from public `DropSquash.dmg` matching the release notes Artifact URL opens cleanly without Gatekeeper warning |  |
