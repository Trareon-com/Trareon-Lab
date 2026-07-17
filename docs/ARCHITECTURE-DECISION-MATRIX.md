# Trareon Lab Architecture Decision Matrix

**Purpose:** Gate A proof-of-capability record for desktop shell and case database/index selection.  
**Status:** Measurement protocol defined; equal spikes scaffolded; **no candidate has passed every mandatory gate on Windows, macOS, and Linux**.  
**Controls:** ADR-001 (desktop stack) and ADR-002 (case database/index) remain `PROPOSED` until this matrix records `PASS` for one candidate on every mandatory gate and the weighted score is computed from raw measurements.  
**Baseline:** `PRD-Digital-Forensic-Analysis-Lab.md` v1.0; preparation plan Task 2.

## Candidates

| ID | Desktop shell candidate | Notes |
|---|---|---|
| C-TAURI | Tauri 2 + Svelte 5 | Matches Trareon Acquire shell family; uses OS webview |
| C-SLINT | Slint + Rust | Native retained-mode UI without system webview |
| C-AVALONIA | Avalonia + Rust FFI | .NET UI shell with Rust forensic core via FFI |

| ID | Database/index candidate | Notes |
|---|---|---|
| D-SQLITE | Bundled SQLite-only indexing | Portable, simple, migration-friendly |
| D-SQLITE-FTS | SQLite plus embedded full-text/index engine | Additional search engine alongside SQLite metadata |
| D-RUST-INDEX | Purpose-built Rust index | Custom engine owned by Trareon |

## Mandatory gates

Reject any desktop candidate that cannot demonstrate **all** of the following on **Windows, macOS, and Linux**. A missing OS measurement is treated as `NOT_RUN`, which blocks Gate A acceptance.

| Gate | Requirement | C-TAURI | C-SLINT | C-AVALONIA |
|---|---|---|---|---|
| G1 | Installation without a separately installed language runtime | NOT_RUN | NOT_RUN | NOT_RUN |
| G2 | Virtualized table rendering 1,000,000 synthetic rows without loading them all into UI memory | NOT_RUN | NOT_RUN | NOT_RUN |
| G3 | Cancellable background work with bounded queues and responsive UI | NOT_RUN | NOT_RUN | NOT_RUN |
| G4 | One-case-per-process launch and crash containment | NOT_RUN | NOT_RUN | NOT_RUN |
| G5 | Secure IPC with schema validation and request correlation IDs | NOT_RUN | NOT_RUN | NOT_RUN |
| G6 | Keyboard-first workspace, dockable or equivalent multi-pane layout, accessibility labels, and scalable text | NOT_RUN | NOT_RUN | NOT_RUN |
| G7 | Signed installer/update artifact support while preserving fully offline operation | NOT_RUN | NOT_RUN | NOT_RUN |
| G8 | Deterministic build inputs with generated SBOM and third-party license inventory | NOT_RUN | NOT_RUN | NOT_RUN |
| G9 | No evidence bytes or secrets in webview/devtools-accessible storage | NOT_RUN | NOT_RUN | NOT_RUN |
| G10 | Stable Rust-core integration without duplicating forensic business logic in the UI layer | NOT_RUN | NOT_RUN | NOT_RUN |

### Database/index mandatory comparison dimensions

The database/index sub-spike must compare D-SQLITE, D-SQLITE-FTS, and D-RUST-INDEX against:

| Dimension | Required evidence | D-SQLITE | D-SQLITE-FTS | D-RUST-INDEX |
|---|---|---|---|---|
| Schema migration safety | Forward/backward migration fixtures | NOT_RUN | NOT_RUN | NOT_RUN |
| Crash recovery | Forced termination mid-write with verified reopen | NOT_RUN | NOT_RUN | NOT_RUN |
| Deterministic query behavior | Identical results for identical case package on supported OS matrix | NOT_RUN | NOT_RUN | NOT_RUN |
| Disk amplification | Measured growth for published Large corpus | NOT_RUN | NOT_RUN | NOT_RUN |
| 100-million-record search latency | p50/p95 on reference hardware tier | NOT_RUN | NOT_RUN | NOT_RUN |
| Licensing | Redistribution cleared for offline installers | NOT_RUN | NOT_RUN | NOT_RUN |
| Case portability | Case directory opens on another supported machine without server | NOT_RUN | NOT_RUN | NOT_RUN |

## Weighted score

Score **only** candidates that pass every mandatory gate. Total = 100.

| Criterion | Weight |
|---|---:|
| Security boundary and sandbox compatibility | 25 |
| Large-dataset UX and rendering performance | 20 |
| Cross-platform packaging and signing | 15 |
| Accessibility and professional desktop interaction | 15 |
| Maintainability and testability | 10 |
| Reuse with Trareon Acquire | 10 |
| Binary size and idle resource use | 5 |

**Tie-break order:** stronger security boundary, then lower peak memory, then reuse with Trareon Acquire.

## Raw measurements

Measurements must use the same reference hardware tier for every candidate comparison. Record cold start, idle RSS, peak RSS, initial table display, filter latency p50/p95, cancellation latency, crash recovery outcome, installer size, and accessibility smoke results on every target OS.

Shared headless harness (`spikes/lab-spike-harness`) validates core workflow mechanics used by every UI candidate. UI candidates still require their own shell measurements.

| Candidate | OS | Cold start (ms) | Idle RSS (MiB) | Peak RSS (MiB) | Table display (ms) | Filter p50 (ms) | Filter p95 (ms) | Cancel (ms) | Crash recovery | Installer size (MiB) | A11y smoke | Evidence path |
|---|---|---:|---:|---:|---:|---:|---:|---:|---|---:|---|---|
| harness-core | macOS | 260 | 182.25 | 347.5 | 260 | 4 | 5 | 60 | PASS_lock_retained;PASS_second_open_blocked;PASS_reopen_after_release | | N/A_headless | `spikes/results/macos-harness-core.json` |
| harness-core | Windows | 1925 | | | 1925 | 27 | 29 | 327 | PASS_lock_retained;PASS_second_open_blocked;PASS_reopen_after_release | | N/A_headless | `spikes/results/windows-harness-core.json` |
| harness-core | Linux | 492 | 224.11 | 346.44 | 492 | 14 | 19 | 46 | PASS_lock_retained;PASS_second_open_blocked;PASS_reopen_after_release | | N/A_headless | `spikes/results/linux-harness-core.json` |
| C-TAURI | macOS | | | | | | | | | | | |
| C-TAURI | Windows | | | | | | | | | | | |
| C-TAURI | Linux | | | | | | | | | | | |
| C-SLINT | macOS | 762 | 219.95 | 375.41 | 263 | 5 | 5 | 51 | PASS_lock_retained;PASS_second_open_blocked;PASS_reopen_after_release | 6.463 | PASS_keyboard_focus_controls_present | `spikes/results/macos-slint.json` |
| C-SLINT | Windows | 749 | | | 734 | 18 | 25 | 131 | PASS_lock_retained;PASS_second_open_blocked;PASS_reopen_after_release | | PASS_keyboard_focus_controls_present | `spikes/results/windows-slint.json` (installer size pending) |
| C-SLINT | Linux | 512 | 239.48 | 361.44 | 459 | 13 | 16 | 55 | PASS_lock_retained;PASS_second_open_blocked;PASS_reopen_after_release | | PASS_keyboard_focus_controls_present | `spikes/results/linux-slint.json` (installer size pending) |
| C-AVALONIA | macOS | | | | | | | | | | | needs .NET SDK |
| C-AVALONIA | Windows | | | | | | | | | | | |
| C-AVALONIA | Linux | | | | | | | | | | | |

Empty numeric cells mean measurement has not been recorded. Empty cells do not count as pass.

## Equal spike workflow

Each spike under `spikes/` must implement the same synthetic workflow:

1. open a fake case;
2. stream one million metadata rows;
3. filter by hash prefix;
4. open a detail pane;
5. start and cancel a background hash job;
6. simulate worker crash;
7. export a deterministic JSON result;
8. reopen the case in a second process only after the first releases its lock.

Synthetic data only. No production evidence parsers.

## Known pre-measurement risks

These are planning risks recorded before spike execution; they are not Gate A results.

| Candidate | Risk relevant to mandatory gates |
|---|---|
| C-TAURI | OS webview may conflict with G1 on Linux (WebKitGTK) and G9 (devtools-accessible storage) unless evidence bytes never enter the webview and packaging proves no separate runtime install is required |
| C-SLINT | Packaging/signing maturity and accessibility tooling must be proven for G6 and G7 |
| C-AVALONIA | Self-contained publish must prove G1 without requiring a separately installed .NET runtime; FFI boundary must keep forensic logic in Rust for G10 |

## Gate A decision rule

1. If any mandatory gate is `FAIL` or `NOT_RUN` for a candidate on any required OS, that candidate cannot be selected.
2. If no candidate passes, ADR-001 remains `PROPOSED`; do not select the highest-scoring failed candidate.
3. Only after one desktop candidate and one database/index candidate pass may ADR-001 and ADR-002 become `ACCEPTED`, with this file linked as Evidence.
4. The RFC desktop-shell section may name the selected stack only after this matrix records `PASS`.

## Current Gate A outcome

**Gate A: NOT PASS**

- Shared headless harness measured on macOS, Windows (ThinkPad X270), and Linux (Kali) for 1,000,000 rows with lock/crash checks PASS on all three.
- Windows RSS helpers are not yet implemented (`idle_rss_mib` / `peak_rss_mib` null); timing and lock/crash evidence are recorded.
- C-SLINT runtime measured on macOS, Windows (ThinkPad), and Linux (Kali); crash/lock checks PASS on all three. Windows RSS null (not implemented).
- C-SLINT offline package size recorded on macOS: **6.463 MiB** (`macos-slint-package.zip`, unsigned spike artifact, no separate language runtime). Windows + Linux package sizes still pending (`spikes/COPY-PASTE-SLINT-PACKAGE.md`).
- Package size is a matrix measurement only; G7 still requires signed installer/update evidence. G6 a11y remains smoke-level (`PASS_keyboard_focus_controls_present`), not full dockable/multi-pane proof.
- Desktop UI candidates C-TAURI / C-AVALONIA have no three-OS evidence yet.
- Desktop shell: no ACCEPTED selection.
- Case database/index: no ACCEPTED selection.
- Next action: finish C-SLINT package size on Windows + Kali, then Tauri/Avalonia spikes.
