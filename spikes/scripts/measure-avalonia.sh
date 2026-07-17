#!/usr/bin/env bash
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
# shellcheck disable=SC1090
source "$HOME/.cargo/env" 2>/dev/null || true

# Prefer user-local .NET install (dotnet-install.sh) when present.
if [[ -x "$HOME/.dotnet/dotnet" ]]; then
  export DOTNET_ROOT="${DOTNET_ROOT:-$HOME/.dotnet}"
  export PATH="$HOME/.dotnet:$HOME/.dotnet/tools:$PATH"
fi

if ! command -v dotnet >/dev/null 2>&1; then
  echo "ERROR: dotnet SDK not found."
  echo "Kali apt may lack SDK 8. Options:"
  echo "  1) Use distro SDK 6+ (spike targets net6.0): sudo apt install -y dotnet-sdk-6.0"
  echo "  2) Or install SDK 8 locally:"
  echo "       curl -fsSL https://dot.net/v1/dotnet-install.sh | bash -s -- --channel 8.0"
  echo "       export PATH=\"\$HOME/.dotnet:\$PATH\""
  exit 2
fi

echo "==> dotnet SDKs:"
dotnet --list-sdks || true

echo "==> build harness"
cd "$ROOT"
cargo build -p lab-spike-harness --release

echo "==> dotnet run Avalonia measure"
cd "$ROOT/avalonia"
dotnet run -c Release -- --measure --os "$OS" --rows 1000000 --filter-prefix 0 \
  --case-dir "$ROOT/results/${OS}-avalonia-case" \
  --out "$ROOT/results/${OS}-avalonia.json"

cat "$ROOT/results/${OS}-avalonia.json"
