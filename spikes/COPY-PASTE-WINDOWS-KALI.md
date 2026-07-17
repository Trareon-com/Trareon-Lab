# Gate A — Copy-Paste Commands

Jalankan **setelah** `git pull` di mesin masing-masing agar kode spike terbaru ada.

Hasil yang diharapkan: file JSON di `spikes/results/`.

---

## A) ThinkPad X270 — Windows (PowerShell)

Buka **PowerShell**, lalu paste **blok demi blok**.

### A1. Masuk ke repo (sesuaikan path)

```powershell
cd $HOME\Projects\Trareon\Trareon-Lab
# kalau path Anda beda, contoh:
# cd D:\Trareon\Trareon-Lab
```

### A2. Update kode

```powershell
git checkout main
git pull origin main
```

### A3. Cek Rust (install sekali saja jika belum ada)

```powershell
rustc --version
cargo --version
```

Jika error “not recognized”, install Rust lalu **buka ulang PowerShell**:

```powershell
winget install Rustlang.Rustup
```

atau buka https://rustup.rs dan ikuti installer Windows.

### A4. Jalankan pengukuran (copy semua)

```powershell
cd spikes
New-Item -ItemType Directory -Force -Path results | Out-Null
cargo run -p lab-spike-harness --release -- measure --candidate harness-core --os windows --rows 1000000 --filter-prefix 0 --case-dir ".\results\windows-case" | Tee-Object -FilePath ".\results\windows-harness-core.json"
```

### A5. Cek hasil

```powershell
Get-Content .\results\windows-harness-core.json
```

Pastikan ada field `"os": "windows"` dan `"crash_recovery"` mengandung `PASS_`.

### A6. (Opsional) commit hasil dari Windows

```powershell
cd ..
git add spikes/results/windows-harness-core.json
git commit -m "test: add Windows Gate A harness measurement"
git push origin main
```

---

## B) Kali Linux — Bash

Buka terminal, paste **blok demi blok**.

### B1. Masuk ke repo (sesuaikan path)

```bash
cd ~/Projects/Trareon/Trareon-Lab
# atau:
# cd ~/Trareon-Lab
```

### B2. Update kode

```bash
git checkout main
git pull origin main
```

### B3. Cek Rust (install sekali saja jika belum ada)

```bash
rustc --version && cargo --version
```

Jika belum ada:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
rustc --version && cargo --version
```

### B4. Dependency build dasar (sekali saja jika compile gagal)

```bash
sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev
```

### B5. Jalankan pengukuran (copy semua)

```bash
cd spikes
mkdir -p results
cargo run -p lab-spike-harness --release -- measure \
  --candidate harness-core \
  --os linux \
  --rows 1000000 \
  --filter-prefix 0 \
  --case-dir "./results/linux-case" \
  | tee "./results/linux-harness-core.json"
```

### B6. Cek hasil

```bash
cat ./results/linux-harness-core.json
```

Pastikan ada `"os": "linux"` dan `"crash_recovery"` mengandung `PASS_`.

### B7. (Opsional) commit hasil dari Kali

```bash
cd ..
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
| `cargo: command not found` | Install rustup, buka ulang terminal, `source ~/.cargo/env` (Kali) |
| Compile lama pertama kali | Normal (bisa beberapa menit) |
| RAM penuh di ThinkPad | Tutup app lain; 1M rows ~300–400 MiB peak |
| `git pull` conflict | Jangan force; kirim pesan error ke chat |
| JSON berisi log cargo di atas `{` | Ambil hanya bagian dari `{` sampai `}` atau jalankan ulang perintah di atas |
