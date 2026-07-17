#!/usr/bin/env bash
# Publish Official GitHub Release with signed assets — fails closed without evidence.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TAG="${1:-v1.0.0}"
EVID="${ROOT}/release-evidence/OFFICIAL-1.0.0"

bash "${ROOT}/release-evidence/OFFICIAL-1.0.0/gather.sh"
test -n "$(git -C "${ROOT}" tag -l "${TAG}")" || {
  echo "MISSING tag ${TAG} — run scripts/cut-official-v1.sh first" >&2
  exit 1
}

ASSETS=()
for f in \
  "${EVID}/windows-sig.txt" \
  "${EVID}/macos-notarization.json" \
  "${EVID}/linux-sig.txt" \
  "${EVID}/linux-signing-pubkey.asc" \
  "${EVID}/sbom.cdx.json" \
  "${EVID}/MANIFEST.txt" \
  "${ROOT}/docs/RELEASE-01-CAPABILITY-MATRIX.md" \
  "${ROOT}/docs/KNOWN-ISSUES.md" \
  "${ROOT}/docs/user/RELEASE-NOTES-1.0.0.md"
do
  test -e "$f" || { echo "MISSING asset: $f" >&2; exit 1; }
  ASSETS+=("$f")
done

# Optional binary packages if present (Windows lab / signed hosts).
for f in \
  "${ROOT}/dist/1.0.0/windows-x64/trareon-lab.exe" \
  "${ROOT}/dist/1.0.0/macos-arm64/trareon-lab.zip" \
  "${ROOT}/dist/1.0.0/linux-x64/trareon-lab-linux-x64.tar.gz" \
  "${ROOT}/dist/1.0.0/linux-x64/trareon-lab-linux-x64.tar.gz.asc"
do
  [[ -e "$f" ]] && ASSETS+=("$f")
done

gh release create "${TAG}" "${ASSETS[@]}" \
  --title "Trareon Lab ${TAG} Official Production" \
  --notes-file "${ROOT}/docs/user/RELEASE-NOTES-1.0.0.md"

git -C "${ROOT}" push origin "${TAG}"
echo "Official release published for ${TAG}"
echo "Next: bash scripts/close-official-program.sh ${TAG}"
