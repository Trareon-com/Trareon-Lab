#!/usr/bin/env bash
# Re-verify O1–O3 evidence files exist and are non-empty.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVID="${ROOT}/release-evidence/OFFICIAL-1.0.0"
fail() { echo "VERIFY FAIL: $1" >&2; exit 1; }

for f in windows-sig.txt macos-notarization.json linux-sig.txt; do
  test -s "${EVID}/${f}" || fail "${f} missing or empty"
done
test -s "${EVID}/linux-signing-pubkey.asc" || fail "linux-signing-pubkey.asc missing"
echo "O1–O3 evidence files present (content authenticity still operator-owned)"
