# C-TAURI measure — copy-paste

Prereqs: Rust + Node.js 20+. On Linux you may need WebKitGTK:

```bash
sudo apt install -y libwebkit2gtk-4.1-dev librsvg2-dev patchelf
```

If `git pull` fails on untracked `spikes/results/*`, move those files aside first (`.local.bak`).

## macOS

```bash
cd "/path/to/Trareon Lab"
git pull origin main
bash spikes/scripts/measure-tauri.sh macos
```

## Windows PowerShell

```powershell
cd "$HOME\Projects\Trareon\Trareon-Lab"
Move-Item -Force .\spikes\results\windows-tauri.json .\spikes\results\windows-tauri.json.local.bak -ErrorAction SilentlyContinue
git pull origin main
powershell -ExecutionPolicy Bypass -File .\spikes\scripts\measure-tauri-windows.ps1
```

## Kali

```bash
cd ~/Projects/Trareon/Trareon-Lab
mv -f spikes/results/linux-tauri.json spikes/results/linux-tauri.json.local.bak 2>/dev/null || true
git pull origin main
sudo apt install -y libwebkit2gtk-4.1-dev librsvg2-dev patchelf \
  libayatana-appindicator3-dev
bash spikes/scripts/measure-tauri.sh linux
```

Paste the JSON (`candidate: C-TAURI`) back into chat.
