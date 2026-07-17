#!/usr/bin/env bash
# Gate A C-TAURI measure for macOS/Linux
set -euo pipefail

OS="${1:-}"
if [[ -z "$OS" ]]; then
  case "$(uname -s)" in
    Darwin) OS=macos ;;
    Linux) OS=linux ;;
    *) echo "usage: $0 <macos|linux>"; exit 2 ;;
  esac
fi

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT/tauri"

# shellcheck disable=SC1090
source "$HOME/.cargo/env" 2>/dev/null || true

echo "==> npm install"
npm install

echo "==> frontend build"
npm run build

echo "==> cargo build release"
cd src-tauri
cargo build --release

BIN="$(cargo metadata --format-version 1 --no-deps | python3 -c 'import json,sys; print(json.load(sys.stdin)["target_directory"])')/release/lab-spike-tauri"
echo "==> binary: $BIN"

OUT="$ROOT/results/${OS}-tauri.json"
CASE="$ROOT/results/${OS}-tauri-case"
mkdir -p "$ROOT/results"

"$BIN" --measure --os "$OS" --rows 1000000 --filter-prefix 0 \
  --case-dir "$CASE" --out "$OUT"

echo "==> result:"
cat "$OUT"
