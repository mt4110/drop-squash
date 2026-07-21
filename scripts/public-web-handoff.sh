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

printf '%s\n' "$snapshot_meta"
echo "next public web gate: cd '$snapshot' && cargo run -p xtask -- public-web-ready"
echo "next site verify: cd '$snapshot/apps/site' && npm run verify:site"
echo "next public web probe: cd '$snapshot' && cargo run -p xtask -- public-web-probe"
echo "next public web track: cd '$snapshot' && cargo run -p xtask -- productization-status --track 'Public web proof'"
echo "next package archive: cd '$snapshot/apps/site' && npm run package:site"
echo "next sites note: save and deploy only after the intended site source is committed from this snapshot"
echo "next canonical host: verify https://dropsquash.app/release-status https://dropsquash.app/pricing https://dropsquash.app/refund after DNS and SSL validation finish"
