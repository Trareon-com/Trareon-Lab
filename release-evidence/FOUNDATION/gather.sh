#!/usr/bin/env bash
# F15: Foundation release-gate evidence checklist (artifact presence).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
EVID="${ROOT}/release-evidence/FOUNDATION"
mkdir -p "${EVID}"

fail() { echo "MISSING: $1" >&2; exit 1; }

test -f "${ROOT}/docs/FOUNDATION-READINESS-CHECKLIST.md" || fail "readiness checklist"
test -f "${ROOT}/schemas/case.schema.json" || fail "case schema"
test -f "${ROOT}/schemas/evidence-object.schema.json" || fail "evidence schema"
test -f "${ROOT}/Cargo.toml" || fail "workspace Cargo.toml"
test -d "${ROOT}/crates/lab-core" || fail "lab-core"
test -d "${ROOT}/crates/lab-fsnap" || fail "lab-fsnap"
test -d "${ROOT}/apps/lab-slint" || fail "lab-slint"
test -f "${EVID}/test-report.txt" || fail "test-report.txt (run cargo test and redirect here)"
test -f "${EVID}/schema-validation.log" || fail "schema-validation.log"
test -f "${EVID}/sbom-cargo-metadata.json" || fail "sbom-cargo-metadata.json"
test -f "${EVID}/MANIFEST.txt" || fail "MANIFEST.txt"

echo "Foundation release-gate evidence OK: ${EVID}"
