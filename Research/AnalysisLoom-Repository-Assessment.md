# Assessment Repository AnalysisLoom untuk Trareon Lab

**Tanggal pemeriksaan:** 17 Juli 2026
**Repository:** [YSF-Studio/analysisloom](https://github.com/YSF-Studio/analysisloom)
**Commit yang diperiksa:** [`f187f614a5be6e7a11788d7b77307e05153c94d8`](https://github.com/YSF-Studio/analysisloom/commit/f187f614a5be6e7a11788d7b77307e05153c94d8)

## Kesimpulan

AnalysisLoom berguna sebagai prototype workflow dan katalog modul analisis forensik, tetapi bukan baseline production-ready yang dapat langsung menjadi Trareon Lab. Pola terbaiknya adalah case-centric three-pane UI, deterministic Rust modules, local SQLite case records, evidence hashing, audit trail, sealed cases, reporting, dan explicit tool limitations.

Repository ini **tidak mengimplementasikan AI/LLM**. Tidak ada provider OpenAI/Ollama/OpenRouter, inference engine, model loader, embedding store, atau AI configuration pada source dan manifest yang diperiksa. Komponen “Chat” adalah parser artefak WhatsApp/Telegram/Signal, bukan chatbot. Karena itu AnalysisLoom tidak menjadi contoh implementasi AI; ia menjadi contoh bahwa deterministic forensic substrate perlu berdiri sendiri sebelum AI ditambahkan.

## Yang sudah baik

- [README](https://github.com/YSF-Studio/analysisloom/blob/f187f614a5be6e7a11788d7b77307e05153c94d8/README.md#L14-L82) memperlihatkan workspace case-centric, inspector, timeline, carving, artifact modules, report, dan evidence bundle dalam satu desktop workflow.
- Backend berisi modul Rust terpisah untuk hashing, NTFS, registry, EVTX, carving, SQLite, browser, email, chat, memory bridge, PCAP, report, dan integrity.
- [Cargo manifest](https://github.com/YSF-Studio/analysisloom/blob/f187f614a5be6e7a11788d7b77307e05153c94d8/packages/analysisloom/src-tauri/Cargo.toml) membundel SQLite dan tidak mendeklarasikan dependency AI atau HTTP client secara langsung.
- [Tauri CSP](https://github.com/YSF-Studio/analysisloom/blob/f187f614a5be6e7a11788d7b77307e05153c94d8/packages/analysisloom/src-tauri/tauri.conf.json#L25-L27) membatasi koneksi frontend ke self dan IPC lokal.
- [Security policy](https://github.com/YSF-Studio/analysisloom/blob/f187f614a5be6e7a11788d7b77307e05153c94d8/SECURITY.md#L22-L43) menyatakan tidak ada telemetry atau external network call selama operasi normal serta memiliki dependency-audit practice.
- [Integrity module](https://github.com/YSF-Studio/analysisloom/blob/f187f614a5be6e7a11788d7b77307e05153c94d8/packages/analysisloom/src-tauri/src/forensic/integrity.rs) sudah memiliki manifest parsing, SHA-256 comparison, Ed25519 verification, acquisition-to-analysis hash-chain report, dan audit-entry chaining.
- [Tool limitation metadata](https://github.com/YSF-Studio/analysisloom/blob/f187f614a5be6e7a11788d7b77307e05153c94d8/packages/analysisloom/src-tauri/src/forensic/report_meta.rs) ikut dimasukkan ke report; pola ini penting untuk ISO/IEC 27042 dan ISO/IEC 17025 support.
- CI mencakup formatting, clippy, tests, npm audit, cargo audit, gitleaks, CodeQL, IPC registry, mock coverage, E2E, dan build matrix tiga OS.

## Batas implementasi yang ditemukan

### Belum memenuhi portable tanpa runtime sistem

- [README prerequisites](https://github.com/YSF-Studio/analysisloom/blob/f187f614a5be6e7a11788d7b77307e05153c94d8/README.md#L223-L258) meminta Node, Rust, build tools, WebKitGTK pada Linux, dan WebView2 pada Windows untuk build/run dari source.
- Produk didistribusikan source-only dan belum menyediakan verified portable release bundle.
- Tauri memakai system webview. Ini bertentangan dengan definisi Trareon Lab yang tidak boleh mengandalkan runtime/aplikasi terpasang, terutama pada Linux.

### Klaim `Ready` masih perlu diperlakukan sebagai prototype capability

- [Memory module](https://github.com/YSF-Studio/analysisloom/blob/f187f614a5be6e7a11788d7b77307e05153c94d8/packages/analysisloom/src-tauri/src/forensic/memory.rs) hanya membaca JSON keluaran Volatility 3; raw memory tidak dianalisis langsung dan workflow tetap bergantung pada tool lain.
- [YARA-style scanner](https://github.com/YSF-Studio/analysisloom/blob/f187f614a5be6e7a11788d7b77307e05153c94d8/packages/analysisloom/src-tauri/src/forensic/yara.rs) hanya membaca text/hex strings sederhana, mengabaikan semantic `condition`, wildcard `??`, modules, imports, regex, dan sebagian besar bahasa YARA. Nama UI tidak boleh menyiratkan compatibility penuh.
- [Plugin SDK](https://github.com/YSF-Studio/analysisloom/blob/f187f614a5be6e7a11788d7b77307e05153c94d8/packages/analysisloom/src-tauri/src/forensic/plugins.rs) hanya mendaftarkan hash, entropy, dan strings plugins yang dikompilasi statis; belum ada signed external-pack loader, sandbox, capability manifest, atau validation gate.
- Report metadata mengakui PST/OST masih header/message-stub extraction, PCAP hanya ringkasan IPv4 TCP/UDP/DNS dasar, Linux belum memproses journald/wtmp binary, dan steganography hanya heuristic.
- Keyword search membaca evidence file satu per satu dan bukan indexed multi-encoding search engine.
- Current DB bersifat global di home directory; Trareon Lab memerlukan case-local workspace, isolation, portable path policy, dan sanitized cache lifecycle.

### Integrity semantics perlu diperketat

- Pada [fungsi `verify_file_hash`](https://github.com/YSF-Studio/analysisloom/blob/f187f614a5be6e7a11788d7b77307e05153c94d8/packages/analysisloom/src-tauri/src/forensic/integrity.rs#L260-L296), keadaan tanpa manifest mengembalikan `verified: true` walaupun pesannya mengatakan hash belum diverifikasi terhadap source. Trareon Lab harus memakai status terpisah seperti `VerifiedMatch`, `VerifiedMismatch`, dan `ComputedUnanchored`.
- Lookup manifest memiliki filename/suffix fallback yang berisiko ambigu bila dua evidence item mempunyai nama sama. Trareon Lab harus menggunakan stable evidence/object ID dan exact relative path/range lineage.

### Security boundary perlu lebih sempit

- [Tauri filesystem capability](https://github.com/YSF-Studio/analysisloom/blob/f187f614a5be6e7a11788d7b77307e05153c94d8/packages/analysisloom/src-tauri/capabilities/default.json) memberi main window scope rekursif ke HOME, Documents, Downloads, `/tmp`, `/mnt`, dan `/media`.
- Banyak IPC menerima path string langsung. Trareon Lab harus memakai opaque evidence handles dan case-root broker agar frontend, parser pack, dan AI tidak dapat membaca path di luar case scope.

## Adopt, adapt, reject

| Keputusan | Pola AnalysisLoom | Arah Trareon Lab |
|---|---|---|
| Adopt | Three-pane case workspace | Sources/case tree, tabbed analysis, persistent provenance inspector |
| Adopt | Inline deterministic forensic core | Shared memory-safe core dengan per-format validated analyzers |
| Adopt | Audit, case sealing, limitations in report | Perlu ditingkatkan menjadi typed provenance dan complete audit verification |
| Adopt | Synthetic fixtures dan CI gates | Tambahkan ground-truth corpora, cross-tool comparison, performance, fuzzing, dan per-capability validation |
| Adapt | SQLite case database | Case-local portable workspace dan content-addressed derived-data store |
| Adapt | Plugin trait | Signed/versioned analysis packs dengan permissions, schema, resource limit, SBOM, validation status, dan rollback |
| Adapt | Memory bridge | Native bundled memory engine; Volatility JSON tetap dapat menjadi interchange input |
| Adapt | YARA-style scanning | Implementasi bahasa yang diuji atau compatibility profile yang eksplisit dan sempit |
| Adapt | Tauri/Svelte UI concept | Pertahankan UX, tetapi gunakan desktop stack yang tidak bergantung system webview |
| Reject | Semua modul berlabel Ready dianggap production | Setiap capability harus mempunyai evidence coverage, limitation, test corpus, dan validation state |
| Reject | Arbitrary path access | Semua akses melalui case-scoped evidence broker |

## Implikasi untuk AI Trareon Lab

AI layak menjadi bagian penting Trareon Lab, tetapi diposisikan di atas deterministic evidence plane:

1. **Evidence plane:** raw evidence, hashes, manifests, parser facts, source offsets, timestamps, dan provenance bersifat immutable.
2. **Deterministic analysis plane:** search, parsing, timeline, correlation, YARA/Sigma-like evaluation, calculations, dan rule-based indicators.
3. **AI assistance plane:** natural-language query, prioritization proposal, entity normalization, timeline narrative, alternative hypotheses, finding summary, dan report drafting.
4. **Human decision plane:** examiner menerima, menolak, memperbaiki, atau mengubah AI suggestion menjadi reviewed finding.

AI harus menggunakan runtime inference lokal yang dibundel serta **signed and versioned model packs**. Aplikasi tidak menyediakan network provider. Model pack mencatat model hash, tokenizer/runtime version, prompt template, context policy, sampling parameters, hardware mode, dan validation results.

Setiap jawaban AI harus:

- hanya membaca case-scoped normalized facts dan source excerpts yang diizinkan;
- mengutip Evidence ID, Artifact ID, parser/version, serta byte/object source reference;
- memisahkan fact, inference, uncertainty, dan alternative explanation;
- disimpan sebagai derived result, bukan evidence asli atau reviewed finding;
- dapat dijalankan ulang dan dibandingkan;
- tidak menjalankan shell, membuka jaringan, memuat arbitrary plugin, mengubah evidence, atau otomatis mengesahkan finding;
- memperlakukan isi evidence sebagai data tidak tepercaya untuk mencegah prompt injection dari dokumen/barang bukti.

## Rekomendasi akhir

Gunakan AnalysisLoom sebagai referensi UX, katalog use case, dan prototype deterministic analyzer. Jangan mengadopsi Tauri stack, label maturity, path boundary, atau simplified parser semantics tanpa desain ulang dan validation. Untuk AI, integrasikan sejak arsitektur awal tetapi pertahankan sebagai optional local assistance layer; seluruh workflow forensik utama wajib tetap berfungsi ketika model pack tidak dipasang.
