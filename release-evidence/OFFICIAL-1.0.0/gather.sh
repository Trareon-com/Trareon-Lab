#!/usr/bin/env bash
# Official R1.0.0 evidence gather — fails closed until O1–O12 artifacts exist.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
EVID="${ROOT}/release-evidence/OFFICIAL-1.0.0"
fail() { echo "MISSING: $1" >&2; exit 1; }

test -f "${EVID}/README.md" || fail "OFFICIAL README"
test -f "${EVID}/GATES.md" || fail "GATES.md"
test -f "${EVID}/MANIFEST.txt" || fail "MANIFEST.txt (fill at freeze)"
test -f "${EVID}/windows-sig.txt" || fail "O1 windows-sig.txt"
test -f "${EVID}/macos-notarization.json" || fail "O2 macos-notarization.json"
test -f "${EVID}/linux-sig.txt" || fail "O3 linux-sig.txt"
test -f "${EVID}/sbom.cdx.json" || fail "O4 sbom.cdx.json"
test -f "${ROOT}/docs/reviews/INDONESIA-OFFICIAL-SIGNOFF.md" || fail "O8 Indonesia sign-off"
test -f "${ROOT}/docs/reviews/CRYPTO-EXTERNAL-REVIEW.md" || fail "O9 crypto review"
test -f "${ROOT}/docs/OFFICIAL-RELEASE-RUNBOOK.md" || fail "official runbook"

echo "Official gather cannot PASS yet: required evidence files are incomplete by design until Path C/D finish."
exit 1
