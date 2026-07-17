#!/usr/bin/env bash
# Official Path C dry-run: prove packaging scripts exist without claiming signatures.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
fail() { echo "MISSING: $1" >&2; exit 1; }

test -f "${ROOT}/packaging/smoke.sh" || fail "packaging/smoke.sh"
test -f "${ROOT}/packaging/sign-macos.sh" || fail "sign-macos.sh"
test -f "${ROOT}/packaging/sign-linux.sh" || fail "sign-linux.sh"
test -f "${ROOT}/packaging/sign-windows.ps1" || fail "sign-windows.ps1"
test -f "${ROOT}/packaging/verify-signatures.sh" || fail "verify-signatures.sh"
test -f "${ROOT}/docs/LINUX-PACKAGE-SIGNING-KEY-PLAN.md" || fail "linux key plan"
test -f "${ROOT}/docs/PACKAGING-UNSIGNED-BUILDS.md" || fail "unsigned packaging doc"
test -f "${ROOT}/docs/WINDOWS-LAB-QUEUE.md" || fail "windows lab queue"
test -f "${ROOT}/release-evidence/OFFICIAL-1.0.0/gather.sh" || fail "official gather"
test -f "${ROOT}/scripts/cut-official-v1.sh" || fail "cut-official-v1.sh"
test -f "${ROOT}/scripts/publish-official-release.sh" || fail "publish-official-release.sh"
test -f "${ROOT}/scripts/close-official-program.sh" || fail "close-official-program.sh"
test -f "${ROOT}/docs/operator/FINAL-8-GATES.md" || fail "FINAL-8-GATES.md"

echo "signing dry-run tooling: PASS (scripts/docs present; no secrets; O1–O3 still NOT_STARTED)"
echo "Windows lab items queued in docs/WINDOWS-LAB-QUEUE.md"
echo "Go-live scripts ready (fail-closed until gather PASS) — see docs/operator/FINAL-8-GATES.md"
echo "Next operator steps: Apple Team ID + Authenticode + Linux public key before signed pipelines."
