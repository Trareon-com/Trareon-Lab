# Design: DFIR workbench research UX (competitive)

**Date:** 2026-07-18  
**Status:** ACCEPTED (implementation on `cursor/dfir-workbench-ux`)  
**Stack:** `apps/lab-slint` (Slint + Rust) — no React/Tauri rewrite  
**License context:** workspace `GPL-3.0-only`

## 1. Competitive position

Win where expensive suites are weak: lightweight offline shell, SoD/finalize, signed transfer, coverage honesty, RunManifest/Exceptions — not AXIOM-scale mobile/cloud catalogs.

## 2. Industry patterns

| Source | Adopt |
|--------|--------|
| Autopsy | Tree/list → result → content viewer (Inspector); keyword entry; tags |
| Magnet AXIOM | Case dashboard, Exceptions inbox, Process noise in status, Portable Case |
| FTK | Index/paging + filter |
| X-Ways | Hex/raw in inspector; power-user shortcuts |

## 3. Academic patterns

- Altiero (2015): visualization reduces cognitive load vs tabular-only
- EPIC: Explore → Investigate → Correlate
- TAIMA / CFTL (Olsson & Boldt): overview timeline + zoom + detail list
- Shneiderman: overview → zoom/filter → details-on-demand

## 4. GitHub adopt / learn / avoid

### Adopt

- **yara-x** (already in `lab-yara`) — UI hits + version pin
- **Hayabusa / Chainsaw** (GPL-3) — optional sidecar CSV import into `lab-timeline`
- **Plaso** — import `l2tcsv` / JSONL (do not embed Python runtime)
- **Timesketch** — export JSONL shape; do not embed server
- **libewf** — evaluate only if native E01 Validated subset insufficient

### Learn

- Autopsy / Sleuth Kit IA; AnalysisLoom three-pane density; OpenForensic plugin ideas

### Avoid

- Fork Autopsy/Tauri shells; embed Timesketch; Volatility in-process (VSL); Scalpel

## 5. Chrome workbench

Five regions: TopBar · collapsible Nav · Main · Inspector · Status/Log (+ Command Palette overlay).

Dark default. Sticky disclosure: UNSIGNED / NOT court-ready / NOT ISO-certified.

## 6. Sprint map

1. Wire chrome + Exceptions + designation + Runs  
2. Hex, search coverage, timeline CFTL-lite, tags, YARA/hash UI, import adapters  
3. SoD report, transfer, export, reviewer pack, prefs, Capabilities  
4. Filters/tree, perf budgets, ID copy, 5-task QA  

## 7. Non-goals

Fake timeline/AI/ATT&CK theater; court-ready claims; React rewrite; full AXIOM artifact catalog.
