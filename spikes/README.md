# Trareon Lab Gate A Spikes

Equal proof-of-capability spikes for desktop shell and case index selection. Synthetic data only.

## Gate A result

**PASS** — selected **C-SLINT** + **D-RUST-INDEX**. See `docs/ARCHITECTURE-DECISION-MATRIX.md`.

## Candidates

| Directory | Candidate | Status |
|---|---|---|
| `lab-spike-core/` + `lab-spike-harness/` | Shared Rust core + headless harness | Measured 3 OS |
| `slint-app/` | Slint + Rust (`C-SLINT`) | **Selected** — measured 3 OS + package |
| `tauri/` | Tauri 2 + Svelte 5 (`C-TAURI`) | Measured 3 OS; runner-up |
| `avalonia/` | Avalonia + Rust harness (`C-AVALONIA`) | Measured 3 OS |
| `lab-spike-index/` | D-SQLITE / D-SQLITE-FTS / D-RUST-INDEX | **D-RUST-INDEX selected** |

## Copy-paste runbooks

- Slint: `COPY-PASTE-SLINT.md`, `COPY-PASTE-SLINT-PACKAGE.md`
- Tauri: `COPY-PASTE-TAURI.md`
- Avalonia: `COPY-PASTE-AVALONIA.md`
- Index: `COPY-PASTE-INDEX.md`
