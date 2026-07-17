# Trareon Lab

Desktop **digital-forensic analysis lab** — offline-first. Membaca evidence (termasuk `.fsnap` dari Trareon Acquire), menjaga provenance, dan mendukung examination/report tanpa server kolaborasi pusat.

## Status

**Sellable unsigned build program** (0 biaya signing). Foundation milestone sudah di `main`. R1 examination (storage/artifacts/bookmark share) dikerjakan per `docs/superpowers/plans/2026-07-17-trareon-lab-daily-sellable-zero-cost.md`.

Bukan klaim ISO accreditation, court-ready, atau installer bertanda tangan Apple/Microsoft.

## OS yang didukung (matrix terbatas)

| OS | Versi | Arch |
|---|---|---|
| Windows | 10 22H2, 11 23H2/24H2 | x64 |
| macOS | 14, 15 | arm64 (Apple Silicon) |
| Linux | Ubuntu 22.04/24.04 LTS, Debian 12, Kali (lab) | x86_64 |

Di luar tabel = tidak diklaim didukung.

## Quick start (pengembang)

```bash
cargo test --workspace --exclude lab-slint
cargo test -p lab-slint --no-default-features
./packaging/smoke.sh
```

UI penuh (perlu display):

```bash
cargo run -p lab-slint --features gui
```

## Installer unsigned (pembeli / distribusi murah)

Lihat **[docs/SELLING-UNSIGNED.md](docs/SELLING-UNSIGNED.md)** — cara membuka build yang belum ditandatangani vendor OS di Mac, Windows, dan Linux.

## Dokumen utama

- PRD: `PRD-Digital-Forensic-Analysis-Lab.md`
- RFC: `RFC-Digital-Forensic-Analysis-Lab.md`
- R1 matrix: `docs/RELEASE-01-CAPABILITY-MATRIX.md`
- Jadwal harian: `docs/superpowers/plans/2026-07-17-trareon-lab-daily-sellable-zero-cost.md`

## Lisensi

Lihat `LICENSE`.
