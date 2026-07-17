# Trareon Lab Gate A Spikes

Equal proof-of-capability spikes for desktop shell selection. Synthetic data only. No production evidence parsers, no real `.fsnap` evidence, and no casework on hostile corpora.

## Candidates

| Directory | Candidate | Implementation status |
|---|---|---|
| `lab-spike-core/` + `lab-spike-harness/` | Shared Rust core + headless harness | Measured on macOS / Windows / Linux |
| `slint-app/` | Slint + Rust (`C-SLINT`) | Measured on 3 OS + package size |
| `tauri/` | Tauri 2 + Svelte 5 (`C-TAURI`) | Measured on macOS; Win/Linux pending |
| `avalonia/` | Avalonia + Rust harness bridge (`C-AVALONIA`) | Scaffold; needs .NET 8 SDK |

## Copy-paste runbooks

- Slint runtime: [`COPY-PASTE-SLINT.md`](COPY-PASTE-SLINT.md)
- Slint package size: [`COPY-PASTE-SLINT-PACKAGE.md`](COPY-PASTE-SLINT-PACKAGE.md)
- Tauri: [`COPY-PASTE-TAURI.md`](COPY-PASTE-TAURI.md)
- Avalonia: [`COPY-PASTE-AVALONIA.md`](COPY-PASTE-AVALONIA.md)

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

## Status

Gate A remains `NOT PASS` until every mandatory gate has evidence for a selected desktop candidate on Windows + macOS + Linux, and a database/index candidate also passes.
