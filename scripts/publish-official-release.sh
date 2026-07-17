#!/usr/bin/env bash
# Deprecated name: Official sell path is storefront-only (no GitHub binary Release).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
VER="${1:-1.0.0}"
VER="${VER#v}"
echo "NOTE: publish-official-release.sh no longer creates GitHub Release binaries." >&2
echo "Use storefront upload per docs/DISTRIBUTION-STOREFRONT.md" >&2
exec bash "${ROOT}/scripts/publish-storefront-release.sh" "$VER"
