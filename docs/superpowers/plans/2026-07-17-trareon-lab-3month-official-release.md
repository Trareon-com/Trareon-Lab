# Trareon Lab — Official Release Grand Plan (3 Bulan, Harian)

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
- [ ] Tulis ADR-008 change control: FR-ART-006 bookmark + transfer collaboration → **R1**; sebutkan Official R1.0.0 target date 2026-10-16
- [ ] Update `docs/RELEASE-01-CAPABILITY-MATRIX.md` baris FR-ART-006 → R1
- [ ] Update `docs/TRACEABILITY-INDEX.md` + `docs/DECISION-REGISTER.md` (ADR-008 amendment row)
- [ ] Buat `docs/contracts/BOOKMARK.md` (field wajib: bookmark_uuid, case_uuid, target_kind, target_ref, byte_range?, citation, author_role, created_at_utc, tags[], review_state)
- [ ] Buat `schemas/bookmark.schema.json` (JSON Schema 2020-12)

**PM (procurement + legal booking)**
- [ ] Buat `docs/OFFICIAL-RELEASE-RUNBOOK.md` berisi DoD O1–O12 + owner
- [ ] Daftarkan / perpanjang **Apple Developer Program** (catat invoice + Team ID di runbook; jangan commit secret)
- [ ] Ajukan / beli **Windows Authenticode** (prefer EV); catat vendor + ETA
- [ ] Buat GPG/age key plan untuk Linux package signing (hanya public key di repo)
- [ ] Kirim undangan jadwal ke **qualified Indonesia legal/quality reviewer** (deadline review draft: 2026-09-25; sign-off: 2026-10-09)
- [ ] Kirim undangan **external crypto review** (deadline: 2026-09-30)
- [ ] Commit: `docs: start official r1.0.0 program and bookmark contract`

**Verify:** `rg -n "FR-ART-006|R1" docs/RELEASE-01-CAPABILITY-MATRIX.md` menunjukkan R1; runbook exists.

---

### Tue 2026-07-21 — Hari 02

**AM**
- [ ] `docs/contracts/TRANSFER-PACKAGE.md` + `schemas/transfer-package.schema.json` (disclosure preview, destination, purpose, authority, selected bookmarks/findings digests, signature envelope)
- [ ] Fixtures: `fixtures/contracts/bookmark.valid.json` + 3 invalid; `transfer.valid.json` + 3 invalid
- [ ] `docs/DEVSECOPS-PIPELINE.md`: map setiap bullet NFR-SEC-008 → job CI + artifact path

**PM**
- [ ] `docs/ux/EXAMINATION-WIREFRAMES.md`: 7 layar + keyboard map (PRD minimum shortcuts)
- [ ] `docs/ux/DESIGN-TOKENS.md`: warna status bukan-warna-saja (icon+text+severity)
- [ ] Update RFC §19 phasing + Official R1.0.0 exit
- [ ] Commit: `docs: transfer contract, DevSecOps map, examination wireframes`

**Verify:** schema `$schema` 2020-12; fixtures parse as JSON.

---

### Wed 2026-07-22 — Hari 03

**AM — CI hardening**
- [ ] Modify `.github/workflows/ci.yml`: job `deny` (`cargo deny check` atau `cargo audit`)
- [ ] Job `secrets` (gitleaks action SHA-pinned)
- [ ] Job `sbom` → upload CycloneDX artifact
- [ ] Job `sast` (Clippy `-D warnings` sudah ada; tambah `cargo geiger` optional warn-only)

**PM**
- [ ] `deny.toml` / audit allowlist policy documented
- [ ] Fail CI on HIGH vuln baru
- [ ] Commit: `ci: official-grade audit secret scan and sbom`
- [ ] Push; pastikan Actions hijau di PR

**Verify:** Actions run hijau; SBOM artifact terunduh.

---

### Thu 2026-07-23 — Hari 04

**AM — schema code**
- [ ] Failing tests `crates/lab-core/tests/bookmark_schema.rs`
- [ ] Extend `schema_validate.rs` untuk `SchemaKind::Bookmark` + `TransferPackage`
- [ ] Green tests

**PM**
- [ ] Generate `release-evidence/OFFICIAL-1.0.0/README.md` skeleton folders
- [ ] Checklist O1–O12 sebagai markdown gates (unchecked)
- [ ] Commit: `feat: validate bookmark and transfer schemas`

**Verify:** `cargo test -p lab-core bookmark_schema transfer`

---

### Fri 2026-07-24 — Hari 05

**AM**
- [ ] Status call Path C: cert ETA written in runbook
- [ ] Status call Path D: reviewer confirmed dates
- [ ] Jika cert ETA > 2026-09-15 → escalate (alternate vendor) hari ini

**PM**
- [ ] Week 1 integration doc review (self)
- [ ] Update PRD status line: `Official R1.0.0 program active`
- [ ] Commit: `docs: week1 official kickoff complete`
- [ ] Tag interim `program/official-w1` (optional)

---

## Week 2 — 2026-07-27 … 07-31  
### Index production + case records + Examination IA

### Mon 2026-07-27 — Hari 06
- [ ] Create `crates/lab-index/` from spike patterns (`spikes/lab-spike-index`)
- [ ] Failing test: durable put/get 10_000 synthetic metadata rows after reopen
- [ ] Implement minimal disk-backed index
- [ ] Wire workspace `Cargo.toml` member
- [ ] Commit: `feat: bootstrap production lab-index`

### Tue 2026-07-28 — Hari 07
- [ ] Persist provenance/audit/coverage tables in `lab-case` migrations `002_*.sql`
- [ ] Tests: append-only audit; coverage counters
- [ ] API: `append_audit`, `upsert_coverage`, `list_provenance`
- [ ] Commit: `feat: persist audit provenance coverage`

### Wed 2026-07-29 — Hari 08
- [ ] Slint: nav shell routes Case/Evidence/Search/Timeline/Bookmarks/Report
- [ ] `apps/lab-slint/ui/app.slint` + `nav.slint`
- [ ] UI model tests for route focus
- [ ] Commit: `feat: examination navigation shell`

### Thu 2026-07-30 — Hari 09
- [ ] Bind open-case → real DB counts in UI
- [ ] Offline docs index lists Examination topics
- [ ] Commit: `feat: bind case db to examination shell`

### Fri 2026-07-31 — Hari 10
- [ ] Index baseline 100k put (record latency JSON under `spikes/results/` or `release-evidence/.../perf/`)
- [ ] Week 2 smoke script `scripts/smoke-week2.sh`
- [ ] Commit: `test: week2 index and shell smoke`
- [ ] Path C check-in (certs)

---

## Week 3 — 2026-08-03 … 08-07  
### Storage: raw/dd + digest + E01 subset

### Mon 2026-08-03 — Hari 11
- [ ] Create `crates/lab-storage/`
- [ ] Failing test: open synthetic raw image → byte_length + sha256
- [ ] `ImageHandle::open_raw(path) -> LabResult<ImageMeta>`
- [ ] Commit: `feat: lab-storage raw open`

### Tue 2026-08-04 — Hari 12
- [ ] Streaming SHA-256 with `lab-worker` cancel
- [ ] Test: cancel → no authoritative digest published
- [ ] Commit: `feat: cancellable streaming image digest`

### Wed 2026-08-05 — Hari 13
- [ ] Register evidence object + provenance on import
- [ ] UI: import disk image action (file picker stub OK)
- [ ] Commit: `feat: register disk image evidence`

### Thu 2026-08-06 — Hari 14
- [ ] E01/Ex01 read feature-flagged (`ewf` optional dep or custom subset)
- [ ] Fail-closed unsupported compression → coverage `Unsupported`
- [ ] Commit: `feat: limited e01 read path`

### Fri 2026-08-07 — Hari 15
- [ ] Hostile storage fixtures (truncated, size mismatch)
- [ ] Method validation draft `docs/validation/methods/storage-raw.md`
- [ ] Commit: `test: storage hostile and validation draft`
- [ ] Path C/D check-in

---

## Week 4 — 2026-08-10 … 08-14  
### Filesystems NTFS / FAT / exFAT

### Mon 2026-08-10 — Hari 16
- [ ] Create `crates/lab-fs/`
- [ ] NTFS MFT enumerate on synthetic corpus under `validation/synthetic/fs/ntfs/`
- [ ] Commit: `feat: ntfs enumerate`

### Tue 2026-08-11 — Hari 17
- [ ] FAT32 + exFAT enumerate + path resolve
- [ ] Coverage for unsupported Alternate Data Streams / etc.
- [ ] Commit: `feat: fat exfat enumerate`

### Wed 2026-08-12 — Hari 18
- [ ] Content extraction → CAS (`lab-store`)
- [ ] Test byte identity vs fixture
- [ ] Commit: `feat: fs content to cas`

### Thu 2026-08-13 — Hari 19
- [ ] Index FS metadata into `lab-index`
- [ ] Evidence browser list UI (windowed)
- [ ] Commit: `feat: evidence browser fs list`

### Fri 2026-08-14 — Hari 20
- [ ] Keyboard a11y pass Evidence Browser vs wireframe
- [ ] Update matrix FS rows toward UNIT_VERIFIED
- [ ] Commit: `feat: evidence browser a11y`
- [ ] Confirm Apple/Windows cert order status

---

## Week 5 — 2026-08-17 … 08-21  
### APFS / ext4 / deleted recovery / Storage exit

### Mon 2026-08-17 — Hari 21
- [ ] ext4 enumerate + read (synthetic)
- [ ] Commit: `feat: ext4 read`

### Tue 2026-08-18 — Hari 22
- [ ] APFS enumerate (primary on macOS runner/device)
- [ ] Document Linux APFS limitation honestly (`Limited`)
- [ ] Commit: `feat: apfs read`

### Wed 2026-08-19 — Hari 23
- [ ] One R1 deleted-recovery method; label partial/ambiguous
- [ ] Tests + coverage reason codes
- [ ] Commit: `feat: r1 deleted recovery`

### Thu 2026-08-20 — Hari 24
- [ ] Complete Storage validation dossiers for claimed methods
- [ ] User guide chapter `docs/user/filesystems.md`
- [ ] Commit: `docs: storage validation dossiers`

### Fri 2026-08-21 — Hari 25
- [ ] Write `docs/reviews/STORAGE-METHOD-REVIEW.md` — verdict PASS/FAIL
- [ ] Fix all CRITICAL/HIGH same day or block Official
- [ ] Commit: `docs: storage exit gate for official`
- [ ] **Gate:** Storage exit PASS required to start Artifacts

---

## Week 6 — 2026-08-24 … 08-28  
### Windows artifacts

### Mon 2026-08-24 — Hari 26
- [ ] Create `crates/lab-artifacts/`
- [ ] Prefetch parser + provenance + tests
- [ ] Commit: `feat: windows prefetch`

### Tue 2026-08-25 — Hari 27
- [ ] LNK + JumpLists (selected)
- [ ] Bookmarkable hit IDs
- [ ] Commit: `feat: lnk jumplist`

### Wed 2026-08-26 — Hari 28
- [ ] Normalize to timeline event schema
- [ ] Commit: `feat: artifact timeline events`

### Thu 2026-08-27 — Hari 29
- [ ] UI Artifact results + Open Provenance
- [ ] Commit: `feat: artifact results ui`

### Fri 2026-08-28 — Hari 30
- [ ] Windows artifact corpus + dossier
- [ ] Commit: `docs: windows artifact validation`
- [ ] Path C: certs must be **in hand** or escalate to management decision

---

## Week 7 — 2026-08-31 … 09-04  
### macOS/Linux artifacts + bookmark + transfer

### Mon 2026-08-31 — Hari 31
- [ ] macOS Unified Logs subset (bounded window)
- [ ] Commit: `feat: macos unified log subset`

### Tue 2026-09-01 — Hari 32
- [ ] Linux auth.log / syslog subset
- [ ] Commit: `feat: linux auth syslog`

### Wed 2026-09-02 — Hari 33
- [ ] Bookmark CRUD in case DB + migration
- [ ] UI Bookmark pane (create/list/filter/open citation)
- [ ] Tests
- [ ] Commit: `feat: bookmark crud and ui`

### Thu 2026-09-03 — Hari 34
- [ ] Create `crates/lab-transfer/`
- [ ] Export transfer package + Ed25519 envelope
- [ ] Import to second case; tamper → `INTEGRITY_FAILED`
- [ ] Disclosure preview fields enforced
- [ ] Commit: `feat: signed transfer package`

### Fri 2026-09-04 — Hari 35
- [ ] Hostile transfer fixtures
- [ ] `docs/reviews/TRANSFER-SECURITY-REVIEW.md`
- [ ] Commit: `test: transfer security review`
- [ ] Send **draft build** + docs pack to legal/quality reviewers (Path D)

---

## Week 8 — 2026-09-07 … 09-11  
### Timeline + findings + report authorization

### Mon 2026-09-07 — Hari 36
- [ ] `crates/lab-timeline/` merge/sort/dedupe
- [ ] Soft perf test 100k events
- [ ] Commit: `feat: unified timeline`

### Tue 2026-09-08 — Hari 37
- [ ] Finding model: claim ↔ bookmarks/artifacts; fact/hypothesis/conclusion separation
- [ ] Commit: `feat: findings model`

### Wed 2026-09-09 — Hari 38
- [ ] Report HTML/JSON deterministic; draft vs sealed; author/reviewer/authorizer states
- [ ] Amendment creates new report version
- [ ] Commit: `feat: official report lifecycle skeleton`

### Thu 2026-09-10 — Hari 39
- [ ] UI Timeline + Findings + Report screens
- [ ] Commit: `feat: timeline findings report ui`

### Fri 2026-09-11 — Hari 40
- [ ] `docs/reviews/ARTIFACTS-METHOD-REVIEW.md` PASS
- [ ] User guides: artifacts, timeline, reporting
- [ ] Commit: `docs: artifacts exit gate official`

---

## Week 9 — 2026-09-14 … 09-18  
### Search scale + command palette + E2E

### Mon 2026-09-14 — Hari 41
- [ ] Index query: path/name/hash filters
- [ ] Commit: `feat: index query api`

### Tue 2026-09-15 — Hari 42
- [ ] Search UI + command palette + PRD shortcuts
- [ ] Commit: `feat: search command palette`

### Wed 2026-09-16 — Hari 43
- [ ] 1M-row navigation smoke; record p95
- [ ] If fail: windowed fetch fix same day
- [ ] Commit: `test: 1m navigation official baseline`

### Thu 2026-09-17 — Hari 44
- [ ] UX polish vs wireframes; a11y review update `docs/reviews/ACCESSIBILITY-AND-UX-REVIEW.md`
- [ ] Commit: `feat: examination ux official polish`

### Fri 2026-09-18 — Hari 45
- [ ] E2E script: case→fsnap→image→fs→artifact→bookmark→transfer→report
- [ ] Commit: `test: official e2e examination smoke`
- [ ] Path C: **signing dry-run** on unsigned artifacts (tooling ready)

---

## Week 10 — 2026-09-21 … 09-25  
### Validation dossiers + second-method + blind PT + legal draft review

### Mon 2026-09-21 — Hari 46
- [ ] Finalize all FS method dossiers; promote eligible → `Validated`
- [ ] Enforce FR-DOC-002: no Validated without guide
- [ ] Commit: `docs: fs validated dossiers`

### Tue 2026-09-22 — Hari 47
- [ ] Artifact method dossiers → `Validated`/`Limited`
- [ ] Commit: `docs: artifact validated dossiers`

### Wed 2026-09-23 — Hari 48
- [ ] Run corpora; update RELEASE-01 validation columns
- [ ] Preserve raw test logs under `release-evidence/OFFICIAL-1.0.0/corpora/`
- [ ] Commit: `docs: corpus validation freeze prep`

### Thu 2026-09-24 — Hari 49
- [ ] Second-method verification workflow (FR-VAL-009) usable in UI
- [ ] Blind PT participant import/export (FR-VAL-010) — participant only
- [ ] Commit: `feat: second-method and blind pt participant`

### Fri 2026-09-25 — Hari 50
- [ ] **Legal/quality review meeting** — collect comments
- [ ] File `docs/reviews/INDONESIA-LEGAL-COMMENTS-2026-09-25.md`
- [ ] Commit: `docs: indonesia legal review comments`
- [ ] Deadline reminder: crypto external review due 09-30

---

## Week 11 — 2026-09-28 … 10-02  
### Packaging signed builds + fuzz + performance

### Mon 2026-09-28 — Hari 51
- [ ] Packaging pipeline macOS: build → sign Developer ID → notarize → staple
- [ ] Record notarization ticket in evidence
- [ ] Commit: `build: macos signed notarized pipeline` (scripts only; secrets in CI env)

### Tue 2026-09-29 — Hari 52
- [ ] Packaging Windows: MSI/EXE + Authenticode sign
- [ ] Verify signature with `signtool`/`osslsigncode`
- [ ] Commit: `build: windows authenticode pipeline`

### Wed 2026-09-30 — Hari 53
- [ ] Linux `.deb` + AppImage signed
- [ ] External crypto review document received → `docs/reviews/CRYPTO-EXTERNAL-REVIEW.md`
- [ ] Commit: `build: linux signed packages` + crypto review file

### Thu 2026-10-01 — Hari 54
- [ ] cargo-fuzz nightly for fsnap+transfer; triage crashes
- [ ] Performance baselines freeze (cold start, hash, search p95)
- [ ] Commit: `test: fuzz and perf freeze`

### Fri 2026-10-02 — Hari 55
- [ ] Physical smoke on ThinkPad (Win), MacBook (macOS), Kali (Linux)
- [ ] Fill O10 evidence logs
- [ ] Commit: `docs: physical platform smoke official`

---

## Week 12 — 2026-10-05 … 10-09  
### Reviews close + legal sign-off + release notes

### Mon 2026-10-05 — Hari 56
- [ ] Security architecture review delta PASS
- [ ] Product readiness review PASS for Official wording
- [ ] Commit: `docs: security and product official reviews`

### Tue 2026-10-06 — Hari 57
- [ ] Fix all open CRITICAL/HIGH from reviews
- [ ] Discrepancy register: only accepted residuals with owners
- [ ] Commit: `fix: official review findings`

### Wed 2026-10-07 — Hari 58
- [ ] Freeze capability matrix SHA
- [ ] Known issues + support period in `docs/user/RELEASE-NOTES-1.0.0.md`
- [ ] SBOM + licenses embedded/linked from About UI
- [ ] Commit: `docs: release notes and matrix freeze`

### Thu 2026-10-08 — Hari 59
- [ ] Address legal comments; prepare sign-off form
- [ ] `docs/reviews/INDONESIA-OFFICIAL-SIGNOFF.md` ready for signature
- [ ] Commit: `docs: indonesia signoff package`

### Fri 2026-10-09 — Hari 60
- [ ] **Obtain Indonesia legal/quality wet/digital sign-off** (O8)
- [ ] Confirm crypto review accepted (O9)
- [ ] Final unsigned→signed rebuild with freeze SHA
- [ ] Commit: `docs: official human signoffs recorded` (metadata only)

---

## Week 13 — 2026-10-12 … 10-16  
### Buffer, tag v1.0.0, GitHub Release

### Mon 2026-10-12 — Hari 61
- [ ] Only P0 regression fixes
- [ ] Re-run full CI on release branch
- [ ] Commit fixes as needed

### Tue 2026-10-13 — Hari 62
- [ ] Rebuild signed artifacts from freeze tag candidate
- [ ] Re-verify O1–O3 signatures
- [ ] Fill `release-evidence/OFFICIAL-1.0.0/MANIFEST.txt`

### Wed 2026-10-14 — Hari 63
- [ ] Run `release-evidence/OFFICIAL-1.0.0/gather.sh` (all O1–O12)
- [ ] Fail script if any gate missing
- [ ] Soft tag `v1.0.0-rc1` if needed

### Thu 2026-10-15 — Hari 64
- [ ] Final gather PASS
- [ ] Annotated tag `v1.0.0` as `yusufsaaas`
- [ ] GitHub Release: attach signed installers + SBOM + matrix + known issues
- [ ] Push tag + release

### Fri 2026-10-16 — Hari 65
- [ ] Post-release: monitor issues 24h checklist
- [ ] Publish internal “Official Production live” note
- [ ] Open ADR backlog for P0-LATER next quarter
- [ ] Close program; update PRD status: `Official Production 1.0.0 released`

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
