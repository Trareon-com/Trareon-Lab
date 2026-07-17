#!/usr/bin/env bash
# Day 53: Linux unsigned tarball.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="${ROOT}/dist/0.9.0/linux-x64"
mkdir -p "${OUT}"
cargo build -p lab-slint --release --features gui
BIN="${ROOT}/target/release/trareon-lab"
test -x "${BIN}" || BIN="${ROOT}/target/release/lab-slint"
cp "${BIN}" "${OUT}/trareon-lab"
tar -C "${OUT}" -czf "${OUT}/trareon-lab-linux-x64.tar.gz" trareon-lab
(cd "${OUT}" && sha256sum trareon-lab trareon-lab-linux-x64.tar.gz > SHA256SUMS.txt)
echo "wrote ${OUT}"
