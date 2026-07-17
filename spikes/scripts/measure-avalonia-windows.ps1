$ErrorActionPreference = "Stop"
$Root = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $Root

$dotnet = Get-Command dotnet -ErrorAction SilentlyContinue
if (-not $dotnet) {
  Write-Host "ERROR: .NET SDK not found."
  Write-Host "Install .NET 8 SDK, then re-open PowerShell and re-run this script."
  Write-Host "  winget install Microsoft.DotNet.SDK.8"
  Write-Host "  or https://dotnet.microsoft.com/download/dotnet/8.0"
  exit 2
}

$sdks = & dotnet --list-sdks 2>$null
if (-not $sdks) {
  Write-Host "ERROR: 'dotnet' exists but no SDKs are installed (runtime-only is not enough)."
  Write-Host "Install .NET 8 SDK:"
  Write-Host "  winget install Microsoft.DotNet.SDK.8"
  exit 2
}
Write-Host "==> dotnet SDKs:"
$sdks | ForEach-Object { Write-Host "  $_" }

Write-Host "==> build harness"
cargo build -p lab-spike-harness --release

$sdks = & dotnet --list-sdks 2>$null
$tfm = if ($sdks -match '^8\.') { "net8.0" } elseif ($sdks -match '^6\.') { "net6.0" } else { throw "need .NET SDK 6 or 8" }

Set-Location (Join-Path $Root "avalonia")
Write-Host "==> dotnet run Avalonia measure (-f $tfm)"
dotnet run -c Release -f $tfm -- --measure --os windows --rows 1000000 --filter-prefix 0 `
  --case-dir (Join-Path $Root "results\windows-avalonia-case") `
  --out (Join-Path $Root "results\windows-avalonia.json")

$out = Join-Path $Root "results\windows-avalonia.json"
if (-not (Test-Path $out)) {
  Write-Host "ERROR: measure finished but $out was not written."
  exit 1
}
Get-Content $out
