# Trareon Lab — Jadwal Day 1 … Day 65 (Gratis · Bisa Dijual)

**Total:** 65 hari kerja berurutan  
**Author commit:** `yusufsaaas <yusuf.shalahuddin@live.com>`  
**Cara pakai:** Kerjakan **Day N** sampai commit hijau, baru lanjut **Day N+1**. Tidak terikat tanggal kalender.

## Definisi “berfungsi sempurna” (0 biaya)

| Boleh | Tidak boleh |
|--------|-------------|
| Jalan di **Windows 10/11 x64**, **macOS 14/15 arm64**, **Ubuntu 22.04/24.04 + Debian 12 + Kali x64** | Klaim “semua versi OS” |
| Installer **unsigned** + panduan buka app | Klaim signed / notarized (berbayar) |
| Case, `.fsnap`, FS R1, artifacts R1, bookmark, share, report | Server kolaborasi live |
| Matrix `Validated`/`Limited` jujur | Klaim ISO / court-ready |

Setiap day = hasil yang bisa dites + **commit** di akhir day.

---

## Day 1
1. Update README: produk lab offline, OS matrix, siap dijual murah.
2. Tulis `docs/SELLING-UNSIGNED.md` (cara buka di Mac/Win/Linux).
3. Naikkan bookmark/share ke R1 di matrix (ADR-008 singkat).
4. Commit.

## Day 2
1. Kontrak Bookmark + `schemas/bookmark.schema.json`.
2. Kontrak Transfer package + skema JSON.
3. Fixture valid/invalid.
4. Commit.

## Day 3
1. Pastikan CI test + clippy + fmt hijau.
2. Tambah `cargo audit` di CI (gratis).
3. Generate SBOM dari `cargo metadata` ke artifact.
4. Commit.

## Day 4
1. Validasi schema bookmark/transfer di `lab-core` + tes.
2. Wireframe teks 7 layar: `docs/ux/EXAMINATION-WIREFRAMES.md`.
3. Commit.

## Day 5
1. `packaging/smoke.sh` jalan di mesin utama.
2. Dokumen build release unsigned 3 OS.
3. Ceklist Day 1–5 selesai.
4. Commit.

## Day 6
1. Buat crate `lab-index`.
2. Tes: simpan 10k baris, tutup, buka lagi masih ada.
3. Commit.

## Day 7
1. Simpan audit/provenance/coverage ke DB case.
2. Tes append-only.
3. Commit.

## Day 8
1. UI Slint: Case / Evidence / Search / Timeline / Bookmarks / Report.
2. Tes model navigasi.
3. Commit.

## Day 9
1. Buka case → angka evidence/coverage nyata dari DB.
2. Update docs offline.
3. Commit.

## Day 10
1. Smoke tipis Day 6–9.
2. Ukur index 100k (catat angka).
3. Commit.

## Day 11
1. Crate `lab-storage`.
2. Buka raw/dd sintetik + SHA-256.
3. Commit.

## Day 12
1. Hash streaming + cancel.
2. Tes cancel tidak menyimpan digest palsu.
3. Commit.

## Day 13
1. Import image → evidence + provenance.
2. Tombol Import di UI.
3. Commit.

## Day 14
1. Baca E01 subset **atau** tandai Limited.
2. Fail-closed format tidak didukung.
3. Commit.

## Day 15
1. Fixture rusak/truncated.
2. Draft user guide “Disk image”.
3. Commit.

## Day 16
1. Crate `lab-fs`.
2. Enumerasi NTFS (corpus sintetik).
3. Commit.

## Day 17
1. FAT32 + exFAT enumerasi.
2. Commit.

## Day 18
1. Baca isi file → CAS.
2. Tes byte cocok.
3. Commit.

## Day 19
1. Metadata masuk index.
2. UI daftar file evidence.
3. Commit.

## Day 20
1. Keyboard: fokus, buka, stub shortcut bookmark.
2. Commit.

## Day 21
1. ext4 enumerasi + baca.
2. Commit.

## Day 22
1. APFS baca (utama Mac; Linux boleh Limited).
2. Tulis batasan jujur di matrix.
3. Commit.

## Day 23
1. Satu metode deleted recovery R1.
2. Label partial/ambiguous.
3. Commit.

## Day 24
1. Dossier validasi storage + bab user guide FS.
2. Commit.

## Day 25
1. Review storage sendiri: PASS/FAIL.
2. Perbaiki temuan kritis.
3. Commit. **Lanjut Day 26 hanya jika PASS.**

## Day 26
1. Crate `lab-artifacts`.
2. Prefetch + tes.
3. Commit.

## Day 27
1. LNK + JumpLists (subset).
2. Commit.

## Day 28
1. Artefak → event timeline.
2. Commit.

## Day 29
1. UI hasil artefak + buka provenance.
2. Commit.

## Day 30
1. Corpus + catatan validasi Windows.
2. Build unsigned Windows (mesin Win atau CI) + catat hasil.
3. Commit.

## Day 31
1. macOS Unified Logs subset.
2. Commit.

## Day 32
1. Linux auth/syslog subset.
2. Commit.

## Day 33
1. Bookmark CRUD di DB + UI panel.
2. Commit.

## Day 34
1. Crate `lab-transfer`: export/import share + Ed25519 lokal.
2. Tes signature rusak → ditolak.
3. Commit.

## Day 35
1. UI Export/Import share pack.
2. Panduan kolaborasi analis offline.
3. Commit.

## Day 36
1. Crate `lab-timeline` gabung event.
2. Commit.

## Day 37
1. Model finding (klaim ↔ bookmark).
2. Commit.

## Day 38
1. Export laporan HTML/JSON (draft vs sealed).
2. Commit.

## Day 39
1. UI Timeline + Findings + Report.
2. Commit.

## Day 40
1. Review artifacts sendiri PASS.
2. User guide reporting.
3. Commit.

## Day 41
1. Query index: path/nama/hash.
2. Commit.

## Day 42
1. UI Search + command palette + shortcut PRD.
2. Commit.

## Day 43
1. Navigasi 1 juta baris sintetik (windowed).
2. Catat kecepatan.
3. Commit.

## Day 44
1. Rapikan UI sesuai wireframe.
2. Commit.

## Day 45
1. Script E2E: case → fsnap → image → FS → artifact → bookmark → share → report.
2. Harus hijau di mesin utama.
3. Commit.

## Day 46
1. Isi dossier FS; status matrix jujur.
2. Commit.

## Day 47
1. Isi dossier artifacts.
2. Commit.

## Day 48
1. Jalankan corpus sintetik/publik; log bukti.
2. Commit.

## Day 49
1. Second-method tipis + blind PT participant stub.
2. Commit.

## Day 50
1. Halaman jual: fitur, batasan, OS, install unsigned, contoh harga.
2. EULA/LICENSE komersial sederhana (teks).
3. Commit.

## Day 51
1. Script build Mac → `.dmg` / `.app.zip` unsigned.
2. Uji buka pakai `SELLING-UNSIGNED.md`.
3. Commit.

## Day 52
1. Script build Windows → `.zip` portable unsigned.
2. Catat SmartScreen + workaround.
3. Commit.

## Day 53
1. Script build Linux → `.tar.gz` / AppImage unsigned.
2. Uji Ubuntu atau Kali.
3. Commit.

## Day 54
1. Folder `dist/0.9.0/` + checksum SHA-256 + `SHA256SUMS.txt`.
2. Jangan force-commit binary besar ke git bila penuh.
3. Commit script/catatan.

## Day 55
1. Smoke 3 OS (fisik atau CI + catat gap).
2. Commit bukti smoke.

## Day 56
1. Perbaiki semua bug P0 dari smoke.
2. Commit.

## Day 57
1. About UI: versi, SBOM ringkas, batasan, link panduan unsigned.
2. Commit.

## Day 58
1. Matrix final + known issues.
2. Commit.

## Day 59
1. Teks/skrip demo penjualan (±5 menit).
2. Commit docs.

## Day 60
1. Tag `v0.9.0-sellable` (boleh dijual; bukan signed Official).
2. Siapkan bahan `v1.0.0` jika semua hijau.

## Day 61
1. Hanya perbaikan regresi.
2. Commit.

## Day 62
1. Rebuild `dist/1.0.0/` 3 OS + checksum.
2. E2E sekali lagi di mesin utama.

## Day 63
1. Tag `v1.0.0`.
2. Upload artifact (GitHub Release / drive).
3. Publikasikan harga murah + link + panduan unsigned.

## Day 64
1. Tes unduh seperti pembeli baru di Mac + Win + Linux.
2. Perbaiki panduan bila ada friksi.
3. Commit.

## Day 65
1. Go-live jual.
2. Catat backlog opsional nanti: signing berbayar jika omzet ada.

---

## Tidak dikerjakan (tetap 0 biaya)

- Apple Developer / notarization  
- Windows Authenticode  
- Reviewer hukum berbayar  
- Klaim semua versi OS  
- Server kolaborasi  

## Kalimat pelanggan

> “Trareon Lab 1.0 — analisis forensik offline untuk Windows 10/11, macOS 14/15 (Apple Silicon), dan Linux LTS terdaftar. Installer belum ditandatangani vendor OS; ikuti panduan buka aplikasi. Cocok lab/training; bukan klaim akreditasi.”

---

**Sekarang:** mulai dari **Day 1**, selesai commit, lanjut **Day 2**.
