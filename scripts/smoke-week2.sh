#!/usr/bin/env bash
# Alias retained for Official Week-2 plan reference — delegates to e2e smoke.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
exec bash "${ROOT}/scripts/e2e-smoke.sh" "$@"
