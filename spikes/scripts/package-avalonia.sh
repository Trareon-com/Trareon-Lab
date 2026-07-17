#!/usr/bin/env bash
# Self-contained Avalonia publish zip for G1 + installer_size_mib.
set -euo pipefail
OS="${1:-}"
RID=""
case "${OS:-}" in
  "") case "$(uname -s)" in Darwin) OS=macos; RID=osx-arm64 ;; Linux) OS=linux; RID=linux-x64 ;; *) exit 2 ;; esac ;;
  macos) RID=osx-arm64 ;;
  linux) RID=linux-x64 ;;
  *) echo "usage: $0 <macos|linux>"; exit 2 ;;
esac

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
# shellcheck disable=SC1090
source "$HOME/.cargo/env" 2>/dev/null || true
if [[ -x "$HOME/.dotnet/dotnet" ]]; then
  export DOTNET_ROOT="${DOTNET_ROOT:-$HOME/.dotnet}"
  export PATH="$HOME/.dotnet:$PATH"
fi

TFM="$(bash "$ROOT/scripts/pick-avalonia-tfm.sh")"
OUTDIR="$ROOT/avalonia/publish/${OS}"
rm -rf "$OUTDIR"
mkdir -p "$OUTDIR"

echo "==> self-contained publish -f $TFM -r $RID"
cd "$ROOT/avalonia"
dotnet publish -c Release -f "$TFM" -r "$RID" --self-contained true \
  -p:PublishSingleFile=true -p:IncludeNativeLibrariesForSelfExtract=true \
  -o "$OUTDIR"

PKG="$ROOT/results/${OS}-avalonia-package.zip"
rm -f "$PKG"
(cd "$ROOT/avalonia/publish" && zip -qr "$PKG" "$OS")
BYTES=$(wc -c < "$PKG" | tr -d ' ')
MIB=$(python3 -c "print(round($BYTES / (1024 * 1024), 3))")
RESULT="$ROOT/results/${OS}-avalonia.json"
NOTE="package=$(basename "$PKG"); package_bytes=$BYTES; self_contained=true; no_separate_dotnet_runtime=true; signing=unsigned_spike_artifact"

python3 - <<PY
import json
from pathlib import Path
result = Path("$RESULT")
data = json.loads(result.read_text()) if result.exists() else {
  "candidate": "C-AVALONIA", "os": "$OS", "cold_start_ms": None, "idle_rss_mib": None,
  "peak_rss_mib": None, "table_display_ms": None, "filter_p50_ms": None, "filter_p95_ms": None,
  "cancel_ms": None, "crash_recovery": None, "installer_size_mib": None, "a11y_smoke": None, "notes": "package_only"
}
data["installer_size_mib"] = float("$MIB")
notes = data.get("notes") or ""
drop = ("package=", "package_bytes=", "self_contained=", "no_separate_dotnet_runtime=", "signing=")
parts = [p for p in notes.split("; ") if p and not p.startswith(drop)]
parts.append("$NOTE")
data["notes"] = "; ".join(parts)
result.write_text(json.dumps(data, indent=2) + "\n")
print(f"updated {result} installer_size_mib={data['installer_size_mib']}")
PY
ls -la "$PKG"
cat "$RESULT"
