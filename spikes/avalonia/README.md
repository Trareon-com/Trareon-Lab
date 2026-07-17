# Spike: Avalonia + Rust FFI

Gate A candidate `C-AVALONIA`.

## Intent

Prove self-contained Avalonia packaging without a separately installed .NET runtime for end users, while forensic spike logic remains in `lab-spike-core` via FFI/CLI bridge.

## Status

Scaffold. Requires .NET 8 SDK to build. Prefer measuring this candidate first on the ThinkPad X270 (Windows), then Kali, then macOS if SDK is installed.

## Build (when SDK present)

```bash
# From spikes/avalonia after project is generated:
dotnet build -c Release
dotnet publish -c Release -r win-x64 --self-contained true
dotnet publish -c Release -r linux-x64 --self-contained true
dotnet publish -c Release -r osx-arm64 --self-contained true
```

Gate G1 passes only if the published self-contained artifact runs without requiring a separately installed .NET runtime on a clean machine.
