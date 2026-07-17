#!/usr/bin/env bash
set -euo pipefail
OS="${1:-macos}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
# shellcheck disable=SC1090
source "$HOME/.cargo/env" 2>/dev/null || true
cd "$ROOT"
cargo build -p lab-spike-index --release
BIN="$(cargo metadata --format-version 1 --no-deps | python3 -c 'import json,sys; print(json.load(sys.stdin)["target_directory"])')/release/lab-spike-index"
mkdir -p results
for c in D-SQLITE D-SQLITE-FTS D-RUST-INDEX; do
  slug=$(echo "$c" | tr 'A-Z' 'a-z')
  echo "==> $c"
  "$BIN" measure --candidate "$c" --os "$OS" --rows 1000000 \
    --case-dir "results/${OS}-${slug}-case" \
    --out "results/${OS}-${slug}.json"
done
echo "==> done"
