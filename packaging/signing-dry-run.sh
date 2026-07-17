#!/usr/bin/env bash
# Official Path C dry-run: prove packaging scripts exist without claiming signatures.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
fail() { echo "MISSING: $1" >&2; exit 1; }

test -x "${ROOT}/packaging/smoke.sh" || test -f "${ROOT}/packaging/smoke.sh" || fail "packaging/smoke.sh"
test -f "${ROOT}/docs/LINUX-PACKAGE-SIGNING-KEY-PLAN.md" || fail "linux key plan"
test -f "${ROOT}/docs/PACKAGING-UNSIGNED-BUILDS.md" || fail "unsigned packaging doc"
test -f "${ROOT}/release-evidence/OFFICIAL-1.0.0/gather.sh" || fail "official gather"

echo "signing dry-run tooling: PASS (scripts/docs present; no secrets; O1–O3 still NOT_STARTED)"
echo "Next operator steps: Apple Team ID + Authenticode + Linux public key before signed pipelines."
