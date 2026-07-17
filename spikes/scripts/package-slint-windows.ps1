# Gate A: build Slint release binary, zip as offline package, record installer_size_mib.
$ErrorActionPreference = "Stop"

$Root = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $Root
Write-Host "==> repo spikes: $(Get-Location)"

New-Item -ItemType Directory -Force -Path ".\results" | Out-Null

Write-Host "==> cargo build -p slint-app --release"
cargo build -p slint-app --release

$meta = cargo metadata --format-version 1 --no-deps | ConvertFrom-Json
$Bin = Join-Path $meta.target_directory "release\lab-spike-slint.exe"
if (-not (Test-Path $Bin)) {
  throw "missing binary: $Bin"
}
Write-Host "==> binary path: $Bin"

$Stage = ".\results\dist\windows"
if (Test-Path $Stage) { Remove-Item -Recurse -Force $Stage }
New-Item -ItemType Directory -Force -Path $Stage | Out-Null
Copy-Item -Force $Bin (Join-Path $Stage "lab-spike-slint.exe")

$Pkg = ".\results\windows-slint-package.zip"
if (Test-Path $Pkg) { Remove-Item -Force $Pkg }

Compress-Archive -Path (Join-Path $Stage "*") -DestinationPath $Pkg -Force

$PkgBytes = (Get-Item $Pkg).Length
$BinBytes = (Get-Item (Join-Path $Stage "lab-spike-slint.exe")).Length
$PkgMib = [math]::Round($PkgBytes / 1MB, 3)
$BinMib = [math]::Round($BinBytes / 1MB, 3)

$Result = ".\results\windows-slint.json"
$Note = "package=windows-slint-package.zip; package_bytes=$PkgBytes; binary_mib=$BinMib; no_separate_runtime=true; signing=unsigned_spike_artifact"

if (Test-Path $Result) {
  $data = Get-Content $Result -Raw | ConvertFrom-Json
} else {
  $data = [pscustomobject]@{
    candidate = "C-SLINT"
    os = "windows"
    cold_start_ms = $null
    idle_rss_mib = $null
    peak_rss_mib = $null
    table_display_ms = $null
    filter_p50_ms = $null
    filter_p95_ms = $null
    cancel_ms = $null
    crash_recovery = $null
    installer_size_mib = $null
    a11y_smoke = $null
    notes = "package_only"
  }
}

$data.installer_size_mib = $PkgMib
$parts = @()
if ($data.notes) {
  foreach ($p in ($data.notes -split "; ")) {
    if (-not $p) { continue }
    if ($p -like "package=*" -or $p -like "package_bytes=*" -or $p -like "binary_mib=*" -or $p -like "no_separate_runtime=*" -or $p -like "signing=*") {
      continue
    }
    $parts += $p
  }
}
$parts += $Note
$data.notes = ($parts -join "; ")

$data | ConvertTo-Json -Depth 8 | Set-Content -Path $Result -Encoding utf8

Write-Host "==> package: $Pkg ($PkgMib MiB)"
Write-Host "==> binary:  $BinMib MiB"
Get-Item $Pkg, (Join-Path $Stage "lab-spike-slint.exe") | Format-Table Name, Length
Write-Host "==> result JSON:"
Get-Content $Result
