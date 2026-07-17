#!/usr/bin/env bash
# Fail if a GitHub Release for TAG already has installer-like assets (operator guard).
set -euo pipefail
TAG="${1:-v1.0.0}"
command -v gh >/dev/null 2>&1 || { echo "gh not installed; skip remote check"; exit 0; }
if ! gh release view "$TAG" >/dev/null 2>&1; then
  echo "No GitHub Release $TAG — OK (binaries must stay off GitHub)"
  exit 0
fi
assets="$(gh release view "$TAG" --json assets --jq '.assets[].name' 2>/dev/null || true)"
if echo "$assets" | grep -Eii '\.(exe|msi|dmg|pkg|appimage|deb|rpm|zip|tar\.gz)$' >/dev/null 2>&1; then
  echo "FORBIDDEN: installer-like assets on GitHub Release $TAG" >&2
  exit 1
fi
echo "Release $TAG has no installer-like assets — OK"
