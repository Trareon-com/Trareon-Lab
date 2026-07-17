#!/usr/bin/env bash
# Move local spike result JSON aside so git pull can update tracked copies.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"
mkdir -p spikes/results
shopt -s nullglob
for f in spikes/results/*-harness-core.json spikes/results/*-slint.json spikes/results/*-tauri.json spikes/results/*-avalonia.json; do
  if [[ -f "$f" ]] && ! git ls-files --error-unmatch "$f" >/dev/null 2>&1; then
    echo "moving untracked $f -> ${f}.local.bak"
    mv -f "$f" "${f}.local.bak"
  fi
done
git pull origin main
echo "pull ok"
