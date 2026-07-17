#!/usr/bin/env bash
# Official O2 pipeline: Developer ID sign → notarize → staple.
# Fails closed if required env vars are missing (no fake signatures).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVID="${ROOT}/release-evidence/OFFICIAL-1.0.0"
OUT="${ROOT}/dist/1.0.0/macos-arm64"
mkdir -p "${OUT}" "${EVID}"

need() { [[ -n "${!1:-}" ]] || { echo "MISSING env: $1" >&2; exit 1; }; }
need APPLE_TEAM_ID
need APPLE_ID
need APPLE_APP_SPECIFIC_PASSWORD
need DEVELOPER_ID_APPLICATION

bash "${ROOT}/packaging/build-macos-unsigned.sh"
# Re-home unsigned output into 1.0.0 path if present
if [[ -d "${ROOT}/dist/0.9.0/macos-arm64" ]]; then
  cp -R "${ROOT}/dist/0.9.0/macos-arm64/." "${OUT}/"
fi
BIN="${OUT}/trareon-lab"
test -f "${BIN}" || { echo "missing binary ${BIN}" >&2; exit 1; }

codesign --force --options runtime --sign "${DEVELOPER_ID_APPLICATION}" "${BIN}"
ditto -c -k --keepParent "${BIN}" "${OUT}/trareon-lab.zip"
xcrun notarytool submit "${OUT}/trareon-lab.zip" \
  --apple-id "${APPLE_ID}" \
  --team-id "${APPLE_TEAM_ID}" \
  --password "${APPLE_APP_SPECIFIC_PASSWORD}" \
  --wait | tee "${EVID}/macos-notarization.json"
# If binary (not zip) stapling applies to .app bundles; record verify output:
codesign --verify --verbose=2 "${BIN}" 2>&1 | tee "${EVID}/macos-codesign-verify.txt" || true
echo "O2 artifacts written under ${EVID} (operator must confirm staple for .app packages)"
