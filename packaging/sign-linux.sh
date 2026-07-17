#!/usr/bin/env bash
# Official O3 pipeline: sign Linux package with GPG; write verification evidence.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVID="${ROOT}/release-evidence/OFFICIAL-1.0.0"
OUT="${ROOT}/dist/1.0.0/linux-x64"
mkdir -p "${OUT}" "${EVID}"

need() { [[ -n "${!1:-}" ]] || { echo "MISSING env: $1" >&2; exit 1; }; }
need LINUX_SIGNING_KEY_ID

bash "${ROOT}/packaging/build-linux-unsigned.sh"
if [[ -d "${ROOT}/dist/0.9.0/linux-x64" ]]; then
  cp -R "${ROOT}/dist/0.9.0/linux-x64/." "${OUT}/"
fi
PKG="${OUT}/trareon-lab-linux-x64.tar.gz"
test -f "${PKG}" || { echo "missing ${PKG}" >&2; exit 1; }

gpg --local-user "${LINUX_SIGNING_KEY_ID}" --detach-sign --armor "${PKG}"
gpg --verify "${PKG}.asc" "${PKG}" 2>&1 | tee "${EVID}/linux-sig.txt"
gpg --armor --export "${LINUX_SIGNING_KEY_ID}" > "${EVID}/linux-signing-pubkey.asc"
echo "O3 evidence written: ${EVID}/linux-sig.txt + linux-signing-pubkey.asc"
