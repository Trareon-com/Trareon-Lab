# Gate A Measurement Runbook

Anda punya tiga mesin. Urutan yang disarankan: **MacBook M4 Pro → ThinkPad X270 (Windows) → Kali Linux**.

## Prasyarat bersama

- Git clone/pull repo `Trareon-com/Trareon-Lab` ke branch `main` terbaru.
- Rust stable (rustup) terpasang.
- Node.js 20+ hanya untuk spike Tauri.
- .NET 8 SDK hanya untuk spike Avalonia (wajib di Windows; opsional di macOS/Kali).

## 1) MacBook M4 Pro (macOS)

```bash
cd "/path/to/Trareon Lab/spikes"
cargo test -p lab-spike-core
cargo run -p lab-spike-harness --release -- measure --candidate harness-core --os macos --rows 1000000
```

Simpan JSON output ke:

`spikes/results/macos-harness-core.json`

UI spikes:

```bash
# Slint
cargo run -p slint-app --release

# Tauri (setelah `npm install` di spikes/tauri)
cd tauri && npm install && npm run tauri build
```

Catat cold start / RSS / filter / cancel dari UI ke matrix, atau lampirkan screenshot + notes di `spikes/results/`.

## 2) ThinkPad X270 (Windows)

Di PowerShell:

```powershell
cd path\to\Trareon-Lab\spikes
cargo test -p lab-spike-core
cargo run -p lab-spike-harness --release -- measure --candidate harness-core --os windows --rows 1000000
```

Simpan ke `spikes/results/windows-harness-core.json`.

Untuk Avalonia (setelah .NET 8 SDK):

```powershell
cd avalonia
dotnet build -c Release
dotnet run -c Release
```

Untuk Tauri: install WebView2 Evergreen Runtime hanya jika installer spike membutuhkannya; catat apakah end-user install memerlukan runtime terpisah (Gate G1).

## 3) Kali Linux

```bash
cd ~/Trareon-Lab/spikes
sudo apt update
# Rust via rustup jika belum ada
cargo test -p lab-spike-core
cargo run -p lab-spike-harness --release -- measure --candidate harness-core --os linux --rows 1000000
```

Simpan ke `spikes/results/linux-harness-core.json`.

Catatan Gate G1 untuk Tauri di Linux: jika package membutuhkan `libwebkit2gtk` dari distro sebagai dependency terpisah yang harus diinstal user, tandai risiko G1 di matrix (bukan otomatis FAIL tanpa bukti installer bundle).

## Setelah ketiga OS

1. Salin angka ke `docs/ARCHITECTURE-DECISION-MATRIX.md` raw measurements.
2. Isi mandatory gates `PASS`/`FAIL` per kandidat per OS.
3. Hanya jika satu shell + satu index lulus semua gate: ADR-001/002 → `ACCEPTED`, RFC di-amend menamai stack.

## Jangan

- Jangan membandingkan kandidat antar hardware tier berbeda.
- Jangan memakai evidence nyata / `.fsnap` production.
- Jangan force-accept ADR bila ada `NOT_RUN` atau `FAIL`.
