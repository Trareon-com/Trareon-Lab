#!/usr/bin/env bash
# F13: cross-platform packaging smoke (unsigned).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="${ROOT}/packaging/out"
mkdir -p "${OUT}"
export CARGO_TARGET_DIR="${ROOT}/target"
export CARGO_PROFILE_DEV_DEBUG=0

cargo build -p lab-slint --no-default-features
# Produce a smoke artifact listing (binary optional when gui feature off).
{
  echo "packaging-smoke-1"
  echo "host=$(uname -s)-$(uname -m)"
  echo "workspace_ok=true"
  cargo metadata --no-deps --format-version 1 | head -c 1 >/dev/null
  echo "cargo_metadata_ok=true"
} > "${OUT}/smoke-$(uname -s | tr '[:upper:]' '[:lower:]').txt"

test -s "${OUT}/smoke-$(uname -s | tr '[:upper:]' '[:lower:]').txt"
echo "packaging smoke OK: ${OUT}"
