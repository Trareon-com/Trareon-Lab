# Lab Core Perfect — Definition of Done

**Branch:** `cursor/lab-presale-hardening`  
**Success:** Lab Core Perfect = M12 gate green only.

## Reuse (do not rebuild)

| Asset | Path |
|-------|------|
| CaseDb / ledgers / lock | `crates/lab-case` |
| fsnap preflight/import | `crates/lab-fsnap` |
| Raw/E01 ImageReader | `crates/lab-storage` |
| Progress/cancel | `crates/lab-core/src/progress.rs` |
| FS parsers | `crates/lab-fs` |
| Transfer Ed25519 | `crates/lab-transfer` |
| Export skeleton | `crates/lab-core/src/export.rs` |
| Index paging | `crates/lab-index` |
| Second-method / PT hooks | `crates/lab-case/src/validation_hooks.rs` |

## CASE/UCO profile pin

- **Profile ID:** `trareon-case-uco-2026.07`
- **Constant:** `lab_core::CASE_UCO_PROFILE_VERSION`

## Golden / fixture inventory

| Fixture | Purpose |
|---------|---------|
| `fixtures/contracts/*.json` | Schema/hostile contract tests |
| `crates/lab-fs` mini NTFS/FAT/ext4 images (tests) | FS golden |
| `crates/lab-storage` E01 roundtrip tests | E01 golden |
| `crates/lab-transfer` tamper tests | Transfer Invalid |
| Empty CaseDb (temp) | Honesty: zero counts |

## Perf budgets

| Metric | Budget |
|--------|--------|
| Case open → interactive | ≤ 2s (≤1k rows) |
| Evidence page flip (200) | ≤ 100ms |
| Hex 512 B | ≤ 50ms |
| Search first page | ≤ 1s |

## Fuzz targets (prep)

- `.fsnap` verify — see `docs/FUZZ-OFFICIAL-PREP.md`
- Transfer envelope parse
- No cargo-fuzz target is checked in as of 2026-07-18. Run the bounded
  adversarial substitute in `docs/FUZZ-SMOKE.md`; record it as smoke only, not
  fuzz-complete evidence.

## Golden CI plan

1. Run workspace formatting and `cargo test --workspace`.
2. Run contract fixtures listed in
   `docs/validation/GOLDEN-CORPUS-INVENTORY.md`.
3. Run `cargo test -p lab-index --test search_operators` and
   `cargo test -p lab-index --test persist_10k`.
4. Run `scripts/check-lab-core-perfect-gate.sh`.
5. Generate `packaging/SHA256SUMS` with `scripts/gen-sha256sums.sh`; archive
   test logs and sums with release evidence.

Golden failures block distribution. Updating a golden requires a reviewed
reason and must not merely accept unexpected output.

## Security checklist and sign-off

- [x] `demo_seed` defaults off and is enforced by the Perfect static gate.
- [x] Positive court-ready claims are rejected; explicit **NOT court-ready**
  disclosures remain allowed.
- [x] CASE/UCO profile constant exists and is pinned.
- [x] Search limits disclose partial coverage instead of silently truncating.
- [x] Hostile contract tests and bounded fuzz smoke are inventoried.
- [x] Packaging/release evidence can be covered by SHA-256 inventory.

**Engineering Alpha sign-off:** 2026-07-18. This sign-off covers the M7
engineering gate implementation, not accreditation, legal admissibility,
independent validation, signing, notarization, or the M12 Perfect release gate.

## Store listing anti-claim

Every listing must say: **Engineering Alpha; UNSIGNED; Lab use only; NOT
court-ready; NOT ISO-certified.** It must not imply accreditation, forensic
admissibility, complete search coverage after truncation, or production signing.

## Perfect gate tracking

`scripts/check-lab-core-perfect-gate.sh` is the repeatable static preflight.
Passing it is necessary but not sufficient for **Lab Core Perfect**. “Perfect”
remains tracked only by the M12 gate after golden CI, performance evidence,
security review, release-evidence hashes, and all milestone exit checks are
green.

## Owners (hats may combine)

| Role | Focus |
|------|-------|
| Engineering | M0–M12 |
| Validation | Golden/dossiers |
| Security | Fuzz/hostile |
| Docs | Guides Present/Missing |

## Feature flags

| Flag | Default | Meaning |
|------|---------|---------|
| `demo_seed` | **off** | Fabricated case/search/import — must stay off for Perfect |
| `gui` | on | Slint UI |
| `local_ai` | **off** | Ollama/LM Studio bridge |

## Milestone exit tracking

- [x] M0 Honesty
- [x] M1 Wire shell
- [x] M2 Trust engines
- [x] M3 Examine
- [x] M4 Report/export
- [x] M5 Lab quality
- [x] M6 Docs/AI/live
- [x] M7 Dist gates
- [x] M8 APFS+E01 Validated
- [x] M9 YARA+hash Validated
- [x] M10 Search+scale Validated
- [x] M11 PDF/A+CASE Validated
- [x] M12 Lab Core Perfect freeze
