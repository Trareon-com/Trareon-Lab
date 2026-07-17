# Gate A index spike — copy-paste

```bash
# macOS / Kali
cd <repo>
git pull origin main
bash spikes/scripts/measure-index.sh macos   # or linux
```

```powershell
# Windows
cd "$HOME\Projects\Trareon\Trareon-Lab"
git pull origin main
powershell -ExecutionPolicy Bypass -File .\spikes\scripts\measure-index-windows.ps1
```

Paste the three JSON files (`*-d-sqlite.json`, `*-d-sqlite-fts.json`, `*-d-rust-index.json`).
