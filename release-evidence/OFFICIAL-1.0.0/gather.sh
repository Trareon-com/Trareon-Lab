#!/usr/bin/env bash
# Official R1.0.0 evidence gather — fails closed until O1–O12 artifacts are real.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
EVID="${ROOT}/release-evidence/OFFICIAL-1.0.0"
fail() { echo "MISSING: $1" >&2; exit 1; }

test -f "${EVID}/README.md" || fail "OFFICIAL README"
test -f "${EVID}/GATES.md" || fail "GATES.md"
test -s "${EVID}/MANIFEST.txt" || fail "MANIFEST.txt (fill at freeze)"

# O1–O4 packaging / SBOM evidence (never accept *.example as PASS)
test -s "${EVID}/windows-sig.txt" || fail "O1 windows-sig.txt"
grep -qi 'EXAMPLE' "${EVID}/windows-sig.txt" && fail "O1 windows-sig.txt still EXAMPLE"
test -s "${EVID}/macos-notarization.json" || fail "O2 macos-notarization.json"
grep -q 'EXAMPLE_ONLY\|REPLACE_TEAM_ID' "${EVID}/macos-notarization.json" && fail "O2 macos-notarization.json still example"
test -s "${EVID}/linux-sig.txt" || fail "O3 linux-sig.txt"
grep -qi 'REPLACE' "${EVID}/linux-sig.txt" && fail "O3 linux-sig.txt still template"
test -s "${EVID}/linux-signing-pubkey.asc" || fail "O3 linux-signing-pubkey.asc"
test -s "${EVID}/sbom.cdx.json" || fail "O4 sbom.cdx.json"

# O8 / O9 human reviews — files must exist and not be stub states
test -f "${ROOT}/docs/reviews/INDONESIA-OFFICIAL-SIGNOFF.md" || fail "O8 Indonesia sign-off"
grep -q '(not signed)' "${ROOT}/docs/reviews/INDONESIA-OFFICIAL-SIGNOFF.md" && fail "O8 still unsigned"
test -f "${ROOT}/docs/reviews/CRYPTO-EXTERNAL-REVIEW.md" || fail "O9 crypto review"
grep -q 'NOT_RECEIVED' "${ROOT}/docs/reviews/CRYPTO-EXTERNAL-REVIEW.md" && fail "O9 still NOT_RECEIVED"

# O10 physical smoke — must not remain NOT_RUN
for f in macos-macbook.json linux-kali.json windows-thinkpad.json; do
  test -s "${EVID}/o10/${f}" || fail "O10 ${f}"
  grep -q '"status": "NOT_RUN"' "${EVID}/o10/${f}" && fail "O10 ${f} still NOT_RUN"
done

# O11 evidence log
test -s "${EVID}/o11-bookmark-transfer.log" || fail "O11 o11-bookmark-transfer.log"

# Runbook + supporting reviews present
test -f "${ROOT}/docs/OFFICIAL-RELEASE-RUNBOOK.md" || fail "official runbook"
test -f "${ROOT}/docs/reviews/SBOM-VULN-LICENSE-REVIEW.md" || fail "O4 SBOM review record"
grep -q 'Security Engineering acceptance | NO' "${ROOT}/docs/reviews/SBOM-VULN-LICENSE-REVIEW.md" && fail "O4 SBOM review not accepted"
test -f "${ROOT}/docs/reviews/TRANSFER-SECURITY-REVIEW.md" || fail "transfer security review"

echo "Official gather PASS: required evidence files present and non-stub."
exit 0
