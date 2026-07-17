#!/usr/bin/env bash
# Print net8.0 or net6.0 based on installed SDKs (prefer 8).
set -euo pipefail
sdks="$(dotnet --list-sdks 2>/dev/null || true)"
if echo "$sdks" | grep -qE '^8\.'; then
  echo net8.0
elif echo "$sdks" | grep -qE '^6\.'; then
  echo net6.0
else
  echo "ERROR: need .NET SDK 6 or 8" >&2
  echo "$sdks" >&2
  exit 2
fi
