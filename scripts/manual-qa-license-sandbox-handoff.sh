#!/bin/sh
set -eu

root=$(git rev-parse --show-toplevel)
cd "$root"

snapshot_output=$(scripts/manual-qa-snapshot-worktree.sh "${1:-}")
snapshot=$(printf '%s\n' "$snapshot_output" | sed -n 's/^snapshot worktree: //p')
snapshot_meta=$(printf '%s\n' "$snapshot_output" | rg '^snapshot (worktree|source|commit):')

if [ -z "$snapshot" ]; then
  echo "failed to create snapshot worktree" >&2
  exit 1
fi

commit=$(git -C "$snapshot" rev-parse --short=7 HEAD)
target_dir=/tmp/dsq-xtask-target
app_state_dir="/tmp/dropsquash-manual-qa-app-state/Library/Application Support/DropSquash"
state_dir=/tmp/dropsquash-manual-qa-state
output_dir=/tmp/dropsquash-manual-qa-output
prepared_md="/tmp/dropsquash-manual-qa-prepared-$commit.md"

printf '%s\n' "$snapshot_meta"
echo "next status: cd '$snapshot' && CARGO_TARGET_DIR=$target_dir cargo run -p xtask -- productization-status --track 'Paid beta'"
echo "next desktop install: cd '$snapshot' && nix develop --command pnpm --dir apps/desktop install --frozen-lockfile"
echo "next web install: cd '$snapshot' && nix develop --command pnpm --dir apps/desktop/web install --frozen-lockfile"
echo "next file-size gate: cd '$snapshot' && CARGO_TARGET_DIR=$target_dir cargo run -p xtask -- file-size-check"
echo "next release gate: cd '$snapshot' && CARGO_TARGET_DIR=$target_dir cargo run -p xtask -- release-check"
echo "next build: cd '$snapshot' && nix develop --command pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci"
echo "next prepare: cd '$snapshot' && CARGO_TARGET_DIR=$target_dir cargo run -p xtask -- manual-qa-prepare --reset-trial --app-artifact target/release/bundle/dmg/DropSquash.dmg --input-sample-set \"short, medium, and large local recordings\" --app-state-dir \"$app_state_dir\" --state-dir $state_dir --output-dir $output_dir --markdown-output $prepared_md"
echo "next license rerun: cd '$snapshot' && CARGO_TARGET_DIR=$target_dir cargo run -p xtask -- manual-qa-license-rerun '$prepared_md'"
echo "next paid beta gate: cd '$snapshot' && CARGO_TARGET_DIR=$target_dir cargo run -p xtask -- paid-beta-check"
