#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
failed=0

require_file() {
  if [[ ! -f "${ROOT}/$1" ]]; then
    echo "FAIL: required Lab Core Perfect artifact missing: $1" >&2
    failed=1
  fi
}

if grep -Eq 'demo_seed:[[:space:]]*true' "${ROOT}/apps/lab-slint/src/ui_model.rs"; then
  echo "FAIL: UiSnapshot defaults demo_seed to true" >&2
  failed=1
fi

bad_claims="$(
  grep -RIniE 'court[- ]ready' "${ROOT}/apps/lab-slint" \
    | grep -viE 'not court-ready' || true
)"
if [[ -n "${bad_claims}" ]]; then
  echo "FAIL: positive court-ready claim found (only NOT court-ready disclosure is allowed):" >&2
  echo "${bad_claims}" >&2
  failed=1
fi

if grep -E '^default[[:space:]]*=' "${ROOT}/apps/lab-slint/Cargo.toml" | grep -q 'local_ai'; then
  echo "FAIL: local_ai must not be enabled by default" >&2
  failed=1
fi

if ! grep -Rqs 'pub const CASE_UCO_PROFILE_VERSION' "${ROOT}/crates/lab-core/src"; then
  echo "FAIL: CASE_UCO_PROFILE_VERSION is missing" >&2
  failed=1
fi

require_file "docs/validation/EXPORT-DOSSIER.md"
require_file "docs/validation/APFS-DOSSIER.md"
require_file "docs/validation/E01-DOSSIER.md"
require_file "packaging/SHA256SUMS"
require_file "docs/superpowers/plans/lab-core-perfect-freeze.md"

if [[ "${failed}" -ne 0 ]]; then
  exit 1
fi
echo "Lab Core Perfect static gate OK"
