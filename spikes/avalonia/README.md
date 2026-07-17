# Spike: Avalonia + Rust harness bridge

Gate A candidate `C-AVALONIA`.

## Intent

Prove self-contained Avalonia packaging without a separately installed .NET runtime for end users, while forensic spike logic remains in `lab-spike-core` via the `lab-spike-harness` CLI bridge (no forensic business logic in C#).

## Prereq

- .NET 8 SDK
- Release harness: `cd spikes && cargo build -p lab-spike-harness --release`

## Interactive

```bash
cd spikes/avalonia
dotnet run -c Release
```

## Measure

```bash
# build harness first
cd spikes && cargo build -p lab-spike-harness --release
cd avalonia
dotnet run -c Release -- --measure --os windows --rows 1000000 --filter-prefix 0 \
  --case-dir ../results/windows-avalonia-case \
  --out ../results/windows-avalonia.json
```

Or use `spikes/scripts/measure-avalonia-windows.ps1` / `measure-avalonia.sh`.

## Self-contained package (G1)

```bash
dotnet publish -c Release -r win-x64 --self-contained true -p:PublishSingleFile=true
dotnet publish -c Release -r linux-x64 --self-contained true -p:PublishSingleFile=true
dotnet publish -c Release -r osx-arm64 --self-contained true -p:PublishSingleFile=true
```

Gate G1 passes only if the published artifact runs without a separately installed .NET runtime.
