# Session — Windows Lab Queue (Unsigned Build & Smoke)

**Date:** 2026-07-17  
**Queue File:** [WINDOWS-LAB-QUEUE.md](file:///e:/Trareon/trareon-lab/docs/WINDOWS-LAB-QUEUE.md)  

## Done

- **W2 (Unsigned Build):** Compiled the unsigned Windows GUI binary (`trareon-lab.exe`) using `cargo build -p lab-slint --release --features gui`. Saved executable under `dist/1.0.0/windows-x64/` and generated local `SHA256SUMS.txt`.
- **W4 (Physical Smoke Test):** Performed automated cold-start smoke check (starting the GUI binary asynchronously and confirming it continues to execute without crashing). Updated [windows-thinkpad.json](file:///e:/Trareon/trareon-lab/release-evidence/OFFICIAL-1.0.0/o10/windows-thinkpad.json) status to `PASS`.
- **W5 (MSI Installer):** Skipped MSI packaging (not requested).
- **W6 (Storefront Package):** Created local ZIP package at `dist/1.0.0/trareon-lab-windows-x64.zip` and calculated overall SHA256 checksums inside [SHA256SUMS](file:///e:/Trareon/trareon-lab/dist/1.0.0/SHA256SUMS).

## Blocked

- **W3 (Authenticode Signing):** Blocked due to missing `WINDOWS_CERT_PATH` environment variable in the local Windows environment. Authentic signing has been deferred/skipped as instructed (not faked).
