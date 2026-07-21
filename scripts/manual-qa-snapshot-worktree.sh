#!/bin/sh
set -eu

root=$(git rev-parse --show-toplevel)
cd "$root"

commit=$(git rev-parse --short HEAD)
stamp=$(date +%Y%m%d-%H%M%S)
target=${1:-"/tmp/dropsquash-qa-snapshot-$commit-$stamp"}
index=$(mktemp /tmp/dropsquash-qa-index.XXXXXX)

if [ -e "$target" ]; then
  echo "snapshot target already exists: $target" >&2
  exit 1
fi

cleanup() {
  code=$?
  rm -f "$index"
  if [ $code -ne 0 ] && [ -d "$target" ]; then
    git worktree remove --force "$target" >/dev/null 2>&1 || true
  fi
  exit $code
}
trap cleanup INT TERM EXIT

GIT_INDEX_FILE=$index git read-tree HEAD
GIT_INDEX_FILE=$index git add -A
tree=$(GIT_INDEX_FILE=$index git write-tree)
snapshot=$(printf 'qa snapshot\n' | git commit-tree "$tree" -p HEAD)
git worktree add --detach "$target" "$snapshot" >/dev/null

if ! git -C "$target" diff --quiet HEAD; then
  echo "snapshot worktree is not clean: $target" >&2
  exit 1
fi

trap - INT TERM EXIT
rm -f "$index"
echo "snapshot worktree: $target"
echo "snapshot source: $root"
echo "snapshot commit: $snapshot"
echo "next build: cd '$target' && nix develop --command pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci"
