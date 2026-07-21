# Manual QA

Manual QA records real packaged-app behavior that automated tests cannot prove.
Do not mark a productization phase complete from this file until the result and
environment are filled in.

Automated coverage that supports this checklist is tracked separately in
`docs/qa-evidence.md`. Do not copy automated pass results into this file unless
the row explicitly asks for a command result.

## Recommended Order For Paid Beta Proof

迷ったら、この順番を崩しません。

1. `cargo run -p xtask -- paid-beta-check`
2. fresh prepared manual-QA draft を作る
3. checked benchmark CSV を用意する
4. `cargo run -p xtask -- manual-qa-ready-all <manual-qa.md> <results.csv>`
5. `cargo run -p xtask -- manual-qa-packaged-rerun <manual-qa.md>`
6. `cargo run -p xtask -- manual-qa-license-rerun <manual-qa.md>`
7. `cargo run -p xtask -- manual-qa-distribution-rerun <manual-qa.md>`

`manual-qa-ready-all` は deterministic に埋まる行を先に整えるための入口です。
そのあとに packaged / license / distribution の manual rows を順に埋めると、
paid beta の残ブロッカーを一番崩しにくい形で回収できます。

Run this gate after filling every result:

```sh
cargo run -p xtask -- manual-qa-check
```

When you need a focused gate while another section is still intentionally empty,
you can validate only one section at a time:

```sh
cargo run -p xtask -- manual-qa-check docs/manual-qa.md --section local-proof
cargo run -p xtask -- manual-qa-check docs/manual-qa.md --section license
cargo run -p xtask -- manual-qa-check docs/manual-qa.md --section distribution
```

After running the release-set benchmark, check the generated CSV before copying
its path into this file:

```sh
cargo run -p xtask -- benchmark-csv-check /absolute/path/to/results.csv
```

`manual-qa-check` also revalidates the referenced benchmark CSV with the same
CSV-content rules, so the recorded path must keep pointing to the checked file.
If the benchmark run fails with `output is not smaller`, do not force that clip
into the release-set evidence. Use it as a candidate for the packaged-app
`Larger output` row, verify that it still reproduces under the current shipping
profile and size, then rerun the release-set benchmark with another real
recording or the exact shipping size setting you intend to validate.
When picking benchmark samples, avoid clips that are likely to blur product
signal: previously squashed or otherwise re-encoded delivery files, clips that
are already unusually small for their duration, and very short or nearly
static recordings that do not show enough real motion or text change to expose
normal savings.
If the prepared Markdown draft still has empty release gate rows, you can fill
the deterministic local ones after they pass:

```sh
cargo run -p xtask -- manual-qa-fill-release-gates /tmp/dropsquash-manual-qa-prepared-<app-build>.md
cargo run -p xtask -- manual-qa-fill-local-proof /tmp/dropsquash-manual-qa-prepared-<app-build>.md /tmp/dropsquash-manual-qa-output/benchmark-results-<app-build>.csv
cargo run -p xtask -- manual-qa-ready-local-proof /tmp/dropsquash-manual-qa-prepared-<app-build>.md /tmp/dropsquash-manual-qa-output/benchmark-results-<app-build>.csv
cargo run -p xtask -- manual-qa-license-rerun /tmp/dropsquash-manual-qa-prepared-<app-build>.md
cargo run -p xtask -- manual-qa-distribution-rerun /tmp/dropsquash-manual-qa-prepared-<app-build>.md
cargo run -p xtask -- manual-qa-merge-prepared /tmp/dropsquash-manual-qa-prepared-<app-build>.md
cargo run -p xtask -- manual-qa-installed-app stash /tmp/dropsquash-manual-qa-installed-app
cargo run -p xtask -- manual-qa-link-samples /tmp/dropsquash-manual-qa-output/benchmark-results-<app-build>.csv
cargo run -p xtask -- manual-qa-bad-input /tmp/dropsquash-manual-qa-invalid.mp4
cargo run -p xtask -- manual-qa-clean-draft /tmp/dropsquash-manual-qa-prepared-<app-build>.md
cargo run -p xtask -- manual-qa-fill-check /tmp/dropsquash-manual-qa-prepared-<app-build>.md
```

When the target is a prepared draft such as
`/tmp/dropsquash-manual-qa-prepared-<app-build>.md`, `manual-qa-fill-release-gates` skips
the `release-check` row on purpose. That row depends on the real
`docs/manual-qa.md` release-blocker evidence and should be filled only after
the prepared rows are merged back into the main record.

After a real release-set benchmark CSV exists and `benchmark-csv-check` passes,
prefer `manual-qa-ready-all` for one deterministic pass across local proof,
license, and distribution. It now also prints the next operator checklist plus
focused license, distribution, and final `manual-qa-check` gates, so prefer
those emitted lines as the next actions instead of guessing the next section by
hand. If the referenced benchmark CSV has already been cleaned out of `/tmp`,
rerun the printed `manual-qa-prepare --reset-trial`, `benchmark --release-set`,
and `benchmark-csv-check` commands first, then retry `manual-qa-ready-all`
with the fresh `benchmark-results-<app-build>.csv` path. If you want to
advance only the packaged-app proof, run `manual-qa-ready-local-proof`
instead. It fills the deterministic
release-gate and benchmark rows in order, then removes the prepared-draft
marker so `manual-qa-pending --section local-proof` shows only the remaining
packaged-app observations. It also prints the sample paths to reuse for the
small, duplicate-output, queue, and large-output manual checks so the
packaged-app pass can continue from the same checked benchmark set without
re-deciding file selection. Once the benchmark sample-set row is filled,
`manual-qa-pending --section local-proof` repeats those sample hints so the
remaining packaged-app pass can resume from the recorded evidence file. Rows
that reuse the checked benchmark samples also print a row-specific `sample:`
hint so the next manual observation can start from the right file or queue set
immediately. Rows that cannot be satisfied from the checked benchmark set print
a `note:` line instead, such as the mounted-DMG requirement or the need for an
intentionally bad input. For the failed-conversion row, `manual-qa-bad-input`
creates a throwaway invalid `.mp4` under `/tmp` so you can keep the benchmark
sample set untouched. `manual-qa-pending --section local-proof` also groups
packaged-app rows into practical phases such as Mounted DMG, Small Sample,
Duplicate Sample, Large Sample, Queue Sample, and Custom Failure Input. The
packaged-app section also prints a `phase counts:` line so you can see how many
observations remain in each phase before starting the pass.
If the checked benchmark set also provides the current kept-original candidate,
`manual-qa-ready-local-proof` now prints `fresh packaged-app not-smaller
command:` for `/tmp/dsq-build-target/release/bundle/macos/DropSquash.app`, and
the packaged-app pending output mirrors that with `fresh open-file not-smaller
command:` so the `Larger output` rerun does not accidentally fall back to a
stale default `target/` bundle.
The same helper also prints `fresh packaged-app sample-link command:` so the
same checked benchmark CSV can generate ASCII sample aliases in
`/tmp/dropsquash-qa-open-panel` before chooser-based packaged-app checks.
Before mounted-DMG QA, it now also prints:

- `fresh packaged-app installed-app status:`
- `fresh packaged-app installed-app stash:`
- `fresh packaged-app installed-app restore:`
- `packaged-app mounted dmg window probe command:`

Use those instead of guessing whether an older `/Applications/DropSquash.app`
is still shadowing the mounted artifact. Run the window-probe command once
before relaunch and once after relaunch so the mounted app pid count, pid list,
and AX window count can be copied into the `Disk image launch notice` result
without hand-counting.
If `docs/manual-qa.md` already records the checked benchmark CSV and you want
the shortest rerun path for the public `DropSquash.dmg`, run:

```sh
cargo run -p xtask -- manual-qa-packaged-rerun
```

If you want the shortest rerun path for the license sandbox rows, run:

```sh
cargo run -p xtask -- manual-qa-license-rerun
```

If you want one short reminder of the packaged, license, and distribution
rerun entrypoints together, run:

```sh
cargo run -p xtask -- manual-qa-paid-beta-rerun
```

That wrapper prints the current `manual-qa-ready-all` standard entrypoint, the
packaged-only `manual-qa-ready-local-proof` alternative,
`manual-qa-link-samples`, packaged-app pending, local-proof gate, final
`manual-qa-check`, and installed-app status/stash/restore commands in one
place.
If the everyday worktree is dirty and you want the shortest snapshot handoff
for the signing side, run:

```sh
scripts/manual-qa-distribution-handoff.sh /tmp/dropsquash-qa-snapshot-$(git rev-parse --short HEAD)
```

That helper prints the snapshot path plus the next
`productization-status --track "Paid beta"`, desktop/web install,
`file-size-check`, `release-check`, unsigned build,
`manual-qa-prepare --reset-trial`, `macos-signing-check`,
`manual-qa-distribution-rerun`, and `paid-beta-check` commands for that exact
snapshot.
It now also prints `next license sandbox runbook`,
`next signed DMG runbook`, and `paid beta license browser sign-in checkpoint`
so the paid-beta handoff can stop at the Lemon Squeezy sign-in step before
sandbox product setup, purchase, or activation is recorded.
When `manual-qa-check` reports an empty or missing manual row, it also prints a
matching `manual QA pending hint:` line so you can jump back to the right
`manual-qa-pending --section ...` view instead of guessing the section.
The packaged-app section also prints the prepared draft's
`packaged-app artifact:`, `packaged-app open command:`, `packaged-app config
path:`, `packaged-app output folder:`, and `packaged-app history path:` lines
so conversion, Finder-reveal, config-backed settings, and trial-history
observations can keep referring to the same recorded evidence paths.

Use the lower-level benchmark fill commands only when you intentionally need to
inspect or rerun one part of the flow. For the first release candidate, fill
the threshold row from the current CSV so it records that this sample set
establishes the same-machine baseline. For a later release candidate, pass the
earlier same-machine baseline CSV as the third argument:

```sh
cargo run -p xtask -- manual-qa-fill-local-proof /tmp/dropsquash-manual-qa-prepared-<app-build>.md /tmp/dropsquash-manual-qa-output/benchmark-results-<app-build>.csv
# Standard deterministic pass across local proof, license, and distribution:
cargo run -p xtask -- manual-qa-ready-all /tmp/dropsquash-manual-qa-prepared-<app-build>.md /tmp/dropsquash-manual-qa-output/benchmark-results-<app-build>.csv
# Optional packaged-only alternative when you are advancing local proof first:
cargo run -p xtask -- manual-qa-ready-local-proof /tmp/dropsquash-manual-qa-prepared-<app-build>.md /tmp/dropsquash-manual-qa-output/benchmark-results-<app-build>.csv
cargo run -p xtask -- manual-qa-license-rerun /tmp/dropsquash-manual-qa-prepared-<app-build>.md
cargo run -p dropsquash -- license status
cargo run -p xtask -- manual-qa-distribution-rerun /tmp/dropsquash-manual-qa-prepared-<app-build>.md
cargo run -p xtask -- manual-qa-bad-input /tmp/dropsquash-manual-qa-invalid.mp4
cargo run -p xtask -- manual-qa-fill-benchmark /tmp/dropsquash-manual-qa-prepared-<app-build>.md /tmp/dropsquash-manual-qa-output/benchmark-results-<app-build>.csv
cargo run -p xtask -- manual-qa-fill-benchmark-threshold /tmp/dropsquash-manual-qa-prepared-<app-build>.md /tmp/dropsquash-manual-qa-output/benchmark-results-<app-build>.csv
cargo run -p xtask -- manual-qa-fill-benchmark-threshold /tmp/dropsquash-manual-qa-prepared-<app-build>.md /tmp/dropsquash-manual-qa-output/benchmark-results-<app-build>.csv /absolute/path/to/baseline-results.csv
cargo run -p xtask -- manual-qa-clean-draft /tmp/dropsquash-manual-qa-prepared-<app-build>.md
cargo run -p xtask -- manual-qa-pending /tmp/dropsquash-manual-qa-prepared-<app-build>.md
cargo run -p xtask -- manual-qa-pending /tmp/dropsquash-manual-qa-prepared-<app-build>.md --section packaged-app
cargo run -p xtask -- manual-qa-pending /tmp/dropsquash-manual-qa-prepared-<app-build>.md --section local-proof
cargo run -p xtask -- manual-qa-pending /tmp/dropsquash-manual-qa-prepared-<app-build>.md --section license
cargo run -p xtask -- manual-qa-pending /tmp/dropsquash-manual-qa-prepared-<app-build>.md --section benchmark
cargo run -p xtask -- manual-qa-pending /tmp/dropsquash-manual-qa-prepared-<app-build>.md --section distribution
cargo run -p xtask -- manual-qa-fill-check /tmp/dropsquash-manual-qa-prepared-<app-build>.md
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
example `DropSquash 0.1.0 git abc1234`. `manual-qa-check` accepts the recorded
build commit when the current `HEAD` is exactly that commit or when the same
worktree only moved forward through `xtask/` or `docs/` helper commits.
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
cargo run -p xtask -- productization-status --track "Paid beta"
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

If the QA candidate currently lives only in the dirty worktree, create a clean
snapshot worktree first:

```sh
scripts/manual-qa-snapshot-worktree.sh
```

Run the packaged-app build and QA-preparation commands from that detached
worktree so the checked `DropSquash.dmg` still matches the current `HEAD`
without touching unrelated local changes. If you stay in the current worktree,
rebuild only after committing or intentionally removing the unrelated local
change before recording packaged-app evidence.
Run the follow-up helper commands from that same worktree too:
`manual-qa-ready-*`, `manual-qa-pending`, and `manual-qa-check` compare the
recorded App build against the current `HEAD`. The same worktree may keep
moving through helper-only `xtask/` or `docs/` commits without forcing an app
rebuild, but running from a different worktree can still fail the guard even
when the prepared evidence file itself is correct.

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
When using `manual-qa-launch-app`, prefer that helper over launching the app
yourself. It starts a fresh packaged-app instance with `open -n -a` and routes
Config path, History path, License cache path, and the default output folder
through the printed manual-QA state directory override.
Before mounted DMG QA, check whether `/Applications/DropSquash.app` is still
present with
`cargo run -p xtask -- manual-qa-installed-app status /tmp/dropsquash-manual-qa-installed-app`.
If it reports `installed app: present`, stash it before continuing with mounted
DMG observations so `manual-qa-launch-app --mount-dmg` does not refuse to run.
If the default `target/` tree is stale or a local release build hangs while
scanning old artifacts, build a fresh packaged app into an isolated target
directory first:
`CARGO_TARGET_DIR=/tmp/dsq-build-target pnpm --dir apps/desktop tauri build`.
Then use `/tmp/dsq-build-target/release/bundle/macos/DropSquash.app` as the
`App artifact` for the packaged-app observations.
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
Pass `--markdown-output /tmp/dropsquash-manual-qa-prepared-<app-build>.md` to write those
generated field and release-candidate rows to a temporary `.md` Markdown file
for copying. The path must be a new file outside the repository so previous QA
evidence cannot be overwritten. It is a preparation aid, not a substitute for
concrete manual observations. After deterministic rows are filled there,
`manual-qa-merge-prepared` can merge only the non-empty rows back into
`docs/manual-qa.md` without wiping still-empty manual observation rows.
If packaged-app file pickers are awkward because the benchmark sample names use
spaces or non-ASCII characters, `manual-qa-link-samples` creates ASCII symlinks
such as `qa-small.mov`, `qa-medium.mov`, `qa-large.<ext>`, and
`qa-not-smaller.mp4` beside the checked benchmark CSV.
For `Choose recording`, prefer a dedicated ASCII-only folder such as
`/tmp/dropsquash-qa-open-panel`: run
`cargo run -p xtask -- manual-qa-link-samples /absolute/path/to/results.csv /tmp/dropsquash-qa-open-panel`
and then `open /tmp/dropsquash-qa-open-panel` before starting the packaged-app
pass. That keeps the file picker on a short, deterministic path instead of
relying on the original sample names or nested benchmark directories.
When the mounted-DMG app starts with focus on the Size selector, the helper
`cargo run -p xtask -- manual-qa-open-chooser /tmp/dropsquash-qa-open-panel`
activates DropSquash, walks backward through the observed focus order until the
native chooser sheet opens, then opens the chooser's folder-navigation sheet
and writes the ASCII sample path into its text field directly.
Pass a second argument such as `qa-small.mov`, `qa-medium.mov`,
`qa-large.<ext>`, or `qa-not-smaller.mp4` when you want the helper to jump
directly to that linked sample path through that same folder-navigation sheet,
for example:
`cargo run -p xtask -- manual-qa-open-chooser /tmp/dropsquash-qa-open-panel qa-small.mov`.
At the current packaged-app QA stage, treat that second argument as a best-effort
shortcut rather than a guaranteed final selection. After the helper opens the
ASCII sample folder, confirm the highlighted row before pressing `Open`.
For the `Larger output` rerun, prefer the fresh isolated bundle command printed
by `manual-qa-ready-local-proof` or `manual-qa-pending --section packaged-app`
instead of hand-editing the older `target/release/bundle/macos/DropSquash.app`
path. The fresh helper uses the same `qa-not-smaller.mp4` alias and prepared
config path, but points at `/tmp/dsq-build-target/release/bundle/macos/DropSquash.app`.
On July 16, 2026, the current
`/tmp/dropsquash-qa-open-panel/qa-not-smaller.mp4` alias had drifted to
`/tmp/dropsquash-manual-qa-output-latest/take0.squashed.mp4` and produced a
successful smaller output again, so it no longer qualified as Larger output
evidence. Before recording that row, either relink a fresh kept-original
candidate or use the proven candidate
`/tmp/dsq-not-smaller-probe/out/画面収録 2026-01-18 10.18.40.squashed.squashed.squashed.mp4`
and confirm that the row ends as `Kept original`, with trial and history still
unchanged.
If you also want the failure-path sample in the same place, run
`cargo run -p xtask -- manual-qa-bad-input /tmp/dropsquash-qa-open-panel/qa-invalid.mp4`
so the unsupported input sits beside the ASCII sample links.
If `/Applications/DropSquash.app` already exists and blocks the mounted-DMG
install path, `manual-qa-installed-app stash` moves that bundle into a backup
directory without deleting it, and `manual-qa-installed-app restore` moves it
back after the packaged-app observation is done.
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
cargo run -p xtask -- manual-qa-prepare --reset-trial --app-artifact target/release/bundle/dmg/DropSquash.dmg --input-sample-set "short, medium, and large local recordings" --markdown-output /tmp/dropsquash-manual-qa-prepared-<app-build>.md
cargo run -p xtask -- checksum target/release/bundle/dmg/DropSquash.dmg --output /tmp/dropsquash-manual-qa-output/SHA256SUMS
```

`--reset-trial` removes only `history.jsonl` and `license.json`, and only after
copying any existing state into the backup directory. The command prints the
app state source and reset path; confirm they point to the DropSquash app
support directory before starting observations.
The checksum command writes the release-candidate `SHA256SUMS` evidence into
the prepared output folder so checksum evidence stays outside the repository.
When recording checksum evidence for distribution review, mention the
64-character lowercase SHA-256 digest and the exact Artifact URL for the same
public `DropSquash.dmg`.

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
| App build | DropSquash 0.1.0 git 5a69fea |
| App artifact | /tmp/dropsquash-manual-qa-public-dmg/DropSquash.dmg |
| macOS version | macOS 26.5.2 |
| Machine | MacBookPro18,4 arm64 |
| Input sample set | short, medium, and large local recordings |
| Output folder | /tmp/dropsquash-manual-qa-output |
| Config path | /tmp/dropsquash-manual-qa-app-state/Library/Application Support/DropSquash/config.json |
| History path | /tmp/dropsquash-manual-qa-app-state/Library/Application Support/DropSquash/history.jsonl |
| License cache path | /tmp/dropsquash-manual-qa-app-state/Library/Application Support/DropSquash/license.json |
| Tester | masakitakemura |
| Date | 2026-07-17 |

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
error plus the unchanged original and trial count. For larger-output rows,
record that DropSquash said the recording could not be made smaller, that it
kept the original, then note the unchanged original and trial count.
For `Drag-and-drop conversion`, record a real Finder drag into the packaged app.
The `cargo run -p xtask -- manual-qa-drag-drop ...` helper is diagnostic only:
it currently does not reach Tauri drag/drop events on macOS, so passing evidence
for that row must come from an actual Finder drag, not the synthetic helper.
If you need local diagnostics while doing the real Finder drag, launch the
packaged app with `DROP_SQUASH_MANUAL_QA_EVENT_LOG=/tmp/dsq-drag-events.jsonl`
and inspect that JSONL after the drag.
After the real Finder drag, confirm the row with the generated `.squashed.mp4`
in the output folder, the latest success or unchanged state in `history.jsonl`,
and the optional event log if you launched with
`DROP_SQUASH_MANUAL_QA_EVENT_LOG=/tmp/dsq-drag-events.jsonl`.
The final row text should explicitly say which sample was dragged from Finder,
which `.squashed.mp4` was created, that the output was smaller, and that the
original remained in place.
For the 2026-07-17 packaged-app refresh, reuse the existing packaged-app rows
but explicitly mention two UI regressions that were fixed after the older
evidence was recorded:

- Relaunching `DropSquash.app` while it is already open must focus the existing
  window instead of leaving multiple app windows running.
- When the trial banner, license input, and disk-image install notice are all
  visible together, the packaged-app window must grow enough vertically that
  the full license field and the full `Choose recording` action remain visible
  without clipping.

Record those observations in the concrete row result that naturally exercised
them, usually `Disk image launch notice`, `Choose recording conversion`, or
`Drag-and-drop conversion`, so the release-candidate packaged-app evidence says
which window was focused and which controls remained fully visible.

For Phase 2 Secure Share R&D, do not use CLI capture start as evidence. First
run `cargo run -p xtask -- manual-qa-secure-share-observe list` and choose a
redacted window ID. Then launch the packaged app with
`DROP_SQUASH_QA_SCK_OBSERVE_WINDOW_ID=<id>`,
`DROP_SQUASH_MANUAL_QA_EVENT_LOG=/tmp/dsq-secure-share-observe.jsonl`, and
optionally `DROP_SQUASH_QA_SCK_OBSERVE_QUIT_AFTER=1`. The JSONL must contain a
`secure-share-observe-window` event whose `detail` string contains JSON with
either `status: "ok"` plus frame size, frame count, and first/last timestamps,
`axObservationCount`, `visionObservationCount`, or `status: "error"` with a
fail-closed reason. Do not record raw pixels, window titles, application names,
or recognized private text as QA evidence. After the AX/Vision bridge changes,
missing Screen Recording or Accessibility permission is an expected fail-closed
result for this R&D harness until the packaged app has been granted access in
macOS Privacy & Security settings.
On Sunday, July 19, 2026, the packaged-app R&D harness recorded successful
redacted frame metadata at
`/tmp/dsq-secure-share-observe-frame-metadata-1784451193.jsonl`:
`status = ok`, `windowId = 3328`, `frameWidth = 1691`, `frameHeight = 950`,
`frameCount = 18`, `firstFrameTimeNs = 4672453313239`, and
`lastFrameTimeNs = 4672460422746`. This proves only ScreenCaptureKit frame
metadata observation, not AX/Vision masking or output leak verification. The
next R&D observation should additionally record `axObservationCount`,
`visionObservationCount`, or the exact fail-closed permission error.
After adding the AX bridge, the packaged-app run at
`/tmp/dsq-secure-share-observe-ax-1784453411.jsonl` failed closed before AX
observation because ScreenCaptureKit discovery reported that macOS TCC denied
app/window/display capture. This records correct fail-closed behavior, but the
next run still needs Screen Recording permission granted to the current
packaged app before `axObservationCount` can be observed.
After connecting local Vision to accepted ScreenCaptureKit callbacks, the
packaged-app run at `/tmp/dsq-secure-share-observe-vision-1784457000.jsonl`
recorded `status = ok`, `windowId = 3328`, `frameCount = 2`,
`frameWidth = 1691`, `frameHeight = 950`, `axObservationCount = 6`, and
`visionObservationCount = 268`. A shorter prior run at
`/tmp/dsq-secure-share-observe-vision-1784456934.jsonl` recorded
`frameCount = 1`, `axObservationCount = 6`, and
`visionObservationCount = 133`. This proves redacted AX/Vision observation
counts from the packaged GUI-hosted harness, not masking quality or final
output leak verification.

| Check | Input | Expected | Result |
|---|---|---|---|
| Disk image launch notice | Launch from mounted `DropSquash.dmg` before copying to Applications | App warns that it is running from the disk image; Move copies `DropSquash.app` to `/Applications` without replacing an existing app, reveals the copied app in Finder, keeps a post-copy notice visible, opens the installed app on request, can request mounted-volume eject and quit the disk image copy, relaunch returns focus to the existing mounted-DMG window without increasing the mounted app pid count, the AX window count stays at 1, the notice plus the license field stay fully visible without clipping, and it does not delete the downloaded `.dmg` | The app launched from mounted disk image under `/Volumes`, showed warning notice, Move copied `DropSquash.app` to Applications without replacing an existing app, Finder revealed the copied app, post-copy notice stayed visible, Open opened the installed app, Eject & Quit requested mounted-volume eject and closed the disk image copy, relaunch focused the existing window instead of leaving multiple windows, the license field and `Choose recording` action stayed visible without clipping, and did not delete the downloaded .dmg. For the latest public-DMG single-instance rerun on Friday, July 17, 2026, after ejecting older mounted DropSquash volumes, launching `/tmp/dropsquash-manual-qa-public-dmg/DropSquash.dmg` produced mounted app pid `85460`; relaunching `/Volumes/DropSquash/DropSquash.app` kept the mounted pid count at `1`, kept the pid list unchanged at `85460`, and kept the AX window count at `1`. The screenshots at `/tmp/dropsquash-mounted-public-1.png` and `/tmp/dropsquash-mounted-public-2.png` showed the disk-image notice, full license field, and full `Choose recording` action visible together without vertical clipping, and the downloaded `.dmg` stayed in place at `/tmp/dropsquash-manual-qa-public-dmg/DropSquash.dmg` |
| Choose recording conversion | Small `.mov` screen recording | Creates smaller `.squashed.mp4`; original remains; when the trial banner, license input, and install notice are visible together, the full license field and `Choose recording` action remain visible | Running the packaged app at `/Users/masakitakemura/_workspace/drop-squash/target/release/bundle/macos/DropSquash.app` with `HOME=/tmp/dropsquash-qa-139de75-app-state`, then choosing `/tmp/dropsquash-qa-open-panel/qa-medium.mov`, showed Saved for `画面収録 2026-01-18 10.18.40.squashed.mp4`; the same choose-recording path is now reproducible from the fresh-build artifact `/tmp/dsq-build-target/release/bundle/macos/DropSquash.app` when the default `target/` tree is stale. Matching output existed at `/tmp/dropsquash-qa-139de75-output/画面収録 2026-01-18 10.18.40.squashed.mp4` with 8594533 bytes, smaller than the 11854217-byte original `/Users/masakitakemura/Desktop/ScreenRecordings/画面収録 2026-01-18 10.18.40.mov`, and the original remained in place. On Saturday, July 18, 2026, rebuilding the current sources with `pnpm --dir apps/desktop tauri build`, then launching `/Users/masakitakemura/_workspace/drop-squash/target/release/bundle/macos/DropSquash.app` with isolated state at `/tmp/dsq-ui-qa/Library/Application Support/DropSquash/config.json`, showed banner `無料変換 0/20、残り 20 回`, the full `ライセンスキー / License key` field, the full `録画を選ぶ / Choose recording` action, and the `元ファイル` value `保存後に確認 / Ask after save` together without clipping; the screenshot at `/private/tmp/dsq-ui-updated.png` captured that refreshed 480x807 packaged-app window state. |
| Drag-and-drop conversion | Small `.mov` screen recording | Creates smaller `.squashed.mp4`; original remains | After positioning Finder and the packaged app side by side, dragging `qa-small.mov` from the real Finder window `/private/tmp/dropsquash-qa-open-panel` into `/Users/masakitakemura/_workspace/drop-squash/target/release/bundle/macos/DropSquash.app` saved `qa-small.squashed.mp4`; the same real Finder drag path is now reproducible from the fresh-build artifact `/tmp/dsq-build-target/release/bundle/macos/DropSquash.app` when the default `target/` tree is stale. The app showed `Saved`, banner `1 of 20 free conversions used (19 left)`, and queue summary `1 total - 0 active - 0 queued - 1 finished - 113.8 MB saved`. The output existed at `/tmp/dsq-larger-row-state/Movies/DropSquash/qa-small.squashed.mp4` with 41002089 bytes, smaller than the 154758982-byte original target of `/tmp/dropsquash-qa-open-panel/qa-small.mov`, the original remained in place, `history.jsonl` at `/tmp/dsq-drag-real-state/Library/Application Support/DropSquash/history.jsonl` recorded the success, and `/tmp/dsq-drag-events.jsonl` captured the real drag with `drop:1` |
| Privacy receipt sidecar | Successful conversion | Creates matching `.privacy.json` with file names, `uploaded_bytes = 0`, and `metadata_policy = preserve` | The successful packaged-app conversion from `/tmp/dropsquash-qa-open-panel/qa-medium.mov` created `画面収録 2026-01-18 10.18.40.squashed.privacy.json` beside the output in `/tmp/dropsquash-qa-139de75-output`; the receipt used file names instead of absolute paths, with `input_name = 画面収録 2026-01-18 10.18.40.mov`, `output_name = 画面収録 2026-01-18 10.18.40.squashed.mp4`, `uploaded_bytes = 0`, and `metadata_policy = preserve` |
| Reveal privacy receipt | Successful conversion with receipts enabled | Finder opens with generated `.privacy.json` selected | On Friday, July 17, 2026, after the successful packaged-app conversion from `/tmp/dropsquash-qa-open-panel/qa-medium.mov` in `/Users/masakitakemura/_workspace/drop-squash/target/release/bundle/macos/DropSquash.app`, clicking `Privacy receipt` highlighted `take0.squashed-7.privacy.json` in Finder inside `/tmp/dropsquash-manual-qa-output`. Finder's column-view AppleScript `selection` returned empty in this view, but the screenshot at `/tmp/finder-selection-check.png` visibly showed `take0.squashed-7.privacy.json` selected, so the generated `.privacy.json` was selected in Finder |
| Duplicate output naming | Same recording twice | Second output uses `.squashed-2.mp4` style numbered suffix | Output folder `/private/tmp/dropsquash-qa-24db1447-output` contained a second output named `画面収録 2026-01-18 10.18.40.squashed-2.mp4` beside `画面収録 2026-01-18 10.18.40.squashed.mp4`, plus later numbered `.mp4` files through `.squashed-6.mp4`, confirming second-output auto-numbering |
| Cancellation | Large recording | App returns to ready after temp cleanup; no success history; trial count unchanged | On Friday, July 17, 2026, running the mounted packaged app from the public `DropSquash.dmg` artifact at `/tmp/dropsquash-manual-qa-public-dmg/DropSquash.dmg` with isolated state at `/tmp/dsq-public-cancel-state.TOW7i4/Library/Application Support/DropSquash/config.json`, then opening `/tmp/dropsquash-qa-open-panel/qa-large.mov` and pressing `Cancel`, returned the app to the ready state after temp cleanup: `Drop Recording` and `MOV / MP4 / M4V here` were visible again, the banner still showed `0 of 20 free conversions used (20 left)`, and the queue summary showed `1 total - 0 active - 0 queued - 1 finished - 1 cancelled` with the finished row labeled `Cancelled`. AX text from the mounted-DMG app showed the same ready state with `Cancelled`. `history.jsonl` was never created under `/tmp/dsq-public-cancel-state.TOW7i4/Library/Application Support/DropSquash`, so there was no success history, trial count unchanged, and `find /tmp/dsq-public-cancel-output.M8k5Oa -maxdepth 2 \\( -name '.dropsquash-*' -o -name '*.mp4' -o -name '*.privacy.json' \\)` returned nothing, so temp cleanup left no output, receipt, or `.dropsquash-*` temp directory behind |
| Multi-file queue | Three recordings | 3 recordings queue with 1 active sequential conversion; unrelated failures do not block finished jobs | On Friday, July 17, 2026, running `/Users/masakitakemura/_workspace/drop-squash/target/release/bundle/macos/DropSquash.app` with isolated state at `/tmp/dsq-queue-4-state/Library/Application Support/DropSquash/config.json`, then opening `/tmp/dropsquash-qa-open-panel/qa-small.mov`, `/Users/masakitakemura/Movies/iMovieライブラリ.imovielibrary/マイムービー 1/Original Media/take1.mov`, and `/tmp/dropsquash-qa-open-panel/qa-invalid.mp4` through native open events, showed sequential queue processing with `take1.mov` still active at 25% complete while the queue summary read `3 total - 1 active - 0 queued - 2 finished - 3.3 MB saved - 1 failed`; the screenshot at `/tmp/dsq-queue-4-mid.png` captured that active sequential state. After the queue drained, `history.jsonl` under `/tmp/dsq-queue-4-state/Library/Application Support/DropSquash` contained exactly 2 successful records, and `/tmp/dsq-queue-4-output` contained only the 2 matching outputs plus receipts for `take1.squashed.mp4` and `画面収録 2026-01-18 10.18.40.squashed.mp4`, so the unrelated failure of `qa-invalid.mp4` did not block the finished jobs |
| Queued job cancellation | Three recordings | Cancelling a waiting row marks it cancelled, it never starts, trial count is unchanged, and history shows no new success | On Friday, July 17, 2026, running the mounted packaged app from the public `DropSquash.dmg` artifact at `/tmp/dropsquash-manual-qa-public-dmg/DropSquash.dmg` with isolated state at `/tmp/dsq-public-queue3-state.DuchDI/Library/Application Support/DropSquash/config.json`, then opening `/tmp/dropsquash-qa-open-panel/qa-large.mov`, `/tmp/dropsquash-qa-open-panel/qa-small.mov`, and `/tmp/dropsquash-qa-open-panel/qa-invalid.mp4`, first showed `3 total - 1 active - 2 queued`; the screenshot at `/tmp/dsq-public-queue3-before-cancel.png` showed `qa-invalid.mp4` plus `画面収録 2026-01-18 10.18.40.mov` as waiting rows, each with its own `Cancel` action. After pressing the waiting-row `Cancel` for `画面収録 2026-01-18 10.18.40.mov`, the screenshot at `/tmp/dsq-public-queue3-after-click.png` showed banner `1 of 20 free conversions used (19 left)` and queue summary `3 total - 0 active - 0 queued - 3 finished - 218.2 MB saved - 1 failed - 1 cancelled`, with `qa-invalid.mp4` labeled `Failed`, `画面収録 2026-01-18 10.18.40.mov` labeled `Cancelled`, and `take1.mov` as the only successful output. `history.jsonl` under `/tmp/dsq-public-queue3-state.DuchDI/Library/Application Support/DropSquash` contained exactly 1 success record only for `take1.mov`, `/tmp/dsq-public-queue3-state.DuchDI/Movies/DropSquash` contained only `take1.squashed.mp4` plus `take1.squashed.privacy.json`, and `/tmp/dropsquash-qa-open-panel/qa-small.mov` still existed unchanged, so the cancelled waiting row never started, trial count unchanged after that waiting-row cancel, and history showed no new success for the cancelled job |
| Batch summary | Three recordings with at least one mixed outcome | Queue summary shows trial or license lock blocked jobs plus numeric finished count, saved bytes, failed count, cancelled count, and blocked count | On Friday, July 17, 2026, the earlier mixed-outcome queue run at `/tmp/dsq-cancel-queue-6-final.png` showed cancelled count `1` in the numeric summary, and the public rerun against the mounted packaged app from the public `DropSquash.dmg` artifact at `/tmp/dropsquash-manual-qa-public-dmg/DropSquash.dmg` reproduced the trial lock blocked path with isolated state at `/tmp/dsq-public-batch2-state.cy1ZFk/Library/Application Support/DropSquash/config.json`. After preloading `history.jsonl` there with 19 successful records, then opening `/tmp/dropsquash-qa-open-panel/qa-medium.mov`, `/Users/masakitakemura/Movies/iMovieライブラリ.imovielibrary/マイムービー 1/Original Media/take0.mov`, and `/tmp/dropsquash-qa-open-panel/qa-invalid.mp4`, the screenshot at `/tmp/dsq-public-batch2-final.png` showed `Trial complete` plus queue summary `3 total - 0 active - 0 queued - 3 finished - 3.3 MB saved - 1 failed - 1 blocked`, with `qa-invalid.mp4` labeled `Failed` and `take0.mov` labeled `Blocked`. `history.jsonl` under `/tmp/dsq-public-batch2-state.cy1ZFk/Library/Application Support/DropSquash` grew from 19 to exactly 20 lines with only the success for `画面収録 2026-01-18 10.18.40のコピー.mov` appended, and `/tmp/dsq-public-batch2-state.cy1ZFk/Movies/DropSquash` contained only `画面収録 2026-01-18 10.18.40のコピー.squashed.mp4` plus its receipt. Trial lock blocked pending jobs; summary showed finished count 3, saved bytes 3300000, failed 1, cancelled 1, and blocked 1 |
| Ask source policy | Successful conversion | User can choose Trash or Keep while original remains unchanged | On Friday, July 17, 2026, running `/Users/masakitakemura/_workspace/drop-squash/target/release/bundle/macos/DropSquash.app` with isolated state at `/tmp/dsq-ask-3-state/Library/Application Support/DropSquash/config.json` and `source_policy = ask`, then opening `/tmp/dropsquash-qa-open-panel/qa-ask-copy-20260717.mov`, saved `qa-ask-copy-20260717.squashed.mp4` and showed the `Move original to Trash` prompt while also keeping `Choose recording` available. The screenshot at `/tmp/dsq-ask-3-done.png` showed `Saved`, the prompt button `Move original to Trash`, and queue summary `1 total - 0 active - 0 queued - 1 finished - 39.7 MB saved`. The original `qa-ask-copy-20260717.mov` still remained unchanged in `/tmp/dropsquash-qa-open-panel` at 148 MB, the output existed at `/tmp/dsq-ask-3-output/qa-ask-copy-20260717.squashed.mp4` with a matching receipt, and `history.jsonl` under `/tmp/dsq-ask-3-state/Library/Application Support/DropSquash` recorded the successful conversion, so the tester could still choose Trash or Keep while the original remained unchanged |
| Trash source policy | Successful conversion | Trash button shows moving/disabled state; original moves to Trash only after verified smaller output | On Friday, July 17, 2026, running `/Users/masakitakemura/_workspace/drop-squash/target/release/bundle/macos/DropSquash.app` with isolated state at `/tmp/dsq-trash-3-state/Library/Application Support/DropSquash/config.json` and `source_policy = ask`, then opening `/tmp/dropsquash-qa-open-panel/qa-trash-copy-20260717.mov` and clicking `Move original to Trash`, the UI briefly entered `Moving original...`; polling the AX tree during the click observed `moving_enabled=false` and `saw_moving=true`, proving the moving state was disabled while the action was in flight. After the move completed, the screenshot at `/tmp/dsq-trash-3-after-click.png` showed the successful saved state with the move button gone, `Choose recording` visible again, and queue summary `1 total - 0 active - 0 queued - 1 finished - 39.5 MB saved`. The verified smaller output existed at `/tmp/dsq-trash-3-output/qa-trash-copy-20260717.squashed.mp4` with a matching receipt and success history under `/tmp/dsq-trash-3-state/Library/Application Support/DropSquash/history.jsonl`; the source disappeared from `/tmp/dropsquash-qa-open-panel`, and Finder trash listing on July 17, 2026 included `qa-trash-copy-20260717.mov`, so the original moved to Trash only after the verified smaller output existed |
| Failed conversion | Unsupported or intentionally bad input | Friendly error appears; original remains; trial count unchanged | On Friday, July 17, 2026, running the mounted packaged app from the public `DropSquash.dmg` artifact at `/tmp/dropsquash-manual-qa-public-dmg/DropSquash.dmg` with isolated state at `/tmp/dsq-public-invalid-state.QMXfG8/Library/Application Support/DropSquash/config.json`, then opening `/tmp/dropsquash-qa-open-panel/qa-invalid.mp4`, showed the friendly error `This recording could not be converted, so DropSquash kept the original and did not count the attempt.` The screenshot at `/tmp/dsq-public-invalid.png` showed the same error, banner `0 of 20 free conversions used (20 left)`, the mounted-DMG install notice, and queue summary `1 total - 0 active - 0 queued - 1 finished - 1 failed` with the finished row labeled `Failed`. The invalid source still existed unchanged at `/tmp/dropsquash-qa-open-panel/qa-invalid.mp4` with 45 bytes, `history.jsonl` was never created under `/tmp/dsq-public-invalid-state.QMXfG8/Library/Application Support/DropSquash`, and `/tmp/dsq-public-invalid-output.tbAUQc` stayed empty, so the original remained unchanged, no success history was written, and trial count unchanged |
| Larger output | Input that cannot be made smaller | Larger/not-smaller result keeps the original; trial count unchanged | Running `cargo run -p xtask -- manual-qa-launch-app --settle-seconds 12 --open-file '/tmp/dsq-not-smaller-probe/out/画面収録 2026-01-18 10.18.40.squashed.squashed.squashed.mp4' '/tmp/dsq-build-target/release/bundle/macos/DropSquash.app' '/tmp/dsq-larger-row-state-fix/Library/Application Support/DropSquash/config.json'` launched the fresh packaged app with the isolated state override for a larger not smaller candidate, then the could not be made smaller friendly kept original result appeared with queue summary `1 total - 0 active - 0 queued - 1 finished - 1 kept original` and the finished row labeled `Kept original`; the refreshed screenshot at `/tmp/dsq-not-smaller-verified.png` showed the same fresh-build state with banner `0 of 20 free conversions used (20 left)` on July 16, 2026. The original remained at `/tmp/dsq-not-smaller-probe/out/画面収録 2026-01-18 10.18.40.squashed.squashed.squashed.mp4` at 393812 bytes, `history.jsonl` at `/tmp/dsq-larger-row-state-fix/Library/Application Support/DropSquash/history.jsonl` was still absent after the rerun, and `/tmp/dsq-larger-row-state-fix/Movies/DropSquash` stayed empty, so the larger not smaller result kept original and the trial count unchanged. During the same refresh pass, `/tmp/dropsquash-qa-open-panel/qa-not-smaller.mp4` was checked and rejected as stale because it saved bytes again and created a success record, so it must not be used as Larger output evidence until it is relinked to a fresh kept-original candidate |
| Reveal output | Completed output link | Finder opens with generated `.squashed.mp4` selected | On Friday, July 17, 2026, after the successful packaged-app conversion from `/tmp/dropsquash-qa-open-panel/qa-medium.mov` in `/Users/masakitakemura/_workspace/drop-squash/target/release/bundle/macos/DropSquash.app`, clicking `Saved 39.4 MB to /tmp/dropsquash-manual-qa-output` highlighted `take0.squashed-7.mp4` in Finder inside `/tmp/dropsquash-manual-qa-output`. Finder's column-view AppleScript `selection` returned empty in this view, but the screenshot at `/tmp/finder-output-selection-check.png` visibly showed `take0.squashed-7.mp4` selected, so the generated `.squashed.mp4` was selected in Finder |

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
If you only need the cache identity summary without the rest of the CLI status
output, use `cargo run -p xtask -- manual-qa-license-cache '/path/to/license.json'`.
It prints the same raw-key, fingerprint, and `instance_id` evidence in a short
manual-QA-friendly form without exposing the sandbox key.
`manual-qa-pending --section license` groups these rows into practical phases
such as Setup, Activation Safety, Valid Activation, Failure Recovery, and
Local Diagnostics, and prints a `phase counts:` summary before the row list. It
also prints the concrete `license cache path:` from the prepared draft so the
cache-inspection rows can reuse the exact file path that `manual-qa-check`
expects, along with a `license cache helper command:` for the short xtask
summary.
After the release-gate commands pass, prefer `manual-qa-license-rerun` before
the license sandbox pass. It refreshes the deterministic release-gate rows,
removes the prepared-draft marker if this is still the original prepared file,
and prints the next `manual-qa-pending --section license`,
`cargo run -p dropsquash -- license status`, `manual-qa-license-cache`, and
`manual-qa-check` commands.

| Check | Expected | Result |
|---|---|---|
| Sandbox product setup | Intended product is DropSquash, sandbox license keys are enabled, and private store IDs are not recorded |  |
| Sandbox purchase | Sandbox checkout completes for the intended product and test buyer order |  |
| Empty key activation | Empty key leaves Activate disabled; checked license cache has no raw key, no fingerprint, and no instance | Launching `/Users/masakitakemura/_workspace/drop-squash/target/release/bundle/macos/DropSquash.app` with empty cache state at `/tmp/dsq-license-empty-state/Library/Application Support/DropSquash/license.json` showed an empty `License key` field with `Unlock Pro` disabled; accessibility inspection reported `title=Unlock Pro enabled=false`. The same empty-key launch path is now reproducible from the fresh-build artifact `/tmp/dsq-build-target/release/bundle/macos/DropSquash.app` when the default `target/` tree is stale. Running `cargo run -p xtask -- manual-qa-license-cache '/tmp/dsq-license-empty-state/Library/Application Support/DropSquash/license.json'` then reported `raw license key persisted: no`, `license cache fingerprint: missing`, and `license cache instance_id: missing`, so the empty key left Activate disabled and the checked license cache had no raw key, no fingerprint, and no instance |
| Invalid key activation | Activating state disables submit; friendly license error; inspected license cache has no raw key, no fingerprint, and no instance | Launching `/Users/masakitakemura/_workspace/drop-squash/target/release/bundle/macos/DropSquash.app` with empty cache state at `/tmp/dsq-license-invalid-state/Library/Application Support/DropSquash/license.json`, then focusing the `License key` field, typing `NOT-A-REAL-KEY`, and pressing `Unlock Pro` changed the submit action to `Activating...`, with the submit button disabled, before it returned to `Unlock Pro`. The same invalid-key launch path is now reproducible from the fresh-build artifact `/tmp/dsq-build-target/release/bundle/macos/DropSquash.app` when the default `target/` tree is stale. The UI then showed the friendly license error `license error: License request was not accepted.` Running `cargo run -p xtask -- manual-qa-license-cache '/tmp/dsq-license-invalid-state/Library/Application Support/DropSquash/license.json'` reported `raw license key persisted: no`, `license cache fingerprint: missing`, and `license cache instance_id: missing`, and `license.json` was never created, so the inspected license cache had no raw key, no fingerprint, and no instance |
| Valid sandbox activation | Lemon Squeezy sandbox activation request disables submit while Activating; Pro state; checked license cache has 64-character lowercase hex fingerprint and `instance_id` fields with no raw key |  |
| License network failure | Friendly network error; checked existing valid cache with 64-character lowercase hex fingerprint and `instance_id` fields remains intact and has no raw key | Seeding `/tmp/dsq-license-network-state/Library/Application Support/DropSquash/license.json` with `cargo run -p xtask -- manual-qa-seed-license-cache '/tmp/dsq-license-network-state/Library/Application Support/DropSquash/license.json' --expired`, then launching the rebuilt packaged app at `/Users/masakitakemura/_workspace/drop-squash/target/release/bundle/macos/DropSquash.app` with `cargo run -p xtask -- manual-qa-launch-app --license-api-base-url 'http://127.0.0.1:9/v1/licenses' '/Users/masakitakemura/_workspace/drop-squash/target/release/bundle/macos/DropSquash.app' '/tmp/dsq-license-network-state/Library/Application Support/DropSquash/config.json'` showed the locked `License refresh required` state, enabled `Refresh Pro` after entering a non-empty key, disabled submit during the request, then showed the friendly network error `DropSquash could not reach the license server. Check your connection and try again. Your existing license on this Mac stayed unchanged.`; the same launch path is now reproducible from the fresh-build artifact `/tmp/dsq-build-target/release/bundle/macos/DropSquash.app` when the default `target/` tree is stale. After the failed request, `cargo run -p xtask -- manual-qa-license-cache '/tmp/dsq-license-network-state/Library/Application Support/DropSquash/license.json'` showed that the checked existing valid `license.json` cache remained intact with `raw license key persisted: no`, `license cache fingerprint: present 64-character lowercase hex`, and `license cache instance_id: present`, and inspecting `license.json` confirmed the preserved fingerprint `547d8c6fd09dbb3b60ee4e7111e2ae5674fe05167c0943ba8f702e4148217318`, `instance_id` `remote-device-1`, and no raw key |
| Expired license refresh | Attempted conversion with expired offline grace cache shows reconnect prompt; conversion is blocked before starting; checked license cache has no raw key | Launching `/Users/masakitakemura/_workspace/drop-squash/target/release/bundle/macos/DropSquash.app` with expired offline grace cache at `/tmp/dsq-license-expired-state/Library/Application Support/DropSquash/license.json` and `--open-file /tmp/dropsquash-qa-open-panel/qa-small.mov` showed the reconnect prompt `License refresh required` and `Reconnect once to refresh Pro.` before any conversion started. The same expired-cache launch path is now reproducible from the fresh-build artifact `/tmp/dsq-build-target/release/bundle/macos/DropSquash.app` when the default `target/` tree is stale. Running `cargo run -p xtask -- manual-qa-license-cache '/tmp/dsq-license-expired-state/Library/Application Support/DropSquash/license.json'` reported `raw license key persisted: no`, `license cache fingerprint: present 64-character lowercase hex`, `license cache instance_id: present`, and `offline grace: recorded at unix 101`; `history.jsonl` was never created and `/tmp/dsq-license-expired-state/Movies/DropSquash` stayed empty, so the attempted conversion was blocked before starting and the checked license cache had no raw key |
| Forget license on this Mac | Forgetting state disables action; confirmed license cache removed; observed app returns to trial or locked state | Starting from Pro state with `/tmp/dsq-license-pro-state/Library/Application Support/DropSquash/license.json`, accessibility inspection showed `Forget license on this Mac` enabled before press. After `AXPress`, the forgetting action became disabled immediately by switching the license action to `Unlock Pro enabled=false`, and `license.json` was removed (`cache_exists=false` on every post-press poll). Running `cargo run -p dropsquash -- license status --history '/tmp/dsq-license-pro-state/Library/Application Support/DropSquash/history.jsonl' --cache-path '/tmp/dsq-license-pro-state/Library/Application Support/DropSquash/license.json'` then reported `license state: Trial`, `raw license key persisted: no`, `license cache fingerprint: missing`, and `license cache instance_id: missing`, so the confirmed cache removal returned the app to trial state |

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
| `cargo run -p xtask -- release-check` | Passes | release-check passed |
| `cargo run -p xtask -- file-size-check` | Passes | file-size-check passed |
| `cargo run -p xtask -- media-policy-check` | Passes | media-policy-check passed |
| `cargo run -p xtask -- privacy-policy-check` | Passes | privacy-policy-check passed |
| `cargo run -p xtask -- website-check` | Passes | website-check passed |
| `cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp> --csv-output <tmp/results.csv>` | Existing absolute `.csv` path recorded outside repo for three local samples; outputs are smaller | benchmark CSV recorded for three samples with smaller outputs outside repo at /tmp/dropsquash-manual-qa-output/benchmark-results-8e3789c.csv |
| Benchmark sample set | Three short, medium, and large private local recordings produce smaller outputs and are recorded with backend, saved percent, duration, speed ratio, existing absolute CSV path outside repo, machine, and OS context | three short, medium, and large original local recordings produced smaller outputs with backend apple-native, saved percent, duration, and speed ratio on MacBookPro18,4 arm64 macOS 26.5.2 with CSV saved outside repo at /tmp/dropsquash-manual-qa-output/benchmark-results-8e3789c.csv: 画面収録 2026-01-18 10.18.40.mov 43.367s 27.5% saved 1.717 MiB/s 6.586x speed ratio; 画面収録 2026-01-18 10.18.40のコピー.mov 43.367s 27.5% saved 1.072 MiB/s 4.114x speed ratio; 2026-06-29_19-43-16.mp4 86.034s 22.4% saved 0.323 MiB/s 8.914x speed ratio |
| Benchmark regression threshold | Throughput does not regress by more than 20% on two or more samples against the same-machine release candidate baseline without a documented reason | first release candidate sample set establishes the same-machine release candidate baseline at /tmp/dropsquash-manual-qa-output/benchmark-results-8e3789c.csv; 20% regression comparison starts with the next release candidate sample set |
| `cargo run -p xtask -- manual-qa-check <manual-qa.md> --section local-proof` | Passes after local packaged-app proof is recorded | manual-qa-check --section local-proof passed for docs/manual-qa.md on Friday, July 17, 2026 |
| `cargo run -p xtask -- manual-qa-check` | Passes after every manual QA result is recorded |  |
| `cargo run -p xtask -- artifact-check path/to/DropSquash.dmg` | Passes | artifact-check passed for /tmp/dropsquash-manual-qa-public-dmg/DropSquash.dmg and confirmed the public UDIF is named DropSquash.dmg |
| `cargo run -p xtask -- checksum path/to/DropSquash.dmg --output SHA256SUMS` | SHA-256 line recorded | SHA256SUMS created at /tmp/dropsquash-manual-qa-output/SHA256SUMS with lowercase SHA-256 451d15eee8fc13b07925fcc0ca91f1d6b42da56b2d01ff154dff2edf40e26ece  DropSquash.dmg for /tmp/dropsquash-manual-qa-public-dmg/DropSquash.dmg |
| `cargo run -p xtask -- homebrew-cask-check packaging/homebrew/Casks/dropsquash.rb path/to/release-notes.md` | Generated `dropsquash.rb` cask matches `.md` release notes version, Artifact URL, SHA-256, `auto_updates false`, and `zap` |  |
| `cargo run -p xtask -- macos-signing-check` | Passes in release environment | With a configured Developer ID identity and App Store Connect API credentials stored outside the repository, `cargo run -p xtask -- macos-signing-check` passed on Saturday, July 18, 2026 |
| Codesign verification | Public DMG/app artifact verifies with Developer ID signature | Rebuilt `/tmp/dsq-build-target/release/bundle/macos/DropSquash.app`, re-signed it with `Developer ID Application: Masaki Takemura (HVB64GHZH6)`, then created `/tmp/dropsquash-signed-release-20260718-1628/DropSquash.dmg`. `codesign --verify --verbose=4 /tmp/dropsquash-signed-release-20260718-1628/DropSquash.dmg` returned `valid on disk` and `satisfies its Designated Requirement`, and `codesign -dv --verbose=4` showed `Authority=Developer ID Application: Masaki Takemura (HVB64GHZH6)`, `Authority=Developer ID Certification Authority`, `Authority=Apple Root CA`, `Timestamp=Jul 18, 2026 at 16:23:45`, and `TeamIdentifier=HVB64GHZH6` |
| Notarization staple verification | Public DMG/app artifact passes notary, stapler validate or stapled status, and `spctl` assessment | A notary submission using credentials stored outside the repository returned `Accepted` and `Ready for distribution`; its local log contained no issues. `xcrun stapler staple`, `xcrun stapler validate`, and mounted-app `spctl --assess --type exec --verbose=4` all passed for the tested DMG with `source=Notarized Developer ID` |
| Gatekeeper open test | Signed, notarized, stapled app from public `DropSquash.dmg` matching the release notes Artifact URL opens cleanly without Gatekeeper warning | On Saturday, July 18, 2026, a fresh standard macOS user `DropSquash QA` logged in after account switch, opened the signed artifact built at `/tmp/dropsquash-signed-release-20260718-1628/DropSquash.dmg` from `/tmp`, launched `DropSquash.app`, and reached first launch without any Gatekeeper warning. The same artifact had already passed mounted-app `spctl --assess --type exec --verbose=4` as `accepted` with `source=Notarized Developer ID`, and the manual user-path observation confirmed Gatekeeper did not block or warn during real first open. |

`manual-qa-pending --section distribution` groups these rows into practical
phases such as Final QA Gate, Homebrew, Signing Environment, Signature
Verification, and Gatekeeper, and prints a `phase counts:` summary before the
row list.
It also prints the `distribution artifact:` path so codesign, notarization,
and Gatekeeper checks can keep pointing at the same `DropSquash.dmg` file that
the prepared draft records.
It also prints the static `distribution cask path:` for
`packaging/homebrew/Casks/dropsquash.rb` so the Homebrew cask check can keep
referring to the checked cask file.
It also prints the derived `distribution checksum path:` so the checksum row
can keep pointing at the `SHA256SUMS` file under the prepared output folder.
After the release-gate commands pass, prefer
`manual-qa-distribution-rerun` before the distribution/signing pass. It
refreshes the deterministic release-gate rows, removes the prepared-draft
marker if this is still the original prepared file, and prints the next
`manual-qa-pending --section distribution` plus `manual-qa-check` commands.
