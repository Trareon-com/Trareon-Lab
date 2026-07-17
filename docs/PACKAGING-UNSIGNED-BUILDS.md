# Unsigned release builds (3 OS)

**Program:** zero-cost sellable (unsigned installers).  
**Signing:** none — operators must follow [`docs/SELLING-UNSIGNED.md`](../SELLING-UNSIGNED.md).

## Supported matrix

| OS | Arch | Notes |
|---|---|---|
| Windows 10/11 | x64 | SmartScreen may warn |
| macOS 14/15 | arm64 | Gatekeeper blocks until Open Anyway |
| Ubuntu 22.04/24.04, Debian 12, Kali | x64 | AppArmor/policy may prompt |

## Prerequisites

- Rust toolchain matching CI (`1.95.0` or workspace-documented pin)
- On macOS: Xcode CLT for linker
- On Windows: MSVC Build Tools
- On Linux: `build-essential`, packaging deps for Slint as needed

## Local smoke (any host)

```bash
./packaging/smoke.sh
```

Writes `packaging/out/smoke-<os>.txt` and confirms workspace metadata.

## Release-oriented build commands

### macOS arm64

```bash
cargo build -p lab-slint --release --features gui
# Binary typically under target/release/lab-slint (name may vary with package binary)
shasum -a 256 target/release/lab-slint > packaging/out/SHA256SUMS-macos-arm64.txt
```

### Linux x64

```bash
cargo build -p lab-slint --release --features gui
sha256sum target/release/lab-slint > packaging/out/SHA256SUMS-linux-x64.txt
```

### Windows x64 (PowerShell)

```powershell
cargo build -p lab-slint --release --features gui
Get-FileHash .\target\release\lab-slint.exe -Algorithm SHA256 |
  Format-List > .\packaging\out\SHA256SUMS-windows-x64.txt
```

## Distributor checklist

1. Publish binary + matching `SHA256SUMS` line.
2. Link `docs/SELLING-UNSIGNED.md` in release notes.
3. Do **not** claim Apple notarization, Authenticode, or ISO/court readiness.
4. Prefer CI artifact SBOM (`sbom-cargo-metadata`) attached to the same tag.

## CI SBOM

Workflow job `sbom` uploads:

- `release-evidence/sbom/sbom-cargo-metadata.json`
- `release-evidence/sbom/package-inventory.txt`
