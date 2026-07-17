#!/usr/bin/env bash
# Gate A Slint measure for Kali/Linux — safe pull + rebuild + run
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT/.."

echo "==> repo: $(pwd)"
mkdir -p spikes/results

# Unblock git pull if local harness JSON conflicts with remote
if [[ -f spikes/results/linux-harness-core.json ]]; then
  echo "==> moving local linux-harness-core.json aside"
  mv -f spikes/results/linux-harness-core.json "spikes/results/linux-harness-core.json.local.bak" || true
fi

echo "==> git pull"
git checkout main
git pull origin main

# shellcheck disable=SC1090
source "$HOME/.cargo/env" 2>/dev/null || true

cd spikes
echo "==> cargo build slint-app (release)"
cargo build -p slint-app --release

echo "==> running measure"
./target/release/lab-spike-slint \
  --measure \
  --os linux \
  --rows 1000000 \
  --filter-prefix 0 \
  --case-dir "./results/linux-slint-case" \
  --out "./results/linux-slint.json"

echo "==> exit=$?"
echo "==> result file:"
ls -la ./results/linux-slint.json
echo "==> contents:"
cat ./results/linux-slint.json
