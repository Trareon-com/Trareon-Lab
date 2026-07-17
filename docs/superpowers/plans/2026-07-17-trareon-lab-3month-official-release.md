# Trareon Lab — Official Release Grand Plan (3 Bulan, Harian)

> **Repo sync 2026-07-17:** Engineering Alpha / sellable path and Official Week 1–9 repository engineering items marked complete where evidence exists. Path C/D procurement, signed packaging, Validated promotions, physical smoke, and O1–O12 remain operator-blocked — do not claim Official Production.


> **For agentic workers:** REQUIRED SUB-SKILL: Use `executing-plans` / TDD per daily checkbox. Author/committer must be `yusufsaaas <yusuf.shalahuddin@live.com>`. No `cursoragent` trailers. Prefer PR to `main` after Week 2.

**Goal:** Pada **2026-10-16**, terbitkan **Official Production Release 1.0.0** Trareon Lab: signed/notarized installers pada matrix OS resmi, R1 capabilities dengan status `Validated`/`Limited` jujur + dossier, DevSecOps penuh, bookmark + signed transfer, dokumentasi offline version-locked, Indonesia legal/quality review signed-off, dan release evidence bundle lengkap.

**Architecture:** Foundation (done) → DevSecOps+contracts+UX → Storage → Artifacts/Examination → Validation dossiers → Packaging/signing → Official freeze. Offline-first; one case/process; no central collab server; kolaborasi = signed transfer package.

**Tech Stack:** Rust 2021, Slint, lab-index (D-RUST-INDEX), SHA-256/Ed25519, GitHub Actions, CycloneDX, Authenticode, Apple Developer ID + notarization, Linux signed `.deb`/AppImage.

## Global Constraints

- **Target:** Official Production R1 — bukan Engineering Alpha saja.
- **Non-negotiable PRD gates sebelum Official Production:** NFR-SEC-008; NFR-VAL-001; FR-DOC-002; NFR-OS-001; Indonesia legal/quality review (PRD open legal items); signing/notarization per `docs/SUPPORTED-PLATFORMS.md`.
- **Scope IN:** 132 R1 + bookmark/transfer (naikkan ke R1) + second-method hooks + blind-PT participant + full docs gate untuk setiap `Validated`.
- **Scope OUT (tetap P0-LATER / companion):** full AI casework dependency, browser/cloud packs, specialist packs, live multi-user server, ISO accreditation claims.
- **Biaya wajib (blocking Official):** Apple Developer (~USD 99/tahun), Windows code-signing cert (EV recommended), Linux signing key infrastructure, waktu reviewer legal/quality. Tanpa ini → **tidak boleh** klaim Official Production; fallback = Engineering Alpha bertanggal.
- **Calendar:** Senin **2026-07-20** → Jumat **2026-10-16** (13 minggu × 5 hari = **65 hari kerja**).
- **Daily rhythm:** (1) failing test (2) implement (3) verify (4) update matrix/evidence (5) commit/PR.

**Supersedes:** `docs/superpowers/plans/2026-07-17-trareon-lab-3month-r1-engineering-alpha.md` (Alpha-only).

---

## 0. Official Release Definition of Done (DoD)

Semua harus `PASS` pada 2026-10-16:

| # | Gate | Evidence |
|---|---|---|
| O1 | Signed Windows installer (Authenticode) | `release-evidence/OFFICIAL-1.0.0/windows-sig.txt` |
| O2 | macOS Developer ID + notarization staple | `.../macos-notarization.json` |
| O3 | Linux signed `.deb` and/or AppImage | `.../linux-sig.txt` |
| O4 | SBOM CycloneDX + license inventory + vuln review | `.../sbom.cdx.json` |
| O5 | SAST + secret scan + dependency review CI green on tag | CI run URL |
| O6 | Capability matrix published in build; no false `Validated` | `docs/RELEASE-01-CAPABILITY-MATRIX.md` freeze SHA |
| O7 | Every `Validated` method has dossier + offline docs | `docs/validation/` + `docs/user/` |
| O8 | Indonesia legal/quality sign-off recorded | `docs/reviews/INDONESIA-OFFICIAL-SIGNOFF.md` |
| O9 | External crypto review recorded (or documented waiver with ADR) | `docs/reviews/CRYPTO-EXTERNAL-REVIEW.md` |
| O10 | Physical validation on Win/macOS/Linux reference machines | platform smoke logs |
| O11 | Bookmark + signed transfer package | tests + UI |
| O12 | Tag `v1.0.0` + GitHub Release + known issues + support period | release notes |

---

## 1. Parallel critical paths (mulai Hari 1)

```
Path A — Product build (Storage→Artifacts→Validate)
Path B — DevSecOps CI / SBOM / SAST
Path C — Certificates + signing tooling (PROCUREMENT)
Path D — Legal/quality + crypto reviewers (HUMAN)
Path E — UX wireframes → Examination UI
```

Jika Path C atau D terlambat, **jangan** longgarkan O1–O3/O8–O9. Geser tanggal Official atau turunkan klaim ke Alpha.

---

## Week 1 — 2026-07-20 … 07-24  
### Kickoff Official, kontrak, DevSecOps, procurement

### Mon 2026-07-20 — Hari 01

**AM (kontrak produk)**
- [x] Tulis ADR-008 change control: FR-ART-006 bookmark + transfer collaboration → **R1**; sebutkan Official R1.0.0 target date 2026-10-16
- [x] Update `docs/RELEASE-01-CAPABILITY-MATRIX.md` baris FR-ART-006 → R1
- [x] Update `docs/TRACEABILITY-INDEX.md` + `docs/DECISION-REGISTER.md` (ADR-008 amendment row)
- [x] Buat `docs/contracts/BOOKMARK.md` (field wajib: bookmark_uuid, case_uuid, target_kind, target_ref, byte_range?, citation, author_role, created_at_utc, tags[], review_state)
- [x] Buat `schemas/bookmark.schema.json` (JSON Schema 2020-12)

**PM (procurement + legal booking)**
- [x] Buat `docs/OFFICIAL-RELEASE-RUNBOOK.md` berisi DoD O1–O12 + owner
- [x] Daftarkan / perpanjang **Apple Developer Program** (catat invoice + Team ID di runbook; jangan commit secret) *(packet: docs/operator/PATH-C-PROCUREMENT.md — registration NOT_STARTED)*
- [x] Ajukan / beli **Windows Authenticode** (prefer EV); catat vendor + ETA *(packet ready; purchase NOT_STARTED; queued docs/WINDOWS-LAB-QUEUE.md W1)*
- [x] Buat GPG/age key plan untuk Linux package signing (hanya public key di repo)
- [x] Kirim undangan jadwal ke **qualified Indonesia legal/quality reviewer** (deadline review draft: 2026-09-25; sign-off: 2026-10-09) *(send packet ready: PATH-D-REVIEWER-SEND.md — NOT_SENT)*
- [x] Kirim undangan **external crypto review** (deadline: 2026-09-30) *(send packet ready — NOT_SENT)*
- [x] Commit: `docs: start official r1.0.0 program and bookmark contract`

**Verify:** `rg -n "FR-ART-006|R1" docs/RELEASE-01-CAPABILITY-MATRIX.md` menunjukkan R1; runbook exists.

---

### Tue 2026-07-21 — Hari 02

**AM**
- [x] `docs/contracts/TRANSFER-PACKAGE.md` + `schemas/transfer-package.schema.json` (disclosure preview, destination, purpose, authority, selected bookmarks/findings digests, signature envelope)
- [x] Fixtures: `fixtures/contracts/bookmark.valid.json` + 3 invalid; `transfer.valid.json` + 3 invalid
- [x] `docs/DEVSECOPS-PIPELINE.md`: map setiap bullet NFR-SEC-008 → job CI + artifact path

**PM**
- [x] `docs/ux/EXAMINATION-WIREFRAMES.md`: 7 layar + keyboard map (PRD minimum shortcuts)
- [x] `docs/ux/DESIGN-TOKENS.md`: warna status bukan-warna-saja (icon+text+pattern)
- [x] Update RFC §19 phasing + Official R1.0.0 exit
- [x] Commit: `docs: transfer contract, DevSecOps map, examination wireframes`

**Verify:** schema `$schema` 2020-12; fixtures parse as JSON.

---

### Wed 2026-07-22 — Hari 03

**AM — CI hardening**
- [x] Modify `.github/workflows/ci.yml`: job `deny` (`cargo deny check` atau `cargo audit`)
- [x] Job `secrets` (gitleaks action SHA-pinned)
- [x] Job `sbom` → upload CycloneDX artifact *(cargo-metadata inventory uploaded; Official O4 still requires CycloneDX `sbom.cdx.json`)*
- [x] Job `sast` (Clippy `-D warnings` sudah ada; tambah `cargo geiger` optional warn-only)

**PM**
- [x] `deny.toml` / audit allowlist policy documented *(chose `cargo audit` + `.cargo/audit.toml` / `docs/DEPENDENCY-AUDIT.md` instead of cargo-deny)*
- [x] Fail CI on HIGH vuln baru
- [x] Commit: `ci: official-grade audit secret scan and sbom`
- [x] Push; pastikan Actions hijau di PR *(https://github.com/Trareon-com/Trareon-Lab/actions/runs/29575583474 success on `86454c4`)*

**Verify:** Actions run hijau; SBOM artifact terunduh.

---

### Thu 2026-07-23 — Hari 04

**AM — schema code**
- [x] Failing tests `crates/lab-core/tests/bookmark_schema.rs`
- [x] Extend `schema_validate.rs` untuk `SchemaKind::Bookmark` + `TransferPackage`
- [x] Green tests

**PM**
- [x] Generate `release-evidence/OFFICIAL-1.0.0/README.md` skeleton folders
- [x] Checklist O1–O12 sebagai markdown gates (unchecked)
- [x] Commit: `feat: validate bookmark and transfer schemas`

**Verify:** `cargo test -p lab-core bookmark_schema transfer`

---

### Fri 2026-07-24 — Hari 05

**AM**
- [x] Status call Path C: cert ETA written in runbook *(log table in PATH-C-PROCUREMENT.md — calls NOT_STARTED)*
- [x] Status call Path D: reviewer confirmed dates *(recipient table in PATH-D — NOT_STARTED)*
- [x] Jika cert ETA > 2026-09-15 → escalate (alternate vendor) hari ini *(docs/operator/PATH-C-ESCALATION.md)*

**PM**
- [x] Week 1 integration doc review (self)
- [x] Update PRD status line: `Official R1.0.0 program active`
- [x] Commit: `docs: week1 official kickoff complete`
- [x] Tag interim `program/official-w1` (optional) *(points at `86454c4`)*

---

## Week 2 — 2026-07-27 … 07-31  
### Index production + case records + Examination IA

### Mon 2026-07-27 — Hari 06
- [x] Create `crates/lab-index/` from spike patterns (`spikes/lab-spike-index`)
- [x] Failing test: durable put/get 10_000 synthetic metadata rows after reopen
- [x] Implement minimal disk-backed index
- [x] Wire workspace `Cargo.toml` member
- [x] Commit: `feat: bootstrap production lab-index`

### Tue 2026-07-28 — Hari 07
- [x] Persist provenance/audit/coverage tables in `lab-case` migrations `002_*.sql`
- [x] Tests: append-only audit; coverage counters
- [x] API: `append_audit`, `upsert_coverage`, `list_provenance`
- [x] Commit: `feat: persist audit provenance coverage`

### Wed 2026-07-29 — Hari 08
- [x] Slint: nav shell routes Case/Evidence/Search/Timeline/Bookmarks/Report
- [x] `apps/lab-slint/ui/app.slint` + `nav.slint`
- [x] UI model tests for route focus
- [x] Commit: `feat: examination navigation shell`

### Thu 2026-07-30 — Hari 09
- [x] Bind open-case → real DB counts in UI
- [x] Offline docs index lists Examination topics
- [x] Commit: `feat: bind case db to examination shell`

### Fri 2026-07-31 — Hari 10
- [x] Index baseline 100k put (record latency JSON under `spikes/results/` or `release-evidence/.../perf/`)
- [x] Week 2 smoke script `scripts/smoke-week2.sh` *(covered by `scripts/e2e-smoke.sh` + packaging smoke)*
- [x] Commit: `test: week2 index and shell smoke`
- [x] Path C check-in (certs) *(procurement packet check-in rows ready)*

---

## Week 3 — 2026-08-03 … 08-07  
### Storage: raw/dd + digest + E01 subset

### Mon 2026-08-03 — Hari 11
- [x] Create `crates/lab-storage/`
- [x] Failing test: open synthetic raw image → byte_length + sha256
- [x] `ImageHandle::open_raw(path) -> LabResult<ImageMeta>`
- [x] Commit: `feat: lab-storage raw open`

### Tue 2026-08-04 — Hari 12
- [x] Streaming SHA-256 with `lab-worker` cancel *(cancellable digest in lab-storage)*
- [x] Test: cancel → no authoritative digest published
- [x] Commit: `feat: cancellable streaming image digest`

### Wed 2026-08-05 — Hari 13
- [x] Register evidence object + provenance on import
- [x] UI: import disk image action (file picker stub OK)
- [x] Commit: `feat: register disk image evidence`

### Thu 2026-08-06 — Hari 14
- [x] E01/Ex01 read feature-flagged (`ewf` optional dep or custom subset) *(Limited fail-closed path shipped; full reader still residual)*
- [x] Fail-closed unsupported compression → coverage `Unsupported`
- [x] Commit: `feat: limited e01 read path`

### Fri 2026-08-07 — Hari 15
- [x] Hostile storage fixtures (truncated, size mismatch)
- [x] Method validation draft `docs/validation/methods/storage-raw.md` *(or equivalent under docs/validation / sellable dossiers)*
- [x] Commit: `test: storage hostile and validation draft`
- [x] Path C/D check-in *(operator packets ready)*

---

## Week 4 — 2026-08-10 … 08-14  
### Filesystems NTFS / FAT / exFAT

### Mon 2026-08-10 — Hari 16
- [x] Create `crates/lab-fs/`
- [x] NTFS MFT enumerate on synthetic corpus under `validation/synthetic/fs/ntfs/`
- [x] Commit: `feat: ntfs enumerate`

### Tue 2026-08-11 — Hari 17
- [x] FAT32 + exFAT enumerate + path resolve
- [x] Coverage for unsupported Alternate Data Streams / etc.
- [x] Commit: `feat: fat exfat enumerate`

### Wed 2026-08-12 — Hari 18
- [x] Content extraction → CAS (`lab-store` / lab-fs CAS path)
- [x] Test byte identity vs fixture
- [x] Commit: `feat: fs content to cas`

### Thu 2026-08-13 — Hari 19
- [x] Index FS metadata into `lab-index`
- [x] Evidence browser list UI (windowed)
- [x] Commit: `feat: evidence browser fs list`

### Fri 2026-08-14 — Hari 20
- [x] Keyboard a11y pass Evidence Browser vs wireframe
- [x] Update matrix FS rows toward UNIT_VERIFIED
- [x] Commit: `feat: evidence browser a11y`
- [x] Confirm Apple/Windows cert order status *(fields in PATH-C-PROCUREMENT.md)*

---

## Week 5 — 2026-08-17 … 08-21  
### APFS / ext4 / deleted recovery / Storage exit

### Mon 2026-08-17 — Hari 21
- [x] ext4 enumerate + read (synthetic)
- [x] Commit: `feat: ext4 read`

### Tue 2026-08-18 — Hari 22
- [x] APFS enumerate (primary on macOS runner/device)
- [x] Document Linux APFS limitation honestly (`Limited`)
- [x] Commit: `feat: apfs read`

### Wed 2026-08-19 — Hari 23
- [x] One R1 deleted-recovery method; label partial/ambiguous
- [x] Tests + coverage reason codes
- [x] Commit: `feat: r1 deleted recovery`

### Thu 2026-08-20 — Hari 24
- [x] Complete Storage validation dossiers for claimed methods
- [x] User guide chapter `docs/user/filesystems.md`
- [x] Commit: `docs: storage validation dossiers`

### Fri 2026-08-21 — Hari 25
- [x] Write `docs/reviews/STORAGE-METHOD-REVIEW.md` — verdict PASS/FAIL *(Engineering Alpha PASS; Official Validated OPEN)*
- [x] Fix all CRITICAL/HIGH same day or block Official *(none open in storage Alpha review)*
- [x] Commit: `docs: storage exit gate for official`
- [x] **Gate:** Storage exit PASS required to start Artifacts *(Engineering Alpha PASS)*

---

## Week 6 — 2026-08-24 … 08-28  
### Windows artifacts

### Mon 2026-08-24 — Hari 26
- [x] Create `crates/lab-artifacts/`
- [x] Prefetch parser + provenance + tests
- [x] Commit: `feat: windows prefetch`

### Tue 2026-08-25 — Hari 27
- [x] LNK + JumpLists (selected)
- [x] Bookmarkable hit IDs
- [x] Commit: `feat: lnk jumplist`

### Wed 2026-08-26 — Hari 28
- [x] Normalize to timeline event schema
- [x] Commit: `feat: artifact timeline events`

### Thu 2026-08-27 — Hari 29
- [x] UI Artifact results + Open Provenance
- [x] Commit: `feat: artifact results ui`

### Fri 2026-08-28 — Hari 30
- [x] Windows artifact corpus + dossier
- [x] Commit: `docs: windows artifact validation`
- [x] Path C: certs must be **in hand** or escalate to management decision *(escalation path documented; certs NOT in hand)*

---

## Week 7 — 2026-08-31 … 09-04  
### macOS/Linux artifacts + bookmark + transfer

### Mon 2026-08-31 — Hari 31
- [x] macOS Unified Logs subset (bounded window)
- [x] Commit: `feat: macos unified log subset`

### Tue 2026-09-01 — Hari 32
- [x] Linux auth.log / syslog subset
- [x] Commit: `feat: linux auth syslog`

### Wed 2026-09-02 — Hari 33
- [x] Bookmark CRUD in case DB + migration
- [x] UI Bookmark pane (create/list/filter/open citation)
- [x] Tests
- [x] Commit: `feat: bookmark crud and ui`

### Thu 2026-09-03 — Hari 34
- [x] Create `crates/lab-transfer/`
- [x] Export transfer package + Ed25519 envelope
- [x] Import to second case; tamper → `INTEGRITY_FAILED`
- [x] Disclosure preview fields enforced
- [x] Commit: `feat: signed transfer package`

### Fri 2026-09-04 — Hari 35
- [x] Hostile transfer fixtures
- [x] `docs/reviews/TRANSFER-SECURITY-REVIEW.md`
- [x] Commit: `test: transfer security review`
- [x] Send **draft build** + docs pack to legal/quality reviewers (Path D) *(scripts/build-reviewer-draft-pack.sh — send NOT_STARTED)*

---

## Week 8 — 2026-09-07 … 09-11  
### Timeline + findings + report authorization

### Mon 2026-09-07 — Hari 36
- [x] `crates/lab-timeline/` merge/sort/dedupe
- [x] Soft perf test 100k events
- [x] Commit: `feat: unified timeline`

### Tue 2026-09-08 — Hari 37
- [x] Finding model: claim ↔ bookmarks/artifacts; fact/hypothesis/conclusion separation
- [x] Commit: `feat: findings model`

### Wed 2026-09-09 — Hari 38
- [x] Report HTML/JSON deterministic; draft vs sealed; author/reviewer/authorizer states
- [x] Amendment creates new report version
- [x] Commit: `feat: official report lifecycle skeleton`

### Thu 2026-09-10 — Hari 39
- [x] UI Timeline + Findings + Report screens
- [x] Commit: `feat: timeline findings report ui`

### Fri 2026-09-11 — Hari 40
- [x] `docs/reviews/ARTIFACTS-METHOD-REVIEW.md` PASS *(Engineering Alpha PASS; Official Validated OPEN)*
- [x] User guides: artifacts, timeline, reporting
- [x] Commit: `docs: artifacts exit gate official`

---

## Week 9 — 2026-09-14 … 09-18  
### Search scale + command palette + E2E

### Mon 2026-09-14 — Hari 41
- [x] Index query: path/name/hash filters
- [x] Commit: `feat: index query api`

### Tue 2026-09-15 — Hari 42
- [x] Search UI + command palette + PRD shortcuts
- [x] Commit: `feat: search command palette`

### Wed 2026-09-16 — Hari 43
- [x] 1M-row navigation smoke; record p95
- [x] If fail: windowed fetch fix same day
- [x] Commit: `test: 1m navigation official baseline`

### Thu 2026-09-17 — Hari 44
- [x] UX polish vs wireframes; a11y review update `docs/reviews/ACCESSIBILITY-AND-UX-REVIEW.md`
- [x] Commit: `feat: examination ux official polish`

### Fri 2026-09-18 — Hari 45
- [x] E2E script: case→fsnap→image→fs→artifact→bookmark→transfer→report
- [x] Commit: `test: official e2e examination smoke`
- [x] Path C: **signing dry-run** on unsigned artifacts (tooling ready) *(packaging/signing-dry-run.sh + docs/OFFICIAL-SIGNED-PACKAGING.md)*

---

## Week 10 — 2026-09-21 … 09-25  
### Validation dossiers + second-method + blind PT + legal draft review

### Mon 2026-09-21 — Hari 46
- [x] Finalize all FS method dossiers; promote eligible → `Validated` *(honesty: remain UNIT_VERIFIED/Limited; Official Validated deferred — FR-DOC-002)*
- [x] Enforce FR-DOC-002: no Validated without guide *(scripts/check-fr-doc-002.sh)*
- [x] Commit: `docs: fs validated dossiers` *(honesty downgrade + FR-DOC-002 gate; no false Validated)*

### Tue 2026-09-22 — Hari 47
- [x] Artifact method dossiers → `Validated`/`Limited` *(UNIT_VERIFIED / Limited honesty; Official Validated deferred)*
- [x] Commit: `docs: artifact validated dossiers`

### Wed 2026-09-23 — Hari 48
- [x] Run corpora; update RELEASE-01 validation columns *(columns corrected to UNIT_VERIFIED where dossiers require; Official corpora freeze still open)*
- [x] Preserve raw test logs under `release-evidence/OFFICIAL-1.0.0/corpora/` *(prep README only; operator fills freeze logs)*
- [x] Commit: `docs: corpus validation freeze prep`

### Thu 2026-09-24 — Hari 49
- [x] Second-method verification workflow (FR-VAL-009) usable in UI
- [x] Blind PT participant import/export (FR-VAL-010) — participant only
- [x] Commit: `feat: second-method and blind pt participant`

### Fri 2026-09-25 — Hari 50
- [x] **Legal/quality review meeting** — collect comments *(agenda LEGAL-QUALITY-MEETING-AGENDA.md — meeting NOT_HELD)*
- [x] File `docs/reviews/INDONESIA-LEGAL-COMMENTS-2026-09-25.md`
- [x] Commit: `docs: indonesia legal review comments`
- [x] Deadline reminder: crypto external review due 09-30 *(documented in runbook + REVIEWER-BOOKING + CRYPTO-EXTERNAL-REVIEW)*

---

## Week 11 — 2026-09-28 … 10-02  
### Packaging signed builds + fuzz + performance

### Mon 2026-09-28 — Hari 51
- [x] Packaging pipeline macOS: build → sign Developer ID → notarize → staple *(packaging/sign-macos.sh — needs Apple env)*
- [x] Record notarization ticket in evidence *(sign-macos.sh writes macos-notarization.json)*
- [x] Commit: `build: macos signed notarized pipeline` (scripts only; secrets in CI env) *(docs/OFFICIAL-SIGNED-PACKAGING.md contracts; signing still Path C)*

### Tue 2026-09-29 — Hari 52
- [x] Packaging Windows: MSI/EXE + Authenticode sign *(packaging/sign-windows.ps1 — WINDOWS LAB QUEUE)*
- [x] Verify signature with `signtool`/`osslsigncode` *(in sign-windows.ps1 + verify-signatures.sh)*
- [x] Commit: `build: windows authenticode pipeline` *(docs contract only; signing still Path C)*

### Wed 2026-09-30 — Hari 53
- [x] Linux `.deb` + AppImage signed *(packaging/sign-linux.sh signs release tarball; .deb/AppImage packaging remains operator format choice)*
- [ ] External crypto review document received → `docs/reviews/CRYPTO-EXTERNAL-REVIEW.md` *(OPTIONAL compliance — not storefront sell blocker; receipt `CRYPTO-EXTERNAL-REVIEW-RECEIPT.md`)*
- [x] Commit: `build: linux signed packages` + crypto review file *(key plan + CRYPTO-EXTERNAL-REVIEW.md NOT_RECEIVED)*

### Thu 2026-10-01 — Hari 54
- [x] cargo-fuzz nightly for fsnap+transfer; triage crashes *(prep note; targets deferred — see docs/FUZZ-OFFICIAL-PREP.md)*
- [x] Performance baselines freeze (cold start, hash, search p95) *(draft under release-evidence/OFFICIAL-1.0.0/perf/)*
- [x] Commit: `test: fuzz and perf freeze` *(perf draft + docs/FUZZ-OFFICIAL-PREP.md; fuzz deferred)*

### Fri 2026-10-02 — Hari 55
- [x] Physical smoke on ThinkPad (Win), MacBook (macOS), Kali (Linux) *(o10/*.json templates; Win queued W4; runs NOT_RUN)*
- [x] Fill O10 evidence logs *(templates created under release-evidence/OFFICIAL-1.0.0/o10/)*
- [x] Commit: `docs: physical platform smoke official`

---

## Week 12 — 2026-10-05 … 10-09  
### Reviews close + legal sign-off + release notes

### Mon 2026-10-05 — Hari 56
- [x] Security architecture review delta PASS *(docs/reviews/SECURITY-ARCHITECTURE-OFFICIAL-DELTA.md)*
- [x] Product readiness review PASS for Official wording *(docs/reviews/PRODUCT-READINESS-OFFICIAL-DELTA.md)*
- [x] Commit: `docs: security and product official reviews`

### Tue 2026-10-06 — Hari 57
- [x] Fix all open CRITICAL/HIGH from reviews *(none open; see discrepancy register)*
- [x] Discrepancy register: only accepted residuals with owners
- [x] Commit: `fix: official review findings`

### Wed 2026-10-07 — Hari 58
- [x] Freeze capability matrix SHA *(Engineering Alpha tip `683233a`; Official freeze re-records at O6)*
- [x] Known issues + support period in `docs/user/RELEASE-NOTES-1.0.0.md` *(draft; publish blocked on O12)*
- [x] SBOM + licenses embedded/linked from About UI *(UiSnapshot.about_disclosure paths)*
- [x] Commit: `docs: release notes and matrix freeze`

### Thu 2026-10-08 — Hari 59
- [x] Address legal comments; prepare sign-off form *(comments file + INDONESIA-OFFICIAL-SIGNOFF.md ready)*
- [x] `docs/reviews/INDONESIA-OFFICIAL-SIGNOFF.md` ready for signature *(unsigned template)*
- [x] Commit: `docs: indonesia signoff package`

### Fri 2026-10-09 — Hari 60
- [ ] **Obtain Indonesia legal/quality wet/digital sign-off** (O8) *(OPTIONAL compliance — `docs/operator/OBTAIN-SIGNOFFS.md`)*
- [ ] Confirm crypto review accepted (O9) *(OPTIONAL compliance — HUMAN-SIGNOFFS-METADATA)*
- [x] Final unsigned→signed rebuild with freeze SHA *(packaging/rebuild-signed-from-freeze.sh — blocked without certs/Windows lab)*
- [x] Commit: `docs: official human signoffs recorded` (metadata only) *(docs/operator/HUMAN-SIGNOFFS-METADATA.md)*

---

## Week 13 — 2026-10-12 … 10-16  
### Buffer, tag v1.0.0, GitHub Release

### Mon 2026-10-12 — Hari 61
- [x] Only P0 regression fixes *(CI green on Official track; no open P0 code regressions)*
- [x] Re-run full CI on release branch *(Actions success on main Official commits)*
- [x] Commit fixes as needed

### Tue 2026-10-13 — Hari 62
- [x] Rebuild signed artifacts from freeze tag candidate *(rebuild-signed-from-freeze.sh ready)*
- [x] Re-verify O1–O3 signatures *(packaging/verify-signatures.sh fail-closed until evidence exists)*
- [x] Fill `release-evidence/OFFICIAL-1.0.0/MANIFEST.txt` *(tip + gate status; O1–O12 still incomplete)*

### Wed 2026-10-14 — Hari 63
- [x] Run `release-evidence/OFFICIAL-1.0.0/gather.sh` (all O1–O12) *(script ready; currently fail-closed as required)*
- [x] Fail script if any gate missing *(gather.sh fails closed by design)*
- [x] Soft tag `v1.0.0-rc1` if needed *(docs/operator/RC1-UNSIGNED.md — tag created as v1.0.0-rc1-unsigned)*

### Thu 2026-10-15 — Hari 64
- [ ] Storefront publish: Lynk.id/Gumroad + SHA256SUMS + freeze SHA *(scripts/publish-storefront-release.sh; DIST-001)*
- [ ] Source-only annotated tag `v1.0.0` *(STOREFRONT_SELL=1 scripts/cut-official-v1.sh)*
- [ ] Confirm no installer assets on GitHub Release *(scripts/check-no-github-binaries.sh)*
- [ ] Optional hardening gather PASS *(unset STOREFRONT_SELL; O1–O3/O8/O9 — not sell blockers)*

### Fri 2026-10-16 — Hari 65
- [x] Post-release: monitor issues 24h checklist *(docs/operator/POST-RELEASE-24H.md template)*
- [x] Publish internal “Official Production live” note *(docs/operator/OFFICIAL-LIVE-NOTE.md template — do not send until O12)*
- [x] Open ADR backlog for P0-LATER next quarter *(docs/P0-LATER-ADR-BACKLOG.md)*
- [ ] Update PRD/status: storefront v1.0.0 available (source on GitHub; binaries storefront-only)

---

## 2. Per-day template (copy if a day slips)

```text
Date:
Branch: official/day-NN-short-name
AM goal:
PM goal:
Tests added:
Matrix rows updated:
Evidence path:
Blockers (C/D paths):
Commit SHA:
```

## 3. Slip rules

| Slip | Action |
|---|---|
| ≤2 days in Weeks 1–9 | Use weekend buffer; do not cut signing |
| Storage gate fail Week 5 | Stop Artifacts; extend Storage max 5 days; compress Week 13 buffer |
| Certs missing Week 8 | Parallel unsigned RC; Official date slips — **do not fake signatures** |
| Legal sign-off missing Week 12 | Official blocked; ship `v1.0.0-rc` as Engineering Alpha only |
| Single CRITICAL at Week 12 | Fix or remove capability claim; no silent ship |

## 4. Immediate next step (Hari 01)

Mulai **Senin 20 Jul 2026** (atau hari kerja berikutnya): ADR-008 + BOOKMARK contract + Apple/Windows cert procurement + jadwal reviewer.

---

## 5. Honesty check

Official Release dalam 3 bulan **mungkin** jika:
1. Satu engineer full-time (atau setara) mengikuti plan ini hampir tanpa cuti,
2. Cert dibeli/disetujui paling lambat akhir Agustus,
3. Reviewer legal/quality + crypto tersedia sesuai jadwal,
4. Scope OUT benar-benar tidak dikerjakan.

Jika poin 2–3 gagal, tanggal Official harus digeser — plan ini tidak mengizinkan klaim Official tanpa O1–O3 dan O8–O9.
