#!/usr/bin/env bash
# Cut Official annotated tag v1.0.0 — fails closed until gather PASS.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TAG="${1:-v1.0.0}"
TAGGER_NAME="${OFFICIAL_TAGGER_NAME:-yusufsaaas}"

bash "${ROOT}/release-evidence/OFFICIAL-1.0.0/gather.sh"
bash "${ROOT}/packaging/verify-signatures.sh"

TIP="$(git -C "${ROOT}" rev-parse HEAD)"
git -C "${ROOT}" tag -a "${TAG}" -m "Official Production ${TAG} by ${TAGGER_NAME}; freeze ${TIP}"
echo "Created annotated tag ${TAG} at ${TIP}"
echo "Next: bash scripts/publish-official-release.sh ${TAG}"
