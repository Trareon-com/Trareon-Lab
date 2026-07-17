$ErrorActionPreference = "Stop"
$Root = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $Root

Write-Host "==> build harness"
cargo build -p lab-spike-harness --release

Set-Location (Join-Path $Root "avalonia")
Write-Host "==> dotnet run Avalonia measure"
dotnet run -c Release -- --measure --os windows --rows 1000000 --filter-prefix 0 `
  --case-dir (Join-Path $Root "results\windows-avalonia-case") `
  --out (Join-Path $Root "results\windows-avalonia.json")

Get-Content (Join-Path $Root "results\windows-avalonia.json")
