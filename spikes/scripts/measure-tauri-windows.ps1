# Gate A C-TAURI measure for Windows
$ErrorActionPreference = "Stop"
$Root = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location (Join-Path $Root "tauri")

Write-Host "==> npm install"
npm install

Write-Host "==> frontend build"
npm run build

Set-Location ".\src-tauri"
Write-Host "==> cargo build release"
cargo build --release

$meta = cargo metadata --format-version 1 --no-deps | ConvertFrom-Json
$Bin = Join-Path $meta.target_directory "release\lab-spike-tauri.exe"
if (-not (Test-Path $Bin)) { throw "missing $Bin" }

$Out = Join-Path $Root "results\windows-tauri.json"
$Case = Join-Path $Root "results\windows-tauri-case"
New-Item -ItemType Directory -Force -Path (Join-Path $Root "results") | Out-Null

Write-Host "==> running measure"
& $Bin --measure --os windows --rows 1000000 --filter-prefix 0 --case-dir $Case --out $Out
Write-Host "EXIT=$LASTEXITCODE"
Get-Content $Out
