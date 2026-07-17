#!/usr/bin/env bash
# Cut annotated source tag. Storefront sell: STOREFRONT_SELL=1 skips signed-installer gather.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TAG="${1:-v1.0.0}"
TAGGER_NAME="${OFFICIAL_TAGGER_NAME:-yusufsaaas}"

if [[ "${STOREFRONT_SELL:-}" == "1" ]]; then
  test -s "${ROOT}/dist/${TAG#v}/SHA256SUMS" || {
    echo "STOREFRONT_SELL=1 requires dist/${TAG#v}/SHA256SUMS" >&2
    exit 1
  }
else
  bash "${ROOT}/release-evidence/OFFICIAL-1.0.0/gather.sh"
  bash "${ROOT}/packaging/verify-signatures.sh"
fi

TIP="$(git -C "${ROOT}" rev-parse HEAD)"
git -C "${ROOT}" tag -a "${TAG}" -m "Source freeze ${TAG} by ${TAGGER_NAME}; tip ${TIP}; binaries storefront-only"
echo "Created annotated source tag ${TAG} at ${TIP}"
echo "Next: bash scripts/publish-storefront-release.sh ${TAG#v}"
