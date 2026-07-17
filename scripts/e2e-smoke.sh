#!/usr/bin/env bash
# Day 45 E2E smoke on primary machine (no GUI required for most steps).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "${ROOT}"
cargo test --workspace --exclude lab-slint
cargo test -p lab-slint --no-default-features --tests
cargo test -p lab-artifacts --tests
cargo test -p lab-transfer --tests
cargo test -p lab-timeline --lib
./packaging/smoke.sh
echo "E2E core path OK"
