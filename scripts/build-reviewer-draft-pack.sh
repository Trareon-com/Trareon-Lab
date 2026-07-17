#!/usr/bin/env bash
# Assemble Path D reviewer draft pack (no secrets, no signed installers).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="${ROOT}/dist/reviewer-draft-pack"
rm -rf "${OUT}"
mkdir -p "${OUT}/docs/user" "${OUT}/docs/validation" "${OUT}/docs/reviews"

copy() {
  local src="$1"
  local dest_rel="$2"
  local dest="${OUT}/${dest_rel}"
  mkdir -p "$(dirname "${dest}")"
  cp "${ROOT}/${src}" "${dest}"
}

copy docs/RELEASE-01-CAPABILITY-MATRIX.md docs/RELEASE-01-CAPABILITY-MATRIX.md
copy docs/SELLING-UNSIGNED.md docs/SELLING-UNSIGNED.md
copy docs/OFFICIAL-RELEASE-RUNBOOK.md docs/OFFICIAL-RELEASE-RUNBOOK.md
copy docs/KNOWN-ISSUES.md docs/KNOWN-ISSUES.md
copy docs/user/RELEASE-NOTES-1.0.0.md docs/user/RELEASE-NOTES-1.0.0.md
copy docs/validation/STORAGE-DOSSIER.md docs/validation/STORAGE-DOSSIER.md
copy docs/validation/ARTIFACTS-DOSSIER.md docs/validation/ARTIFACTS-DOSSIER.md
copy docs/reviews/REVIEWER-BOOKING.md docs/reviews/REVIEWER-BOOKING.md
copy docs/reviews/INDONESIA-OFFICIAL-SIGNOFF.md docs/reviews/INDONESIA-OFFICIAL-SIGNOFF.md
copy docs/reviews/CRYPTO-EXTERNAL-REVIEW.md docs/reviews/CRYPTO-EXTERNAL-REVIEW.md
copy docs/reviews/LEGAL-QUALITY-MEETING-AGENDA.md docs/reviews/LEGAL-QUALITY-MEETING-AGENDA.md
copy docs/DECISION-REGISTER.md docs/DECISION-REGISTER.md
copy docs/WINDOWS-LAB-QUEUE.md docs/WINDOWS-LAB-QUEUE.md

{
  echo "Trareon Lab reviewer draft pack"
  echo "assembled_utc=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo "git_tip=$(git -C "${ROOT}" rev-parse HEAD)"
  echo "note=Engineering Alpha / Official program materials; not an Official Production claim"
} > "${OUT}/PACK-MANIFEST.txt"

echo "Reviewer draft pack ready: ${OUT}"
