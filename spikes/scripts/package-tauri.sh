#!/usr/bin/env bash
# Package C-TAURI release binary (+ note webview runtime) and record installer_size_mib.
set -euo pipefail
OS="${1:-}"
if [[ -z "$OS" ]]; then
  case "$(uname -s)" in
    Darwin) OS=macos ;;
    Linux) OS=linux ;;
    *) echo "usage: $0 <macos|linux>"; exit 2 ;;
  esac
fi

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT/tauri"
# shellcheck disable=SC1090
source "$HOME/.cargo/env" 2>/dev/null || true

npm install
npm run build
cd src-tauri
cargo build --release

TARGET_DIR="$(cargo metadata --format-version 1 --no-deps | python3 -c 'import json,sys; print(json.load(sys.stdin)["target_directory"])')"
BIN="$TARGET_DIR/release/lab-spike-tauri"
[[ -f "$BIN" ]] || { echo "missing $BIN"; exit 1; }
if command -v strip >/dev/null 2>&1; then strip "$BIN" || true; fi

STAGE="$ROOT/results/dist/${OS}-tauri"
rm -rf "$STAGE"
mkdir -p "$STAGE"
cp "$BIN" "$STAGE/lab-spike-tauri"
chmod +x "$STAGE/lab-spike-tauri"
# Document OS webview dependency for G1 notes (not shipped inside zip).
cat > "$STAGE/RUNTIME-NOTES.txt" <<EOF
C-TAURI offline spike package contains the native binary only.
OS webview is required at runtime:
- Windows: WebView2 (often preinstalled / Evergreen)
- macOS: WKWebView (system)
- Linux: WebKitGTK (distro package)
This is an OS component dependency, not a language runtime (.NET/Node/Python).
Signing: unsigned_spike_artifact (production code signing deferred).
EOF

PKG="$ROOT/results/${OS}-tauri-package.zip"
rm -f "$PKG"
(cd "$ROOT/results/dist" && zip -qr "../${OS}-tauri-package.zip" "${OS}-tauri")

BYTES=$(wc -c < "$PKG" | tr -d ' ')
MIB=$(python3 -c "print(round($BYTES / (1024 * 1024), 3))")
RESULT="$ROOT/results/${OS}-tauri.json"
NOTE="package=$(basename "$PKG"); package_bytes=$BYTES; no_language_runtime=true; os_webview_required=true; signing=unsigned_spike_artifact"

python3 - <<PY
import json
from pathlib import Path
result = Path("$RESULT")
data = json.loads(result.read_text()) if result.exists() else {
  "candidate": "C-TAURI", "os": "$OS", "cold_start_ms": None, "idle_rss_mib": None,
  "peak_rss_mib": None, "table_display_ms": None, "filter_p50_ms": None, "filter_p95_ms": None,
  "cancel_ms": None, "crash_recovery": None, "installer_size_mib": None, "a11y_smoke": None, "notes": "package_only"
}
data["installer_size_mib"] = float("$MIB")
notes = data.get("notes") or ""
drop = ("package=", "package_bytes=", "no_language_runtime=", "os_webview_required=", "signing=")
parts = [p for p in notes.split("; ") if p and not p.startswith(drop)]
parts.append("$NOTE")
data["notes"] = "; ".join(parts)
result.write_text(json.dumps(data, indent=2) + "\n")
print(f"updated {result} installer_size_mib={data['installer_size_mib']}")
PY
ls -la "$PKG"
cat "$RESULT"
