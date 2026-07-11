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
`.app` bundle directory or a UDIF `.dmg` file. `Date` must use a real
`YYYY-MM-DD` calendar date in year 2000 or later.
`App build` must include both the tested app version and the git commit, for
example `DropSquash 0.1.0 git abc1234`.
`macOS version` must look like `macOS 15.5`, `Machine` must include the CPU
architecture, and `Output folder` must point to an existing directory.
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
run. Copy the printed `manual QA App build` value into the `App build` field.
Copy the printed `manual QA App artifact` value into the `App artifact` field,
or pass `--app-artifact <path>` when testing a DMG or a non-default app bundle.
Pass `--input-sample-set <text>` and copy the printed value into the `Input
sample set` field.
Copy the printed macOS version, Machine, Output folder, and Date values into
the matching fields before starting observations. Copy the printed Config path,
History path, and License cache path values into the matching state path fields.
Copy the printed Tester value into the matching field.

Then start from a known trial state if the run is meant to verify trial counts.
The reset command requires the sample-set description so the run cannot begin
without naming the short, medium, and large recordings. It also requires an
existing `.app` or `.dmg` artifact, either at the default packaged-app path or
through `--app-artifact <path>`:

```sh
cargo run -p xtask -- manual-qa-prepare --reset-trial --input-sample-set "short, medium, and large local recordings"
```

`--reset-trial` removes only `history.jsonl` and `license.json`, and only after
copying any existing state into the backup directory.

After QA, restore the backed up local state when needed:

```sh
cargo run -p xtask -- manual-qa-prepare --restore-state
```

`--restore-state` copies only backed up `config.json`, `history.jsonl`, and
`license.json` files back into the app state directory. It fails if the backup
directory does not exist, so an empty restore cannot be mistaken for success.

## macOS Packaged App

| Field | Value |
|---|---|
| App build |  |
| App artifact |  |
| macOS version |  |
| Machine |  |
| Input sample set |  |
| Output folder |  |
| Config path | `$HOME/Library/Application Support/DropSquash/config.json` |
| History path | `$HOME/Library/Application Support/DropSquash/history.jsonl` |
| License cache path | `$HOME/Library/Application Support/DropSquash/license.json` |
| Tester |  |
| Date |  |

Packaged-app results must include the concrete thing observed, not only that
the row passed. Use output file names such as `.squashed.mp4`, Finder selection
targets, queue counts, trial/history observations, or Trash/source state as
appropriate for the row.
For receipt and reveal rows, record the exact privacy values and Finder
selection state: `uploaded_bytes = 0`, `metadata_policy = preserve`, and
`selected`. For duplicate output naming, record the numbered file name such as
`.squashed-2.mp4`.

| Check | Input | Expected | Result |
|---|---|---|---|
| Choose recording conversion | Small `.mov` screen recording | Creates smaller `.squashed.mp4`; original remains |  |
| Drag-and-drop conversion | Small `.mov` screen recording | Creates smaller `.squashed.mp4`; original remains |  |
| Privacy receipt sidecar | Successful conversion | Creates matching `.privacy.json` with file names, `uploaded_bytes = 0`, and `metadata_policy = preserve` |  |
| Reveal privacy receipt | Successful conversion with receipts enabled | Finder opens with generated `.privacy.json` selected |  |
| Duplicate output naming | Same recording twice | Second output uses `.squashed-2.mp4` style numbered suffix |  |
| Cancellation | Large recording | App returns to ready; no success history; no trial count |  |
| Multi-file queue | Three recordings | One active job at a time; unrelated failures do not block queue |  |
| Queued job cancellation | Three recordings | Cancelling a waiting row marks it cancelled and it never starts |  |
| Batch summary | Three recordings | Queue summary shows finished count and total saved bytes |  |
| Ask source policy | Successful conversion | User can choose whether to Trash original |  |
| Trash source policy | Successful conversion | Original moves to Trash only after verified smaller output |  |
| Failed conversion | Unsupported or intentionally bad input | Original remains; trial count unchanged |  |
| Larger output | Input that cannot be made smaller | Treated as failure; original remains; trial count unchanged |  |
| Reveal output | Completed output link | Finder opens with generated MP4 selected |  |

## License Sandbox

Record concrete results. The checker requires:

- Sandbox product setup: mention DropSquash, the intended product, and license keys enabled.
- Sandbox purchase: mention the intended product, test buyer, and order.
- Empty key activation: mention a friendly validation error and that `license.json` or the license cache has no raw key.
- Invalid key activation: mention a friendly error and that `license.json` or the license cache has no raw key.
- Valid sandbox activation: mention Pro state and that `license.json` or the license cache has no raw key.
- License network failure: mention a friendly network error, preserved existing valid cache, and no raw key in `license.json` or the license cache.
- Forget license on this Mac: mention cache removal and the resulting app state.

Inspect the license cache without recording the sandbox key itself. A good
result says the cache path was checked, that the raw key was absent, and whether
only the fingerprint/instance fields were present.

| Check | Expected | Result |
|---|---|---|
| Sandbox product setup | DropSquash sandbox product exists with license keys enabled |  |
| Sandbox purchase | Checkout completes for the intended product and test buyer order |  |
| Empty key activation | Friendly validation error |  |
| Invalid key activation | Friendly license error; no raw key persisted |  |
| Valid sandbox activation | Pro state; raw key absent from cache |  |
| License network failure | Friendly network error; existing valid cache remains intact |  |
| Forget license on this Mac | Local cache clears; app returns to trial or locked state |  |

## Release Candidate

Release candidate results must name the artifact or command evidence. Record
the `DropSquash.dmg` path/name, SHA-256 line, Developer ID codesign result,
notary/staple assessment, and Gatekeeper clean/fresh-machine observation where
the row asks for them.

| Check | Expected | Result |
|---|---|---|
| `cargo run -p xtask -- release-check` | Passes |  |
| `cargo run -p xtask -- file-size-check` | Passes |  |
| `cargo run -p xtask -- media-policy-check` | Passes |  |
| `cargo run -p xtask -- privacy-policy-check` | Passes |  |
| `cargo run -p xtask -- website-check` | Passes |  |
| `cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp>` | CSV recorded for three local samples; outputs are smaller |  |
| Benchmark sample set | Short, medium, and large private local recordings are recorded with machine and OS context |  |
| Benchmark regression threshold | Throughput does not regress by more than 20% on two or more samples without a documented reason |  |
| `cargo run -p xtask -- manual-qa-check` | Passes after every manual QA result is recorded |  |
| `cargo run -p xtask -- artifact-check path/to/DropSquash.dmg` | Passes |  |
| `cargo run -p xtask -- checksum path/to/DropSquash.dmg` | SHA-256 line with 64-character digest and `DropSquash.dmg` recorded |  |
| `cargo run -p xtask -- macos-signing-check` | Passes in release environment |  |
| Codesign verification | Public DMG/app artifact verifies with Developer ID signature |  |
| Notarization staple verification | Public DMG/app artifact passes notary/staple assessment |  |
| Gatekeeper open test | Signed and notarized app opens cleanly |  |
