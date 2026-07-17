# Official O1 Windows Authenticode pipeline (run on Windows lab host)
#
# QUEUED FOR WINDOWS LAB — see docs/WINDOWS-LAB-QUEUE.md
#
# Required env (never commit):
#   WINDOWS_CERT_PATH, WINDOWS_CERT_PASSWORD  (or hardware token via signtool)
#
# Steps:
# 1) Build unsigned per packaging/WINDOWS-UNSIGNED.md into dist\1.0.0\windows-x64\
# 2) Sign:
#      signtool sign /fd SHA256 /f $env:WINDOWS_CERT_PATH /p $env:WINDOWS_CERT_PASSWORD `
#        dist\1.0.0\windows-x64\trareon-lab.exe
# 3) Verify:
#      signtool verify /pa dist\1.0.0\windows-x64\trareon-lab.exe |
#        Out-File release-evidence\OFFICIAL-1.0.0\windows-sig.txt
# 4) Optional: osslsigncode verify -in trareon-lab.exe

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
if (-not $env:WINDOWS_CERT_PATH) { throw "MISSING env: WINDOWS_CERT_PATH" }

$Out = Join-Path $Root "dist\1.0.0\windows-x64"
$Evid = Join-Path $Root "release-evidence\OFFICIAL-1.0.0"
New-Item -ItemType Directory -Force -Path $Out | Out-Null
New-Item -ItemType Directory -Force -Path $Evid | Out-Null

cargo build -p lab-slint --release --features gui
$Bin = Join-Path $Root "target\release\trareon-lab.exe"
if (-not (Test-Path $Bin)) { $Bin = Join-Path $Root "target\release\lab-slint.exe" }
Copy-Item $Bin (Join-Path $Out "trareon-lab.exe") -Force

$Target = Join-Path $Out "trareon-lab.exe"
if ($env:WINDOWS_CERT_PASSWORD) {
  & signtool sign /fd SHA256 /f $env:WINDOWS_CERT_PATH /p $env:WINDOWS_CERT_PASSWORD $Target
} else {
  & signtool sign /fd SHA256 /f $env:WINDOWS_CERT_PATH $Target
}
& signtool verify /pa $Target 2>&1 | Tee-Object -FilePath (Join-Path $Evid "windows-sig.txt")
Write-Host "O1 evidence written: release-evidence/OFFICIAL-1.0.0/windows-sig.txt"
