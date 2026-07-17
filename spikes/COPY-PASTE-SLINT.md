# Gate A — Slint UI measure (copy-paste)

Jalankan **setelah** `git pull`. Membuka jendela sebentar, lalu menulis JSON hasil.

Tidak perlu Administrator/root.

---

## MacBook (sudah diukur oleh agent, atau ulang)

```bash
cd "/Users/user/Projects/Trareon/Trareon Lab/spikes"
git pull origin main
mkdir -p results
cargo run -p slint-app --release -- --measure --os macos --rows 1000000 --filter-prefix 0 \
  --case-dir "./results/macos-slint-case" \
  --out "./results/macos-slint.json"
cat ./results/macos-slint.json
```

---

## Windows (PowerShell)

```powershell
cd "$HOME\Projects\Trareon\Trareon-Lab"
git pull origin main
cd spikes
New-Item -ItemType Directory -Force -Path results | Out-Null
cargo run -p slint-app --release -- --measure --os windows --rows 1000000 --filter-prefix 0 --case-dir ".\results\windows-slint-case" --out ".\results\windows-slint.json"
Get-Content .\results\windows-slint.json
```

---

## Kali (bash)

```bash
cd ~/Projects/Trareon/Trareon-Lab
git pull origin main
source "$HOME/.cargo/env"
cd spikes
mkdir -p results
# dependency UI (sekali saja jika build gagal terkait OpenGL/X11)
sudo apt install -y libfontconfig1-dev libx11-dev libxcursor-dev libxkbcommon-dev libwayland-dev libgl1-mesa-dev
cargo run -p slint-app --release -- --measure --os linux --rows 1000000 --filter-prefix 0 \
  --case-dir "./results/linux-slint-case" \
  --out "./results/linux-slint.json"
cat ./results/linux-slint.json
```

---

Setelah Windows + Kali JSON ada, balas: **“slint windows dan kali selesai”**.
