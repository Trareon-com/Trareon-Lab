# Foundation Code Review

**Date:** 2026-07-17  
**Scope:** Foundation milestone F1–F15 on branch merge to `main`  
**Verdict:** PASS — no critical or high findings remain open

## Scope reviewed

- Workspace: `Cargo.toml`, `rust-toolchain.toml`, `.github/workflows/ci.yml`
- Crates: `lab-core`, `lab-case`, `lab-store`, `lab-crypto`, `lab-fsnap`, `lab-worker`
- App: `apps/lab-slint` (UI model + optional `gui` feature)
- Packaging smoke: `packaging/smoke.sh`
- Release evidence checklist: `release-evidence/FOUNDATION/gather.sh`

## Findings

| ID | Severity | Status | Note |
|---|---|---|---|
| FCR-001 | LOW | CLOSED | Slint `gui` feature optional so CI/disk-constrained hosts can run `--no-default-features` UI model tests |
| FCR-002 | LOW | CLOSED | `.fsnap` Foundation layout is directory+manifest synthetic; binary Acquire package wire format remains R1 hardening |
| FCR-003 | INFO | ACCEPTED | Production signing/notarization remains cert-gated (ADR-009) |

## Verification run

- `cargo test --workspace --exclude lab-slint`
- `cargo test -p lab-slint --no-default-features`
- `./packaging/smoke.sh`
- `./release-evidence/FOUNDATION/gather.sh` (after local test artifacts)

## Residual risks (not blocking Foundation)

- Index 100M replicate on Windows/Kali still recommended before claiming cross-OS search budgets
- Named external cryptography review before Official Production signing keys
- R1 parsers and corpus validation not in this milestone
