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

After running the release-set benchmark, check the generated CSV before copying
its path into this file:

```sh
cargo run -p xtask -- benchmark-csv-check /absolute/path/to/results.csv
```

`manual-qa-check` also revalidates the referenced benchmark CSV with the same
CSV-content rules, so the recorded path must keep pointing to the checked file.
If the prepared Markdown draft still has empty release gate rows, you can fill
the deterministic local ones after they pass:

```sh
cargo run -p xtask -- manual-qa-fill-release-gates /tmp/dropsquash-manual-qa-prepared.md
cargo run -p xtask -- manual-qa-fill-local-proof /tmp/dropsquash-manual-qa-prepared.md /tmp/dropsquash-manual-qa-output/benchmark-results.csv
cargo run -p xtask -- manual-qa-ready-local-proof /tmp/dropsquash-manual-qa-prepared.md /tmp/dropsquash-manual-qa-output/benchmark-results.csv
cargo run -p xtask -- manual-qa-clean-draft /tmp/dropsquash-manual-qa-prepared.md
cargo run -p xtask -- manual-qa-fill-check /tmp/dropsquash-manual-qa-prepared.md
```

After a real release-set benchmark CSV exists and `benchmark-csv-check` passes,
prefer `manual-qa-ready-local-proof` for the normal local packaged-app proof
flow. It fills the deterministic release-gate and benchmark rows in order, then
removes the prepared-draft marker so `manual-qa-pending --section local-proof`
shows only the remaining packaged-app observations. It also prints the sample
paths to reuse for the small, duplicate-output, queue, and large-output manual
checks so the packaged-app pass can continue from the same checked benchmark
set without re-deciding file selection. Once the benchmark sample-set row is
filled, `manual-qa-pending --section local-proof` repeats those sample hints so
the remaining packaged-app pass can resume from the recorded evidence file.
Rows that reuse the checked benchmark samples also print a row-specific
`sample:` hint so the next manual observation can start from the right file or
queue set immediately.

Use the lower-level benchmark fill commands only when you intentionally need to
inspect or rerun one part of the flow. For the first release candidate, fill
the threshold row from the current CSV so it records that this sample set
establishes the same-machine baseline. For a later release candidate, pass the
earlier same-machine baseline CSV as the third argument:

```sh
cargo run -p xtask -- manual-qa-fill-local-proof /tmp/dropsquash-manual-qa-prepared.md /tmp/dropsquash-manual-qa-output/benchmark-results.csv
cargo run -p xtask -- manual-qa-ready-local-proof /tmp/dropsquash-manual-qa-prepared.md /tmp/dropsquash-manual-qa-output/benchmark-results.csv
cargo run -p xtask -- manual-qa-fill-benchmark /tmp/dropsquash-manual-qa-prepared.md /tmp/dropsquash-manual-qa-output/benchmark-results.csv
cargo run -p xtask -- manual-qa-fill-benchmark-threshold /tmp/dropsquash-manual-qa-prepared.md /tmp/dropsquash-manual-qa-output/benchmark-results.csv
cargo run -p xtask -- manual-qa-fill-benchmark-threshold /tmp/dropsquash-manual-qa-prepared.md /tmp/dropsquash-manual-qa-output/benchmark-results.csv /absolute/path/to/baseline-results.csv
cargo run -p xtask -- manual-qa-clean-draft /tmp/dropsquash-manual-qa-prepared.md
cargo run -p xtask -- manual-qa-pending /tmp/dropsquash-manual-qa-prepared.md
cargo run -p xtask -- manual-qa-pending /tmp/dropsquash-manual-qa-prepared.md --section packaged-app
cargo run -p xtask -- manual-qa-pending /tmp/dropsquash-manual-qa-prepared.md --section local-proof
cargo run -p xtask -- manual-qa-pending /tmp/dropsquash-manual-qa-prepared.md --section license
cargo run -p xtask -- manual-qa-pending /tmp/dropsquash-manual-qa-prepared.md --section benchmark
cargo run -p xtask -- manual-qa-pending /tmp/dropsquash-manual-qa-prepared.md --section distribution
cargo run -p xtask -- manual-qa-fill-check /tmp/dropsquash-manual-qa-prepared.md
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

Before building the QA artifact, run the productization status gate and follow
its Local packaged-app proof preflight:

```sh
cargo run -p xtask -- productization-status
nix develop --command pnpm --dir apps/desktop install --frozen-lockfile
nix develop --command pnpm --dir apps/desktop/web install --frozen-lockfile
nix develop --command pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci
cargo run -p xtask -- normalize-dmg target/release/bundle/dmg
cargo run -p xtask -- artifact-check target/release/bundle/dmg/DropSquash.dmg
```

Use an app artifact built from a clean git worktree. If the main worktree is
dirty because of unrelated local changes, prefer a detached QA worktree:

```sh
git worktree add --detach /tmp/dropsquash-qa-$(git rev-parse --short HEAD) HEAD
```

Run the packaged-app build and QA-preparation commands from that detached
worktree so the checked `DropSquash.dmg` still matches the current `HEAD`
without touching unrelated local changes. If you stay in the current worktree,
rebuild only after committing or intentionally removing the unrelated local
change before recording packaged-app evidence.

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
It also prints the release-set benchmark command and the matching
`benchmark-csv-check` command, followed by Release Candidate benchmark rows
that can be pasted into the table before filling concrete results. The suggested
CSV path lives under the prepared QA output folder so benchmark evidence and
generated outputs stay together outside the repository. Replace the three input
placeholders with the actual short, medium, and large local recording paths
before running them.
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
Custom `--app-state-dir` values must still point at an
`Application Support/DropSquash` directory so the generated `config.json`,
`history.jsonl`, and `license.json` evidence matches `manual-qa-check`.

Then start from a known trial state if the run is meant to verify trial counts.
The reset command requires the sample-set description so the run cannot begin
without naming the short, medium, and large recordings. It also requires an
existing `DropSquash.app` or `DropSquash.dmg` artifact, either at the default
packaged-app path or through `--app-artifact <path>`:

```sh
cargo run -p xtask -- manual-qa-prepare --reset-trial --app-artifact target/release/bundle/dmg/DropSquash.dmg --input-sample-set "short, medium, and large local recordings" --markdown-output /tmp/dropsquash-manual-qa-prepared.md
cargo run -p xtask -- checksum target/release/bundle/dmg/DropSquash.dmg --output /tmp/dropsquash-manual-qa-output/SHA256SUMS
```

`--reset-trial` removes only `history.jsonl` and `license.json`, and only after
copying any existing state into the backup directory. The command prints the
app state source and reset path; confirm they point to the DropSquash app
support directory before starting observations.
The checksum command writes the release-candidate `SHA256SUMS` evidence into
the prepared output folder so checksum evidence stays outside the repository.

After QA, restore the backed up local state when needed:

```sh
cargo run -p xtask -- manual-qa-prepare --restore-state
```

If the prepare step used custom `--app-state-dir` or `--state-dir` values,
use the printed `trial state restore command` so restore reads from and writes
to the same locations that were used during reset.

`--restore-state` copies only backed up `config.json`, `history.jsonl`, and
`license.json` files back into the app state directory. It fails if the backup
directory does not exist or contains no restorable state files, so an empty
restore cannot be mistaken for success. It only accepts `--app-state-dir` and
`--state-dir`; rerun without restore to create new Markdown or trial-reset
preparation fields.

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
| Disk image launch notice | Launch from mounted `DropSquash.dmg` before copying to Applications | App warns that it is running from the disk image; Move copies `DropSquash.app` to `/Applications` without replacing an existing app, reveals the copied app in Finder, keeps a post-copy notice visible, opens the installed app on request, can request mounted-volume eject and quit the disk image copy, and does not delete the downloaded `.dmg` |  |
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
Use `cargo run -p dropsquash -- license status` for local diagnostics; record
the lines for `raw license key persisted`, `license cache fingerprint`, and
`license cache instance_id` instead of pasting the sandbox license key.

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
| `cargo run -p xtask -- homebrew-cask-check packaging/homebrew/Casks/dropsquash.rb path/to/release-notes.md` | Generated `dropsquash.rb` cask matches `.md` release notes version, Artifact URL, SHA-256, `auto_updates false`, and `zap` |  |
| `cargo run -p xtask -- macos-signing-check` | Passes in release environment |  |
| Codesign verification | Public DMG/app artifact verifies with Developer ID signature |  |
| Notarization staple verification | Public DMG/app artifact passes notary, stapler validate or stapled status, and `spctl` assessment |  |
| Gatekeeper open test | Signed, notarized, stapled app from public `DropSquash.dmg` matching the release notes Artifact URL opens cleanly without Gatekeeper warning |  |
