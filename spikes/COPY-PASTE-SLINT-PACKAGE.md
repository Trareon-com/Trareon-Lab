# Slint installer-size package measure (3 OS)

Builds a release binary, zips it as an offline spike package, and writes `installer_size_mib` into `results/<os>-slint.json`.

Signing is **not** done here (`signing=unsigned_spike_artifact`). That is evidence for package size / G1 self-contained binary only — not full G7 signed-installer PASS.

## macOS

```bash
cd "/path/to/Trareon Lab"
git pull origin main
bash spikes/scripts/package-slint.sh macos
```

## Windows PowerShell

```powershell
cd "$HOME\Projects\Trareon\Trareon-Lab"
git pull origin main
powershell -ExecutionPolicy Bypass -File .\spikes\scripts\package-slint-windows.ps1
Get-Content .\spikes\results\windows-slint.json
```

## Kali / Linux

```bash
cd ~/Projects/Trareon/Trareon-Lab
git pull origin main
bash spikes/scripts/package-slint.sh linux
cat spikes/results/linux-slint.json
```

Paste the updated JSON (or just the `installer_size_mib` line) back into chat.
