# Day 52 — Windows unsigned portable zip notes

```powershell
cargo build -p lab-slint --release --features gui
New-Item -ItemType Directory -Force dist\0.9.0\windows-x64 | Out-Null
Copy-Item target\release\trareon-lab.exe dist\0.9.0\windows-x64\
Get-FileHash dist\0.9.0\windows-x64\trareon-lab.exe -Algorithm SHA256 |
  Out-File dist\0.9.0\windows-x64\SHA256SUMS.txt
```

SmartScreen may warn; see `docs/SELLING-UNSIGNED.md`.
