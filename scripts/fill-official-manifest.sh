#!/usr/bin/env bash
# Refresh MANIFEST tip SHA without claiming O1–O12 PASS.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
MF="${ROOT}/release-evidence/OFFICIAL-1.0.0/MANIFEST.txt"
TIP="$(git -C "${ROOT}" rev-parse HEAD)"
DATE="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
tmp="$(mktemp)"
awk -v tip="$TIP" -v date="$DATE" '
  BEGIN { done=0 }
  /^git_tip_at_manifest_authoring:/ {
    print "git_tip_at_manifest_authoring: " tip " @ " date
    done=1
    next
  }
  { print }
  END {
    if (!done) print "git_tip_at_manifest_authoring: " tip " @ " date
  }
' "${MF}" > "${tmp}"
mv "${tmp}" "${MF}"
echo "MANIFEST tip refreshed to ${TIP}"
