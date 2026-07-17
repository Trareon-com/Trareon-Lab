# Trareon Lab Foundation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `executing-plans` / TDD task loop. Do not expand into Storage/Artifacts milestones here.

**Goal:** Implement the Foundation milestone only: toolchain, case isolation, content-addressed store, evidence/provenance/audit/coverage, `.fsnap` preflight/import, bounded workers, minimal Slint UI, deterministic export skeleton, docs shell, packaging smoke, and release-gate evidence.

**Architecture:** Authoritative Rust core (`lab-core`), purpose-built Rust index adapter (`lab-index`), Slint presentation (`lab-slint`), one-case-per-process locks, sandboxed pack host stub (no hostile parsers yet).

**Tech Stack:** Rust 2021 stable; Slint; JSON Schema 2020-12; SHA-256; Ed25519 envelopes; Cargo workspace.

## Repository layout (exact)

```
crates/lab-core/
crates/lab-case/
crates/lab-store/
crates/lab-fsnap/
crates/lab-index/
crates/lab-crypto/
crates/lab-worker/
apps/lab-slint/
docs/ (existing planning docs remain authoritative)
```

---

### Task F1: Workspace / toolchain / CI bootstrap

**Files:** `Cargo.toml`, `.github/workflows/ci.yml`, `rust-toolchain.toml`, `crates/lab-core/src/lib.rs`

1. **Write the failing test:** `crates/lab-core/tests/workspace_boots.rs` asserts `lab_core::version()` returns semver string.
2. **Run test to verify it fails**
3. **Implement minimal:** version constant + CI job `cargo test --workspace`
4. **Run test to verify it passes**
5. **Commit:** `build: bootstrap Trareon Lab foundation workspace`

### Task F2: Typed error and result model

**Files:** `crates/lab-core/src/error.rs`, `crates/lab-core/src/result.rs`

1. **Write the failing test:** map `LabError::FsNapRejected` to stable error code string.
2. **Run test to verify it fails**
3. **Implement minimal:** typed errors with codes; no stringly API for trust states.
4. **Run test to verify it passes**
5. **Commit:** `feat: add foundation typed error model`

### Task F3: Versioned case database and migration harness

**Files:** `crates/lab-case/src/db.rs`, `crates/lab-case/migrations/`

1. **Write the failing test:** open empty case dir, migrate v0→v1, reopen.
2. **Run test to verify it fails**
3. **Implement minimal:** SQLite metadata sidecar for case admin tables; forensic index remains D-RUST-INDEX adapter stub.
4. **Run test to verify it passes**
5. **Commit:** `feat: add case metadata migrations`

### Task F4: Case state machine and exclusive lock

**Files:** `crates/lab-case/src/lifecycle.rs`, `crates/lab-case/src/lock.rs`

1. **Write the failing test:** second open blocked while lock held; stale dead-PID recovery → `RECOVERY_REQUIRED`.
2. **Run test to verify it fails**
3. **Implement minimal:** states from `docs/contracts/CASE-LIFECYCLE.md`.
4. **Run test to verify it passes**
5. **Commit:** `feat: enforce case lifecycle and exclusive lock`

### Task F5: Content-addressed blob / derived store

**Files:** `crates/lab-store/src/cas.rs`

1. **Write the failing test:** put bytes → sha256 path; get returns identical bytes; overwrite denied.
2. **Run test to verify it fails**
3. **Implement minimal:** content-addressed store under case directory.
4. **Run test to verify it passes**
5. **Commit:** `feat: add content-addressed derived store`

### Task F6: Evidence, provenance, audit, coverage schemas

**Files:** `crates/lab-core/src/schema_validate.rs`, fixtures under `fixtures/contracts/`

1. **Write the failing test:** valid fixtures pass; invalid fixtures fail (`missing`, `enum`, `integrity`).
2. **Run test to verify it fails**
3. **Implement minimal:** JSON Schema validation against `schemas/*.json`.
4. **Run test to verify it passes**
5. **Commit:** `feat: validate foundation record schemas`

### Task F7: Hash-chain and signature-envelope verification

**Files:** `crates/lab-crypto/src/digest.rs`, `crates/lab-crypto/src/signature.rs`

1. **Write the failing test:** Ed25519 envelope `VALID_TRUSTED` vs `INVALID`; legacy MD5 labeled non-authoritative.
2. **Run test to verify it fails**
3. **Implement minimal:** per `docs/CRYPTOGRAPHIC-PROFILE.md`.
4. **Run test to verify it passes**
5. **Commit:** `feat: add digest and signature envelope verification`

### Task F8: Hostile `.fsnap` preflight and safe importer

**Files:** `crates/lab-fsnap/src/preflight.rs`, `crates/lab-fsnap/src/import.rs`

1. **Write the failing test:** path traversal, duplicate members, bomb limits, partial package → reject; happy path → deterministic import result.
2. **Run test to verify it fails**
3. **Implement minimal:** `docs/contracts/FSNAP-READ-CONTRACT.md`.
4. **Run test to verify it passes**
5. **Commit:** `feat: add safe fsnap preflight and importer`

### Task F9: Bounded worker / cancellation protocol

**Files:** `crates/lab-worker/src/queue.rs`

1. **Write the failing test:** start job, cancel within 500 ms, no duplicate derived objects on resume.
2. **Run test to verify it fails**
3. **Implement minimal:** bounded queue + cancel token.
4. **Run test to verify it passes**
5. **Commit:** `feat: add cancellable bounded worker protocol`

### Task F10: Minimal Slint case / evidence / coverage UI

**Files:** `apps/lab-slint/ui/app.slint`, `apps/lab-slint/src/main.rs`

1. **Write the failing test:** UI automation/smoke opens case, shows coverage counts, keyboard focus on primary actions.
2. **Run test to verify it fails**
3. **Implement minimal:** Slint shell calling Rust core only.
4. **Run test to verify it passes**
5. **Commit:** `feat: add minimal Slint foundation UI`

### Task F11: Deterministic export / report skeleton

**Files:** `crates/lab-core/src/export.rs`

1. **Write the failing test:** same case seal → identical export digest; draft labeled non-final.
2. **Run test to verify it fails**
3. **Implement minimal:** JSON/HTML skeleton export.
4. **Run test to verify it passes**
5. **Commit:** `feat: add deterministic export skeleton`

### Task F12: Documentation shell

**Files:** `apps/lab-slint/docs_shell/`, `docs/user/`

1. **Write the failing test:** installed docs index opens offline and lists Foundation topics.
2. **Run test to verify it fails**
3. **Implement minimal:** offline docs viewer matching Acquire installed-docs experience.
4. **Run test to verify it passes**
5. **Commit:** `feat: add offline documentation shell`

### Task F13: Cross-platform packaging smoke tests

**Files:** `packaging/`, CI matrix

1. **Write the failing test:** package job produces artifact for win/mac/linux smoke targets.
2. **Run test to verify it fails**
3. **Implement minimal:** unsigned smoke packages; signing hooked but cert-gated.
4. **Run test to verify it passes**
5. **Commit:** `build: add cross-platform packaging smoke`

### Task F14: Security / fuzz / property / rollback tests

**Files:** `crates/lab-fsnap/fuzz/`, `crates/lab-case/tests/rollback.rs`

1. **Write the failing test:** fuzz harness entry + migration rollback property.
2. **Run test to verify it fails**
3. **Implement minimal:** cargo-fuzz targets for preflight; DB rollback test.
4. **Run test to verify it passes**
5. **Commit:** `test: add foundation fuzz and rollback coverage`

### Task F15: Release-gate evidence bundle

**Files:** `release-evidence/FOUNDATION/`

1. **Write the failing test:** checklist script fails if any Foundation gate artifact missing.
2. **Run test to verify it fails**
3. **Implement minimal:** gather SBOM, test reports, schema validation log, platform smoke log.
4. **Run test to verify it passes**
5. **Commit:** `docs: add foundation release-gate evidence bundle`

## Acceptance commands (planning link targets)

| Area | Command |
|---|---|
| Unit/schema | `cargo test --workspace` |
| Schema fixtures | `cargo test -p lab-core schema_fixtures` |
| Fsnap hostile | `cargo test -p lab-fsnap hostile_` |
| UI smoke | `cargo test -p lab-slint ui_smoke` |
| Packaging smoke | `./packaging/smoke.sh` |
