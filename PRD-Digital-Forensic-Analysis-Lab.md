# PRD — Trareon Lab

## Kontrol Dokumen

| Field | Nilai |
|---|---|
| Nama produk | Trareon Lab |
| Jenis produk | Desktop digital-forensic analysis laboratory |
| Platform target | Windows, macOS, Linux |
| Distribusi utama | Install-first, offline-first |
| Versi dokumen | 1.0 |
| Status | Official R1.0.0 program active (target 2026-10-16); Engineering Alpha sellable path complete; Official O1–O12 gates open |
| Tanggal baseline | 17 Juli 2026 |
| Tanggal finalisasi | 17 Juli 2026 |
| Pemilik produk | Trareon |
| Bahasa produk | Bahasa Indonesia dan Inggris |
| Official remote | https://github.com/Trareon-com/Trareon-Lab |

Dokumen ini mengikuti struktur `Template/Desktop/PRD-Desktop.md`. Istilah “mendukung” pada dokumen ini berarti menyediakan kontrol dan rekaman yang dapat membantu organisasi menerapkan proses tertentu; istilah tersebut bukan klaim sertifikasi produk, akreditasi laboratorium, admissibility otomatis, atau jaminan bahwa suatu hasil selalu benar.

**change control:** Setiap perubahan ruang lingkup setelah baseline ini disetujui harus dicatat pada `docs/DECISION-REGISTER.md` dan hanya berlaku setelah gate yang relevan dinyatakan lulus.

### Revision History

| Versi | Tanggal | Ringkasan |
|---|---|---|
| 0.9 | 17 Juli 2026 | Product baseline approved; architecture pending; controlled planning registers established |
| 1.0 | 17 Juli 2026 | Best-practice finalization: FR-CASE-016, FR-VAL-009–011; ADR-013–016; SWGDE 18-Q-001 source; Research provenance locked |
| 1.0.1 | 17 Juli 2026 | Official R1.0.0 program status line activated; runbook O1–O12 linked |

### Research provenance

Input riset terkendali (bukan normative standards text):

- `Research/Digital-Forensic-Analysis-Best-Practices.md`
- `Research/AnalysisLoom-Repository-Assessment.md`
- `Research/Trareon-Lab-Standards-Compliance-Baseline.md`
- `Research/Trareon-Lab-Pain-Point-Gap-Analysis.md`
- `Research/Trareon-Lab-Indonesia-Regulatory-Baseline.md`

Keputusan produk yang menyimpang dari prioritas Research awal dikunci pada `docs/DECISION-REGISTER.md` ADR-013–015.

---

## 1. Ringkasan

- **Nama produk:** Trareon Lab
- **Platform target:** Windows, macOS, Linux
- **Versi dokumen:** 1.0
- **Status:** Product Baseline Finalized; Architecture Accepted (Gates A–E PASS); Foundation Complete; R1 Next

Trareon Lab adalah aplikasi desktop install-first dan offline-first untuk pemeriksaan serta analisis barang bukti digital dari media penyimpanan, filesystem, RAM, network capture, artefak sistem operasi, aplikasi, file, dokumen, gambar, video, dan audio. Produk mengonsumsi paket `.fsnap` dari Trareon Acquire dan format evidence umum, serta dapat menganalisis komputer tempat aplikasi dipasang melalui workflow live-host yang terkontrol.

Trareon Lab diposisikan sebagai **case-centric, defensible digital-forensic laboratory**, bukan kumpulan viewer atau tombol parser. Produk harus menjaga evidence asli tetap immutable, memperlihatkan processing coverage dan kegagalan secara jujur, menautkan setiap hasil ke sumbernya, memisahkan fakta dari interpretasi, mendukung pengulangan analisis, serta menghasilkan rekaman yang dapat ditinjau secara independen.

Seluruh fungsi forensik inti berjalan tanpa internet dan tanpa bergantung pada aplikasi forensik eksternal. AI bersifat opsional dan menggunakan inference provider lokal yang dikelola pengguna melalui Ollama atau LM Studio. Ketika provider AI tidak tersedia, seluruh workflow inti tetap berfungsi.

Distribusi utama adalah installer bertanda tangan untuk setiap OS. Portable edition tidak menjadi syarat rilis P0 dan ditempatkan pada roadmap P2.

### Hubungan dengan Trareon Acquire

- **Trareon Acquire:** identification, collection, acquisition, preservation, hashing, manifest, audit, dan pembuatan `.fsnap`.
- **Trareon Lab:** intake verification, processing, examination, interpretation, correlation, review, reporting, validation, dan case closure.
- Trareon Lab tidak mengubah Trareon Acquire menjadi analysis tool.
- Untuk live-host analysis, Trareon Lab mengutamakan snapshot/capture yang kompatibel dengan Trareon Acquire sebelum melakukan pemeriksaan.

### Prinsip keputusan produk

1. Validitas dan transparansi lebih penting daripada jumlah parser.
2. Evidence, deterministic result, AI suggestion, dan human conclusion berada pada lapisan berbeda.
3. Tidak ada status sukses yang menyembunyikan data gagal, dilewati, tidak didukung, rusak, atau ambigu.
4. Seluruh hasil penting mempunyai lineage ke evidence source.
5. Capability claim selalu dibatasi oleh fungsi, format, versi, OS, dataset validasi, dan known limitation.
6. Produk mendukung proses laboratorium; produk tidak menggantikan kewenangan, kompetensi, SOP, legal review, atau keputusan pengadilan.

---

## 2. Masalah yang Ingin Diselesaikan

### Masalah utama

Digital-forensic examiner menghadapi kombinasi masalah berikut:

1. **Tool fragmentation:** disk, RAM, network, artefak, timeline, media, review, dan report dikerjakan di aplikasi berbeda sehingga provenance dan audit trail terputus.
2. **False completeness:** job terlihat selesai walaupun sebagian evidence gagal diproses, format tidak didukung, parser timeout, atau encrypted content dilewati.
3. **Opaque parser trust:** pengguna sulit mengetahui versi artefak yang didukung, dasar interpretasi, status validasi, serta risiko false positive/negative.
4. **Weak reproducibility:** hasil berubah setelah parser, OS, symbol, rule, timezone database, model AI, atau konfigurasi berubah.
5. **Lost provenance:** timeline event, graph edge, carved file, entity, tag, dan finding sering tidak dapat ditelusuri hingga byte/object sumber.
6. **Scale and backlog:** evidence berukuran multi-terabyte dan jutaan objek membuat examiner menunggu ingest selesai sebelum dapat bekerja.
7. **Silent parser failure:** aplikasi menyembunyikan kegagalan dalam log teknis atau menyamakannya dengan “tidak ditemukan”.
8. **Bias and premature conclusions:** heuristic, threat score, rule hit, atau AI narrative dapat dianggap sebagai fakta.
9. **Privacy and over-collection:** data di luar scope, privileged material, credential, dan PII pihak tidak terkait mudah ikut diproses atau diekspor.
10. **Hostile evidence:** file, archive, codec, macro, script, atau metadata dapat mengeksploitasi forensic workstation.
11. **Weak review lifecycle:** technical review, administrative review, disagreement, amendment, dan report authorization tidak terintegrasi.
12. **Tool-generated reporting:** report yang rapi belum tentu mendokumentasikan keseluruhan scope, metode, error, limitation, dan hasil yang bertentangan.
13. **Validation burden:** laboratorium harus membuktikan capability dan limitation setiap metode serta mengulang validasi ketika lingkungan berubah.
14. **Offline capability drift:** parser, symbol, signature, rule, documentation, dan validation data cepat usang tetapi casework tidak boleh bergantung pada cloud.
15. **AI risk:** hallucination, prompt injection dari evidence, context leakage, nondeterminism, dan model drift dapat mencemari kesimpulan.
16. **Indonesia-specific operations:** kebutuhan Bahasa Indonesia, WIB/WITA/WIT, pola PII lokal, legal authority, perlindungan data pribadi, serta format laporan lokal sering tidak didukung baik oleh produk global.

### Siapa yang terdampak

- Digital-forensic examiner dan investigator.
- Technical reviewer dan administrative reviewer.
- Report authorizer dan laboratory manager.
- Incident responder yang membutuhkan analisis post-acquisition.
- Auditor mutu, assessor, dan validation lead.
- Penegak hukum, regulator, tim hukum, dan pihak berwenang yang menerima hasil pemeriksaan.
- Organisasi swasta yang melakukan internal investigation dengan otorisasi yang sah.
- Pendidik dan peserta proficiency/training exercise.

### Dampak bisnis dan pengguna

- Waktu pemeriksaan meningkat akibat perpindahan tool dan pengulangan parsing.
- Risiko kesimpulan salah meningkat ketika limitation atau coverage gap tidak terlihat.
- Biaya validasi, pelatihan, storage, dan pemeliharaan menjadi sulit diprediksi.
- Audit dan legal review memerlukan rekonstruksi manual terhadap langkah examiner.
- Data sensitif dapat terekspos melalui tool cloud atau export yang tidak terkendali.
- Laporan dapat dipertanyakan karena tidak mempunyai lineage, method record, atau review history yang memadai.
- Kasus lama sulit dibuka kembali ketika format, tool, atau environment berubah.

---

## 3. Tujuan

### Tujuan utama

1. Menyediakan satu workspace offline untuk analisis disk, filesystem, artefak desktop, RAM, network capture, file, dan media dengan provenance terpadu.
2. Memastikan evidence asli dibuka read-only dan setiap derived result dapat ditelusuri ke input serta metode yang menghasilkannya.
3. Menghilangkan false completeness melalui typed integrity state, processing coverage map, transparent errors, dan report blocking untuk kondisi kritis.
4. Mendukung repeatability, reproducibility, independent scrutiny, method validation, technical review, dan report authorization.
5. Mengurangi backlog melalui streaming pipeline, persistent index, partial results, checkpoint/resume, deduplication, dan resource-aware parallelism.
6. Menyediakan pengalaman yang bersih seperti aplikasi macOS namun tetap mempunyai dense tables, shortcuts, query builder, batch operation, context menus, dan command palette seperti aplikasi Windows untuk power user.
7. Menyediakan bantuan AI lokal yang evidence-cited, case-scoped, optional, dan selalu berada di bawah keputusan manusia.
8. Menyediakan dokumentasi offline, validation evidence, limitation matrix, dan regulatory profile Indonesia sebagai release gate.

### Tujuan sekunder

- Mendukung format terbuka dan interchange melalui `.fsnap`, machine-readable export, serta CASE/UCO profile.
- Menyediakan validated and signed analysis-pack ecosystem tanpa membiarkan pack mengakses path, network, atau resource di luar izin.
- Mendukung laboratory-quality record seperti competency authorization, deviation, nonconforming work, proficiency exercise, dan management review tanpa mengklaim akreditasi otomatis.
- Menyediakan fondasi universal untuk pemeriksaan teknis file, dokumen, gambar, video, dan audio sebelum pack autentikasi spesialis P2 tersedia.
- Memungkinkan perluasan regulasi dari Indonesia ke yurisdiksi global melalui versioned jurisdiction profiles.

### Definisi keberhasilan produk

Trareon Lab dianggap berhasil bila examiner dapat menerima evidence, memahami apa yang berhasil dan tidak berhasil diproses, memeriksa hasil hingga sumber mentah, membangun serta meninjau finding, mengulang analisis dengan konfigurasi yang sama, dan menghasilkan report package yang independently verifiable tanpa membutuhkan internet.

---

## 4. Non-Tujuan

Yang tidak akan dibangun sebagai capability P0:

- Akuisisi fisik perangkat sebagai pengganti Trareon Acquire.
- Remediation, quarantine, process termination, deletion, atau perubahan target system.
- Malware detonation pada workstation utama.
- Public sandbox submission atau threat-intelligence lookup tanpa connected companion terpisah.
- OpenAI, OpenRouter, atau provider AI berbasis internet.
- Verdict otomatis tentang guilt, intent, malware, authenticity, identity, atau admissibility.
- Jaminan bahwa seluruh evidence telah ditemukan atau suatu metode bebas error.
- Klaim “ISO-certified forensic software”, “ISO/IEC 17025 compliant tool”, “ISO/IEC 17043 accredited software”, “court-approved”, atau “court-admissible”.
- Formal administration platform untuk penyelenggara proficiency-testing ISO/IEC 17043.
- Device acquisition dan key extraction mobile sebagai bagian Lab P0.
- Full cloud, IoT, vehicle, dan blockchain investigation.
- Password-cracking farm.
- Deepfake, speaker identification, PRNU source attribution, dan multimedia authenticity conclusion sebagai capability P0.
- Portable edition dengan feature parity pada rilis P0.
- Central collaborative server dan cross-case intelligence secara default.

---

## 5. Ruang Lingkup

### In scope — P0 defensible core

#### A. Case governance dan lifecycle

- Request, acceptance, legal authority, scope, minimization, due date, handling restriction, dan requester communication.
- State: Request, Acceptance, Intake, Verification, Planning, Processing, Examination, Interpretation, Review, Reporting, Closure, Retention/Disposition, Amendment.
- Role separation: Examiner, Technical Reviewer, Administrative Reviewer, Report Authorizer, Administrator, Validation Lead.
- Case-local encrypted workspace, append-only audit, case seal, retention, legal hold, sanitization, dan signed offline handoff.
- Supporting, conflicting, contradictory, dan exculpatory evidence sebagai first-class records.

#### B. Evidence intake, integrity, dan coverage

- Native `.fsnap` verification terhadap manifest, audit, hash, signature, acquisition limitation, dan chain of custody.
- Import RAW/split RAW, E01/Ex01, AFF4, VHD/VHDX, VMDK, QCOW2, logical file set, ZIP/TAR, RAM image, PCAP/PCAPNG, serta adapter untuk machine-readable export umum.
- Typed state: `VerifiedMatch`, `VerifiedMismatch`, `ComputedUnanchored`, `SignatureInvalid`, `IncompleteInput`, dan `Unsupported`.
- Read-only evidence layer, working-copy relationship, stable Evidence ID, exact object/path/range lineage, dan content-addressed derived store.
- Partition, volume, filesystem, encryption, hidden area, truncated/damaged input, serta unsupported-feature detection.
- Coverage map untuk processed, failed, skipped, unsupported, encrypted, ambiguous, truncated, dan timeout items.

#### C. Filesystem, recovery, search, dan preview

- NTFS, ReFS, FAT/exFAT; APFS/HFS+; ext2/3/4, XFS, Btrfs, dan ZFS read-only berdasarkan capability matrix per fungsi.
- Allocated/deleted view, slack/unallocated search, recovery, carving, nested-container expansion, ADS/xattr/resource fork, sparse/compressed file, link semantics, permission/ACL, dan journal examination.
- Full-text dan metadata index, Boolean, exact phrase, wildcard, regex, byte/hex, encoding-aware, fuzzy search, saved query, dan search-plan provenance.
- Offline known-file filtering dengan NSRL/custom known-good serta versioned known-bad sets.
- Safe preview untuk text, code, log, image, archive, PDF, Office, SQLite, PLIST, Registry, ESE, email, hex, dan structured metadata tanpa mengeksekusi active content.

#### D. Artefak desktop dan aplikasi

- Windows Registry, Event Logs, Prefetch, Amcache/Shimcache, SRUM, LNK/Jump Lists, Shellbags, Recycle Bin, Task Scheduler, services, USB/device history, browser, user profile, dan persistence artefacts.
- macOS plist, Unified Logs, FSEvents, Spotlight metadata, quarantine/download history, launch agent/daemon, TCC, browser, users, dan mount history.
- Linux systemd journal, audit/auth logs, shell history, cron/systemd unit, package history, login/session, mount/device, browser, dan common desktop artefacts.
- Browser, email/mailbox, locally stored messaging/collaboration data, document metadata, archive, cloud-sync client artefacts, SQLite, Registry, ESE, dan PLIST.

#### E. Timeline, entity, graph, pattern, dan triage

- Normalized super-timeline yang mempertahankan raw timestamp, interpreted timestamp, timezone, clock offset, precision, parser, dan limitation.
- Cross-source pivot untuk user/account, device, path, hash, process, domain/IP, URL, email, application, dan time window.
- Evidence graph dan event sequence dengan reversible navigation ke source.
- Entity extraction termasuk pola Indonesia seperti NIK, nomor telepon, rekening, NPWP, serta PII lainnya dengan access control.
- Pattern-of-life dan anti-forensics indicators yang selalu menyajikan alternative explanation.
- YARA/Sigma-compatible rules sesuai capability profile yang diuji; rule hit menjadi indication, bukan conclusion.
- Automated triage hanya menghasilkan proposed priority, tag, atau hypothesis dan memerlukan examiner review.

#### F. RAM dan network

- Identification memory image serta offline symbol-pack management.
- Process, thread, module, handle, socket, command line, environment, injected/mapped memory, driver/module, persistence clue, dan extracted object berdasarkan validated capability.
- PCAP/PCAPNG flow/sessionization, DNS, HTTP, TLS metadata, common protocol parsing, object extraction, capture-gap awareness, dan host/process correlation bila data mendukung.
- Import Volatility-compatible structured output sebagai interchange; core P0 tidak boleh diam-diam bergantung pada instalasi Volatility eksternal.

#### G. Universal file and media examination foundation

- Cryptographic hash, magic/signature, container/codec identification, metadata, raw structure, embedded object, source-offset provenance, safe preview, dan malformed-input detection.
- Image metadata dan structure inspection tanpa authenticity verdict.
- Video container, stream, codec, frame/keyframe, dan timestamp inspection tanpa source/authenticity verdict.
- Audio container, codec, metadata, waveform, dan spectrogram inspection tanpa speaker/authenticity verdict.
- Document container, metadata, digital signature status, revision/incremental structure, macro/script indicator, hidden/embedded object, dan safe text extraction.
- C2PA/Content Credentials verification dengan pemisahan tegas antara manifest integrity dan truth of content.

#### H. Findings, review, dan reporting

- Model: Extracted Fact → Observation → Automated Indication → Hypothesis → Reviewed Finding → Approved Conclusion.
- Notes, bookmarks, tags, tasks, saved views, supporting data, contradictory/exculpatory information, deviation, dan unresolved questions.
- Technical review, administrative review, disagreement, resolution, sign-off, authorization, amendment, dan original-report preservation.
- Report templates Bahasa Indonesia/Inggris, executive dan technical report, redaction, exhibit numbering, PDF/A, standalone HTML, machine-readable package, dan version-pinned CASE/UCO interchange.
- Report tidak dapat difinalkan ketika integrity mismatch kritis atau relevant critical processing errors belum didisposisi dan diungkapkan.

#### I. Validation Center dan quality support

- Stable Method ID, intended use, capability matrix, limitation, validation status, owner, dan revalidation trigger.
- Golden datasets, known ground truth, regression, fuzzing, malformed inputs, cross-version comparison, performance baseline, anomaly/discrepancy register, dan signed validation dossier.
- Function-based tool testing records sebelum casework, mengacu SWGDE 18-Q-001 melalui source register.
- Exact rerun, compare-runs, dan second-method/cross-tool verification dengan discrepancy disposition.
- Competency/method authorization gate, deviation, nonconforming work, corrective action, equipment/environment record, dan management-review export.
- Versioned examination plan dengan contemporaneous notes linkage.
- Import blind challenge dan export signed participant result untuk membantu laboratory exercise; penyelenggaraan formal PT provider tetap di luar P0.

#### J. Local AI assistance

- Provider: Ollama dan LM Studio melalui loopback-only adapter.
- Core tetap berfungsi ketika AI tidak dikonfigurasi.
- Use case P0: natural-language query terhadap normalized facts, prioritization proposal, evidence summary draft, alternative-hypothesis prompt, timeline narrative draft, dan report drafting.
- Context broker membatasi AI pada case-scoped normalized facts serta examiner-approved source excerpts.
- Output AI wajib mengutip Evidence ID/Artifact ID/source reference, memisahkan fact/inference/uncertainty, dan berstatus suggestion.
- Rekaman provider/version, model identifier/digest bila tersedia, prompt-template hash, context policy, sampling parameters, retrieved sources, output, dan reviewer disposition.
- AI tidak mempunyai shell, internet, arbitrary filesystem, write-to-evidence, autonomous tool execution, atau final-approval authority.

#### K. Dokumentasi dan Indonesia-first regulatory profile

- Offline Start Here, User Guide, Examiner Workflow, Live-host Guide, AI Guide, Format/Capability Matrix, Validation Manual, Laboratory Quality Guide, Reporting Guide, Admin Guide, Security Guide, Troubleshooting, Training Labs, Release Notes, dan Migration/Reprocessing Guide.
- Contextual help terhubung ke parser/version, field meaning, source semantics, validation status, limitation, warning, dan manual verification.
- Versioned Indonesia regulatory/control register dan safe claims wording yang ditinjau sebelum major release.
- International jurisdiction profiles ditambahkan bertahap setelah Indonesia.

### In scope — P2 specialist packs dan roadmap

- Portable edition.
- Image Authentication and Comparison Pack.
- Video/CCTV Authentication, Enhancement, Synchronization, and Comparison Pack.
- Audio Authentication, Enhancement, Transcription, and Speaker Comparison Pack.
- Advanced Document Authentication Pack.
- Deepfake/AI-generated Media Research Pack.
- Steganalysis Pack.
- Mobile-backup parsing dan, melalui PRD terpisah, device acquisition/key recovery.
- Container/Docker/Kubernetes forensics.
- Cloud, IoT, vehicle, dan blockchain forensic packs.
- Advanced cross-case intelligence dan multi-case comparison dengan explicit authority.
- Local distributed processing pada worker jaringan lab yang diaudit.
- Advanced RAID/LVM/dynamic disk/Storage Spaces/APFS Fusion reconstruction.
- Advanced OCR, geolocation, media comparison, password recovery, dan malware static-analysis packs.
- AI-assisted advanced correlation dan similar-case matching dengan privacy controls.
- Formal proficiency-testing provider module melalui PRD terpisah.

### Out of scope atau companion terpisah

- Connected threat-intelligence lookup, blockchain history query, public sandbox submission, URL scanning, dan provider AI internet.
- Malware detonation, quarantine, remediation, process termination, atau deletion.
- Central cloud case storage.
- Covert monitoring atau surveillance.
- Capability yang melanggar otorisasi, hukum, lisensi, atau cryptographic access controls.

---

## 6. User Story

### Examiner

- Sebagai examiner, saya ingin mengimpor `.fsnap` dan melihat status hash, signature, audit, serta acquisition limitation supaya saya mengetahui apakah evidence layak diproses.
- Sebagai examiner, saya ingin melihat item yang gagal, dilewati, terenkripsi, ambigu, atau tidak didukung supaya saya tidak menarik kesimpulan dari hasil yang tampak lengkap tetapi sebenarnya parsial.
- Sebagai examiner, saya ingin membuka artefak sampai ke raw value, byte range, atau object sumber supaya saya dapat memverifikasi hasil parser secara manual.
- Sebagai examiner, saya ingin mulai meninjau partial results saat indexing berjalan supaya waktu tunggu kasus besar berkurang.
- Sebagai examiner, saya ingin membatasi processing, search, AI, dan export berdasarkan scope supaya data di luar otorisasi tidak terekspos.
- Sebagai examiner, saya ingin membandingkan beberapa hipotesis dan menandai conflicting/exculpatory evidence supaya analisis tidak mendorong confirmation bias.
- Sebagai examiner, saya ingin menjalankan ulang metode dengan versi dan konfigurasi yang sama supaya hasil dapat diuji kembali.
- Sebagai examiner, saya ingin menggunakan Ollama atau LM Studio untuk membantu mencari dan merangkum evidence tanpa mengirim data kasus ke internet.

### Reviewer dan authorizer

- Sebagai technical reviewer, saya ingin melihat supporting evidence, source reference, metode, error, limitation, serta alternative explanation supaya finding dapat diuji secara independen.
- Sebagai administrative reviewer, saya ingin memeriksa identitas kasus, scope, format laporan, amendment, dan approval record supaya laporan konsisten dengan prosedur.
- Sebagai report authorizer, saya ingin aplikasi memblokir finalisasi ketika integrity mismatch atau critical undisclosed error masih ada supaya report tidak dilepas secara prematur.

### Laboratory manager dan validation lead

- Sebagai laboratory manager, saya ingin melihat method authorization, competency record, deviation, nonconforming work, dan validation status supaya hanya capability yang disetujui dipakai untuk casework.
- Sebagai validation lead, saya ingin menjalankan golden datasets, repeatability, reproducibility, regression, dan differential tests supaya status capability didukung evidence.
- Sebagai auditor, saya ingin mengekspor audit, method, validation, review, dan amendment records supaya proses dapat ditinjau tanpa mengakses raw evidence yang tidak diperlukan.

### Administrator

- Sebagai administrator, saya ingin menginstal aplikasi, analysis packs, symbols, rules, documentation, dan update melalui signed offline bundle supaya lab dapat beroperasi di jaringan terisolasi.
- Sebagai administrator, saya ingin backup, restore, retention, legal hold, dan sanitization mempunyai verification record supaya lifecycle kasus dapat dibuktikan.

### Investigator/requester

- Sebagai requester, saya ingin menyatakan tujuan, authority, scope, due date, dan handling restriction supaya examiner memahami batas pemeriksaan.
- Sebagai investigator, saya ingin menerima executive summary yang dapat dipahami serta technical annex yang dapat diverifikasi supaya hasil dapat digunakan secara tepat.

### Pengguna baru dan peserta training

- Sebagai pengguna baru, saya ingin Guided Mode dan bantuan lokal Bahasa Indonesia supaya saya dapat mengikuti workflow aman tanpa menguasai seluruh opsi teknis.
- Sebagai peserta training, saya ingin menjalankan synthetic cases dengan expected results tanpa bercampur dengan real casework supaya kompetensi dapat dilatih dan dinilai.

---

## 7. Kebutuhan Fungsional

Kata **harus** menunjukkan requirement wajib P0. Kata **dapat** menunjukkan capability opsional atau P2. Setiap requirement harus mempunyai owner, verification method, evidence artifact, dan release status pada traceability register.

### 7.1 Case governance dan isolation

- **FR-CASE-001:** Sistem harus membuat identifier unik untuk setiap case, evidence item, examination action, derived object, finding, report, dan export.
- **FR-CASE-002:** Sistem harus menyimpan requester, authority/reference, purpose, scope, permitted evidence, date/artifact/search boundaries, due date, handling restrictions, dan exclusions sebelum intake diterima.
- **FR-CASE-003:** Sistem harus mendukung keputusan `Accepted`, `Conditionally Accepted`, `Returned for Clarification`, atau `Rejected` beserta alasan dan pihak yang menyetujui.
- **FR-CASE-004:** Sistem harus menegakkan scope pada processing, preview, search, AI context, export, dan report; scope tidak boleh hanya menjadi filter tampilan.
- **FR-CASE-005:** Scope override harus menyimpan alasan, approver, masa berlaku, affected operations, dan residual risk.
- **FR-CASE-006:** Setiap case harus berjalan pada workspace, cache, index, temporary extraction, AI context, dan audit namespace yang terisolasi.
- **FR-CASE-007:** Satu case harus dibuka pada satu isolated window/process; membuka case lain membuat window/process lain.
- **FR-CASE-008:** Transfer data antarkasus harus melalui signed export/import workflow dan dicatat pada kedua case.
- **FR-CASE-009:** Sistem harus menyediakan append-only, hash-chained activity log; correction membuat superseding entry tanpa menghapus histori.
- **FR-CASE-010:** Case closure harus membuat seal yang dapat diverifikasi dan mendeteksi perubahan setelah penutupan.
- **FR-CASE-011:** Sistem harus mendukung legal hold, retention, archive, disposition, sanitization, dan verified deletion untuk derivative/cache sesuai policy.
- **FR-CASE-012:** Sistem harus memisahkan Examiner, Technical Reviewer, Administrative Reviewer, Report Authorizer, Administrator, dan Validation Lead.
- **FR-CASE-013:** Sistem harus mencegah pengguna menyetujui finding/report sendiri bila policy lab mensyaratkan separation of duties.
- **FR-CASE-014:** Sistem harus menyimpan competency/method authorization dan memblokir penggunaan method yang tidak diotorisasi pada official casework.
- **FR-CASE-015:** Sistem harus memisahkan `Legal Authority Record Status` (`Documented`, `Incomplete`, `Disputed`, `NotAssessed`) dari `Technical Authenticity/Integrity Status`; aplikasi tidak boleh memutuskan legalitas perolehan atau admissibility.
- **FR-CASE-016:** Sistem harus menyimpan versioned examination plan yang mencakup tujuan pemeriksaan, tasks, methods yang diotorisasi, assigned roles, dan tautan ke contemporaneous notes; perubahan plan membuat versi baru tanpa menghapus versi sebelumnya.

### 7.2 Evidence intake, integrity, dan chain of custody

- **FR-EVI-001:** Sistem harus membaca dan memverifikasi `.fsnap` tanpa mengubah package atau source evidence.
- **FR-EVI-002:** Verifikasi `.fsnap` harus mencakup manifest/schema version, source/output hashes, signature, audit continuity, path safety, acquisition coverage, gaps, dan declared limitations.
- **FR-EVI-003:** Sistem harus mendukung import format P0 yang tercantum pada capability matrix dan menolak klaim dukungan umum bila hanya sebagian fungsi/versi yang tervalidasi.
- **FR-EVI-004:** Setiap imported item harus memperoleh stable Evidence ID yang tidak bergantung pada filename.
- **FR-EVI-005:** Sistem harus membedakan `VerifiedMatch`, `VerifiedMismatch`, `ComputedUnanchored`, `SignatureInvalid`, `IncompleteInput`, dan `Unsupported`.
- **FR-EVI-006:** Ketiadaan manifest/hash pembanding tidak boleh menghasilkan status `VerifiedMatch`.
- **FR-EVI-007:** Sistem harus menyimpan received hash, locally computed hash, algorithm, time, operator, tool/build, dan verification result.
- **FR-EVI-008:** Sistem harus membedakan original, forensic copy, verified working copy, logical view, dan derivative object.
- **FR-EVI-009:** Physical device atau source path harus read-only secara default; exceptional write access memerlukan explicit authorization, warning, dan audit.
- **FR-EVI-010:** Sistem harus merekam custody transfer, seal condition, storage location, handler, waktu/zona waktu, purpose, dan acknowledgement.
- **FR-EVI-011:** Reopen, copy, backup/restore, export, dan handoff harus dapat memicu integrity verification sesuai policy.
- **FR-EVI-012:** Critical mismatch harus bersifat fail-closed untuk report final tetapi tetap memungkinkan examiner mendokumentasikan dan menyelidiki kondisi tersebut.

### 7.3 Processing orchestration dan coverage

- **FR-PROC-001:** Sistem harus membuat immutable run manifest berisi input hashes, method/tool/build, parser/rule/symbol/hash-set versions, configuration, locale, timezone database, hardware mode, operator, start/end time, warnings, errors, dan output hashes.
- **FR-PROC-002:** Sistem harus menyediakan stages dan dependency graph untuk verification, enumeration, extraction, parsing, indexing, correlation, dan reporting.
- **FR-PROC-003:** Sistem harus mendukung pause, resume, cancel, checkpoint, retry, reprioritize, dan resource budget.
- **FR-PROC-004:** Retry/resume harus idempotent dan tidak membuat duplicate derived objects.
- **FR-PROC-005:** Partial results yang sudah tervalidasi harus dapat ditinjau ketika processing lain masih berjalan.
- **FR-PROC-006:** Sistem harus menampilkan processed, failed, skipped, unsupported, encrypted, ambiguous, truncated, timeout, dan cancelled counts serta affected objects.
- **FR-PROC-007:** Sistem harus membedakan status `Complete`, `CompletedWithLimitations`, `Partial`, `Cancelled`, dan `Failed`.
- **FR-PROC-008:** Parser failure tidak boleh diterjemahkan sebagai “no artifact found”.
- **FR-PROC-009:** Sistem harus menyediakan Parser Failure Inbox dengan reason code, impact, retryability, affected evidence, log bundle, dan mitigation.
- **FR-PROC-010:** Sistem harus menyediakan `Re-run exactly` jika environment masih tersedia dan menjelaskan perbedaan bila exact rerun tidak mungkin.
- **FR-PROC-011:** Sistem harus menyediakan `Compare runs` untuk output, errors, warnings, coverage, versions, dan provenance.
- **FR-PROC-012:** Update parser/pack harus menampilkan impact analysis terhadap case lama dan menawarkan selective reprocessing.

### 7.4 Filesystem, carving, archive, dan recovery

- **FR-FS-001:** Sistem harus menampilkan partition, volume, filesystem, allocation state, metadata, ownership, permission, timestamp, link, compression, encryption, dan damage status sesuai capability.
- **FR-FS-002:** Sistem harus menyediakan allocated/deleted file view, recovery, carving, slack/unallocated examination, dan journal-derived results dengan provenance.
- **FR-FS-003:** Recovered/carved result harus diberi status `Complete`, `Partial`, `Ambiguous`, atau `PossiblyConflated` beserta dasar rekonstruksi.
- **FR-FS-004:** Sistem harus mempertahankan source offset/range, signature used, carving parameters, overlap, gap, dan output hash.
- **FR-FS-005:** Nested archives/containers harus diproses dengan configurable depth, size, file-count, recursion, timeout, dan decompression-ratio limits.
- **FR-FS-006:** Archive extraction harus menolak path traversal, symlink escape, device path, reserved-name abuse, archive bomb, dan overwrite collision.
- **FR-FS-007:** Source archive tidak boleh dimodifikasi; enkripsi hanya berlaku pada derivative export package.
- **FR-FS-008:** Filesystem feature yang unsupported atau hanya best-effort harus terlihat pada object, summary, dan report.

### 7.5 Search, index, hash sets, dan filtering

- **FR-SRCH-001:** Sistem harus menyediakan indexed dan ad-hoc search untuk nama, metadata, content, exact phrase, Boolean, wildcard, regex, fuzzy, byte/hex, dan multi-encoding text.
- **FR-SRCH-002:** Setiap search harus menyimpan query text/AST, version, scope, filter, encodings, container expansion, exclusions, errors, start/end time, dan result count.
- **FR-SRCH-003:** Search result harus menampilkan path/object, context, source range, decoder, evidence ID, dan match method.
- **FR-SRCH-004:** Sistem harus mengungkapkan kondisi yang dapat membuat search tidak lengkap, termasuk encryption, compression, unsupported format, malformed data, encoding, timeout, dan scope.
- **FR-SRCH-005:** Sistem harus mendukung saved query, query comparison, export, dan rerun.
- **FR-SRCH-006:** Known-good, known-bad/notable, dan organization-defined hash sets harus dipisahkan secara semantik.
- **FR-SRCH-007:** Hash-set import harus offline, versioned, hashed, signed bila tersedia, dan exact dataset version harus masuk run manifest.
- **FR-SRCH-008:** Known-good match tidak boleh disajikan sebagai bukti bahwa file aman atau tidak relevan; examiner harus dapat menampilkan kembali item yang disaring.

### 7.6 Artefak sistem operasi dan aplikasi

- **FR-ART-001:** Setiap artifact parser harus memiliki Method ID, supported source/version, extracted fields, timestamp semantics, validation status, limitation, dan manual verification guide.
- **FR-ART-002:** Sistem harus memisahkan raw value, decoded value, normalized value, dan examiner interpretation.
- **FR-ART-003:** Windows artefact pack P0 harus mencakup kategori pada scope dengan capability matrix per Windows/app version.
- **FR-ART-004:** macOS artefact pack P0 harus mencakup kategori pada scope dengan capability matrix per macOS/app version.
- **FR-ART-005:** Linux artefact pack P0 harus mencakup kategori pada scope dengan capability matrix per distribution/component version.
- **FR-ART-006:** Browser/email/messaging/document/database results harus dapat dicari, ditimeline-kan, dipivot, dibookmark, dan ditelusuri ke source.
- **FR-ART-007:** WAL/journal/recovery-derived database records harus dibedakan dari active records dan diberi uncertainty/overlap information.
- **FR-ART-008:** Encrypted application data hanya dapat diproses ketika authorized key/source tersedia; unsupported/key-missing state harus eksplisit.

### 7.7 Timeline, correlation, entity, dan graph

- **FR-COR-001:** Sistem harus mempertahankan raw timestamp, parsed value, normalized UTC, displayed local time, source timezone, clock offset, precision, ambiguity, dan conversion rule.
- **FR-COR-002:** WIB, WITA, WIT, UTC, unknown timezone, DST, clock drift, timestamp rollover, dan conflicting clock source harus ditangani eksplisit.
- **FR-COR-003:** Timeline gap hanya boleh ditampilkan sebagai unexplained gap dengan alternative causes; bukan verdict tampering.
- **FR-COR-004:** Graph node/edge harus mempunyai provenance, confidence basis, creation method, and reversible navigation ke supporting object.
- **FR-COR-005:** Entity normalization/merge harus menyimpan original values, rule/model, merge rationale, uncertainty, dan undo history.
- **FR-COR-006:** Pattern-of-life, anomaly, anti-forensics, YARA, Sigma, dan triage results harus berstatus indication sampai direview.
- **FR-COR-007:** High entropy, extension mismatch, missing logs, unusual time, atau threat-list match harus menampilkan alternative explanation dan tidak membuat finding otomatis.
- **FR-COR-008:** Cross-case matching tidak boleh aktif pada P0 dan hanya dapat diaktifkan pada P2 dengan explicit authority, data markings, retention, access approval, serta false-match review.

### 7.8 RAM dan network

- **FR-MEM-001:** Sistem harus mengidentifikasi memory image, OS/build hints, architecture, acquisition metadata, required symbols, dan limitation sebelum plugin dijalankan.
- **FR-MEM-002:** Symbol packs harus offline, versioned, hashed, origin-recorded, dan terikat pada run manifest.
- **FR-MEM-003:** Memory results harus mempertahankan layer/address/source reference, plugin/method, parameters, warnings, dan confidence/limitation.
- **FR-MEM-004:** Perbedaan list-based dan scan-based results harus dapat dibandingkan tanpa mengasumsikan salah satunya otomatis benar.
- **FR-NET-001:** Network analysis harus mempertahankan capture interface/source, timestamp precision, snap length, packet loss/drop/truncation indicators, dan packet/flow provenance.
- **FR-NET-002:** Extracted network object harus menyimpan packet/range reference, reassembly status, gap, decoder, output hash, dan limitation.
- **FR-NET-003:** TLS metadata tidak boleh disajikan sebagai decrypted content; encrypted/unknown protocol state harus eksplisit.
- **FR-NET-004:** Host-process-network correlation hanya boleh dibuat bila linking evidence tersedia dan harus menyimpan basis korelasi.

### 7.9 Universal file, document, image, video, dan audio foundation

- **FR-MEDIA-001:** Sistem harus menyediakan safe technical inspection untuk file umum tanpa mengeksekusi active content atau memanggil network resource.
- **FR-MEDIA-002:** File inspection harus menyimpan format identification basis, declared vs detected type, container structure, metadata, embedded objects, hash, source reference, dan parse errors.
- **FR-MEDIA-003:** Image P0 harus mendukung metadata/structure inspection, thumbnail comparison, color/profile data, dimensions, encoding, and safe rendering tanpa authenticity conclusion.
- **FR-MEDIA-004:** Video P0 harus mendukung container/stream/codec metadata, frame/keyframe extraction, timestamp/sequence observations, dan safe decoding berdasarkan allowlisted bundled decoder.
- **FR-MEDIA-005:** Audio P0 harus mendukung container/codec metadata, duration, channels, sample properties, waveform, spectrogram, dan discontinuity observations tanpa speaker/authenticity conclusion.
- **FR-MEDIA-006:** Document P0 harus mendukung metadata, revision/incremental structure, signature status, embedded object, hidden content, macro/script indicators, dan safe text extraction.
- **FR-MEDIA-007:** C2PA result harus membedakan manifest presence, signature/integrity validation, trust-chain status, assertion content, dan absence; valid manifest tidak boleh dianggap bukti bahwa content factually true.
- **FR-MEDIA-008:** Enhancement, authentication, comparison, PRNU, speaker comparison, deepfake detection, dan steganalysis harus menggunakan P2 pack dengan validation dossier terpisah.
- **FR-MEDIA-009:** P2 algorithmic probability harus menyertakan tested dataset, operating conditions, threshold, calibration, false-positive/negative observations, dan mandatory human review.

### 7.10 Live-host analysis

- **FR-LIVE-001:** Live-host workflow harus menjalankan preflight yang menjelaskan authority/scope, data volatility, privilege, expected footprint, active jobs, storage, encryption, security controls, dan perubahan yang mungkin ditimbulkan.
- **FR-LIVE-002:** Sistem harus menggunakan snapshot/capture melalui komponen atau contract yang kompatibel dengan Trareon Acquire bila data dapat diperoleh secara memadai melalui metode tersebut.
- **FR-LIVE-003:** Direct live query hanya boleh digunakan untuk state yang membutuhkan kondisi hidup dan harus menyimpan alasan pemilihan, requested data, start/end time, operator, privileges, serta limitation.
- **FR-LIVE-004:** Setiap helper/driver load, process start, file read, temp/cache write, registry/config access, privilege elevation, security-control interaction, dan network effect yang diketahui harus masuk footprint record.
- **FR-LIVE-005:** Live-host result harus dibedakan dari analysis of acquired evidence dan mempertahankan consistency/volatility class serta collection time window.
- **FR-LIVE-006:** Privileged helper harus mempunyai authenticated minimal command surface, explicit consent, scoped handles, timeout, audit, dan cleanup verification.
- **FR-LIVE-007:** Trareon Lab tidak boleh menghentikan proses, menghapus/mengubah file, melakukan quarantine, remediation, credential reset, atau security-control disable sebagai bagian analysis workflow.
- **FR-LIVE-008:** Jika kondisi target membuat perubahan tak dapat dihindari, sistem harus menjelaskan, mencatat, dan memasukkan dampaknya pada limitation/report; tidak boleh menjanjikan zero modification.

### 7.11 Local AI assistance

- **FR-AI-001:** Sistem harus mendukung Ollama pada loopback default dan LM Studio pada loopback default melalui provider abstraction yang sama.
- **FR-AI-002:** Sistem harus menolak non-loopback endpoint, redirect ke non-loopback, dan arbitrary provider pada official offline profile.
- **FR-AI-003:** Ollama/LM Studio harus opsional; absence, incompatible API, unavailable model, atau inference failure tidak boleh menghambat deterministic workflow.
- **FR-AI-004:** AI context broker harus menerapkan case scope, legal scope, data minimization, excerpt preview, token/resource budget, dan auditable source selection.
- **FR-AI-005:** Evidence content harus diperlakukan sebagai untrusted data; instruction di dalam evidence tidak boleh memperoleh tool authority.
- **FR-AI-006:** Tool calling harus disabled secara default; jika diaktifkan hanya read-only allowlisted deterministic functions yang tidak mempunyai shell, network, arbitrary path, write, delete, atau approval authority.
- **FR-AI-007:** AI output harus memisahkan fact, inference, uncertainty, alternative explanation, dan missing evidence.
- **FR-AI-008:** Setiap material claim harus mempunyai citation ke Evidence ID/Artifact ID/source reference atau diberi label unsupported.
- **FR-AI-009:** AI output harus disimpan sebagai derived suggestion dan tidak boleh otomatis menjadi tag final, finding, case status, atau conclusion.
- **FR-AI-010:** Sistem harus merekam provider/runtime version, model identifier/digest bila tersedia, prompt/template hash, context manifest, parameters, hardware mode, start/end, output, citations, dan reviewer disposition.
- **FR-AI-011:** Sistem harus memberi warning reproducibility ketika provider/model tidak memberikan determinism atau digest yang memadai.
- **FR-AI-012:** Sistem harus menyediakan AI off switch per installation, case, user, workflow, dan export.

### 7.12 Findings, review, reporting, dan export

- **FR-REP-001:** Sistem harus memisahkan extracted fact, observation, automated indication, hypothesis, examiner opinion, reviewed finding, dan approved conclusion sebagai tipe data berbeda.
- **FR-REP-002:** Finding harus mengikat setiap material statement ke supporting evidence, method, limitation, dan author.
- **FR-REP-003:** Finding kritis harus mempunyai disposition untuk conflicting/exculpatory evidence dan alternative hypothesis.
- **FR-REP-004:** Technical review harus mencatat scope reviewed, checks performed, disagreement, correction, resolution, reviewer, dan sign-off.
- **FR-REP-005:** Administrative review harus memeriksa case identity, authority/scope, template/version, required sections, page accountability, signatures, attachments, dan release destination.
- **FR-REP-006:** Report final harus mencantumkan items received, items examined/not examined, methods/tool/build/pack versions, scope, results, coverage, errors, limitations, deviations, opinions, supporting data, reviewer, authorizer, dan disposition.
- **FR-REP-007:** Amendment harus membuat report baru yang merujuk original, menjelaskan perubahan/alasan, mempertahankan original, dan membuat audit event.
- **FR-REP-008:** Redaction harus non-destructive terhadap evidence dan menyimpan redaction reason, author, reviewer, scope, serta export-specific rendering.
- **FR-REP-009:** Export package harus memiliki manifest, hashes, signatures bila tersedia, schema/version, selected scope, requester/destination, creation tool/build, audit excerpt, dan independent verification instructions.
- **FR-REP-010:** Sistem harus mendukung PDF/A, standalone HTML, JSON/CSV sesuai schema, dan version-pinned CASE/UCO profile tanpa mengklaim seluruh ontology didukung.
- **FR-REP-011:** DOCX dapat tersedia sebagai editable derivative tetapi tidak menjadi satu-satunya authoritative report artifact.
- **FR-REP-012:** UI harus menggunakan wording “records that may support legal review”, bukan `court-ready` atau `court-admissible`.

### 7.13 Validation Center, compliance traceability, dan documentation

- **FR-VAL-001:** Setiap capability harus memiliki stable Method ID, intended use, owner, format/version/OS scope, status, limitation, validation dossier, dan revalidation trigger.
- **FR-VAL-002:** Status capability harus `Validated`, `Limited`, `Experimental`, `Deprecated`, atau `NotSupported`; keberadaan tombol tidak membuktikan dukungan.
- **FR-VAL-003:** Validation dossier harus mencakup positive, negative, boundary, malformed, adversarial, partial, unsupported, repeatability, reproducibility, performance, dan differential tests bila feasible.
- **FR-VAL-004:** Validation data harus memiliki hashes, construction/ground-truth notes, license/handling constraints, expected output, actual output, raw logs, reviewer, discrepancy, dan release decision.
- **FR-VAL-005:** Perubahan core, parser/rule, dependency/compiler, OS, symbol, dataset, model, atau defect yang memengaruhi result harus memicu impact assessment dan revalidation sesuai risk.
- **FR-VAL-006:** One-click validation hanya boleh dinyatakan sebagai menjalankan installed validation suite dan menghasilkan evidence untuk human approval; bukan bukti universal.
- **FR-VAL-007:** Standards register harus menyimpan edition/status, verification date, owner, mapped controls, exceptions, dan review cadence.
- **FR-VAL-008:** Traceability matrix harus memetakan source → control objective → PRD requirement → component/owner → verification → evidence → acceptance → exception → revalidation trigger.
- **FR-VAL-009:** Sistem harus mendukung second-method atau cross-tool verification pada casework: merekam method A dan method B, input yang dibandingkan, output, discrepancy, reviewer disposition, dan residual risk; perbedaan hasil tidak boleh diselesaikan dengan majority-vote diam-diam.
- **FR-VAL-010:** Sistem harus mendukung import blind challenge package dan export signed participant result untuk laboratory proficiency exercise, termasuk participant isolation, embargoed expected results, scheme/round ID, provider identity, instructions, deadlines, dan submission lock; scheme design, assigned values, scoring, appeals, dan provider impartiality tetap di luar scope produk.
- **FR-VAL-011:** Validation sebelum casework harus mengikuti function-based tool testing sesuai register sumber `SRC-SWGDE-TEST` (SWGDE 18-Q-001): capability dan limitation method harus ditentukan dan direkam sebelum method tersebut dipakai pada official casework.
- **FR-REG-001:** Indonesia regulatory register harus menyimpan sumber resmi, status/berlaku, tanggal verifikasi, pasal/ruang lingkup yang dipetakan, product control, role responsibility, exception, owner, dan review trigger.
- **FR-REG-002:** Sistem harus menyimpan dokumen/record kewenangan, scope penggeledahan/penyitaan atau authority lain yang diberikan pengguna tanpa menyatakan dokumen tersebut sah atau cukup secara hukum.
- **FR-REG-003:** Sistem harus memisahkan technical authenticity/integrity, documented authority, disputed acquisition, dan court/legal disposition sebagai record yang berbeda.
- **FR-REG-004:** Deployment profile harus dapat merekam pihak yang ditetapkan organisasi sebagai pengendali/prosesor/authorized handler, tujuan/authority pemrosesan, data category, retention, restriction, disclosure/transfer, dan contact/approval record tanpa aplikasi menentukan status hukum pihak tersebut.
- **FR-REG-005:** Sistem harus menyediakan incident/disclosure register untuk affected case/data category, discovery time, containment, notification decision/deadline, authority/recipient, content sent, dan evidence of action; kewajiban serta deadline ditentukan policy/legal review organisasi.
- **FR-REG-006:** Export lintas organisasi atau yurisdiksi harus memerlukan disclosure preview, destination, purpose, authority/approval, selected data, redaction/minimization, transfer safeguards record, dan audit.
- **FR-DOC-001:** Documentation bundle P0 harus tersedia offline dalam Bahasa Indonesia dan Inggris serta dikunci ke build/analysis-pack version.
- **FR-DOC-002:** Capability tidak boleh berstatus `Validated` bila user guide, limitation, validation evidence, dan troubleshooting yang cocok dengan build tidak tersedia.
- **FR-DOC-003:** Setiap field/result penting harus mempunyai contextual help untuk semantics, source, parser/version, validation, limitation, error, dan manual verification.
- **FR-DOC-004:** Training Mode harus menggunakan synthetic/non-case evidence dan tidak boleh mengakses case production secara default.
- **FR-DOC-005:** Release notes harus menjelaskan perubahan result semantics, validation impact, migration, reprocessing need, known issue, dan rollback path.

---

## 8. Kebutuhan Desktop Khusus

### Perilaku jendela

- Dashboard utama menampilkan case list, recent activity, validation status, pack/update status, documentation, dan system health tanpa membuka raw case data.
- Setiap case dibuka pada isolated process/window sendiri.
- Main case window menggunakan tiga area: Case Navigator, Analysis Workspace, dan Provenance Inspector.
- Provenance Inspector dapat disembunyikan tetapi selalu tersedia melalui shortcut dan context action.
- Viewer, timeline, graph, report preview, atau media examination dapat dilepas menjadi secondary window dalam process case yang sama.
- Menutup window saat job aktif harus menawarkan `Keep Processing`, `Pause and Close`, atau `Cancel Safely` sesuai kemampuan platform; pilihan dicatat.
- Aplikasi harus menyimpan layout sebagai user preference tanpa menyimpan sensitive result di luar case workspace.

### Multi-window / single-window

- Dashboard dapat membuka banyak case window.
- Multi-case tabs dalam satu process tidak digunakan pada P0 untuk mencegah context/copy/export contamination.
- Cross-case clipboard internal, drag-and-drop, AI context, dan search harus diblokir pada P0.

### Mode penggunaan

- **Guided:** wizard, safe defaults, risk explanation, contextual documentation.
- **Standard:** workflow utama dengan filters, saved views, timeline, dan findings.
- **Expert:** raw query, parser configuration, resource budget, batch operation, comparison, dan advanced export.
- **Review:** evidence citations, findings, disagreements, limitation, dan sign-off.
- **Training:** synthetic cases, expected results, guided exercises, dan isolated training records.
- Mode hanya mengubah progressive disclosure; seluruh mode menggunakan deterministic core yang sama.

### Offline support

- Seluruh P0 harus berfungsi tanpa koneksi internet, online account, activation server, telemetry, CDN, remote documentation, cloud licensing, atau external forensic application.
- Installer harus membawa seluruh runtime/library inti yang diperlukan atau menggunakan komponen OS yang tercantum pada supported-platform matrix.
- Analysis pack, rule, symbol, signature, hash set, documentation, validation data, schema, license data, dan update harus dapat dipasang melalui signed offline bundle.
- Satu-satunya koneksi AI yang diizinkan official offline profile adalah loopback ke Ollama/LM Studio.
- Connected companion tidak boleh berada dalam process atau permission profile yang sama dengan offline core.

### Penyimpanan lokal

- Case workspace menggunakan database transactional lokal, content-addressed derived store, persistent index, append-only audit, dan encrypted secrets vault.
- Evidence source tidak disalin otomatis kecuali examiner memilih verified working copy dan destination telah dikualifikasi.
- Temporary file harus berada pada case-scoped directory, mempunyai lifecycle policy, dan disanitasi secara terverifikasi.
- Workspace harus mendukung storage forecasting, low-space threshold, migration ke volume lain, backup/restore, legal hold, dan archive.
- Encryption-at-rest harus mendukung key recovery policy organisasi tanpa menyimpan key di dalam case package yang sama.

### Update aplikasi

- Tidak ada forced online update.
- Installer/update harus signed, versioned, hashed, mempunyai SBOM/provenance, compatibility manifest, rollback path, dan offline verification instructions.
- Update core/pack harus menampilkan impacted methods/cases dan revalidation/reprocessing implications sebelum diterapkan.
- Case harus dapat mempertahankan/pin versi pack lama yang dibutuhkan untuk rerun, dengan security warning bila versi tersebut mempunyai known vulnerability.

### Akses file sistem

- Frontend dan pack tidak menerima arbitrary path access.
- Seluruh akses melalui case-scoped broker dan opaque handles.
- File picker hanya memilih calon source/destination; core melakukan canonicalization, identity, scope, permission, capacity, collision, dan safety checks.
- Evidence source harus read-only; output hanya ditulis ke approved destination.
- Symlink, junction, mount change, removable-media disconnect, path traversal, network share instability, dan case-insensitive collision harus ditangani fail-closed.

### Notifikasi desktop

- Notifikasi bersifat lokal dan tidak memuat nama kasus, PII, credential, atau finding detail pada lock screen.
- Notifikasi boleh digunakan untuk job completion/limitation, approval request, low disk, integrity warning, update/validation expiry, dan backup failure.
- Notifikasi tidak boleh menggantikan audit event atau in-app error disposition.

### Shortcut keyboard dan command palette

- Semua tindakan umum mempunyai shortcut yang terdokumentasi dan dapat diubah tanpa menyebabkan shortcut konflik diam-diam.
- Minimum: global search, command palette, open provenance, bookmark, tag, add note, create hypothesis, create finding, next/previous result, raw view, copy cited value, pause job, dan help.
- Destructive atau scope-changing command tidak boleh dapat dieksekusi melalui satu shortcut tanpa confirmation/authorization.
- Command palette harus menerapkan role, scope, case state, dan capability restrictions yang sama dengan UI.

### Responsivitas lintas resolusi

- UI harus berfungsi pada minimum effective viewport 1280×720 dan dioptimalkan untuk 1920×1080, 2560×1440, ultrawide, HiDPI/Retina, serta multi-monitor.
- Dense tables harus memakai virtual scrolling, resizable/pinnable columns, saved column sets, keyboard navigation, dan accessible focus order.
- Light/dark theme, text scaling, high contrast, reduced motion, dan color-independent status indicators harus tersedia.

### Instalasi dan privilege

- Produk P0 adalah install-first.
- Main UI harus berjalan tanpa elevation untuk operasi normal.
- Privileged helper hanya digunakan untuk live-host functions yang memerlukannya, mempunyai command surface minimal, authenticated IPC, explicit consent, audit, dan lifecycle terbatas.
- Installer/uninstaller tidak boleh menghapus case data tanpa pilihan eksplisit dan verification record.
- Portable edition ditempatkan pada P2 dan tidak boleh mengurangi kontrol integrity, audit, atau validation hanya untuk mencapai portability.

---

## 9. Kebutuhan Non-Fungsional

### Performa dan skalabilitas

- **NFR-PERF-001:** Processing harus streaming, bounded-memory, resource-aware, parallel, cancellable, resumable, dan checkpointed.
- **NFR-PERF-002:** Evidence size tidak boleh dibatasi oleh RAM; batas praktis harus ditentukan oleh storage, filesystem, supported format, dan published validation profile.
- **NFR-PERF-003:** Architecture harus mampu menangani multi-terabyte evidence dan puluhan juta indexed objects; klaim angka hanya boleh diterbitkan bersama reference hardware, dataset, configuration, dan observed limitations.
- **NFR-PERF-004:** UI input, navigation, selection, scrolling, dan provenance view harus tetap responsif selama background processing.
- **NFR-PERF-005:** Hasil parsial pertama harus tersedia setelah stage terkait menghasilkan output tervalidasi, tanpa menunggu seluruh ingest.
- **NFR-PERF-006:** Search indexed harus mendukung pagination/streaming dan tidak memuat seluruh result set ke memory.
- **NFR-PERF-007:** Processing Center harus menampilkan throughput, CPU/RAM/GPU/disk, queue, current stage, cache, errors, warnings, dan estimasi yang diberi label bila confidence rendah.
- **NFR-PERF-008:** Performance regression terhadap published baseline harus memblokir release bila dapat menyebabkan omission, timeout, nondeterminism, atau usability failure.

### Keamanan

- **NFR-SEC-001:** Evidence harus diperlakukan sebagai hostile input.
- **NFR-SEC-002:** Parser/decoder/pack harus diisolasi dengan least privilege, resource limits, no-network default, no arbitrary path, dan crash containment.
- **NFR-SEC-003:** Active content, macro, script, embedded URL, external font/resource, autorun, dan media callback tidak boleh dieksekusi.
- **NFR-SEC-004:** Installers, core, helpers, packs, rules, symbols, updates, dan official documentation harus signed dan diverifikasi sebelum digunakan.
- **NFR-SEC-005:** Secrets vault harus melindungi recovered credentials/keys, AI tokens bila ada, signing keys, dan encryption keys; reveal/copy/export harus diaudit dan role-controlled.
- **NFR-SEC-006:** Case data harus encrypted at rest sesuai configuration organisasi dan backup package harus encrypted serta signed.
- **NFR-SEC-007:** Produk tidak boleh memiliki telemetry, analytics, crash upload, atau remote feature flag tersembunyi.
- **NFR-SEC-008:** Release harus memiliki SBOM, dependency/license policy, vulnerability review, SAST, secret scan, provenance/attestation, dan platform signing status.
- **NFR-SEC-009:** Security finding pada parser/pack harus dapat menonaktifkan capability tanpa merusak kemampuan membaca case lama; affected-case register harus tersedia.
- **NFR-SEC-010:** Security profile harus memisahkan offline core, loopback AI, local lab worker P2, dan connected companion.

### Reliabilitas dan data integrity

- **NFR-REL-001:** UI tidak boleh menjadi source of truth untuk completion; status berasal dari core state machine dan verification rules.
- **NFR-REL-002:** Forced termination, power loss, disk full, removable-media disconnect, parser crash, dan OS restart tidak boleh menghasilkan complete status palsu.
- **NFR-REL-003:** Checkpoint harus transactional dan diverifikasi sebelum resume.
- **NFR-REL-004:** Audit event yang telah committed tidak boleh hilang ketika process crash.
- **NFR-REL-005:** Backup harus mempunyai restore verification; backup tanpa successful restore test harus berstatus `UnverifiedBackup`.
- **NFR-REL-006:** Migration harus reversible atau menyediakan verified pre-migration backup serta recovery instructions.
- **NFR-REL-007:** Deterministic methods harus menghasilkan equivalent normalized output pada supported platforms untuk input/config/version yang sama, kecuali documented platform-specific limitation.
- **NFR-REL-008:** Parser failure harus terisolasi sehingga evidence lain tetap dapat diproses dan failure coverage tetap utuh.

### Validitas, repeatability, dan reproducibility

- **NFR-VAL-001:** Setiap P0 method harus mempunyai approved validation dossier sebelum status `Validated`.
- **NFR-VAL-002:** Repeatability harus diuji pada environment sama dan reproducibility pada supported OS/architecture matrix.
- **NFR-VAL-003:** Test harus mencakup positive, negative, boundary, partial, corrupt, malformed, adversarial, unsupported, timeout, dan resource-exhaustion cases.
- **NFR-VAL-004:** Ground truth, expected/actual output, raw logs, discrepancies, reviewer, dan release decision harus dipertahankan.
- **NFR-VAL-005:** Tidak ada deadline atau product milestone yang dapat mengubah failed validation menjadi passed secara diam-diam.

### Aksesibilitas dan usability

- **NFR-UX-001:** Desain menggunakan progressive disclosure: safe clean default surface dengan advanced panels/menus/tables/shortcuts tetap tersedia.
- **NFR-UX-002:** Status tidak boleh bergantung pada warna saja; icon, text, severity, dan action harus tersedia.
- **NFR-UX-003:** Keyboard-only workflow, screen-reader semantics, focus order, text scaling, high contrast, reduced motion, dan accessible report preview harus diuji.
- **NFR-UX-004:** Warning harus menjelaskan dampak forensik dan pilihan aman, bukan hanya error code.
- **NFR-UX-005:** Bahasa Indonesia dan Inggris harus menggunakan glossary terkendali agar istilah evidence, observation, finding, limitation, dan opinion tidak tertukar.

### Kompatibilitas OS dan architecture

- **NFR-OS-001:** Official release harus mempunyai supported-platform matrix yang menyebut exact OS version/build, architecture, security state, filesystem, installer/signing status, dan capability differences.
- **NFR-OS-002:** P0 harus menyediakan official artifact untuk Windows x86_64, macOS ARM64, dan Linux x86_64; platform/architecture tambahan dapat dirilis hanya setelah gate yang sama lulus.
- **NFR-OS-003:** Dukungan Linux harus menyebut exact distribution/version dan package format; label “Linux supported” tanpa matrix tidak diperbolehkan.
- **NFR-OS-004:** Perbedaan filesystem, path, locale, timezone, permission, sandbox, code signing, GPU, dan codec antar-OS harus diuji dan didokumentasikan.
- **NFR-OS-005:** Core domain, evidence, provenance, run manifest, audit, dan report schema harus platform-neutral; OS-specific code berada pada adapter yang terisolasi.

### Maintainability dan supply chain

- **NFR-MNT-001:** Core, platform adapters, parsers, pack runtime, AI broker, UI, verifier, dan documentation harus memiliki contract yang terpisah dan versioned.
- **NFR-MNT-002:** External library dapat digunakan bila dibundel secara legal, diaudit, dipin, diuji, dan tidak menjadi prerequisite manual; fungsi P0 tidak boleh mengandalkan executable pihak ketiga yang harus diinstal pengguna.
- **NFR-MNT-003:** Reimplementation dependency hanya dilakukan bila lisensi, security, validation, portability, maintenance, atau offline requirement membuat bundling/adoption tidak dapat diterima.
- **NFR-MNT-004:** Pack SDK harus menggunakan permission manifest, schema, resource limit, deterministic output contract, SBOM, signature, validation status, dan rollback.

### Privasi dan regulatory support

- **NFR-PRIV-001:** Data minimization, purpose limitation, access control, disclosure log, retention, legal hold, redaction, dan secure disposition harus dapat dikonfigurasi per case/policy.
- **NFR-PRIV-002:** Raw evidence tidak boleh masuk AI context, report, export, screenshot, diagnostic bundle, atau training set tanpa explicit scoped selection.
- **NFR-PRIV-003:** Regulatory profile harus versioned dan memisahkan product controls dari tanggung jawab examiner, lab, investigator, legal counsel, regulator, serta pengadilan.
- **NFR-PRIV-004:** Klaim hukum dan standar harus ditinjau sebelum major release serta ketika source register berubah.

---

## 10. Edge Case

### Evidence dan format

- `.fsnap` valid, signature tidak tersedia, signature invalid, audit discontinuity, manifest schema unknown, file hilang, file ekstra, unsafe path, atau acquisition coverage parsial.
- Dua evidence item mempunyai filename sama tetapi Evidence ID/path/range berbeda.
- Split image kehilangan segmen, urutan segmen salah, ukuran terakhir tidak biasa, atau hash satu segmen mismatch.
- Evidence lebih besar/kecil dari ukuran yang dideklarasikan, sparse file berubah saat copy, atau storage melaporkan hidden area.
- Partition table rusak, filesystem bertumpuk, volume encrypted, RAID member tidak lengkap, snapshot inconsistency, atau sector size tidak umum.
- Symlink/junction/hardlink cycle, case-sensitive collision, illegal/reserved filename, alternate stream, xattr/resource fork, sparse/compressed data, dan permission denied.
- Archive recursive, encrypted, password wrong, archive bomb, overlapping entries, path traversal, symlink escape, malformed header, atau unsupported compression.
- Carved object overlap, missing footer, false signature, fragment dari objek berbeda, dan partially overwritten data.

### Search, parsing, dan timestamp

- Keyword berada pada unsupported encoding, compressed/encrypted content, OCR-only image, fragmented data, binary field, deleted journal, atau truncated container.
- Regex catastrophic backtracking, query terlalu luas, result count sangat besar, atau search cancelled lalu dilanjutkan.
- Parser menghasilkan sebagian field lalu crash; output parsial tidak boleh terlihat complete.
- OS/app version baru mengubah schema atau timestamp semantics.
- Timestamp mempunyai unknown timezone, DST ambiguity, invalid date, low precision, clock drift, rollback, rollover, atau conflicting source.
- Timeline tidak mempunyai event pada periode tertentu; sistem tidak boleh otomatis menandai tampering.

### Processing dan storage

- Disk penuh saat index/extraction/report/export/backup.
- Evidence/removable destination terputus lalu muncul kembali dengan device identity berbeda.
- Aplikasi/OS mati setelah output ditulis tetapi sebelum audit commit, atau sebaliknya.
- Parser hang, memory leak, fork bomb behavior, GPU out-of-memory, corrupt cache, atau stale checkpoint.
- Pack update dilakukan ketika job sedang berjalan.
- Case lama membutuhkan pack yang dinonaktifkan karena vulnerability.
- Backup selesai tetapi restore verification gagal.

### Security dan hostile content

- Malicious PDF/Office/SVG/HTML/media/font/archive mencoba menjalankan code, membuka URL, membaca path, atau menghabiskan resource.
- Evidence berisi prompt injection yang meminta AI mengabaikan scope, mengirim data, atau menjalankan tool.
- Ollama/LM Studio endpoint redirect/proxy ke host non-loopback.
- Signed pack certificate expired/revoked, signature valid tetapi publisher tidak dipercaya, atau bundle rollback menyerang versi lama.
- User menyalin credential/PII ke clipboard OS atau report tanpa otorisasi.
- Diagnostic bundle berpotensi memuat raw evidence atau secret.

### AI

- Provider tidak terpasang, server mati, API incompatible, model hilang, context overflow, output malformed, inference timeout, GPU failure, atau model berubah dengan nama sama.
- AI citation menunjuk object yang tidak ada, berada di luar scope, atau tidak mendukung klaim.
- Dua run model sama menghasilkan jawaban berbeda.
- AI mencampur case context atau mengubah hypothesis menjadi statement faktual.
- User mematikan AI setelah suggestions tersimpan; deterministic workflow dan review history harus tetap berfungsi.

### Review, reporting, dan lifecycle

- Examiner dan reviewer adalah orang yang sama tetapi policy membutuhkan separation.
- Finding mempunyai supporting evidence tetapi conflicting/exculpatory evidence belum didisposisi.
- Evidence integrity mismatch ditemukan setelah report dirilis.
- Amendment mengubah conclusion, redaction, atau exhibit sementara original harus tetap tersedia.
- Report dipindahkan ke OS/locale lain dan font/layout berubah.
- Case ditutup ketika job, approval, backup, atau retention action belum selesai.
- Retention expiry terjadi ketika legal hold aktif.
- Sanitization interrupted atau target directory berisi data dari case lain.

---

## 11. Acceptance Criteria

### Product boundary dan deployment

- [ ] Installer bertanda tangan tersedia untuk official Windows, macOS, dan Linux platform matrix.
- [ ] Instalasi baru dapat menjalankan seluruh P0 deterministic workflow tanpa internet atau aplikasi forensik eksternal.
- [ ] Produk tetap berfungsi penuh ketika Ollama/LM Studio tidak tersedia.
- [ ] Tidak ada telemetry, activation call, remote configuration, atau hidden network request pada offline profile.
- [ ] Portable edition tidak dipresentasikan sebagai requirement P0.

### Case, scope, dan isolation

- [ ] Case tidak dapat masuk Intake sebelum authority, purpose, scope, handling restriction, dan acceptance decision direkam.
- [ ] Versioned examination plan tersimpan sebelum/during examination dan perubahan plan membuat versi baru tanpa menghapus versi sebelumnya.
- [ ] Scope policy terbukti membatasi processing, preview, search, AI context, export, dan report.
- [ ] Dua case yang dibuka bersamaan mempunyai process, cache, index, temp, AI context, audit, dan clipboard internal yang terisolasi.
- [ ] Cross-case transfer hanya berhasil melalui workflow export/import yang diaudit.
- [ ] Role dan separation-of-duties policy diterapkan pada finding/report approval.
- [ ] Legal-authority record status dapat ditandai incomplete/disputed tanpa mengubah technical hash/integrity result, dan sebaliknya.
- [ ] Case seal mendeteksi perubahan setelah closure.

### Evidence integrity dan coverage

- [ ] Valid `.fsnap` diterima dan fixture mutated, missing-file, unsafe-path, audit-discontinuous, truncated, atau unsupported-version ditolak/dibatasi dengan reason tepat.
- [ ] Input tanpa anchor menghasilkan `ComputedUnanchored`, bukan `VerifiedMatch`.
- [ ] Hash/signature mismatch kritis memblokir report final.
- [ ] Original, forensic copy, working copy, logical view, dan derivative mempunyai relationship serta identifier berbeda.
- [ ] Setiap P0 result mempunyai machine-verifiable path ke input, action/method, tool/build/configuration, operator, dan time.
- [ ] Coverage dashboard dan report menunjukkan processed, failed, skipped, unsupported, encrypted, ambiguous, truncated, dan timeout items.
- [ ] Parser crash tidak menghasilkan “no result” atau `Complete` palsu.

### Processing, recovery, dan performa

- [ ] Job dapat dipause, dilanjutkan, dibatalkan, dan direstart dari verified checkpoint tanpa duplicate result.
- [ ] Forced termination, disk-full, parser hang/crash, dan removable-media disconnect mempunyai deterministic recovery test.
- [ ] Examiner dapat meninjau partial results ketika background processing masih berjalan.
- [ ] UI tetap responsif pada published Small, Medium, Large, dan Stress benchmark profiles.
- [ ] Search dan table results memakai pagination/virtualization dan tidak memuat seluruh dataset ke memory.
- [ ] Performance claim menyebut exact dataset, reference hardware, OS, configuration, dan limitations.

### Filesystem, search, artefak, RAM, dan network

- [ ] Capability matrix menyatakan fungsi/format/version/OS yang `Validated`, `Limited`, `Experimental`, `Deprecated`, atau `NotSupported`.
- [ ] Recovered/carved object menampilkan completeness/ambiguity, source range, method, parameters, dan output hash.
- [ ] Search manifest menyimpan query, scope, encodings, exclusions, errors, container coverage, dan version.
- [ ] Known-good filtering dapat dibalik dan tidak menyatakan file otomatis aman.
- [ ] Raw, decoded, normalized, dan interpreted artifact values dapat dibedakan.
- [ ] Memory result mempertahankan layer/address/source reference dan exact symbol/plugin version.
- [ ] Network extracted object mempertahankan packet/range reference, reassembly/gap status, decoder, dan hash.
- [ ] Timeline mempertahankan raw timestamp, timezone, precision, clock offset, conversion, dan ambiguity.

### Universal file/media dan hostile input

- [ ] Preview P0 tidak mengeksekusi macro, script, embedded URL, external resource, autorun, atau arbitrary codec/plugin.
- [ ] Malicious archive, document, image, audio, dan video corpora tidak dapat keluar dari parser sandbox atau mengakses network/arbitrary path.
- [ ] C2PA UI membedakan provenance-manifest integrity dari truth/authenticity conclusion.
- [ ] Image/video/audio/document P0 hanya membuat technical observations; authenticity/source/identity verdict tidak tersedia tanpa P2 validated pack.
- [ ] P2 probability output selalu menyertakan validation conditions dan mandatory human review.

### Live-host analysis

- [ ] Live-host workflow tidak dapat dimulai sebelum authority/scope dan footprint preflight dikonfirmasi.
- [ ] Snapshot-first path melalui contract kompatibel Trareon Acquire digunakan ketika data dapat diperoleh tanpa direct query.
- [ ] Direct query fixture mencatat reason, state window, privileges, helper/process/file/temp/config/network footprint, dan limitations.
- [ ] Live-host results dapat dibedakan dari acquired-evidence results pada UI, provenance, timeline, review, dan report.
- [ ] Privileged helper command-surface, authentication, timeout, cleanup, dan failure paths lulus security review.
- [ ] Tidak ada remediation, quarantine, process termination, deletion, credential reset, atau security-control disable pada official Lab profile.

### AI lokal

- [ ] Ollama dan LM Studio dapat dideteksi/dikonfigurasi pada loopback dan non-loopback endpoint ditolak pada official offline profile.
- [ ] Evidence prompt injection tidak dapat memperoleh shell, network, arbitrary file, write, delete, approval, atau tool authority.
- [ ] AI hanya menerima scoped normalized facts dan examiner-approved excerpts yang tercantum pada context manifest.
- [ ] Material AI claim tanpa valid evidence citation diberi label unsupported.
- [ ] AI output tidak dapat menjadi final tag, finding, case status, atau conclusion tanpa examiner action.
- [ ] Provider/runtime/model/prompt/context/parameter/output/reviewer metadata tersimpan.
- [ ] Menonaktifkan AI tidak merusak deterministic results, findings, audit, atau report.

### Findings, review, reporting, dan export

- [ ] Fact, observation, indication, hypothesis, opinion, finding, dan conclusion tersimpan sebagai tipe berbeda.
- [ ] Finding kritis tidak dapat disetujui sebelum supporting dan conflicting/exculpatory evidence didisposisi.
- [ ] Technical review, administrative review, disagreement, correction, resolution, dan authorization tersimpan.
- [ ] Report final memuat scope, items, methods/versions, results, coverage, errors, limitations, deviations, opinion basis, reviewers, authorizer, dan disposition.
- [ ] Amendment menghasilkan report baru, merujuk original, menjelaskan perubahan, dan mempertahankan original.
- [ ] Export package dapat diverifikasi menggunakan manifest/instructions tanpa membuka aplikasi utama.
- [ ] Redaction bersifat derivative, audited, dan tidak mengubah evidence asli.

### Validation, standards, regulasi, dan dokumentasi

- [ ] Tidak ada P0 method berstatus `Validated` tanpa approved validation dossier dan preserved test evidence.
- [ ] Method yang dipakai pada official casework mempunyai recorded capability/limitation dari function-based tool testing sebelum casework.
- [ ] Second-method atau cross-tool verification merekam kedua method, discrepancy, reviewer disposition, dan residual risk tanpa majority-vote diam-diam.
- [ ] Blind challenge import dan signed participant-result export mempertahankan isolation, embargoed expected results, scheme/round ID, dan submission lock tanpa mengklaim PT-provider conformity.
- [ ] Repeatability dan cross-platform reproducibility tests tersedia untuk official platform matrix.
- [ ] Revalidation impact assessment berjalan ketika core/parser/rule/dependency/compiler/OS/symbol/model/ground-truth berubah.
- [ ] Standards register mencantumkan exact edition/status, verification date, mapped controls, exceptions, dan owner.
- [ ] Wording produk tidak mengklaim sertifikasi, akreditasi, admissibility, error-free result, atau universal compliance.
- [ ] Indonesia regulatory profile telah ditinjau oleh qualified Indonesian legal/quality personnel sebelum Official Production.
- [ ] Seluruh documentation bundle P0 tersedia offline dalam Bahasa Indonesia dan Inggris serta cocok dengan build.
- [ ] Capability tidak dapat dinyatakan `Validated` bila guide, limitation, validation evidence, atau troubleshooting belum tersedia.
- [ ] Contextual help dapat membuka semantics, source, parser/version, validation, limitation, error, dan manual verification untuk result terkait.

---

## 12. Metrik Keberhasilan

### Validitas dan transparansi

- **0 false-complete** pada seluruh validation corpus untuk kondisi missing input, parser crash, timeout, disk full, truncation, dan unsupported structure.
- **100% P0 material results** mempunyai provenance path yang lulus verifier.
- **100% critical processing errors** muncul pada dashboard, run manifest, dan report limitation/disposition.
- **100% `Validated` capabilities** mempunyai approved dossier, compatible documentation, owner, dan revalidation trigger.
- **100% final reports** mempunyai technical review, administrative review, dan authorizer sesuai policy.

### Reproducibility dan reliability

- Equivalent normalized deterministic output pada official OS matrix untuk golden datasets, kecuali documented platform-specific limitation.
- Tidak ada audit event committed yang hilang pada crash/power-failure test.
- Checkpoint/resume tidak membuat duplicate/missing output pada failure-injection corpus.
- Backup/restore verification pass rate 100% untuk official backup profiles sebelum status backup dianggap verified.

### Produktivitas

- Partial review dapat dimulai sebelum full ingest selesai pada seluruh benchmark profiles yang mendukung streaming output.
- Waktu examiner untuk menemukan source reference dari result penting maksimal dua direct UI actions dari result view.
- Search, filter, sort, dan pivot tetap usable pada published Large profile tanpa full-result memory load.
- Processing ulang setelah pack update hanya menyentuh affected stages/objects bila dependency graph memungkinkan.

### AI safety dan usefulness

- 100% AI responses mempunyai context manifest dan provenance metadata.
- 100% material AI claims mempunyai valid evidence citation atau label unsupported.
- 0 autonomous finding/report approval atau unauthorized tool/network/filesystem operation pada adversarial test suite.
- Acceptance/rejection/edit rate dicatat untuk mengukur usefulness tanpa menjadikan acceptance sebagai bukti kebenaran.

### Dokumentasi dan usability

- 100% P0 capability mempunyai contextual help, user workflow, limitation, dan troubleshooting.
- Guided synthetic-case completion dapat dilakukan oleh target pengguna baru tanpa menggunakan dokumentasi online.
- Accessibility gate lulus pada keyboard-only, screen reader, text scaling, contrast, focus, dan reduced-motion scenarios yang dipublikasikan.
- Bahasa Indonesia dan Inggris mempunyai glossary consistency review pada setiap major release.

### Security dan privacy

- 0 network egress dari official offline profile selain explicitly enabled loopback AI.
- 0 parser sandbox escape, arbitrary path access, active-content execution, atau case-context crossover pada release security suite.
- 100% release artifacts dan official offline bundles mempunyai verified signature, hashes, provenance, dan SBOM.
- 100% sensitive reveal/export actions mempunyai user, case, purpose, time, object, dan result audit record.

---

## 13. Risiko

| Risiko | Dampak | Mitigasi produk | Residual decision owner |
|---|---|---|---|
| Scope P0 terlalu luas | Release terlambat dan parser dangkal | Vertical slices, per-capability status, no date-based promotion, feature freeze per pack | Product + Validation Lead |
| False completeness | Kesimpulan salah karena data terlewat | Coverage map, typed status, failure inbox, critical report gate | Core + Validation Lead |
| Parser salah/usang | Artefak dimaknai salah | Version matrix, golden corpora, differential tests, signed updates, manual verification | Pack Owner |
| Evidence hostile | Workstation compromise atau data corruption | Sandbox, least privilege, no active content, fuzzing, resource limits | Security Lead |
| Evidence multi-terabyte | Backlog, disk exhaustion, unusable UI | Streaming, persistent index, partial results, forecast, checkpoints, benchmark profiles | Performance Owner |
| Cross-platform divergence | Hasil berbeda antar-OS | Platform-neutral schemas, OS adapters, cross-platform reproducibility tests | Platform Owners |
| AI hallucination/prompt injection | Suggestion dianggap fakta atau data bocor | Context broker, citations, untrusted evidence model, no tools/network/write, human gate | AI Safety Owner |
| Ollama/LM Studio/model drift | Reproducibility lemah | Record runtime/model/config, digest when available, compatibility checks, warning | AI Pack Owner |
| Third-party dependency vulnerability | Supply-chain compromise | Pinning, SBOM, audit, sandbox, signature, rollback, reimplementation threshold | Security + Architecture |
| Proprietary format/codec licensing | Capability tidak dapat didistribusikan | License review, documented subset, safe fallback, separate pack, no unsupported claim | Legal + Pack Owner |
| Privacy/over-scope processing | Pelanggaran hak, policy, atau hukum | Enforced scope, minimization, access control, audited reveal/export, retention | Lab + Privacy Owner |
| Indonesia regulatory change | Workflow/wording menjadi usang | Versioned legal register, official-source watch, qualified review before release | Legal/Compliance Owner |
| ISO edition under review berubah | Mapping dan claim tidak akurat | Standards register, quarterly check untuk under-review items, impact assessment | Quality Owner |
| Report disalahartikan sebagai legal verdict | Risiko hukum/reputasi | Controlled wording, limitation, opinion labeling, review/authorization | Legal + Report Authorizer |
| Cross-case contamination | Data salah kasus atau unauthorized disclosure | One case/process, isolated stores, signed transfer, no cross-case clipboard | Security + Case Owner |
| Encryption key hilang | Case workspace tidak dapat dipulihkan | Organization key policy, tested recovery, separation from case package | Administrator |
| Validation data tidak representatif | False confidence | Diverse ground truth, adversarial/realistic corpus, external review, discrepancy register | Validation Lead |
| P2 multimedia/deepfake overclaim | False authenticity/identity conclusion | Separate pack/dossier, calibrated output, limitations, domain-expert review | Specialist Pack Owner |
| Connected companion membocorkan evidence | Confidentiality breach | Separate product/process/profile, explicit disclosure preview and approval, network audit | Companion Owner |

---

## 14. Open Questions

Open questions berikut tidak mengubah requirement produk yang telah disetujui; jawabannya harus ditentukan pada RFC atau validation plan sebelum implementation track terkait dimulai.

1. Desktop framework dan rendering stack mana yang paling sesuai dengan installer-first, sandbox, accessibility, GPU, and cross-platform support?
2. Case database/index combination mana yang memenuhi transactional recovery, multi-terabyte scale, portable schemas, dan reproducibility?
3. Cryptographic profile apa yang digunakan untuk case seal, audit chain, pack/update signature, export signature, key rotation, dan long-term verification?
4. Reference hardware profiles apa yang mewakili minimum workstation, recommended workstation, dan high-performance lab workstation?
5. Exact first-release OS/distribution/build matrix apa yang realistis untuk hardware validation dan signing?
6. File/media decoders mana yang dapat dibundel secara legal dan aman; mana yang harus menjadi isolated optional pack?
7. Seberapa jauh P0 memory engine diimplementasikan native dibanding mengadopsi/bundling compatible components dengan license serta validation yang dapat diterima?
8. Apakah role separation P0 menggunakan local OS identities, application-local identities, hardware tokens, atau kombinasi berdasarkan deployment profile?
9. Bagaimana encrypted workspace key recovery dibagi antara single-examiner lab dan multi-person organization?
10. CASE/UCO profile subset/version mana yang menjadi first official interchange contract?
11. Qualified Indonesian legal/quality reviewers mana yang menyetujui regulatory mapping dan controlled wording sebelum Official Production?
12. Apa release boundary P0 pertama: minimum set format/artefak yang dapat divalidasi mendalam tanpa menunda defensible core?

---

## Lampiran A — Pemetaan Feature Inventory ke Prioritas

| No. | Kelompok fitur riset | Keputusan | Batas penting |
|---:|---|---|---|
| 1 | Case Workspace | P0 | Satu case per isolated process/window; dashboard dapat membuka banyak window |
| 2 | File System Browser | P0 | Status per filesystem/function/version; read-only; unsupported terlihat |
| 3 | File Preview | P0 | Sandboxed safe preview; tidak mengeksekusi active content |
| 4 | Archive Analysis | P0 | Inspect/extract dengan bomb/path protections; AES hanya untuk derivative export |
| 5 | File Carving | P0 | Completeness/ambiguity dan source-range provenance wajib |
| 6 | Keyword Search | P0 | Indexed/ad-hoc; search coverage, encoding, exclusion, dan error manifest |
| 7 | Timeline Analysis | P0 | Raw/normalized time, timezone, precision, ambiguity; gap bukan verdict tampering |
| 8 | Bookmarks & Tags | P0 | Tag examiner dipisahkan dari automated indication dan reviewed finding |
| 9 | Report Generation | P0 | PDF/A, HTML, machine-readable; DOCX derivative; tidak memakai klaim court-ready |
| 10 | Audit Trail | P0 | Append-only, hash-chained, correction via superseding event |
| 11 | Browser Forensics | P0 | Capability matrix per browser/version; credentials protected |
| 12 | Social Media & Messaging | Split | Desktop/local stored artifacts P0; mobile backup/decryption/key recovery P2/PRD terpisah |
| 13 | Windows Event Log Analyzer | P0 | Event interpretation versioned; detection hanya indication |
| 14 | SQLite Database Viewer | P0 | Read-only; active vs WAL/journal/recovered records dibedakan |
| 15 | Windows Registry Hive Viewer | P0 | Raw/decoded/normalized values dan source cell/offset lineage |
| 16 | ESE Database Parser | P0 | Per-database/version capability dan partial/unsupported state |
| 17 | PLIST Parser | P0 | Binary/XML, raw value, decoder/version, known-key semantics |
| 18 | EXT Family Parser | P0 | Per ext2/3/4 function; journal/recovery limitation eksplisit |
| 19 | APFS/HFS+ Parser | P0 | Snapshot/compression/deleted recovery status per function |
| 20 | Email Forensics | Split | Container/header/content/thread P0; BEC AI/advanced source authentication P2 |
| 21 | Memory Forensics | P0 | Native/bundled capability; offline symbols; no silent external Volatility dependency |
| 22 | Container & Kubernetes | P2 pack | Separate formats, runtime, scale, dan validation profile |
| 23 | Blockchain & Cryptocurrency | P2/companion | Artifact extraction offline P2; live chain/entity lookup connected companion |
| 24 | IoT/Vehicle | P2/PRD terpisah | Acquisition path dan legal/validation model berbeda |
| 25 | Sigma Rule Engine | P0 profile | Tested compatibility subset; match hanya automated indication |
| 26 | YARA Scanner | P0 profile | Language/module compatibility harus eksplisit dan tervalidasi |
| 27 | Anti-Forensics Detection | Split | Deterministic desktop indicators P0; mobile/steganography/advanced media P2 |
| 28 | Pattern of Life | Split | Desktop/browser deterministic P0; mobile-specific P2 |
| 29 | Entity Extraction | P0 | Source-cited; PII access control; Indonesia patterns; false-positive handling |
| 30 | Threat Intelligence Enrichment | Companion/P0 import | Offline signed dataset import P0; public API lookup companion terpisah |
| 31 | Evidence Correlation & Graph | P0/P2 | Within-case graph P0; cross-case graph P2 opt-in |
| 32 | Multi-Case Comparison | P2 | Explicit authority, isolated intelligence store, privacy/retention controls |
| 33 | Automated Triage Rules | P0 | Hanya proposed priority/tag/hypothesis; tidak membuat final finding/status |
| 34 | Deepfake & AI-Generated Detection | P2 research pack | Calibration, false rates, dataset/condition disclosure, human review |
| 35 | Audio & Video Forensics | Split | Technical metadata/structure foundation P0; authentication/enhancement/comparison P2 |
| 36 | Automated Timeline Reconstruction | Split | Deterministic timeline/correlation P0; advanced AI phase narrative P2 or experimental profile |
| 37 | Malware Lab Integration | P2/produk terpisah | Static pack P2; public/local detonation, quarantine, remediation terpisah |
| 38 | Validation & Tool Verification | P0 | Validation evidence untuk approval; tidak memakai klaim universal/court-admissible |
| 39 | AI-Assisted Features | Split | Local provider, citations, context broker, query/draft P0; similar-case/deepfake/advanced automation P2; Collection AI milik Trareon Acquire |

### Aturan prioritas

- **P0** berarti required product outcome. Official Production pertama harus memiliki minimum validated vertical slice pada setiap kategori P0 yang dinyatakan tersedia; format/version tambahan dapat muncul sebagai `Experimental` atau `Limited`, tetapi tidak memenuhi baseline P0 dan tidak boleh menaikkan klaim kategori tersebut.
- **P2** berarti arsitektur harus menyediakan extension boundary, tetapi implementasi tidak boleh menunda P0 defensible core.
- **Companion/PRD terpisah** digunakan bila capability membutuhkan internet, destructive/active execution, acquisition path, legal authority, containment, atau validation discipline yang berbeda.
- Tidak ada P1 feature bucket pada baseline ini. Hardening dan validation menuju Official Production merupakan kewajiban P0, bukan fitur opsional.

---

## Lampiran B — Arsitektur Konseptual dan Data Flow

### Lapisan produk

1. **Evidence Plane:** raw evidence, `.fsnap`, received hashes, manifests, signatures, acquisition records, dan immutable source references.
2. **Deterministic Analysis Plane:** parsers, search, carving, timeline, entity extraction, correlation, YARA/Sigma evaluation, measurements, dan run manifests.
3. **AI Assistance Plane:** local natural-language query, prioritization, hypothesis prompts, narrative/report drafts, dan citation checks.
4. **Human Decision Plane:** notes, observations, hypotheses, findings, review, disagreements, authorization, amendments, dan conclusions.

### Data flow utama

`Case Request → Acceptance/Scope → Evidence Intake → Integrity/Coverage Verification → Examination Plan → Deterministic Processing → Partial Review → Examination/Correlation → Hypotheses → Findings → Technical Review → Administrative Review → Authorization → Signed Report/Export → Closure/Retention`

### Unit dan boundary

| Unit | Tanggung jawab | Tidak boleh dilakukan |
|---|---|---|
| Desktop shell | Window, navigation, presentation, accessibility | Menentukan completion atau memproses raw evidence langsung |
| Case service | Lifecycle, roles, scope, audit, retention | Membypass evidence broker |
| Evidence broker | Read-only handles, identity, ranges, access policy | Memberi arbitrary path ke UI/pack/AI |
| Processing orchestrator | Jobs, resources, checkpoints, coverage | Menyembunyikan failure atau mengubah result semantics |
| Analysis-pack runtime | Sandboxed parsing/rules | Network, arbitrary path, evidence write, unlimited resource |
| Index/evidence graph | Search, normalized facts, provenance relationships | Menghapus raw values atau uncertainty |
| AI broker | Loopback provider, context/citation policy | Shell, internet, approval, unscoped raw evidence |
| Findings/review service | Typed reasoning records dan approvals | Mengubah source evidence atau audit history |
| Report/export service | Controlled rendering, manifest, signatures | Menghilangkan limitation/deviation/error material |
| Validation Center | Method registry, tests, dossiers, status | Mempromosikan capability berdasarkan tombol/tanggal |
| Independent verifier | Memeriksa package, audit, signatures, hashes | Mempercayai status dari UI tanpa verifikasi |

---

## Lampiran C — Standards and Best-Practice Baseline

| Acuan | Edisi/status baseline 17 Juli 2026 | Penggunaan produk | Batas klaim |
|---|---|---|---|
| ISO/IEC 27037 | 2012, published; catalogue menunjukkan review state | Identification/acquisition/preservation handoff, integrity, chain of custody | Guidance, bukan product certification |
| ISO/IEC 27041 | 2015, systematic review | Method fitness, requirements, validation evidence | Clause mapping memerlukan licensed standard |
| ISO/IEC 27042 | 2015, systematic review | Analysis/interpretation, continuity, validity, repeatability, reproducibility, scrutiny | Tidak menjamin conclusion benar |
| ISO/IEC 27043 | 2015, review state | Case lifecycle dari readiness sampai closure | Process support, bukan accreditation |
| ISO/IEC 17025 | 2017, confirmed 2023 | Laboratory competence/quality records | Berlaku pada lab, bukan sertifikasi software |
| ISO/IEC 17043 | 2023 | Proficiency-testing provider/scheme support | Lab exercise feature tidak membuat provider conformant |
| ISO/IEC 27001 | 2022 + Amd 1:2024 | Organizational ISMS enabling controls | Tidak memvalidasi hasil forensik |
| ISO/IEC 27701 | 2025 | Organizational PIMS enabling controls | Tidak menggantikan lawful basis/privacy governance |
| NISTIR 8354 | Final 2022 | Scientific-foundation risk input dan capability/limitation awareness | Bukan implementation/compliance standard |
| NIST CFTT/Federated Testing | Current program | Function-based test design, criteria, datasets, reports | Test scope harus disebut; bukan universal validation |
| NIST NSRL RDS | Versioned offline dataset | Known-file filtering | Match bukan verdict harmless |
| SWGDE 18-F-001-2.0 | Final approved 28 Juli 2025 | Computer forensic examination workflow dan limitations | Best practice harus disesuaikan dengan authority/lab policy |
| SWGDE 18-Q-001 | Published minimum requirements for testing tools | Function-based tool testing sebelum casework; capability dan limitation method | Testing menentukan expected performance; bukan sertifikasi produk |
| SWGDE 18-Q-002-1.0 | Published reporting requirement | Minimum report content dan amendments | Tool report bukan full examination report |
| INTERPOL Global Guidelines for DFL | 2019 | Laboratory operations, evidence, review, report, training | Template harus diterapkan sesuai hukum nasional |
| CASE/UCO | Version-pinned profile | Provenance dan cyber-investigation interchange | Produk hanya mengklaim tested subset/profile |
| C2PA | Content Credentials 2.3 baseline | Media/document provenance manifest verification | Manifest validity bukan truth/authenticity verdict |

Untuk Indonesia, product record harus menjaga pemisahan antara dokumen kewenangan/perolehan, technical integrity/authenticity, dan penilaian hukum. Trareon Lab hanya merekam serta memverifikasi fakta teknis dan dokumen yang dimasukkan; penilaian autentikasi dan perolehan tidak melawan hukum tetap berada pada examiner, penyidik, penasihat hukum, dan pengadilan yang berwenang.

### Compliance traceability record

Setiap atomic control harus mempunyai:

- `TR-ID`;
- source, edition/version, official URL, status, dan verification date;
- control objective;
- PRD requirement ID;
- component dan accountable owner;
- risk class;
- verification method;
- evidence artifact;
- objective acceptance criteria;
- implementation/validation status;
- exception, approver, expiry, residual risk;
- revalidation trigger.

---

## Lampiran D — Dokumentasi sebagai Release Gate

| Dokumen offline | Isi minimum |
|---|---|
| Start Here | Install, first case, `.fsnap` import, verification, first analysis |
| User Guide | Seluruh screen, action, modes, shortcuts, saved views |
| Examiner Workflow | Request sampai amendment/closure dengan responsibilities |
| Live-host Analysis Guide | Footprint, privilege, snapshot-first, limitations, audit |
| AI Assistance Guide | Ollama/LM Studio, model choice, context, citations, prompt injection, review |
| Evidence Format Guide | Formats, versions, functions, platforms, statuses, limitations |
| Capability/Limitations Matrix | `Validated`, `Limited`, `Experimental`, `Deprecated`, `NotSupported` |
| Validation Manual | Datasets, methods, expected/actual, repeatability/reproducibility, revalidation |
| Indonesia Regulatory Profile | Official-source register, product controls, role responsibilities, review date |
| International Standards Profile | ISO/NIST/SWGDE/INTERPOL/CASE/C2PA mappings dan claims boundary |
| Laboratory Quality Guide | Roles, competence, methods, deviations, nonconforming work, PT support |
| Reporting Guide | Findings, citations, opinions, reviews, redaction, amendments, exports |
| Administrator Guide | Install, storage, encryption, identities, backup/restore, updates, sanitization |
| Security Guide | Threat model, hostile evidence, disclosure, SBOM, packs, AI, incident response |
| Troubleshooting/Recovery | Parser failure, corrupt input, disk full, crash, resume, provider failure |
| Training Labs | Synthetic cases, expected results, tamper/failure demonstrations |
| Release Notes/Migration | Semantic changes, validation impact, affected cases, reprocessing, rollback |

---

## Lampiran E — Maturity dan Release Gates

| Kelas hasil | Makna | Penggunaan barang bukti nyata |
|---|---|---|
| Prototype | Workflow/UI dibuktikan dengan fixture | Tidak |
| Engineering Alpha | Core bekerja pada synthetic/file-backed evidence dan failure injection | Tidak |
| Lab Beta | Capability bekerja pada allowlisted lab datasets/devices dengan limitation matrix | Hanya pengujian laboratorium |
| Release Candidate | Cross-platform, documentation, security, provenance, dan validation candidate selesai | Hanya validation protocol |
| Official Production | Per-capability release gate lulus dan validation statement diterbitkan | Sesuai capability matrix dan policy lab |

### Release gate minimum

1. Tidak ada parser/analyzer mencapai `Validated` tanpa approved dossier dan preserved test evidence.
2. Tidak ada report final ketika source integrity failed atau relevant critical errors belum diungkapkan dan didisposisi.
3. Semua report menyebut evidence IDs/hashes, methods/versions, scope, results, coverage, limitations, deviations, reviewer, dan authorizer.
4. Prior run dapat direproduksi pada preserved supported environment atau sistem menjelaskan mengapa exact reproduction tidak mungkin.
5. Standards/regulatory register dan controlled wording ditinjau sebelum major release.
6. Documentation bundle, accessibility, security, backup/restore, migration, dan independent verifier gate lulus.
7. Signing/notarization, SBOM, provenance, support period, known issues, dan capability matrix dapat diperiksa dari official build.

---

## Lampiran F — Indonesia Regulatory Baseline

Baseline ini adalah product-control register awal, bukan nasihat hukum. Exact applicability bergantung pada jenis perkara, kewenangan, sektor, deployment, pihak yang memproses data, dan aturan yang berlaku ketika casework dilakukan. Official Production memerlukan review oleh qualified Indonesian legal/quality personnel.

### Hukum acara dan bukti elektronik

| Sumber resmi | Status baseline | Implikasi untuk Trareon Lab | Tanggung jawab di luar produk |
|---|---|---|---|
| [UU 20/2025 KUHAP](https://peraturan.go.id/id/uu-no-20-tahun-2025) dan [PDF resmi](https://peraturan.go.id/files/uu-no-20-tahun-2025.pdf) | Berlaku 2 Januari 2026; mencabut UU 8/1981 dengan ketentuan transisi | Record authority/scope, object/evidence identity, search/seizure documentation, chain of custody, acquisition dispute, technical authenticity, dan lawful-acquisition status dipisahkan | Penyidik/pihak berwenang menentukan authority dan prosedur; hakim menilai autentikasi serta perolehan tidak melawan hukum |
| [UU 11/2008 ITE](https://jdih.komdigi.go.id/produk_hukum/view/id/167/t/undangundang%2Bnomor%2B11%2Btahun%2B2008), sebagaimana terakhir diubah [UU 1/2024](https://jdih.komdigi.go.id/produk_hukum/view/id/884/t/undangundang%20nomor%201%20tahun%202024) | Berlaku; perubahan kedua diundangkan 2 Januari 2024 | Membantu menjaga accessibility, displayability, integrity, accountability, privacy/confidentiality, dan documented examination melalui hashes, provenance, audit, reproducible export, dan role controls | Pengguna/lab memastikan Sistem Elektronik, authority, procedure, dan ahli memenuhi ketentuan yang benar-benar applicable |

Catatan implementasi:

- KUHAP 20/2025 Pasal 235 mencantumkan bukti elektronik dan memisahkan autentikasi dari perolehan tidak melawan hukum; aplikasi tidak boleh menyatukan keduanya menjadi label `Valid`.
- KUHAP 20/2025 Pasal 242 mencakup Informasi Elektronik, Dokumen Elektronik, dan/atau sistem elektronik yang berkaitan dengan tindak pidana.
- Requirement hash, source-offset provenance, audit chain, repeatability, dan reproducibility merupakan kontrol teknis untuk membantu defensibility; PRD tidak mengklaim undang-undang menentukan algoritma atau format teknis tertentu.
- Evidence yang perolehannya dipersoalkan tetap dapat dicatat dan dianalisis sesuai authorization/policy, tetapi status sengketa dan batas penggunaan harus terlihat serta ikut dalam report/review.

### Pelindungan data dan sistem elektronik

| Sumber resmi | Status baseline | Product-enabling controls | Boundary |
|---|---|---|---|
| [UU 27/2022 Pelindungan Data Pribadi](https://peraturan.bpk.go.id/Details/229798/uu-no-27-tahun-2022%20) | Berlaku sejak 17 Oktober 2022; mencakup jenis data, hak subjek, pemrosesan, kewajiban pengendali/prosesor, transfer, sanksi, dan larangan | Purpose/scope/minimization, data classification, role/access, sensitive reveal, disclosure/transfer log, retention/legal hold, security, incident register, redaction, dan secure disposition | Organisasi menetapkan peran pengendali/prosesor, dasar/authority pemrosesan, pengecualian, respons hak subjek, notification, dan transfer safeguards |
| [PP 71/2019 PSTE](https://peraturan.bpk.go.id/Details/122030/pp-no-71-tahun-2019) | Berlaku sejak 10 Oktober 2019; mencabut PP 82/2012 | Reliable operation, audit trail, security, integrity, availability, retention, incident record, and controlled electronic-system operation | Applicability pada vendor, lab, instansi publik, atau deployment tertentu harus dinilai per peran dan penggunaan; aplikasi offline tidak otomatis berada di luar atau di dalam seluruh kewajiban PSE |

Trareon Lab tidak menentukan lawful basis, kewajiban notifikasi, atau apakah deployment tertentu merupakan PSE lingkup publik/privat. Produk menyediakan record dan kontrol agar keputusan tersebut dapat diterapkan serta diaudit oleh organisasi.

### SNI yang telah diverifikasi pada katalog BSN

| SNI | Status katalog BSN | Relevansi |
|---|---|---|
| [SNI ISO/IEC 17025:2017](https://pesta.bsn.go.id/produk/detail/11951-sniisoiec170252017) | Berlaku; adopsi identik ISO/IEC 17025:2017 | Laboratory competence, impartiality, consistent operation; bukan sertifikasi software |
| [SNI ISO/IEC 17043:2023](https://pesta.bsn.go.id/index.php/produk/detail/14477-sniisoiec170432023) | Berlaku; adopsi identik ISO/IEC 17043:2023 | Competence/impartiality PT provider; blind exercise support tidak membuat produk/provider otomatis conformant |
| [SNI ISO IEC 27037:2014 Konfirmasi 2019](https://pesta.bsn.go.id/produk/detail/2703720142019-sniisoiec27037%3A2014konfirmasi2019) | Berlaku; identik ISO/IEC 27037:2012 | Identification, collection, acquisition, preservation handoff |
| [SNI ISO/IEC 27042:2015](https://pesta.bsn.go.id/produk/detail/13818-sniisoiec270422015) | Berlaku; identik ISO/IEC 27042:2015 | Analysis dan interpretation of digital evidence |
| [SNI ISO/IEC 27043:2016](https://pesta.bsn.go.id/produk/detail/11035-sniisoiec270432016) | Berlaku; identik ISO/IEC 27043:2015 | Incident investigation principles/processes |

Clause-level mapping harus menggunakan salinan standar berlisensi yang dimiliki organisasi dan ditinjau quality/legal personnel. Catalogue metadata saja tidak cukup untuk menyatakan conformity.

### Indonesia-first product controls

1. Bahasa Indonesia, English translation, dan controlled forensic/legal glossary.
2. WIB/WITA/WIT, UTC, unknown timezone, clock offset, raw timestamp, precision, dan ambiguity.
3. Case authority/scope/handling records dan attachment untuk izin, berita acara, custody, serta approval.
4. Typed separation antara technical integrity/authenticity, authority documentation, disputed acquisition, examiner opinion, dan court/legal disposition.
5. PII classification untuk data umum/spesifik dan pola lokal seperti NIK, NPWP, rekening, nomor telepon, biometrik, kesehatan, anak, keuangan, serta catatan kejahatan.
6. Enforced minimization pada processing, search, preview, AI context, export, dan report.
7. Role-based reveal/export, disclosure/transfer register, legal hold, retention, redaction, dan secure disposition.
8. Incident register dan configurable policy deadlines tanpa menyatakan satu deadline selalu berlaku untuk semua role/sector.
9. Versioned legal/SNI register dengan official-source links, owner, verification date, change trigger, dan qualified review.

### Open legal-review items sebelum Official Production

1. Applicability UU PDP dan PP 71/2019 per deployment profile: private corporate lab, public-sector lab, law enforcement, consultant, education/training, dan standalone examiner.
2. Sector-specific confidentiality, retention, incident-notification, data localization, state-secret, financial, health, employment, child, dan professional-privilege rules.
3. Exact authority/approval fields untuk pidana, perdata, internal investigation, regulatory investigation, dan incident response.
4. Treatment of transition cases under KUHAP 20/2025 and prior procedures.
5. Electronic signature/certificate profile accepted for reports, exports, custody, and long-term verification.
6. Cross-border disclosure/transfer and international cooperation requirements.
7. Accredited-laboratory interpretation of SNI ISO/IEC 17025/17043 clauses using licensed copies and KAN/sector guidance applicable to the organization.
