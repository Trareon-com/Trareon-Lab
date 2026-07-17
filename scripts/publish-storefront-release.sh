#!/usr/bin/env bash
# Prepare / verify local storefront publish — NEVER uploads to GitHub Releases.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
VER="${1:-1.0.0}"
VER="${VER#v}"
DIST="${ROOT}/dist/${VER}"
fail() { echo "MISSING: $1" >&2; exit 1; }

test -d "$DIST" || fail "$DIST (build full binaries into dist/${VER}/ first)"
test -s "${DIST}/SHA256SUMS" || fail "${DIST}/SHA256SUMS"
bash "${ROOT}/scripts/check-no-github-binaries.sh" "v${VER}"

echo "Storefront publish checklist for ${VER}:"
echo "  1. Freeze SHA: $(git -C "$ROOT" rev-parse HEAD)"
echo "  2. Upload contents of ${DIST} to Lynk.id / Gumroad"
echo "  3. Paste SHA256SUMS + freeze SHA on the product page"
echo "  4. Link public GitHub source + docs/SELLING-UNSIGNED.md"
echo "  5. Do NOT: gh release upload binaries"
echo "OK — local evidence present; upload is manual."
