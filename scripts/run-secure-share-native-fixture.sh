#!/bin/sh
set -eu

root=$(git rev-parse --show-toplevel)
fixture="$root/tests/fixtures/secure-share/NativeAccessibilityFixture.swift"
bundle=/tmp/DropSquashNativeAccessibilityFixture.app
binary="$bundle/Contents/MacOS/DropSquashNativeAccessibilityFixture"
scenario=${1:-baseline}

case "$scenario" in
  baseline) ;;
  hide) export DROP_SQUASH_FIXTURE_AUTORUN_HIDE_WINDOW=1 ;;
  resize) export DROP_SQUASH_FIXTURE_AUTORUN_RESIZE_WINDOW=1 ;;
  move) export DROP_SQUASH_FIXTURE_AUTORUN_MOVE_WINDOW=1 ;;
  move-return) export DROP_SQUASH_FIXTURE_AUTORUN_MOVE_RETURN=1 ;;
  focus-loss) export DROP_SQUASH_FIXTURE_AUTORUN_FOREGROUND_SWITCH=1 ;;
  same-app-focus) export DROP_SQUASH_FIXTURE_AUTORUN_SAME_APP_WINDOW_FOCUS=1 ;;
  rapid-burst) export DROP_SQUASH_FIXTURE_AUTORUN_RAPID_BURST=1 ;;
  input-burst) export DROP_SQUASH_FIXTURE_AUTORUN_INPUT_BURST=1 ;;
  popover) export DROP_SQUASH_FIXTURE_AUTORUN_POPOVER=1 ;;
  sheet) export DROP_SQUASH_FIXTURE_AUTORUN_SHEET=1 ;;
  title) export DROP_SQUASH_FIXTURE_AUTORUN_TITLE_CHANGE=1 ;;
  *)
    echo "usage: $0 [baseline|hide|resize|move|move-return|focus-loss|same-app-focus|rapid-burst|input-burst|popover|sheet|title]" >&2
    exit 64
    ;;
esac

rm -rf "$bundle"
mkdir -p "$bundle/Contents/MacOS"
swiftc "$fixture" -o "$binary"
cat >"$bundle/Contents/Info.plist" <<'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>CFBundleExecutable</key><string>DropSquashNativeAccessibilityFixture</string>
<key>CFBundleIdentifier</key><string>app.dropsquash.native-fixture</string>
<key>CFBundleName</key><string>DropSquash Native Fixture</string>
<key>CFBundlePackageType</key><string>APPL</string>
</dict></plist>
EOF
open -n "$bundle"
sleep 1
pid=$(pgrep -f "$binary" | head -n 1 || true)
[ -n "$pid" ] || { echo "fixture did not start" >&2; exit 1; }
echo "fixture scenario: $scenario"
echo "fixture pid: $pid"
echo "fixture bundle: $bundle"
