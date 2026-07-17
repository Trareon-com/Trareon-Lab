# Checklist — Day 1–5 (zero-cost sellable)

Plan: `docs/superpowers/plans/2026-07-17-trareon-lab-daily-sellable-zero-cost.md`

## Day 1 — Sellable framing

- [x] README: offline lab product, OS matrix, unsigned sell path
- [x] `docs/SELLING-UNSIGNED.md`
- [x] FR-ART-006 bookmark/transfer elevated to R1 (matrix + ADR-008 note)

## Day 2 — Contracts

- [x] `docs/contracts/BOOKMARK.md`
- [x] `docs/contracts/TRANSFER-PACKAGE.md`
- [x] `schemas/bookmark.schema.json`
- [x] `schemas/transfer-package.schema.json`
- [x] Valid/invalid fixtures under `fixtures/contracts/`

## Day 3 — CI hardening (gratis)

- [x] Existing test + clippy + fmt jobs retained
- [x] `cargo audit` job in `.github/workflows/ci.yml`
- [x] SBOM from `cargo metadata` uploaded as CI artifact

## Day 4 — Validation + UX sketch

- [x] `lab-core` validates Bookmark + TransferPackage
- [x] Fixture tests for valid/invalid classified failures
- [x] `docs/ux/EXAMINATION-WIREFRAMES.md` (7 screens)

## Day 5 — Packaging

- [x] `./packaging/smoke.sh` documented and runnable
- [x] `docs/PACKAGING-UNSIGNED-BUILDS.md` (3 OS)
- [x] This checklist complete

## Exit gate

Days 1–5 complete when commits land and CI stays green. Next: Day 6 (`lab-index`).
