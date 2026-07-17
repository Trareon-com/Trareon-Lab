$ErrorActionPreference = "Stop"
$Root = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $Root
cargo build -p lab-spike-index --release
$meta = cargo metadata --format-version 1 --no-deps | ConvertFrom-Json
$Bin = Join-Path $meta.target_directory "release\lab-spike-index.exe"
New-Item -ItemType Directory -Force -Path ".\results" | Out-Null
foreach ($c in @("D-SQLITE", "D-SQLITE-FTS", "D-RUST-INDEX")) {
  $slug = $c.ToLower()
  Write-Host "==> $c"
  & $Bin measure --candidate $c --os windows --rows 1000000 `
    --case-dir ".\results\windows-$slug-case" `
    --out ".\results\windows-$slug.json"
}
