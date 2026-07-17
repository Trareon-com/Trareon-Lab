# Trareon Lab — Grand Plan 3 Bulan (R1 Engineering Alpha)

> **SUPERSEDED (2026-07-17):** Target diganti ke **Official Production**. Gunakan `docs/superpowers/plans/2026-07-17-trareon-lab-3month-official-release.md` sebagai sumber kebenaran. Dokumen ini dipertahankan hanya sebagai histori Alpha-scoped plan.

> **For agentic workers:** REQUIRED SUB-SKILL: Use `executing-plans` / TDD task loop per daily checkbox. Do not expand into Official Production or Specialist Packs without ADR-008 change control.

**Goal:** Dalam 90 hari kalender (~65 hari kerja), selesaikan **R1 Engineering Alpha** yang defensible: 132 requirement R1 diimplementasi + divalidasi corpus sebatas feasible, DevSecOps CI hijau, UI Examination usable, bookmark + signed transfer package, packaging smoke 3 OS — lalu freeze sebagai lab-use Engineering Alpha.

**Architecture:** Lanjutkan roadmap Gate B: Foundation (done) → Storage → Artifacts (+ Examination UX/bookmark) → thin Volatile (opsional jika buffer) → Validation + Release evidence. Rust core authoritative; Slint shell; D-RUST-INDEX; offline-first; no central collab server.

**Tech Stack:** Rust 2021, Slint, SQLite case sidecar, SHA-256/Ed25519, GitHub Actions, CycloneDX SBOM, synthetic + public corpora.

## Global Constraints

- **Definition of “sempurna” (3 bulan):** R1 Engineering Alpha — **bukan** Official Production, **bukan** 57 P0-LATER, **bukan** Specialist Packs.
- Author/committer: `yusufsaaas <yusuf.shalahuddin@live.com>` — no `cursoragent` trailers.
- One case per process; `.fsnap` no silent repair; AI loopback-only if touched.
- TDD for every crate feature; commit after each green day-slice.
- No production signing keys required for Engineering Alpha (unsigned smoke OK; signed path cert-gated).
- Calendar: **2026-07-20 → 2026-10-17** (13 minggu kerja; Week 13 = buffer/freeze).

---

## A. Audit PRD / RFC / Code / Review (baseline 2026-07-17)

### A.1 Verdict ringkas

| Jalur | Status | Sempurna? | Gap utama |
|---|---|---|---|
| **PRD v1.0** | Baseline terkunci | **Cukup untuk eksekusi** | Bookmark/share belum punya schema kontrak tersendiri; UI flows belum digambar |
| **RFC + Gates A–E** | ACCEPTED | **Cukup** | Teks sudah di-sync; residual Official Production masih terbuka |
| **Code** | Foundation F1–F15 di `main` | **Tidak** — skeleton saja | Belum Storage/Artifacts/bookmark/DevSecOps CI penuh |
| **Review** | Gate E + Foundation code PASS | **Cukup untuk Foundation** | Belum review Storage/Artifacts/UX Examination; belum named legal/crypto reviewers |

### A.2 PRD — apa yang sudah baik / perlu ditambah

**Sudah baik**

- Prinsip defensible lab, typed integrity, coverage, isolation, Indonesia profile, no false ISO claims.
- Bookmark/tag/finding disebut sebagai first-class examination UX (FR-ART-006, NFR keyboard shortcuts).
- Kolaborasi lewat **audited transfer/export**, bukan central server (non-goal eksplisit).
- DevSecOps controls di NFR-SEC-008 (SBOM, SAST, secret scan, signing status).

**Perlu ditambah / dipertegas (Week 1 deliverables)**

1. **`docs/contracts/BOOKMARK.md` + schema** — UUID, target evidence/object/byte-range, author, citation, tags, review state, export eligibility.
2. **`docs/contracts/TRANSFER-PACKAGE.md`** — signed minimized share pack untuk kolaborasi offline (bookmark/finding subset + disclosure preview).
3. **`docs/ux/EXAMINATION-WIREFRAMES.md`** — 7 layar: Case Home, Evidence Browser, Search, Timeline, Bookmark Pane, Review, Report/Export.
4. **Traceability:** FR-ART-006 / keyboard bookmark naik dari `P0-LATER` ke **R1** via ADR-008 change control (kolaborasi share = diferensiator; wajib di Engineering Alpha).
5. **DevSecOps checklist atomik** di PRD atau `docs/DEVSECOPS-PIPELINE.md` yang memetakan NFR-SEC-008 → job CI konkret.

### A.3 RFC — apa yang sudah baik / perlu ditambah

**Sudah baik**

- ADR-001–016 ACCEPTED; phasing Foundation COMPLETE; residual Official Production tercantum.

**Perlu ditambah**

1. Amendment singkat: **R1 Engineering Alpha exit criteria** (link ke plan ini).
2. Catat bahwa **bookmark transfer package** adalah jalur kolaborasi resmi (bukan live multi-user).
3. Update §19 phasing: Storage/Artifacts target dates dari plan ini.

### A.4 Code — inventaris vs gap

**Ada sekarang**

`lab-core`, `lab-case`, `lab-store`, `lab-crypto`, `lab-fsnap`, `lab-worker`, `apps/lab-slint` (UI tipis).

**Belum (harus dibangun di 3 bulan)**

| Area | Crates / artifacts target |
|---|---|
| DevSecOps | CI: audit, secret scan, CycloneDX, fuzz nightly |
| Index production | `crates/lab-index` (D-RUST-INDEX beyond spike) |
| Storage / FS | `crates/lab-storage`, `crates/lab-fs` |
| Artifacts / timeline | `crates/lab-artifacts`, `crates/lab-timeline` |
| Examination | bookmark/finding/note models + Slint panes |
| Transfer | `crates/lab-transfer` signed package export/import |
| Pack host stub | capability broker skeleton (unsigned blocked) |
| Packaging | `packaging/` per OS smoke + release-evidence R1 |
| Volatile (stretch) | thin PCAP metadata **atau** defer ke backlog |

### A.5 Review — gap

| Review | Status | Action 3 bulan |
|---|---|---|
| Gate E product/security/forensic/legal/a11y | PASS | Keep; re-open only on ADR change |
| Foundation code | PASS | Closed |
| Storage method validation | Missing | Week 5–6 |
| Artifacts / timeline validation | Missing | Week 8–9 |
| Examination UX / a11y pass-2 | Missing | Week 4 + Week 10 |
| Transfer-package security | Missing | Week 7 |
| Named Indonesia legal + external crypto | Missing | Schedule Week 12 (can be OUTSIDE Alpha if unpaid) |

### A.6 Explicitly OUT of 3-month “sempurna”

- Official Production signed/notarized installers (needs paid certs)
- 57 P0-LATER (full AI, browser/cloud packs, HFS+/ReFS, full memory profiles, …)
- Specialist Packs milestone
- Central collaborative server / live multi-user editing
- ISO accreditation claims
- Connected threat-intel companion

---

## B. Success criteria — R1 Engineering Alpha (Day 90)

- [ ] Semua **132 R1** requirement: `IMPLEMENTED` + evidence link (unit/corpus as required by matrix)
- [ ] FR-ART-006 bookmark + transfer package: `IMPLEMENTED` + hostile/share tests
- [ ] CI: test + clippy + fmt + `cargo deny`/`audit` + secret scan + CycloneDX artifact
- [ ] UI: Examination wireframes implemented (not just foundation smoke)
- [ ] Packaging smoke Win/macOS/Linux unsigned
- [ ] `release-evidence/R1-ENGINEERING-ALPHA/` lengkap
- [ ] Reviews Storage + Artifacts + Transfer + UX pass-2 = PASS
- [ ] README: **Lab use only / Engineering Alpha** — honest limitations
- [ ] Backlog ADR-008 untuk P0-LATER & Official Production

---

## C. Calendar overview (13 weeks)

| Week | Dates (2026) | Theme | Exit |
|---|---|---|---|
| 1 | Jul 20–24 | Audit closeout + DevSecOps + contracts + UX wireframes | Docs + CI green |
| 2 | Jul 27–31 | Index + provenance/audit persistence + Examination shell IA | Index crate boots |
| 3 | Aug 3–7 | Storage kickoff — raw/dd + hash pipeline | Raw image open |
| 4 | Aug 10–14 | FS R1: NTFS/FAT/exFAT read paths | FS smoke corpus |
| 5 | Aug 17–21 | FS: APFS/ext4 + deleted recovery selected | Storage exit gate |
| 6 | Aug 24–28 | Artifacts Win (Prefetch/LNK/JumpLists) | Win artifacts green |
| 7 | Aug 31–Sep 4 | Artifacts macOS/Linux + Bookmark model + transfer pack | Share pack MVP |
| 8 | Sep 7–11 | Timeline unified + findings/report path | Timeline first paint |
| 9 | Sep 14–18 | Search/index 1M nav + Examination UI polish | 1M nav smoke |
| 10 | Sep 21–25 | Validation corpora + method dossiers R1 | CORPUS_VALIDATED subset |
| 11 | Sep 28–Oct 2 | Packaging 3OS + fuzz + performance baselines | Smoke PASS_3OS |
| 12 | Oct 5–9 | Reviews + discrepancy close + docs freeze | Reviews PASS |
| 13 | Oct 12–16 | Buffer, Alpha freeze, backlog Official Production | Tag `r1-engineering-alpha` |

---

## D. Daily task plan (Mon–Fri)

Legend: tiap hari = desain/test → implementasi → verifikasi → commit. Weekend = buffer opsional / corpus prep (tidak wajib).

### Week 1 — Jul 20–24 — Contracts, DevSecOps, UX

**Mon Jul 20**
- [ ] ADR-008 change: FR-ART-006 + transfer collaboration → R1
- [ ] Draft `docs/contracts/BOOKMARK.md` + `schemas/bookmark.schema.json`
- [ ] Draft `docs/contracts/TRANSFER-PACKAGE.md` + schema
- [ ] Commit: `docs: elevate bookmark/transfer to R1 contracts`

**Tue Jul 21**
- [ ] Write `docs/ux/EXAMINATION-WIREFRAMES.md` (7 screens + keyboard map)
- [ ] Write `docs/DEVSECOPS-PIPELINE.md` mapping NFR-SEC-008 → CI jobs
- [ ] Update TRACEABILITY-INDEX + RELEASE-01 matrix rows
- [ ] Commit: `docs: add examination UX and DevSecOps pipeline map`

**Wed Jul 22**
- [ ] CI: add `cargo deny` / audit job; fail on known HIGH
- [ ] CI: add secret scanning (gitleaks or GitHub native)
- [ ] CI: emit CycloneDX via `cargo cyclonedx` or syft on lockfile
- [ ] Commit: `ci: add audit, secret scan, and SBOM artifacts`

**Thu Jul 23**
- [ ] Failing tests for bookmark schema validate (mirror F6 pattern)
- [ ] Implement schema validation for bookmark + transfer
- [ ] Fixtures valid/invalid under `fixtures/contracts/`
- [ ] Commit: `feat: validate bookmark and transfer package schemas`

**Fri Jul 24**
- [ ] RFC amendment: Engineering Alpha exit + collab-via-transfer
- [ ] Update FOUNDATION checklist → “R1 in progress”
- [ ] Week 1 review checklist (docs only)
- [ ] Commit: `docs: RFC amendment for R1 Engineering Alpha`

### Week 2 — Jul 27–31 — Index + case records + UI IA

**Mon Jul 27**
- [ ] Scaffold `crates/lab-index` (D-RUST-INDEX adapter from spike)
- [ ] Failing test: put/get 10k synthetic rows; reopen durable
- [ ] Commit: `feat: bootstrap lab-index crate`

**Tue Jul 28**
- [ ] Persist provenance/audit/coverage records into case DB
- [ ] Tests: append-only audit; coverage counts update
- [ ] Commit: `feat: persist provenance audit and coverage`

**Wed Jul 29**
- [ ] Slint IA: navigation shell (Case / Evidence / Search / Timeline / Bookmarks / Report)
- [ ] UI model tests for route focus + keyboard primary actions
- [ ] Commit: `feat: examination navigation shell`

**Thu Jul 30**
- [ ] Wire case open → show real coverage/evidence counts from DB
- [ ] Docs shell lists new Examination topics
- [ ] Commit: `feat: bind case metadata to examination shell`

**Fri Jul 31**
- [ ] Performance baseline harness for index put 100k (record, not gate fail)
- [ ] Week 2 integration smoke script
- [ ] Commit: `test: index baseline and week2 smoke`

### Week 3 — Aug 3–7 — Storage raw/dd

**Mon Aug 3**
- [ ] Scaffold `crates/lab-storage` + image open API
- [ ] Failing test: open synthetic raw image; byte-length + sha256
- [ ] Commit: `feat: bootstrap lab-storage`

**Tue Aug 4**
- [ ] Streaming hash (SHA-256) with cancel token (reuse lab-worker)
- [ ] Test: cancel mid-hash; no partial authoritative digest
- [ ] Commit: `feat: cancellable streaming digest`

**Wed Aug 5**
- [ ] Evidence object registration on image import
- [ ] Provenance events for open/hash
- [ ] Commit: `feat: register disk image evidence objects`

**Thu Aug 6**
- [ ] E01/Ex01 read spike (selected subset) behind feature flag
- [ ] Fail-closed on unsupported compression
- [ ] Commit: `feat: limited e01 read path`

**Fri Aug 7**
- [ ] Storage hostile fixtures (truncated image, wrong size)
- [ ] Week 3 review
- [ ] Commit: `test: storage hostile fixtures`

### Week 4 — Aug 10–14 — Filesystems NTFS/FAT/exFAT

**Mon Aug 10**
- [ ] Scaffold `crates/lab-fs`
- [ ] NTFS MFT enumerate smoke on synthetic corpus
- [ ] Commit: `feat: ntfs enumerate smoke`

**Tue Aug 11**
- [ ] FAT32/exFAT enumerate + path resolve
- [ ] Coverage records for unsupported streams
- [ ] Commit: `feat: fat exfat read paths`

**Wed Aug 12**
- [ ] File content read by cluster/runlist; CAS derived store
- [ ] Test: get bytes match fixture
- [ ] Commit: `feat: fs content extraction to cas`

**Thu Aug 13**
- [ ] Index file metadata into lab-index
- [ ] UI: Evidence browser list (virtualized stub OK)
- [ ] Commit: `feat: index fs metadata for evidence browser`

**Fri Aug 14**
- [ ] UX polish pass-1 vs wireframes (Evidence Browser)
- [ ] A11y keyboard traversal checklist
- [ ] Commit: `feat: evidence browser keyboard pass`

### Week 5 — Aug 17–21 — APFS/ext4 + deleted + Storage gate

**Mon Aug 17**
- [ ] ext4 read enumerate (synthetic)
- [ ] Commit: `feat: ext4 read enumerate`

**Tue Aug 18**
- [ ] APFS read enumerate (macOS corpus; Linux best-effort)
- [ ] Document OS-specific limitations honestly
- [ ] Commit: `feat: apfs read enumerate`

**Wed Aug 19**
- [ ] Deleted-file recovery **selected** algorithm for R1 (one method)
- [ ] Label recovered material partial/ambiguous
- [ ] Commit: `feat: r1 deleted recovery selected path`

**Thu Aug 20**
- [ ] Storage method validation dossier draft
- [ ] Corpus governance entries for FS fixtures
- [ ] Commit: `docs: storage validation dossier draft`

**Fri Aug 21**
- [ ] **Storage exit gate review** (`docs/reviews/STORAGE-METHOD-REVIEW.md`)
- [ ] Fix P0 findings same day or park with severity
- [ ] Commit: `docs: storage exit gate`

### Week 6 — Aug 24–28 — Windows artifacts

**Mon Aug 24**
- [ ] Scaffold `crates/lab-artifacts`
- [ ] Prefetch parse synthetic + provenance
- [ ] Commit: `feat: windows prefetch artifact`

**Tue Aug 25**
- [ ] LNK parse + JumpLists selected
- [ ] Bookmarkable artifact hits
- [ ] Commit: `feat: lnk and jumplist artifacts`

**Wed Aug 26**
- [ ] Artifact → timeline event normalization stub
- [ ] Tests for event schema
- [ ] Commit: `feat: artifact to timeline events`

**Thu Aug 27**
- [ ] UI: Artifact results pane + open provenance action
- [ ] Commit: `feat: artifact results ui`

**Fri Aug 28**
- [ ] Windows artifact corpus dossier
- [ ] Week 6 smoke
- [ ] Commit: `test: windows artifact corpus smoke`

### Week 7 — Aug 31–Sep 4 — macOS/Linux artifacts + bookmark transfer

**Mon Aug 31**
- [ ] macOS Unified Logs subset parser (bounded)
- [ ] Commit: `feat: macos unified log subset`

**Tue Sep 1**
- [ ] Linux auth/syslog subset
- [ ] Commit: `feat: linux auth syslog subset`

**Wed Sep 2**
- [ ] Implement bookmark CRUD in case DB + tests
- [ ] UI Bookmark pane (create/list/filter/open citation)
- [ ] Commit: `feat: bookmark crud and pane`

**Thu Sep 3**
- [ ] Scaffold `crates/lab-transfer`
- [ ] Export signed transfer package (Ed25519) with disclosure preview fields
- [ ] Import into second temp case; fail-closed on tamper
- [ ] Commit: `feat: signed bookmark transfer package`

**Fri Sep 4**
- [ ] Transfer security review draft
- [ ] Hostile transfer fixtures (path escape, oversized)
- [ ] Commit: `test: transfer hostile fixtures`

### Week 8 — Sep 7–11 — Timeline + findings + report

**Mon Sep 7**
- [ ] `crates/lab-timeline` merge/sort/dedupe events
- [ ] First-paint budget test (synthetic 100k events — soft)
- [ ] Commit: `feat: unified timeline`

**Tue Sep 8**
- [ ] Finding model: claim ↔ supporting bookmarks/artifacts
- [ ] Separation fact vs hypothesis vs conclusion
- [ ] Commit: `feat: findings with evidence links`

**Wed Sep 9**
- [ ] Report export: HTML/JSON deterministic; draft vs sealed labels
- [ ] Technical review / authorize skeleton states
- [ ] Commit: `feat: examination report export skeleton`

**Thu Sep 10**
- [ ] UI: Timeline + Findings + Report screens
- [ ] Commit: `feat: timeline findings report ui`

**Fri Sep 11**
- [ ] Artifacts exit gate review
- [ ] Commit: `docs: artifacts exit gate`

### Week 9 — Sep 14–18 — Search scale + UI polish

**Mon Sep 14**
- [ ] lab-index: path/name/hash filters + query API
- [ ] Commit: `feat: index query filters`

**Tue Sep 15**
- [ ] UI Search + command palette MVP
- [ ] Keyboard shortcuts from PRD minimum set
- [ ] Commit: `feat: search and command palette`

**Wed Sep 16**
- [ ] 1M-row navigation smoke (synthetic) — record latency
- [ ] Virtualized list in Slint or windowed fetch
- [ ] Commit: `test: 1m row navigation smoke`

**Thu Sep 17**
- [ ] UX polish pass-2 (wireframe delta close)
- [ ] A11y review update
- [ ] Commit: `feat: examination ux polish pass2`

**Fri Sep 18**
- [ ] Integration gate: open case → import → FS → artifact → bookmark → transfer → report
- [ ] Commit: `test: end to end examination smoke`

### Week 10 — Sep 21–25 — Validation dossiers

**Mon Sep 21**
- [ ] Fill method validation records for FS R1 methods
- [ ] Commit: `docs: fs method validation records`

**Tue Sep 22**
- [ ] Artifact method validation records
- [ ] Commit: `docs: artifact method validation records`

**Wed Sep 23**
- [ ] Run public/synthetic corpora; log CORPUS_VALIDATED vs LIMITED
- [ ] Update RELEASE-01 validation columns honestly
- [ ] Commit: `docs: corpus validation status update`

**Thu Sep 24**
- [ ] Second-method verification **hooks** (FR-VAL-009) — even if thin
- [ ] Blind PT participant import/export stub (FR-VAL-010) — participant side only
- [ ] Commit: `feat: validation hooks second-method and blind pt`

**Fri Sep 25**
- [ ] Forensic method review pass-2
- [ ] Commit: `docs: forensic method review pass2`

### Week 11 — Sep 28–Oct 2 — Packaging, fuzz, performance

**Mon Sep 28**
- [ ] Packaging smoke macOS + Linux scripts
- [ ] Commit: `build: packaging smoke posix`

**Tue Sep 29**
- [ ] Packaging smoke Windows script + docs COPY-PASTE
- [ ] Commit: `build: packaging smoke windows`

**Wed Sep 30**
- [ ] cargo-fuzz targets for fsnap + transfer (nightly CI)
- [ ] Commit: `test: fuzz fsnap and transfer`

**Thu Oct 1**
- [ ] Performance baselines: cold start, case open, hash throughput, search p95
- [ ] Record in `docs/PERFORMANCE-BENCHMARK-PROTOCOL.md` results folder
- [ ] Commit: `docs: r1 performance baselines`

**Fri Oct 2**
- [ ] `release-evidence/R1-ENGINEERING-ALPHA/gather.sh`
- [ ] Commit: `docs: r1 engineering alpha evidence gather`

### Week 12 — Oct 5–9 — Reviews + closeout

**Mon Oct 5**
- [ ] Security architecture review delta (transfer, packs stub)
- [ ] Commit: `docs: security review r1 delta`

**Tue Oct 6**
- [ ] Product readiness + Indonesia legal delta (honest Alpha claims)
- [ ] Commit: `docs: product and legal review r1 delta`

**Wed Oct 7**
- [ ] Discrepancy register: close or explicitly defer with owners
- [ ] Commit: `docs: r1 discrepancy register`

**Thu Oct 8**
- [ ] README + user guide Alpha freeze
- [ ] Capability matrix final R1 statuses
- [ ] Commit: `docs: alpha freeze readme and matrix`

**Fri Oct 9**
- [ ] Tag preparation checklist; no Official Production claims
- [ ] Schedule named reviewers (external) as **post-Alpha** tasks
- [ ] Commit: `docs: engineering alpha freeze checklist`

### Week 13 — Oct 12–16 — Buffer & tag

**Mon Oct 12**
- [ ] Fix only P0/P1 regressions from Week 12 reviews
- [ ] Commit as needed

**Tue Oct 13**
- [ ] Cross-OS retest packaging smoke
- [ ] Update evidence pack

**Wed Oct 14**
- [ ] Cut tag `r1-engineering-alpha` on main (author yusufsaaas)
- [ ] Push tag

**Thu Oct 15**
- [ ] Write Official Production backlog plan (certs, legal, P0-LATER)
- [ ] Commit: `docs: official production backlog after alpha`

**Fri Oct 16**
- [ ] Retro: velocity vs plan; adjust Q4 if needed
- [ ] Close 3-month program

---

## E. Daily operating rhythm (every working day)

1. Pull `main`; branch `r1/day-YYYYMMDD` or weekly feature branch.
2. Write failing test for today’s checkbox slice.
3. Implement minimal; keep offline/forensic constraints.
4. `cargo test --workspace` (+ UI `--no-default-features` if needed).
5. Update matrix/evidence if status changes.
6. Commit as `yusufsaaas`; strip Cursor co-author trailers.
7. Push PR or merge per repo policy (prefer PR for R1 code).

---

## F. Risk register (3-month)

| Risk | Impact | Mitigation |
|---|---|---|
| Single developer bandwidth | Slip Storage/Artifacts | Protect Week 13 buffer; cut Volatile entirely if late |
| Disk/corpus access | Validation stays LIMITED | Prefer synthetic + public; mark honest LIMITED |
| Signing certs unavailable | Cannot Official Production | Already out of Alpha scope |
| Slint 1M UI perf | Nav fails | Windowed fetch; soft budget; document |
| Scope creep (AI, browser packs) | Miss Alpha | ADR-008 hard gate; say no |

---

## G. Immediate next action (today / next business day)

1. Approve this plan’s **definition of sempurna = R1 Engineering Alpha**.
2. Start **Week 1 Mon**: ADR-008 + BOOKMARK + TRANSFER contracts.
3. Open the companion canvas for program tracking.

---

## H. Spec self-review

- No TBD placeholders for Alpha exit criteria.
- Official Production explicitly deferred.
- Daily tasks are independently testable slices.
- Bookmark collaboration modeled as signed transfer (matches PRD non-goals).
