# Manual QA

Manual QA records real packaged-app behavior that automated tests cannot prove.
Do not mark a productization phase complete from this file until the result and
environment are filled in.

## macOS Packaged App

| Field | Value |
|---|---|
| App build |  |
| macOS version |  |
| Machine |  |
| Tester |  |
| Date |  |

| Check | Input | Expected | Result |
|---|---|---|---|
| Choose recording conversion | Small `.mov` screen recording | Creates smaller `.squashed.mp4`; original remains |  |
| Drag-and-drop conversion | Small `.mov` screen recording | Creates smaller `.squashed.mp4`; original remains |  |
| Duplicate output naming | Same recording twice | Second output uses numbered suffix |  |
| Cancellation | Large recording | App returns to ready; no success history; no trial count |  |
| Multi-file queue | Three recordings | One active job at a time; unrelated failures do not block queue |  |
| Ask source policy | Successful conversion | User can choose whether to Trash original |  |
| Trash source policy | Successful conversion | Original moves to Trash only after verified smaller output |  |
| Failed conversion | Unsupported or intentionally bad input | Original remains; trial count unchanged |  |
| Larger output | Input that cannot be made smaller | Treated as failure; original remains; trial count unchanged |  |
| Reveal output | Completed output link | Finder opens with generated MP4 selected |  |

## License Sandbox

| Check | Expected | Result |
|---|---|---|
| Empty key activation | Friendly validation error |  |
| Invalid key activation | Friendly license error; no raw key persisted |  |
| Valid sandbox activation | Pro state; raw key absent from cache |  |
| Forget license on this Mac | Local cache clears; app returns to trial or locked state |  |

## Release Candidate

| Check | Expected | Result |
|---|---|---|
| `cargo run -p xtask -- release-check` | Passes |  |
| `cargo run -p xtask -- file-size-check` | Passes |  |
| `cargo run -p xtask -- artifact-check path/to/DropSquash.dmg` | Passes |  |
| `cargo run -p xtask -- checksum path/to/DropSquash.dmg` | SHA-256 line recorded |  |
| `cargo run -p xtask -- macos-signing-check` | Passes in release environment |  |
| Gatekeeper open test | Signed and notarized app opens cleanly |  |
