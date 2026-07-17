# Gate A Slint measure for Windows — safe pull + rebuild + run
$ErrorActionPreference = "Stop"
Set-Location "$HOME\Projects\Trareon\Trareon-Lab"
Write-Host "==> repo: $(Get-Location)"

New-Item -ItemType Directory -Force -Path ".\spikes\results" | Out-Null

if (Test-Path ".\spikes\results\windows-harness-core.json") {
  Write-Host "==> moving local windows-harness-core.json aside"
  Move-Item -Force ".\spikes\results\windows-harness-core.json" ".\spikes\results\windows-harness-core.json.local.bak"
}

Write-Host "==> git pull"
git checkout main
git pull origin main

Set-Location ".\spikes"
Write-Host "==> cargo build slint-app (release)"
cargo build -p slint-app --release

Write-Host "==> running measure"
& ".\target\release\lab-spike-slint.exe" `
  --measure `
  --os windows `
  --rows 1000000 `
  --filter-prefix 0 `
  --case-dir ".\results\windows-slint-case" `
  --out ".\results\windows-slint.json" `
  *>&1 | Tee-Object -FilePath ".\results\windows-slint-run.log"

Write-Host "EXIT=$LASTEXITCODE"
Write-Host "==> result file:"
Get-ChildItem ".\results\windows-slint*"
Write-Host "==> contents:"
Get-Content ".\results\windows-slint.json"
