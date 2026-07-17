#!/usr/bin/env bash
# Close Official program docs after v1.0.0 release — fails closed until gather + tag exist.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TAG="${1:-v1.0.0}"

bash "${ROOT}/release-evidence/OFFICIAL-1.0.0/gather.sh"
test -n "$(git -C "${ROOT}" tag -l "${TAG}")" || {
  echo "MISSING tag ${TAG}" >&2
  exit 1
}

PRD="${ROOT}/PRD-Digital-Forensic-Analysis-Lab.md"
python3 - <<PY
from pathlib import Path
prd = Path("${PRD}")
text = prd.read_text()
old = "| Status | Official R1.0.0 program active (target 2026-10-16); Engineering Alpha sellable path complete; Official O1–O12 gates open |"
new = "| Status | Official Production 1.0.0 released (${TAG}); support period per docs/user/RELEASE-NOTES-1.0.0.md |"
if old not in text:
    raise SystemExit("PRD status line not found or already updated")
prd.write_text(text.replace(old, new, 1))
print("PRD status updated to Official Production 1.0.0 released")
PY

NOTE="${ROOT}/docs/operator/OFFICIAL-PROGRAM-CLOSED.md"
cat > "${NOTE}" <<EOF
# Official program closed

- Tag: ${TAG}
- Tip: $(git -C "${ROOT}" rev-parse "${TAG}^{}")
- Closed UTC: $(date -u +%Y-%m-%dT%H:%M:%SZ)
- Next: docs/P0-LATER-ADR-BACKLOG.md; docs/operator/POST-RELEASE-24H.md
EOF

echo "Wrote ${NOTE}"
echo "Commit these doc updates, then run post-release 24h checklist."
