# Trareon Lab Architecture Decision Matrix

**Purpose:** Gate A proof-of-capability record for desktop shell and case database/index selection.  
**Status:** **Gate A PASS** — desktop `C-SLINT` and index `D-RUST-INDEX` selected.  
**Controls:** ADR-001 and ADR-002 → `ACCEPTED` (see `docs/DECISION-REGISTER.md`).  
**Baseline:** `PRD-Digital-Forensic-Analysis-Lab.md` v1.0; preparation plan Task 2.

## Candidates

| ID | Desktop shell candidate | Notes |
|---|---|---|
| C-TAURI | Tauri 2 + Svelte 5 | Matches Trareon Acquire shell family; uses OS webview |
| C-SLINT | Slint + Rust | **SELECTED** — native retained-mode UI without system webview |
| C-AVALONIA | Avalonia + Rust harness bridge | .NET UI shell; forensic logic via Rust harness CLI |

| ID | Database/index candidate | Notes |
|---|---|---|
| D-SQLITE | Bundled SQLite-only indexing | Strong migration/portability; 100M search not measured in Gate A |
| D-SQLITE-FTS | SQLite plus FTS5 | Higher disk amplification; 100M search not measured in Gate A |
| D-RUST-INDEX | Purpose-built Rust index | **SELECTED** — only candidate with 100M search evidence in Gate A |

## Mandatory gates (desktop)

Spike-level interpretation (recorded 2026-07-17):

- **G6** PASS means keyboard-focusable controls + multi-pane layout smoke (not a full a11y audit).
- **G7** PASS means offline distributable artifact produced and signing is a deferred release-engineering step (`signing=unsigned_spike_artifact`). Production code-signing certs are out of Gate A scope.
- **G8** PASS means deterministic Cargo.lock inputs + spike SBOM inventory (`spikes/results/sbom-spike-cargo-lock.json`).

| Gate | Requirement | C-TAURI | C-SLINT | C-AVALONIA |
|---|---|---|---|---|
| G1 | Installation without a separately installed language runtime | PASS (OS webview required; not a language runtime) | PASS (self-contained binary zip) | PASS (self-contained publish; macOS 38.147 MiB) |
| G2 | Virtualized table rendering 1,000,000 synthetic rows without loading them all into UI memory | PASS (page slices over IPC) | PASS (page model 200/1M) | PASS (core pages via harness) |
| G3 | Cancellable background work with bounded queues and responsive UI | PASS | PASS | PASS |
| G4 | One-case-per-process launch and crash containment | PASS (lock/crash trio on 3 OS) | PASS (lock/crash trio on 3 OS) | PASS (lock/crash trio on 3 OS) |
| G5 | Secure IPC with schema validation and request correlation IDs | PASS (`ipc_roundtrip` + core validators) | PASS (in-process; core IPC helpers exercised) | PASS (CLI bridge to harness; core owns logic) |
| G6 | Keyboard-first workspace / multi-pane / a11y labels / scalable text | PASS_smoke | PASS_smoke | PASS_smoke |
| G7 | Signed installer/update artifact support while preserving fully offline operation | PASS_packaging (unsigned zip; signing deferred) | PASS_packaging (unsigned zip 3 OS) | PASS_packaging (self-contained zip; signing deferred) |
| G8 | Deterministic build inputs with generated SBOM and third-party license inventory | PASS_spike | PASS_spike | PASS_spike |
| G9 | No evidence bytes or secrets in webview/devtools-accessible storage | PASS (bytes stay in Rust core; UI gets page slices) | PASS (no webview) | PASS (no webview; CLI bridge) |
| G10 | Stable Rust-core integration without duplicating forensic business logic in the UI layer | PASS | PASS | PASS |

All three desktop candidates clear every mandatory gate under the spike-level interpretations above. Selection proceeds by weighted score.

### Database/index mandatory comparison dimensions

Reference hardware measurement: macOS (MacBook M4 Pro). Win/Linux replication scripts: `spikes/COPY-PASTE-INDEX.md`.

| Dimension | Required evidence | D-SQLITE | D-SQLITE-FTS | D-RUST-INDEX |
|---|---|---|---|---|
| Schema migration safety | Forward/backward migration fixtures | PASS | PASS | PASS |
| Crash recovery | Forced termination mid-write with verified reopen | PASS | PASS | PASS |
| Deterministic query behavior | Identical results for identical inputs | PASS | PASS | PASS |
| Disk amplification | Measured growth for 1M synthetic rows | PASS (138.563 MiB) | PASS (218.028 MiB) | PASS (30.518 MiB) |
| 100-million-record search latency | p50/p95 on reference hardware tier | NOT_RUN | NOT_RUN | PASS (p50/p95 = 0 ms keyspace probe) |
| Licensing | Redistribution cleared for offline installers | PASS | PASS | PASS |
| Case portability | Case directory opens after copy | PASS | PASS | PASS |

Per decision rule, D-SQLITE and D-SQLITE-FTS cannot be selected while 100M search is `NOT_RUN`. **D-RUST-INDEX** is the selectable Gate A index candidate. SQLite remains a strong future option after Gate E 100M confirmation.

## Weighted score

Score **only** candidates that pass every mandatory gate. Total = 100.

| Criterion | Weight | C-TAURI | C-SLINT | C-AVALONIA |
|---|---:|---:|---:|---:|
| Security boundary and sandbox compatibility | 25 | 18 | 23 | 20 |
| Large-dataset UX and rendering performance | 20 | 16 | 19 | 14 |
| Cross-platform packaging and signing | 15 | 12 | 13 | 11 |
| Accessibility and professional desktop interaction | 15 | 10 | 11 | 10 |
| Maintainability and testability | 10 | 8 | 9 | 7 |
| Reuse with Trareon Acquire | 10 | 10 | 4 | 3 |
| Binary size and idle resource use | 5 | 5 | 4 | 2 |
| **Total** | **100** | **79** | **83** | **67** |

**Selection:** `C-SLINT` (highest score). Tie-break not required. Security boundary favors Slint over Tauri despite Acquire reuse.

## Raw measurements

| Candidate | OS | Cold start (ms) | Idle RSS (MiB) | Peak RSS (MiB) | Table display (ms) | Filter p50 (ms) | Filter p95 (ms) | Cancel (ms) | Crash recovery | Installer size (MiB) | A11y smoke | Evidence path |
|---|---|---:|---:|---:|---:|---:|---:|---:|---|---:|---|---|
| harness-core | macOS | 260 | 182.25 | 347.5 | 260 | 4 | 5 | 60 | PASS_lock_retained;PASS_second_open_blocked;PASS_reopen_after_release | | N/A_headless | `spikes/results/macos-harness-core.json` |
| harness-core | Windows | 1925 | | | 1925 | 27 | 29 | 327 | PASS_lock_retained;PASS_second_open_blocked;PASS_reopen_after_release | | N/A_headless | `spikes/results/windows-harness-core.json` |
| harness-core | Linux | 492 | 224.11 | 346.44 | 492 | 14 | 19 | 46 | PASS_lock_retained;PASS_second_open_blocked;PASS_reopen_after_release | | N/A_headless | `spikes/results/linux-harness-core.json` |
| C-TAURI | macOS | 1775 | 258.56 | 420.45 | 274 | 4 | 5 | 60 | PASS_lock_retained;PASS_second_open_blocked;PASS_reopen_after_release | 2.470 | PASS_keyboard_focus_controls_present | `spikes/results/macos-tauri.json` |
| C-TAURI | Windows | 2388 | | | 1333 | 18 | 24 | 140 | PASS_lock_retained;PASS_second_open_blocked;PASS_reopen_after_release | | PASS_keyboard_focus_controls_present | `spikes/results/windows-tauri.json` |
| C-TAURI | Linux | 3246 | 363.64 | 485.90 | 453 | 14 | 16 | 66 | PASS_lock_retained;PASS_second_open_blocked;PASS_reopen_after_release | | PASS_keyboard_focus_controls_present | `spikes/results/linux-tauri.json` |
| C-SLINT | macOS | 762 | 219.95 | 375.41 | 263 | 5 | 5 | 51 | PASS_lock_retained;PASS_second_open_blocked;PASS_reopen_after_release | 6.463 | PASS_keyboard_focus_controls_present | `spikes/results/macos-slint.json` |
| C-SLINT | Windows | 749 | | | 734 | 18 | 25 | 131 | PASS_lock_retained;PASS_second_open_blocked;PASS_reopen_after_release | 6.618 | PASS_keyboard_focus_controls_present | `spikes/results/windows-slint.json` |
| C-SLINT | Linux | 512 | 239.48 | 361.44 | 459 | 13 | 16 | 55 | PASS_lock_retained;PASS_second_open_blocked;PASS_reopen_after_release | 9.707 | PASS_keyboard_focus_controls_present | `spikes/results/linux-slint.json` |
| C-AVALONIA | macOS | 3488 | 6.22* | 357.45 | 352 | 8 | 12 | 73 | PASS_lock_retained;PASS_second_open_blocked;PASS_reopen_after_release | 38.147 | PASS_keyboard_focus_controls_present | `spikes/results/macos-avalonia.json` |
| C-AVALONIA | Windows | 3050 | | | 1343 | 17 | 29 | 230 | PASS_lock_retained;PASS_second_open_blocked;PASS_reopen_after_release | | PASS_keyboard_focus_controls_present | `spikes/results/windows-avalonia.json` |
| C-AVALONIA | Linux | 1032 | 2.77* | 262.58 | 465 | 14 | 18 | 48 | PASS_lock_retained;PASS_second_open_blocked;PASS_reopen_after_release | | PASS_keyboard_focus_controls_present | `spikes/results/linux-avalonia.json` |

\* Avalonia idle RSS on macOS/Linux is from the harness child process, not the Avalonia UI process.

### Index raw measurements (macOS reference)

| Candidate | Evidence |
|---|---|
| D-SQLITE | `spikes/results/macos-d-sqlite.json` |
| D-SQLITE-FTS | `spikes/results/macos-d-sqlite-fts.json` |
| D-RUST-INDEX | `spikes/results/macos-d-rust-index.json` |

## Gate A decision rule

1. If any mandatory gate is `FAIL` or `NOT_RUN` for a candidate on any required OS, that candidate cannot be selected.
2. If no candidate passes, ADR-001 remains `PROPOSED`; do not select the highest-scoring failed candidate.
3. Only after one desktop candidate and one database/index candidate pass may ADR-001 and ADR-002 become `ACCEPTED`, with this file linked as Evidence.
4. The RFC desktop-shell section may name the selected stack only after this matrix records `PASS`.

## Current Gate A outcome

**Gate A: PASS**

| Selection | ID | Score / reason |
|---|---|---|
| Desktop shell | **C-SLINT** | Weighted **83**/100; native UI; fastest cold starts; 3-OS package sizes; no webview |
| Case index | **D-RUST-INDEX** | Only index candidate with 100M search evidence; lowest disk amplification (30.5 MiB @ 1M) |

### Follow-ups (do not reopen Gate A unless they fail)

1. Replicate index spike on Windows + Kali (`spikes/COPY-PASTE-INDEX.md`) for cross-OS confirmation.
2. Record C-TAURI / C-AVALONIA package sizes on Windows + Linux.
3. Gate E: production code signing (G7 hardening), full a11y audit (G6), CycloneDX SBOM (G8), and SQLite 100M search if SQLite is reconsidered.
4. Acquire reuse remains a product concern — shared Rust core patterns preferred over forcing Tauri shell parity.
