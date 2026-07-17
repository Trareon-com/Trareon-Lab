# Trareon Lab — Indonesia Regulatory Baseline

**Snapshot:** 17 Juli 2026  
**Tujuan:** baseline product-control Indonesia untuk PRD Trareon Lab.  
**Batas:** bukan nasihat hukum, legal opinion, atau pernyataan conformity/akreditasi.

## Posisi aman

Trareon Lab harus membantu organisasi merekam kewenangan, scope, chain of custody, technical integrity/authenticity, processing, disclosure, dan review. Aplikasi tidak boleh memutuskan bahwa perolehan evidence sah, suatu item admissible, organisasi otomatis memenuhi UU/SNI, atau laboratorium terakreditasi.

Applicability bergantung pada jenis perkara, kewenangan, sektor, deployment, pihak yang memproses data, dan aturan saat casework dilakukan. Official Production memerlukan review oleh qualified Indonesian legal/quality personnel.

## 1. KUHAP yang berlaku dan bukti elektronik

### Status

- [UU 20/2025 tentang KUHAP](https://peraturan.go.id/id/uu-no-20-tahun-2025) disahkan dan diundangkan 17 Desember 2025; [teks PDF resmi](https://peraturan.go.id/files/uu-no-20-tahun-2025.pdf) menetapkan mulai berlaku 2 Januari 2026.
- Pasal 362 mencabut UU 8/1981; Pasal 361 mengatur perkara transisi dan Pasal 365 mempertahankan aturan pelaksana lama sepanjang tidak bertentangan.
- Gunakan tanggal pada PDF/Lembaran Negara sebagai sumber utama bila metadata portal berbeda.

### Ketentuan relevan

- Pasal 1 angka 35 mencakup penyitaan benda bergerak/tidak bergerak dan berwujud/tidak berwujud; angka 38–39 mendefinisikan Informasi Elektronik dan Dokumen Elektronik.
- Pasal 112 memasukkan Informasi Elektronik dan Dokumen Elektronik sebagai objek penggeledahan.
- Pasal 113 serta Pasal 119–122 mengatur kontrol pengadilan, kondisi mendesak, objek/alasan, saksi, berita acara, dan salinan dokumen penggeledahan/penyitaan.
- Pasal 235(1)(f) mencantumkan bukti elektronik. Pasal 235(3)–(5) memisahkan autentikasi dari perolehan yang tidak melawan hukum serta menempatkan penilaiannya pada hakim.
- Pasal 242 mencakup Informasi Elektronik, Dokumen Elektronik, dan/atau sistem elektronik yang berkaitan dengan tindak pidana.

### Product controls

1. Record authority/reference, purpose, object, permitted source, date/artifact/search scope, restriction, approval, berita acara, dan custody attachments.
2. Pisahkan `Legal Authority Record Status` (`Documented`, `Incomplete`, `Disputed`, `NotAssessed`) dari `Technical Authenticity/Integrity Status`.
3. Evidence yang perolehannya dipersoalkan harus dapat diberi marking, restriction, review, dan report disclosure tanpa mengubah technical hash result.
4. Hash, provenance, immutable audit, reproducible export, dan source-offset lineage diposisikan sebagai technical defensibility controls; jangan mengklaim KUHAP menentukan algoritma atau format tersebut.

## 2. UU ITE

- [UU 11/2008 tentang ITE](https://jdih.komdigi.go.id/produk_hukum/view/id/167/t/undangundang%2Bnomor%2B11%2Btahun%2B2008) telah diubah, terakhir dengan [UU 1/2024](https://jdih.komdigi.go.id/produk_hukum/view/id/884/t/undangundang%20nomor%201%20tahun%202024), yang berlaku sejak 2 Januari 2024.
- Pasal 5 menempatkan Informasi/Dokumen Elektronik dan hasil cetaknya sebagai alat bukti hukum yang sah/perluasan alat bukti, dengan pengecualian bila undang-undang mengatur lain.
- Pasal 6 mensyaratkan accessibility, kemampuan ditampilkan, keutuhan, dan accountability agar informasi menerangkan suatu keadaan.
- Pasal 43 mengaitkan penyidikan dengan privasi, kerahasiaan, layanan publik, integritas/keutuhan data, serta hukum acara pidana untuk penggeledahan/penyitaan sistem elektronik.

Product controls yang relevan: read-only evidence, integrity verification, stable IDs, provenance, audit, reproducible export, confidentiality/access control, limitation records, dan method/tool/version documentation.

Kehadiran kontrol tersebut tidak membuat setiap file otomatis sah atau admissible; authority, procedure, system operation, expert competence, dan penilaian hakim tetap berada di luar aplikasi.

## 3. Pelindungan Data Pribadi

- [UU 27/2022 tentang Pelindungan Data Pribadi](https://peraturan.bpk.go.id/Details/229798/uu-no-27-tahun-2022%20) berstatus berlaku sejak 17 Oktober 2022.
- Portal resmi merangkum ruang lingkupnya sebagai jenis data, hak subjek, pemrosesan, kewajiban pengendali/prosesor, transfer, sanksi, kelembagaan, larangan, dan ketentuan pidana.
- Data spesifik mencakup antara lain kesehatan, biometrik, genetika, catatan kejahatan, anak, dan keuangan pribadi; general forensic case data juga dapat mengidentifikasi individu.

### Product controls

- Purpose, authority, scope, minimization, legal hold, retention, restriction, dan secure disposition.
- Data classification termasuk NIK, NPWP, rekening, nomor telepon, biometrik, kesehatan, anak, keuangan, dan catatan kejahatan.
- Role/access control, audited reveal/copy/export, case isolation, encryption, redaction, dan disclosure/transfer register.
- Deployment record untuk pihak yang ditetapkan organisasi sebagai pengendali/prosesor/authorized handler, tujuan/authority, data category, contact, dan approval.
- Incident register untuk affected case/category, discovery, containment, notification decision/deadline, authority/recipient, content, dan evidence of action.
- Cross-organization/jurisdiction export memerlukan destination, purpose, authority/approval, minimization/redaction, dan safeguards record.

Organisasi tetap bertanggung jawab menentukan peran, dasar/authority pemrosesan, pengecualian, respons hak subjek, notification, serta transfer safeguards.

## 4. PP 71/2019 PSTE

- [PP 71/2019 tentang Penyelenggaraan Sistem dan Transaksi Elektronik](https://peraturan.bpk.go.id/Details/122030/pp-no-71-tahun-2019) berlaku sejak 10 Oktober 2019 dan mencabut PP 82/2012.
- Definisi PSE mencakup pihak yang menyediakan, mengelola, dan/atau mengoperasikan Sistem Elektronik bagi dirinya dan/atau pihak lain; PP membedakan lingkup publik dan privat.

Trareon Lab dapat menyediakan enabling controls untuk reliability, security, integrity, availability, audit, retention, incident records, dan controlled system operation. Namun aplikasi offline tidak otomatis berada di luar atau di dalam seluruh kewajiban PSE. Applicability pada vendor, lab, instansi publik, badan usaha, atau deployment tertentu harus dinilai per peran/penggunaan.

## 5. Adopsi SNI yang diverifikasi

| SNI | Status katalog BSN | Relevansi |
|---|---|---|
| [SNI ISO/IEC 17025:2017](https://pesta.bsn.go.id/produk/detail/11951-sniisoiec170252017) | Berlaku; ISO/IEC 17025:2017, IDT | Laboratory competence, impartiality, consistent operation; bukan sertifikasi software |
| [SNI ISO/IEC 17043:2023](https://pesta.bsn.go.id/index.php/produk/detail/14477-sniisoiec170432023) | Berlaku; ISO/IEC 17043:2023, IDT | PT provider/scheme; exercise support tidak membuat provider conformant |
| [SNI ISO IEC 27037:2014 Konfirmasi 2019](https://pesta.bsn.go.id/produk/detail/2703720142019-sniisoiec27037%3A2014konfirmasi2019) | Berlaku; ISO/IEC 27037:2012, IDT | Identification, collection, acquisition, preservation handoff |
| [SNI ISO/IEC 27042:2015](https://pesta.bsn.go.id/produk/detail/13818-sniisoiec270422015) | Berlaku; ISO/IEC 27042:2015, IDT | Analysis dan interpretation of digital evidence |
| [SNI ISO/IEC 27043:2016](https://pesta.bsn.go.id/produk/detail/11035-sniisoiec270432016) | Berlaku; ISO/IEC 27043:2015, IDT | Incident-investigation principles/processes |

Clause-level mapping memerlukan salinan berlisensi yang dimiliki organisasi dan review oleh quality/legal personnel. Catalogue metadata tidak cukup untuk menyatakan conformity.

## 6. Product controls versus role responsibility

| Area | Trareon Lab | Examiner/lab/investigator/legal/court |
|---|---|---|
| Authority | Menyimpan record, status, scope, attachment, approval | Menentukan authority dan kecukupan prosedur |
| Integrity | Menghitung/verifikasi hash, provenance, audit, signatures | Menentukan metode/policy dan menilai limitation |
| Authenticity | Menyimpan technical observations dan review | Examiner/ahli memberi opinion; hakim menilai sesuai hukum |
| Lawful acquisition | Menyimpan documented/disputed/not-assessed state | Pihak berwenang/legal/court menilai perolehan |
| PDP | Menyediakan minimization, access, retention, transfer, incident controls | Organisasi menetapkan role, authority, exception, notification, transfer safeguards |
| SNI/ISO | Menyediakan records, validation, review, traceability | Lab menerapkan QMS; assessor/accreditation body menilai conformity |

## 7. Open legal-review items

1. Applicability per deployment: private corporate lab, public-sector lab, law enforcement, consultant, education/training, dan standalone examiner.
2. Sector-specific confidentiality, retention, notification, localization, state-secret, health, finance, employment, child, dan privilege rules.
3. Authority fields untuk pidana, perdata, internal investigation, regulatory investigation, dan incident response.
4. Transition cases di bawah UU 20/2025 dan prosedur lama.
5. Electronic-signature/certificate profile untuk report, custody, export, serta long-term verification.
6. Cross-border disclosure/transfer dan international cooperation.
7. KAN/sector guidance dan clause-level interpretation untuk SNI ISO/IEC 17025/17043.

## 8. Safe PRD wording

Gunakan:

- “Mendukung workflow yang dipetakan ke sumber dan edisi tertentu.”
- “Menyediakan records dan controls yang dapat membantu legal/quality review.”
- “Technical integrity verified” hanya bila anchor dan metode verifikasi jelas.

Hindari tanpa substantiation independen:

- “Sah sebagai alat bukti”, “court-admissible”, atau “court-approved”.
- “ISO/SNI compliant software”, “laboratorium otomatis terakreditasi”.
- “Hash membuktikan perolehan sah” atau “dokumen kewenangan membuktikan integritas teknis”.

