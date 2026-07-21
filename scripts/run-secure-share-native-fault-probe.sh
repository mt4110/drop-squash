#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
app=${DROP_SQUASH_APP_PATH:-"$root/target/release/bundle/macos/DropSquash.app"}
binary="$app/Contents/MacOS/dropsquash-desktop"

if [[ ! -x "$binary" ]]; then
  printf 'Signed DropSquash.app was not found: %s\n' "$binary" >&2
  exit 1
fi
if pgrep -f "^$binary$" >/dev/null; then
  printf 'Quit the running DropSquash app before starting the native fault probe.\n' >&2
  exit 1
fi

exec env \
  DROP_SQUASH_QA_ENABLE_NATIVE_FAULTS=1 \
  DROP_SQUASH_QA_NATIVE_VISION_FAIL_FRAME="${DROP_SQUASH_QA_NATIVE_VISION_FAIL_FRAME:-0}" \
  "$binary"
