#!/usr/bin/env bash
# Gate A: build Slint release binary, zip as offline package, record installer_size_mib.
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
cd "$ROOT"

# shellcheck disable=SC1090
source "$HOME/.cargo/env" 2>/dev/null || true

mkdir -p results
echo "==> cargo build -p slint-app --release"
cargo build -p slint-app --release

TARGET_DIR="$(cargo metadata --format-version 1 --no-deps | python3 -c 'import json,sys; print(json.load(sys.stdin)["target_directory"])')"
BIN="$TARGET_DIR/release/lab-spike-slint"
if [[ ! -f "$BIN" ]]; then
  echo "missing binary: $BIN" >&2
  exit 1
fi
echo "==> binary path: $BIN"

if command -v strip >/dev/null 2>&1; then
  echo "==> strip $BIN"
  strip "$BIN" || true
fi

STAGE="results/dist/$OS"
rm -rf "$STAGE"
mkdir -p "$STAGE"
cp "$BIN" "$STAGE/lab-spike-slint"
chmod +x "$STAGE/lab-spike-slint"

PKG_ZIP="results/${OS}-slint-package.zip"
PKG_TGZ="results/${OS}-slint-package.tar.gz"
rm -f "$PKG_ZIP" "$PKG_TGZ"

if command -v zip >/dev/null 2>&1; then
  (cd results/dist && zip -qr "../${OS}-slint-package.zip" "$OS")
  PKG="$PKG_ZIP"
else
  (cd results/dist && tar -czf "../${OS}-slint-package.tar.gz" "$OS")
  PKG="$PKG_TGZ"
fi

BYTES=$(wc -c < "$PKG" | tr -d ' ')
BIN_BYTES=$(wc -c < "$STAGE/lab-spike-slint" | tr -d ' ')
MIB=$(python3 -c "print(round($BYTES / (1024 * 1024), 3))")
BIN_MIB=$(python3 -c "print(round($BIN_BYTES / (1024 * 1024), 3))")

RESULT="results/${OS}-slint.json"
NOTE_SUFFIX="package=$(basename "$PKG"); package_bytes=$BYTES; binary_mib=$BIN_MIB; no_separate_runtime=true; signing=unsigned_spike_artifact"

python3 - <<PY
import json
from pathlib import Path

result = Path("$RESULT")
pkg_mib = float("$MIB")
note = "$NOTE_SUFFIX"
os_name = "$OS"

if result.exists():
    data = json.loads(result.read_text())
else:
    data = {
        "candidate": "C-SLINT",
        "os": os_name,
        "cold_start_ms": None,
        "idle_rss_mib": None,
        "peak_rss_mib": None,
        "table_display_ms": None,
        "filter_p50_ms": None,
        "filter_p95_ms": None,
        "cancel_ms": None,
        "crash_recovery": None,
        "installer_size_mib": None,
        "a11y_smoke": None,
        "notes": "package_only",
    }

data["installer_size_mib"] = pkg_mib
notes = data.get("notes") or ""
drop_prefixes = ("package=", "package_bytes=", "binary_mib=", "no_separate_runtime=", "signing=")
parts = [p for p in notes.split("; ") if p and not p.startswith(drop_prefixes)]
parts.append(note)
data["notes"] = "; ".join(parts)
result.write_text(json.dumps(data, indent=2) + "\n")
print(f"updated {result} installer_size_mib={pkg_mib}")
PY

echo "==> package: $PKG ($MIB MiB)"
echo "==> binary:  $STAGE/lab-spike-slint ($BIN_MIB MiB)"
ls -la "$PKG" "$STAGE/lab-spike-slint"
echo "==> result JSON:"
cat "$RESULT"
