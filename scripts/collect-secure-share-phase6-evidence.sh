#!/bin/sh
set -eu

usage() {
  echo "usage: $0 <fixture-id> <fixture-version> <video.mp4> <mask-plan.json> [report.json]" >&2
  exit 64
}

[ "$#" -ge 4 ] && [ "$#" -le 5 ] || usage
fixture_id=$1
fixture_version=$2
video=$3
receipt=$4
report=${5:-"${video%.mp4}.phase6-evidence.json"}

case "$fixture_id:$fixture_version" in
  *[!a-z0-9._:-]* | :) usage ;;
esac
[ -f "$video" ] && [ -f "$receipt" ] || {
  echo "video and receipt must exist" >&2
  exit 66
}

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
