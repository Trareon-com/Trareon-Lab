#!/usr/bin/env bash
# Generate CycloneDX-ish SBOM stub from cargo metadata for Official O4 slot.
# Prefer a real cyclonedx-cargo output when the tool is installed; otherwise write inventory JSON.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVID="${ROOT}/release-evidence/OFFICIAL-1.0.0"
mkdir -p "${EVID}"

if command -v cyclonedx-cargo >/dev/null 2>&1; then
  (cd "${ROOT}" && cyclonedx-cargo --format json --output-file "${EVID}/sbom.cdx.json")
  echo "Wrote CycloneDX via cyclonedx-cargo: ${EVID}/sbom.cdx.json"
  exit 0
fi

# Fallback: cargo metadata package inventory (not full CycloneDX; mark schema accordingly).
META="$(mktemp)"
trap 'rm -f "${META}"' EXIT
cargo metadata --format-version 1 --all-features --manifest-path "${ROOT}/Cargo.toml" > "${META}"
python3 - <<PY
import json
from pathlib import Path
from datetime import datetime, timezone
evid = Path("${EVID}")
meta = json.loads(Path("${META}").read_text())
components = []
for p in meta.get("packages", []):
    name, ver = p.get("name"), p.get("version")
    if name and ver:
        components.append({
            "type": "library",
            "name": name,
            "version": ver,
            "bom-ref": f"cargo:{name}@{ver}",
        })
bom = {
    "bomFormat": "CycloneDX",
    "specVersion": "1.5",
    "version": 1,
    "metadata": {
        "timestamp": datetime.now(timezone.utc).isoformat(),
        "tools": [{"name": "trareon-lab-scripts/generate-official-sbom.sh", "version": "fallback-metadata"}],
        "component": {"type": "application", "name": "trareon-lab", "version": "1.0.0-rc1-unsigned"},
        "properties": [{"name": "trareon:sbom_quality", "value": "cargo_metadata_fallback_not_full_cdx_tooling"}],
    },
    "components": sorted(components, key=lambda c: c["bom-ref"]),
}
out = evid / "sbom.cdx.json"
out.write_text(json.dumps(bom, indent=2) + "\n")
print(f"Wrote fallback SBOM ({len(components)} components): {out}")
print("Install cyclonedx-cargo later for a fuller Official O4 artifact.")
PY
