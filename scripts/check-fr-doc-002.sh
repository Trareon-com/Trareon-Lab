#!/usr/bin/env bash
# FR-DOC-002: refuse Official "Validated" / over-claimed CORPUS_VALIDATED without evidence.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
MATRIX="${ROOT}/docs/RELEASE-01-CAPABILITY-MATRIX.md"
fail() { echo "FR-DOC-002 FAIL: $1" >&2; exit 1; }

test -f "$MATRIX" || fail "missing capability matrix"

# Absolute Official "Validated" label (not UNIT_VERIFIED / CORPUS_VALIDATED working levels).
if grep -nE '\|[[:space:]]*Validated[[:space:]]*\|' "$MATRIX" >/dev/null; then
  test -d "${ROOT}/docs/validation" || fail "Validated claim without docs/validation/"
  test -d "${ROOT}/docs/user" || fail "Validated claim without docs/user/"
  ls "${ROOT}/docs/validation"/*.md >/dev/null 2>&1 || fail "no validation dossiers"
  ls "${ROOT}/docs/user"/*.md >/dev/null 2>&1 || fail "no user guides"
fi

# Check validation-level column (3rd cell) for domains that must stay UNIT_VERIFIED / Limited.
python3 - <<'PY'
from pathlib import Path
import sys
text = Path("docs/RELEASE-01-CAPABILITY-MATRIX.md").read_text().splitlines()
required = {
    "Disk images": ("UNIT_VERIFIED", "Limited"),
    "OS artifacts": ("UNIT_VERIFIED",),
    "Timeline": ("UNIT_VERIFIED",),
    "Search": ("UNIT_VERIFIED",),
}
for line in text:
    if not line.startswith("| ") or line.startswith("|---") or "Domain" in line:
        continue
    cells = [c.strip() for c in line.strip("|").split("|")]
    if len(cells) < 3:
        continue
    domain, level = cells[0], cells[2]
    if domain in required:
        ok = any(tok in level for tok in required[domain])
        if not ok or level.strip() == "CORPUS_VALIDATED":
            print(f"FR-DOC-002 FAIL: {domain} validation level is {level!r}; expected {required[domain]}", file=sys.stderr)
            sys.exit(1)
print("FR-DOC-002 gate: PASS (no unsupported Official Validated promotions)")
PY
