# Trareon Lab Gate A Spikes

Equal proof-of-capability spikes for desktop shell selection. Synthetic data only. No production evidence parsers, no real `.fsnap` evidence, and no casework on hostile corpora.

## Candidates

| Directory | Candidate | Implementation status |
|---|---|---|
| `lab-spike-core/` + `lab-spike-harness/` | Shared Rust core + headless harness | Runnable on macOS/Windows/Linux |
| `slint-app/` | Slint + Rust (`C-SLINT`) | Builds on macOS |
| `tauri/` | Tauri 2 + Svelte 5 (`C-TAURI`) | Scaffold / pending install |
| `avalonia/` | Avalonia + Rust FFI (`C-AVALONIA`) | Scaffold; needs .NET 8 SDK |

## Required equal workflow

Every spike must implement:

1. open a fake case workspace;
2. stream 1,000,000 synthetic metadata rows into a virtualized table;
3. filter by hash prefix;
4. open a detail pane for one selected row;
5. start a background hash job with bounded queue;
6. cancel the job and prove UI remains responsive;
7. simulate worker crash and recover without corrupting case lock state;
8. export a deterministic JSON result;
9. reopen the case in a second process only after the first releases its lock.

## Measurement protocol

Follow [`MEASUREMENT-RUNBOOK.md`](MEASUREMENT-RUNBOOK.md).

Record results into `docs/ARCHITECTURE-DECISION-MATRIX.md` and store raw JSON under `spikes/results/`.

## Current macOS harness result (MacBook)

See `results/macos-harness-core.json` (1,000,000 rows). Crash/lock checks: `PASS_lock_retained;PASS_second_open_blocked;PASS_reopen_after_release`.

## Status

Gate A remains `NOT PASS` until Tauri/Slint/Avalonia UI measurements and Windows + Linux harness/UI results are recorded for every mandatory gate.
