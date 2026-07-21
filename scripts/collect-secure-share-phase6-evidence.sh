#!/bin/sh
set -eu

usage() {
  echo "usage: $0 <fixture-id> <fixture-version> <DropSquash.app> <video.mp4> <mask-plan.json> [report.json]" >&2
  exit 64
}

[ "$#" -ge 5 ] && [ "$#" -le 6 ] || usage
fixture_id=$1
fixture_version=$2
app=$3
video=$4
receipt=$5
report=${6:-"${video%.mp4}.phase6-evidence.json"}

case "$fixture_id:$fixture_version" in
  *[!a-z0-9._:-]* | :) usage ;;
esac
[ -f "$video" ] && [ -f "$receipt" ] || {
  echo "video and receipt must exist" >&2
  exit 66
}
[ -f "$app/Contents/Info.plist" ] || {
  echo "DropSquash.app must contain Contents/Info.plist" >&2
  exit 66
}

signature=$(codesign -dv --verbose=4 "$app" 2>&1) || {
  echo "DropSquash.app must have a valid Developer ID signature" >&2
  exit 65
}
case "$signature" in
  *"Authority=Developer ID Application:"*) ;;
  *) echo "DropSquash.app is not Developer ID signed" >&2; exit 65 ;;
esac

executable="$app/Contents/MacOS/dropsquash-desktop"
[ -f "$executable" ] || {
  echo "DropSquash.app executable is missing" >&2
  exit 66
}
bundle_id=$(plutil -extract CFBundleIdentifier raw "$app/Contents/Info.plist")
app_version=$(plutil -extract CFBundleShortVersionString raw "$app/Contents/Info.plist")
app_sha=$(shasum -a 256 "$executable" | awk '{print $1}')

root=$(git rev-parse --show-toplevel)
target=${CARGO_TARGET_DIR:-/tmp/dropsquash-phase6-evidence-target}
CARGO_TARGET_DIR="$target" cargo run -p xtask -- secure-share-evidence-check "$video" "$receipt"

video_name=$(basename "$video")
receipt_name=$(basename "$receipt")
video_sha=$(shasum -a 256 "$video" | awk '{print $1}')
receipt_sha=$(shasum -a 256 "$receipt" | awk '{print $1}')
timestamp=$(date -u +%Y-%m-%dT%H:%M:%SZ)

cat >"$report" <<EOF
{
  "schemaVersion": 1,
  "fixtureId": "$fixture_id",
  "fixtureVersion": "$fixture_version",
  "platform": "macos",
  "outcome": "strict-shield-pass",
  "appBundleId": "$bundle_id",
  "appVersion": "$app_version",
  "appExecutableSha256": "$app_sha",
  "developerIdSigned": true,
  "videoFileName": "$video_name",
  "videoSha256": "$video_sha",
  "receiptFileName": "$receipt_name",
  "receiptSha256": "$receipt_sha",
  "verification": "independent-decoder-and-signed-receipt-pass",
  "verifiedAtUtc": "$timestamp"
}
EOF

echo "phase 6 evidence report: $report"
echo "repository: $root"
