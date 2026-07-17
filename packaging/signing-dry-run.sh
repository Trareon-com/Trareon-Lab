#!/usr/bin/env bash
# Official Path C dry-run: prove packaging/storefront scripts exist without claiming signatures.
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
test -f "${ROOT}/docs/DISTRIBUTION-STOREFRONT.md" || fail "DISTRIBUTION-STOREFRONT.md"
test -f "${ROOT}/docs/WINDOWS-LAB-QUEUE.md" || fail "windows lab queue"
test -f "${ROOT}/release-evidence/OFFICIAL-1.0.0/gather.sh" || fail "official gather"
test -f "${ROOT}/scripts/cut-official-v1.sh" || fail "cut-official-v1.sh"
test -f "${ROOT}/scripts/publish-storefront-release.sh" || fail "publish-storefront-release.sh"
test -f "${ROOT}/scripts/publish-official-release.sh" || fail "publish-official-release.sh"
test -f "${ROOT}/scripts/check-no-github-binaries.sh" || fail "check-no-github-binaries.sh"
test -f "${ROOT}/scripts/close-official-program.sh" || fail "close-official-program.sh"
test -f "${ROOT}/docs/operator/FINAL-8-GATES.md" || fail "FINAL-8-GATES.md"
test -f "${ROOT}/docs/reviews/TRANSFER-SECURITY-REVIEW.md" || fail "TRANSFER-SECURITY-REVIEW.md"
test -f "${ROOT}/docs/MACOS-LINUX-SIGNING-QUEUE.md" || fail "MACOS-LINUX-SIGNING-QUEUE.md"
test -f "${ROOT}/docs/OFFICIAL-END-STATE.md" || fail "OFFICIAL-END-STATE.md"
test -d "${ROOT}/release-evidence/OFFICIAL-1.0.0/templates" || fail "evidence templates/"

echo "signing dry-run tooling: PASS (storefront docs/scripts present; O1–O3 optional hardening)"
echo "Sell path: docs/DISTRIBUTION-STOREFRONT.md (Lynk.id / Gumroad; no GitHub installers)"
echo "Windows lab = optional post-sell hardening — docs/WINDOWS-LAB-QUEUE.md"
echo "Next sell steps: build dist/ → SHA256SUMS → publish-storefront-release.sh → upload storefront"
