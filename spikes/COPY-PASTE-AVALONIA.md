# C-AVALONIA measure — copy-paste

Prereq: **.NET 6+ SDK** (spike targets `net6.0`; Windows may use .NET 8 SDK). Prefer ThinkPad first.

If `git pull` fails on untracked results JSON, move them to `*.local.bak` first.

## Windows (preferred first)

```powershell
cd "$HOME\Projects\Trareon\Trareon-Lab"

# 1) unblock pull if needed
Move-Item -Force .\spikes\results\windows-tauri.json .\spikes\results\windows-tauri.json.local.bak -ErrorAction SilentlyContinue
Move-Item -Force .\spikes\results\windows-slint.json .\spikes\results\windows-slint.json.local.bak -ErrorAction SilentlyContinue
git pull origin main

# 2) install .NET 8 SDK (once), then CLOSE and REOPEN PowerShell
winget install Microsoft.DotNet.SDK.8
# verify after new shell:
dotnet --list-sdks

# 3) measure
powershell -ExecutionPolicy Bypass -File .\spikes\scripts\measure-avalonia-windows.ps1
```

Manual download if winget fails: https://dotnet.microsoft.com/download/dotnet/8.0

## Kali

```bash
cd ~/Projects/Trareon/Trareon-Lab
mv -f spikes/results/linux-tauri.json spikes/results/linux-tauri.json.local.bak 2>/dev/null || true
git pull origin main

# Distro often has SDK 6 only (enough — spike is net6.0):
dotnet --list-sdks
# If no SDK at all:
#   sudo apt install -y dotnet-sdk-6.0
# Optional SDK 8 (Microsoft installer):
#   curl -fsSL https://dot.net/v1/dotnet-install.sh | bash -s -- --channel 8.0
#   export PATH="$HOME/.dotnet:$PATH"

bash spikes/scripts/measure-avalonia.sh linux
```

## macOS

```bash
cd "/path/to/Trareon Lab"
git pull origin main
brew install --cask dotnet-sdk
bash spikes/scripts/measure-avalonia.sh macos
```

Paste `*-avalonia.json` back into chat.
