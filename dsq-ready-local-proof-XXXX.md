| App build | DropSquash 0.1.0 git 44d7e56 |
| App artifact | /private/tmp/dropsquash-qa-fullsnapshot-5a69fea9/target/release/bundle/dmg/DropSquash.dmg |
| Input sample set | short, medium, and large local recordings |
| macOS version | macOS 26.5.2 |
| Machine | MacBookPro18,4 arm64 |
| Output folder | /tmp/dropsquash-manual-qa-output |
| Config path | /Users/masakitakemura/Library/Application Support/DropSquash/config.json |
| History path | /Users/masakitakemura/Library/Application Support/DropSquash/history.jsonl |
| License cache path | /Users/masakitakemura/Library/Application Support/DropSquash/license.json |
| Tester | masakitakemura |
| Date | 2026-07-16 |
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
| Sandbox product setup | Intended product is DropSquash, sandbox license keys are enabled, and private store IDs are not recorded |  |
| Sandbox purchase | Sandbox checkout completes for the intended product and test buyer order |  |
| Empty key activation | Empty key leaves Activate disabled; checked license cache has no raw key, no fingerprint, and no instance |  |
| Invalid key activation | Activating state disables submit; friendly license error; inspected license cache has no raw key, no fingerprint, and no instance |  |
| Valid sandbox activation | Lemon Squeezy sandbox activation request disables submit while Activating; Pro state; checked license cache has 64-character lowercase hex fingerprint and `instance_id` fields with no raw key |  |
| License network failure | Friendly network error; checked existing valid cache with 64-character lowercase hex fingerprint and `instance_id` fields remains intact and has no raw key |  |
| Expired license refresh | Attempted conversion with expired offline grace cache shows reconnect prompt; conversion is blocked before starting; checked license cache has no raw key |  |
| Forget license on this Mac | Forgetting state disables action; confirmed license cache removed; observed app returns to trial or locked state |  |
| `cargo run -p dropsquash -- license status` | Record `raw license key persisted`, `license cache fingerprint`, and `license cache instance_id` without pasting the sandbox key |  |
| `cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp> --csv-output <tmp/results.csv>` | Existing absolute `.csv` path recorded outside repo for three local samples; outputs are smaller | benchmark CSV recorded for three samples with smaller outputs outside repo at /tmp/dropsquash-manual-qa-output/benchmark-results-44d7e56.csv |
| Benchmark sample set | Three short, medium, and large private local recordings produce smaller outputs and are recorded with backend, saved percent, duration, speed ratio, existing absolute CSV path outside repo, machine, and OS context | three short, medium, and large original local recordings produced smaller outputs with backend apple-native, saved percent, duration, and speed ratio on MacBookPro18,4 arm64 macOS 26.5.2 with CSV saved outside repo at /tmp/dropsquash-manual-qa-output/benchmark-results-44d7e56.csv: take0.mov 107.735s 25.6% saved 5.650 MiB/s 4.124x speed ratio; 画面収録 2026-01-18 10.18.40.mov 43.367s 27.5% saved 1.431 MiB/s 5.489x speed ratio; _neko.mp4 99.584s 0.0% saved 3.566 MiB/s 15.759x speed ratio |
| Benchmark regression threshold | Throughput does not regress by more than 20% on two or more samples against the same-machine release candidate baseline without a documented reason | first release candidate sample set establishes the same-machine release candidate baseline at /tmp/dropsquash-manual-qa-output/benchmark-results-44d7e56.csv; 20% regression comparison starts with the next release candidate sample set |
| `cargo run -p xtask -- release-check` | Passes | release-check passed |
| `cargo run -p xtask -- file-size-check` | Passes | file-size-check passed |
| `cargo run -p xtask -- media-policy-check` | Passes | media-policy-check passed |
| `cargo run -p xtask -- privacy-policy-check` | Passes | privacy-policy-check passed |
| `cargo run -p xtask -- website-check` | Passes | website-check passed |
| `cargo run -p xtask -- manual-qa-check` | Passes after every manual QA result is recorded |  |
| `cargo run -p xtask -- homebrew-cask-check packaging/homebrew/Casks/dropsquash.rb path/to/release-notes.md` | Generated `dropsquash.rb` cask matches `.md` release notes version, Artifact URL, SHA-256, `auto_updates false`, and `zap` |  |
| `cargo run -p xtask -- macos-signing-check` | Passes in release environment |  |
| Codesign verification | Public DMG/app artifact verifies with Developer ID signature |  |
| Notarization staple verification | Public DMG/app artifact passes notary, stapler validate or stapled status, and `spctl` assessment |  |
| Gatekeeper open test | Signed, notarized, stapled app from public `DropSquash.dmg` matching the release notes Artifact URL opens cleanly without Gatekeeper warning |  |
| `cargo run -p xtask -- artifact-check path/to/DropSquash.dmg` | Passes | artifact-check passed for public UDIF /private/tmp/dropsquash-qa-fullsnapshot-5a69fea9/target/release/bundle/dmg/DropSquash.dmg DropSquash.dmg |
| `cargo run -p xtask -- checksum path/to/DropSquash.dmg --output SHA256SUMS` | SHA-256 line recorded | SHA256SUMS created with lowercase SHA-256 4d5025d6ba65e9e6fd802378c97895e963b7f6b385d035784cf617e162b7090d  DropSquash.dmg for /private/tmp/dropsquash-qa-fullsnapshot-5a69fea9/target/release/bundle/dmg/DropSquash.dmg |