# C-AVALONIA measure — copy-paste

Prereq: **.NET 8 SDK**. Prefer ThinkPad (Windows) first.

If `git pull` fails on untracked results JSON, move them to `*.local.bak` first.

## Windows (preferred first)

```powershell
cd "$HOME\Projects\Trareon\Trareon-Lab"
git pull origin main
# Install .NET 8 SDK if needed: https://dotnet.microsoft.com/download
powershell -ExecutionPolicy Bypass -File .\spikes\scripts\measure-avalonia-windows.ps1
```

## Kali

```bash
cd ~/Projects/Trareon/Trareon-Lab
git pull origin main
# sudo apt install -y dotnet-sdk-8.0   # if not installed
bash spikes/scripts/measure-avalonia.sh linux
```

## macOS

```bash
cd "/path/to/Trareon Lab"
git pull origin main
# brew install --cask dotnet-sdk   # if not installed
bash spikes/scripts/measure-avalonia.sh macos
```

Paste `*-avalonia.json` back into chat.
