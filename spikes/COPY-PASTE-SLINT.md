# Slint Windows/Kali — copy-paste (fixed)

## Windows PowerShell — paste semua

```powershell
cd "$HOME\Projects\Trareon\Trareon-Lab"

# Kalau git pull gagal karena file lokal windows-harness-core.json:
Remove-Item -Force ".\spikes\results\windows-harness-core.json" -ErrorAction SilentlyContinue

git pull origin main
cd spikes
New-Item -ItemType Directory -Force -Path results | Out-Null

cargo run -p slint-app --release -- --measure --os windows --rows 1000000 --filter-prefix 0 --case-dir ".\results\windows-slint-case" --out ".\results\windows-slint.json"

Write-Host "---- stderr/log di atas; isi file: ----"
Get-Content .\results\windows-slint.json
```

Harus muncul baris seperti:
`measure: wrote ...\windows-slint.json`
lalu JSON dengan `"candidate": "C-SLINT"`.

Jika masih gagal, jalankan binary langsung agar error terlihat:

```powershell
cd "$HOME\Projects\Trareon\Trareon-Lab\spikes"
.\target\release\lab-spike-slint.exe --measure --os windows --rows 1000000 --filter-prefix 0 --case-dir ".\results\windows-slint-case" --out ".\results\windows-slint.json"
echo EXIT:$LASTEXITCODE
Get-Content .\results\windows-slint.json
```

---

## Kali — paste semua

```bash
cd ~/Projects/Trareon/Trareon-Lab
git pull origin main
source "$HOME/.cargo/env"
cd spikes
mkdir -p results
sudo apt install -y libfontconfig1-dev libx11-dev libxcursor-dev libxkbcommon-dev libwayland-dev libgl1-mesa-dev
cargo run -p slint-app --release -- --measure --os linux --rows 1000000 --filter-prefix 0 \
  --case-dir "./results/linux-slint-case" \
  --out "./results/linux-slint.json"
cat ./results/linux-slint.json
```
