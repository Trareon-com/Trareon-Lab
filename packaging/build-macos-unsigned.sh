#!/usr/bin/env bash
# Day 51: macOS unsigned .app.zip style packaging (binary + checksum).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="${ROOT}/dist/0.9.0/macos-arm64"
mkdir -p "${OUT}"
cargo build -p lab-slint --release --features gui
BIN="$(ls "${ROOT}"/target/release/trareon-lab 2>/dev/null || ls "${ROOT}"/target/release/lab-slint 2>/dev/null || true)"
if [[ -z "${BIN}" ]]; then
  echo "binary not found; listing target/release:" >&2
  ls "${ROOT}/target/release" | head
  exit 1
fi
cp "${BIN}" "${OUT}/trareon-lab"
(cd "${OUT}" && shasum -a 256 trareon-lab > SHA256SUMS.txt)
echo "wrote ${OUT}"
