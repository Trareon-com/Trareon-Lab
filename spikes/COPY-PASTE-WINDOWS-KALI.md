# Gate A — Copy-Paste Commands

Repo **belum otomatis ada** di Windows/Kali. Langkah pertama: **clone** dari GitHub, baru ukur.

Repo: `https://github.com/Trareon-com/Trareon-Lab` (private — perlu login GitHub).

Hasil yang diharapkan: file JSON di `spikes/results/`.

---

## A) ThinkPad X270 — Windows (PowerShell)

Buka **PowerShell biasa** (bukan Administrator). Paste **blok demi blok**.

### A0. Prasyarat sekali saja

```powershell
git --version
winget install --id Git.Git -e --source winget
winget install Rustlang.Rustup
```

Tutup lalu buka lagi PowerShell setelah install Rust/Git.

Login GitHub CLI (karena repo private):

```powershell
winget install --id GitHub.cli -e --source winget
gh auth login
```

Pilih GitHub.com → HTTPS → login via browser.

### A1. Clone repo (sekali saja)

```powershell
cd $HOME
New-Item -ItemType Directory -Force -Path "$HOME\Projects\Trareon" | Out-Null
cd "$HOME\Projects\Trareon"
gh repo clone Trareon-com/Trareon-Lab
cd Trareon-Lab
```

Alternatif tanpa `gh` (akan minta credential GitHub):

```powershell
cd $HOME
New-Item -ItemType Directory -Force -Path "$HOME\Projects\Trareon" | Out-Null
cd "$HOME\Projects\Trareon"
git clone https://github.com/Trareon-com/Trareon-Lab.git
cd Trareon-Lab
```

### A2. Update kode (setiap kali sebelum ukur)

```powershell
cd "$HOME\Projects\Trareon\Trareon-Lab"
git checkout main
git pull origin main
```

### A3. Jalankan pengukuran

```powershell
cd "$HOME\Projects\Trareon\Trareon-Lab\spikes"
New-Item -ItemType Directory -Force -Path results | Out-Null
cargo run -p lab-spike-harness --release -- measure --candidate harness-core --os windows --rows 1000000 --filter-prefix 0 --case-dir ".\results\windows-case" | Tee-Object -FilePath ".\results\windows-harness-core.json"
Get-Content .\results\windows-harness-core.json
```

Pastikan ada `"os": "windows"` dan `"crash_recovery"` mengandung `PASS_`.

### A4. (Opsional) commit hasil dari Windows

```powershell
cd "$HOME\Projects\Trareon\Trareon-Lab"
git add spikes/results/windows-harness-core.json
git commit -m "test: add Windows Gate A harness measurement"
git push origin main
```

---

## B) Kali Linux — Bash

Buka terminal **user biasa** (bukan root). Paste **blok demi blok**.

### B0. Prasyarat sekali saja

```bash
sudo apt update
sudo apt install -y git build-essential pkg-config libssl-dev curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
rustc --version && cargo --version
```

Login GitHub (repo private):

```bash
sudo apt install -y gh
gh auth login
```

### B1. Clone repo (sekali saja)

```bash
mkdir -p ~/Projects/Trareon
cd ~/Projects/Trareon
gh repo clone Trareon-com/Trareon-Lab
cd Trareon-Lab
```

Alternatif tanpa `gh`:

```bash
mkdir -p ~/Projects/Trareon
cd ~/Projects/Trareon
git clone https://github.com/Trareon-com/Trareon-Lab.git
cd Trareon-Lab
```

### B2. Update kode (setiap kali sebelum ukur)

```bash
cd ~/Projects/Trareon/Trareon-Lab
git checkout main
git pull origin main
source "$HOME/.cargo/env"
```

### B3. Jalankan pengukuran

```bash
cd ~/Projects/Trareon/Trareon-Lab/spikes
mkdir -p results
cargo run -p lab-spike-harness --release -- measure \
  --candidate harness-core \
  --os linux \
  --rows 1000000 \
  --filter-prefix 0 \
  --case-dir "./results/linux-case" \
  | tee "./results/linux-harness-core.json"
cat ./results/linux-harness-core.json
```

Pastikan ada `"os": "linux"` dan `"crash_recovery"` mengandung `PASS_`.

### B4. (Opsional) commit hasil dari Kali

```bash
cd ~/Projects/Trareon/Trareon-Lab
git add spikes/results/linux-harness-core.json
git commit -m "test: add Linux Gate A harness measurement"
git push origin main
```

---

## C) Setelah Windows + Kali selesai

Balas di chat Cursor: **“windows dan kali sudah selesai”**  
(atau pastikan kedua file JSON sudah di-push). Saya akan masukkan angka ke matrix dan lanjut UI spike / Task berikutnya.

### File yang harus ada

- `spikes/results/windows-harness-core.json`
- `spikes/results/linux-harness-core.json`

(macOS sudah ada: `spikes/results/macos-harness-core.json`)

---

## Troubleshooting singkat

| Gejala | Perbaikan |
|---|---|
| `Cannot find path ... Trareon-Lab` | Repo belum di-clone — jalankan blok **A1** / **B1** dulu |
| `fatal: not a git repository` | Anda masih di home folder; clone dulu, lalu `cd` ke `Trareon-Lab` |
| `could not find Cargo.toml` | Harus berada di folder `...\Trareon-Lab\spikes` |
| `Repository not found` / auth gagal | Repo private — jalankan `gh auth login` dulu |
| `cargo: command not found` | Install rustup, buka ulang terminal, `source ~/.cargo/env` (Kali) |
| Compile lama pertama kali | Normal (bisa beberapa menit) |
| RAM penuh di ThinkPad | Tutup app lain; 1M rows ~300–400 MiB peak |
| JSON berisi log cargo di atas `{` | Ambil hanya bagian dari `{` sampai `}` atau jalankan ulang perintah ukur |
