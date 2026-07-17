#!/usr/bin/env bash
# Final unsigned→signed rebuild orchestrator (fails closed without certs).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
echo "Freeze tip: $(git -C "${ROOT}" rev-parse HEAD)"
echo "1) macOS sign (requires Apple env)"
bash "${ROOT}/packaging/sign-macos.sh"
echo "2) Linux sign (requires LINUX_SIGNING_KEY_ID)"
bash "${ROOT}/packaging/sign-linux.sh"
echo "3) Windows sign must run on Windows lab — see docs/WINDOWS-LAB-QUEUE.md and packaging/sign-windows.ps1"
echo "4) Verify"
bash "${ROOT}/packaging/verify-signatures.sh"
echo "5) Gather"
bash "${ROOT}/release-evidence/OFFICIAL-1.0.0/gather.sh"
