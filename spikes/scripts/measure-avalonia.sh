#!/usr/bin/env bash
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
# shellcheck disable=SC1090
source "$HOME/.cargo/env" 2>/dev/null || true

echo "==> build harness"
cd "$ROOT"
cargo build -p lab-spike-harness --release

echo "==> dotnet run Avalonia measure"
cd "$ROOT/avalonia"
dotnet run -c Release -- --measure --os "$OS" --rows 1000000 --filter-prefix 0 \
  --case-dir "$ROOT/results/${OS}-avalonia-case" \
  --out "$ROOT/results/${OS}-avalonia.json"

cat "$ROOT/results/${OS}-avalonia.json"
