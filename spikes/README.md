# Trareon Lab Gate A Spikes

Equal proof-of-capability spikes for desktop shell selection. Synthetic data only. No production evidence parsers, no real `.fsnap` evidence, and no casework on hostile corpora.

## Candidates

| Directory | Candidate |
|---|---|
| `spikes/tauri/` | Tauri 2 + Svelte 5 |
| `spikes/slint/` | Slint + Rust |
| `spikes/avalonia/` | Avalonia + Rust FFI |

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

Record results into `docs/ARCHITECTURE-DECISION-MATRIX.md` raw measurement table:

- cold start;
- idle RSS;
- peak RSS;
- initial table display;
- filter latency p50/p95;
- cancellation latency;
- crash recovery outcome;
- installer size;
- accessibility smoke.

Use one reference hardware tier per comparison. Do not compare candidates measured on different tiers.

## Status

Scaffold only. Implementations and measurements are not yet recorded. Gate A remains `NOT PASS` until every mandatory gate is measured on Windows, macOS, and Linux.
