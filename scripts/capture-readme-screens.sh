#!/usr/bin/env bash
# Recapture docs/media/feature-*.png on macOS.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
MEDIA="$ROOT/docs/media"
CASE="${TRAREON_README_CASE:-/tmp/trareon-readme-case}"
BIN="$ROOT/target/debug/trareon-lab"
mkdir -p "$CASE" "$MEDIA"
[[ -f "$CASE/sample.dd" ]] || dd if=/dev/zero of="$CASE/sample.dd" bs=1024 count=64 status=none
[[ -x "$BIN" ]] || {
  echo "build first: cargo build -p lab-slint --bin trareon-lab --features gui"
  exit 1
}

move_win() {
  osascript <<'APPLESCRIPT'
tell application "System Events"
  set p to first process whose name is "trareon-lab"
  set frontmost of p to true
  tell window 1 of p
    set position to {80, 60}
    set size to {1280, 800}
  end tell
end tell
APPLESCRIPT
}

win_id() {
  swift -e 'import Cocoa; let opts=CGWindowListOption(arrayLiteral:.optionOnScreenOnly,.excludeDesktopElements); guard let info=CGWindowListCopyWindowInfo(opts,kCGNullWindowID) as? [[String:Any]] else {fatalError()}; for w in info { let o=w[kCGWindowOwnerName as String] as? String ?? ""; if o=="trareon-lab" { print(w[kCGWindowNumber as String] as! Int); break } }'
}

capture_running() {
  local out="$1"
  move_win
  sleep 0.45
  screencapture -l "$(win_id)" -o -x "$MEDIA/$out.png"
  echo "$out $(md5 -q "$MEDIA/$out.png")"
}

start_bin() {
  pkill -x trareon-lab 2>/dev/null || true
  sleep 0.35
  (cd "$ROOT" && env "$@" "$BIN") &
  sleep 3.2
}

start_bin -u TRAREON_CASE_DIR -u TRAREON_AUTO_OPEN -u TRAREON_IMPORT_AUTO -u TRAREON_START_SCREEN -u TRAREON_PALETTE
capture_running feature-home

for pair in \
  "feature-case-with-evidence:Case" \
  "feature-evidence:Evidence" \
  "feature-hex:Hex" \
  "feature-artifacts:Artifacts" \
  "feature-search:Search" \
  "feature-timeline:Timeline" \
  "feature-bookmarks:Bookmarks" \
  "feature-graph:Graph" \
  "feature-runs:Runs" \
  "feature-report:Report" \
  "feature-transfer:Transfer" \
  "feature-capabilities:Capabilities" \
  "feature-about:About"; do
  out="${pair%%:*}"
  screen="${pair##*:}"
  start_bin \
    TRAREON_CASE_DIR="$CASE" \
    TRAREON_IMPORT_PATH="$CASE/sample.dd" \
    TRAREON_AUTO_OPEN=1 \
    TRAREON_IMPORT_AUTO=1 \
    TRAREON_START_SCREEN="$screen"
  capture_running "$out"
done

start_bin \
  TRAREON_CASE_DIR="$CASE" \
  TRAREON_IMPORT_PATH="$CASE/sample.dd" \
  TRAREON_AUTO_OPEN=1 \
  TRAREON_IMPORT_AUTO=1 \
  TRAREON_START_SCREEN=Case \
  TRAREON_PALETTE=1
capture_running feature-palette

cp -f "$MEDIA/feature-home.png" "$MEDIA/trareon-lab-home.png"
pkill -x trareon-lab 2>/dev/null || true
uniq=$(md5 -r "$MEDIA"/feature-*.png | awk '{print $1}' | sort -u | wc -l | tr -d ' ')
echo "done — ${uniq} unique feature hashes"
