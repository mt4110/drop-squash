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

For manual observation rows, do not write only `Pass`, `OK`, or `Done` in the
result. Record the concrete evidence you saw, such as the generated file name,
trial count, Finder selection, or cache state.
`App artifact` must point to the existing local artifact used for the run: a
`.app` bundle directory or a `.dmg` file. `Date` must use a real `YYYY-MM-DD`
calendar date in year 2000 or later.

Before starting packaged-app QA, preserve the current local app state instead
of deleting it:

```sh
mkdir -p /tmp/dropsquash-qa-state
cp "$HOME/Library/Application Support/DropSquash/config.json" /tmp/dropsquash-qa-state/ 2>/dev/null || true
cp "$HOME/Library/Application Support/DropSquash/history.jsonl" /tmp/dropsquash-qa-state/ 2>/dev/null || true
cp "$HOME/Library/Application Support/DropSquash/license.json" /tmp/dropsquash-qa-state/ 2>/dev/null || true
```

Then start from a known trial state if the run is meant to verify trial counts:

```sh
rm -f "$HOME/Library/Application Support/DropSquash/history.jsonl"
rm -f "$HOME/Library/Application Support/DropSquash/license.json"
```

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

| Check | Input | Expected | Result |
|---|---|---|---|
| Choose recording conversion | Small `.mov` screen recording | Creates smaller `.squashed.mp4`; original remains |  |
| Drag-and-drop conversion | Small `.mov` screen recording | Creates smaller `.squashed.mp4`; original remains |  |
| Privacy receipt sidecar | Successful conversion | Creates matching `.privacy.json` with `uploaded_bytes = 0` and `metadata_policy = preserve` |  |
| Reveal privacy receipt | Successful conversion with receipts enabled | Finder opens with generated `.privacy.json` selected |  |
| Duplicate output naming | Same recording twice | Second output uses numbered suffix |  |
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

| Check | Expected | Result |
|---|---|---|
| Sandbox purchase | Checkout completes for the intended product and test buyer |  |
| Empty key activation | Friendly validation error |  |
| Invalid key activation | Friendly license error; no raw key persisted |  |
| Valid sandbox activation | Pro state; raw key absent from cache |  |
| Forget license on this Mac | Local cache clears; app returns to trial or locked state |  |

## Release Candidate

| Check | Expected | Result |
|---|---|---|
| `cargo run -p xtask -- release-check` | Passes |  |
| `cargo run -p xtask -- file-size-check` | Passes |  |
| `cargo run -p xtask -- media-policy-check` | Passes |  |
| `cargo run -p xtask -- privacy-policy-check` | Passes |  |
| `cargo run -p xtask -- website-check` | Passes |  |
| `cargo run -p xtask -- benchmark --input <sample> --output-dir <tmp>` | CSV recorded for three local samples; outputs are smaller |  |
| Benchmark sample set | Short, medium, and large private local recordings are recorded with machine and OS context |  |
| Benchmark regression threshold | Throughput does not regress by more than 20% on two or more samples without a documented reason |  |
| `cargo run -p xtask -- manual-qa-check` | Passes after every manual QA result is recorded |  |
| `cargo run -p xtask -- artifact-check path/to/DropSquash.dmg` | Passes |  |
| `cargo run -p xtask -- checksum path/to/DropSquash.dmg` | SHA-256 line recorded |  |
| `cargo run -p xtask -- macos-signing-check` | Passes in release environment |  |
| Codesign verification | Public DMG/app artifact verifies with Developer ID signature |  |
| Notarization staple verification | Public DMG/app artifact passes notary/staple assessment |  |
| Gatekeeper open test | Signed and notarized app opens cleanly |  |
