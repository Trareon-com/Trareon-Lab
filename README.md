# Trareon Lab

**Offline digital-forensic examination workstation** — open a case folder, import disk images, search the index, bookmark findings, and export reports. Everything stays local.

<p align="center">
  <img src="docs/media/feature-home.png" alt="Trareon Lab — case home (no case open)" width="920" />
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-GPL--3.0--only-blue.svg" alt="License: GPL-3.0-only" /></a>
  <a href=".github/workflows/ci.yml"><img src="https://img.shields.io/badge/CI-GitHub%20Actions-2088FF.svg" alt="CI" /></a>
  <img src="https://img.shields.io/badge/UI-Slint-3D9B8F.svg" alt="UI: Slint" />
  <img src="https://img.shields.io/badge/status-Engineering%20Alpha-orange.svg" alt="Engineering Alpha" />
  <img src="https://img.shields.io/badge/installers-UNSIGNED-critical.svg" alt="Unsigned" />
</p>

<p align="center">
  <a href="docs/user/TUTORIAL.md"><strong>Complete tutorial →</strong></a>
  ·
  <a href="docs/SELLING-UNSIGNED.md">Unsigned install notes</a>
  ·
  <a href="docs/RELEASE-01-CAPABILITY-MATRIX.md">Capability matrix</a>
</p>

---

## Honesty first

| Claim | Reality |
|---|---|
| Court / ISO | **NOT court-ready** · **NOT ISO-certified** |
| Installers | Typically **UNSIGNED** (Gatekeeper / SmartScreen) |
| Distribution | Source here under **GPL-3.0-only**; product binaries on the storefront, not GitHub Releases |
| Empty states | No fake timeline, search hits, or AI theater — empty means empty |

---

## Feature gallery

Every primary workbench screen (captured from the live `trareon-lab` GUI):

<table>
  <tr>
    <td width="50%">
      <a href="docs/media/feature-home.png"><img src="docs/media/feature-home.png" alt="Home" /></a>
      <p align="center"><sub><b>Home</b> — open / create case</sub></p>
    </td>
    <td width="50%">
      <a href="docs/media/feature-case-with-evidence.png"><img src="docs/media/feature-case-with-evidence.png" alt="Case with evidence" /></a>
      <p align="center"><sub><b>Case Home</b> — next action, coverage, exceptions</sub></p>
    </td>
  </tr>
  <tr>
    <td>
      <a href="docs/media/feature-evidence.png"><img src="docs/media/feature-evidence.png" alt="Evidence" /></a>
      <p align="center"><sub><b>Evidence</b> — ingest list, designation, integrity</sub></p>
    </td>
    <td>
      <a href="docs/media/feature-hex.png"><img src="docs/media/feature-hex.png" alt="Hex" /></a>
      <p align="center"><sub><b>Hex</b> — inspector-backed dump guidance</sub></p>
    </td>
  </tr>
  <tr>
    <td>
      <a href="docs/media/feature-artifacts.png"><img src="docs/media/feature-artifacts.png" alt="Artifacts" /></a>
      <p align="center"><sub><b>Artifacts</b> — parser / index hits</sub></p>
    </td>
    <td>
      <a href="docs/media/feature-search.png"><img src="docs/media/feature-search.png" alt="Search" /></a>
      <p align="center"><sub><b>Search</b> — path / name / hash + coverage banner</sub></p>
    </td>
  </tr>
  <tr>
    <td>
      <a href="docs/media/feature-timeline.png"><img src="docs/media/feature-timeline.png" alt="Timeline" /></a>
      <p align="center"><sub><b>Timeline</b> — honest empty or CSV import (Plaso/Hayabusa)</sub></p>
    </td>
    <td>
      <a href="docs/media/feature-bookmarks.png"><img src="docs/media/feature-bookmarks.png" alt="Bookmarks" /></a>
      <p align="center"><sub><b>Bookmarks</b> — examiner citations</sub></p>
    </td>
  </tr>
  <tr>
    <td>
      <a href="docs/media/feature-graph.png"><img src="docs/media/feature-graph.png" alt="Graph" /></a>
      <p align="center"><sub><b>Graph</b> — correlation edges or empty state</sub></p>
    </td>
    <td>
      <a href="docs/media/feature-runs.png"><img src="docs/media/feature-runs.png" alt="Runs" /></a>
      <p align="center"><sub><b>Runs</b> — RunManifest / second-method compare</sub></p>
    </td>
  </tr>
  <tr>
    <td>
      <a href="docs/media/feature-report.png"><img src="docs/media/feature-report.png" alt="Report" /></a>
      <p align="center"><sub><b>Report</b> — SoD, blockers, export</sub></p>
    </td>
    <td>
      <a href="docs/media/feature-transfer.png"><img src="docs/media/feature-transfer.png" alt="Transfer" /></a>
      <p align="center"><sub><b>Transfer</b> — signed offline package status</sub></p>
    </td>
  </tr>
  <tr>
    <td>
      <a href="docs/media/feature-capabilities.png"><img src="docs/media/feature-capabilities.png" alt="Capabilities" /></a>
      <p align="center"><sub><b>Capabilities</b> — validated vs deferred modules</sub></p>
    </td>
    <td>
      <a href="docs/media/feature-about.png"><img src="docs/media/feature-about.png" alt="About" /></a>
      <p align="center"><sub><b>About</b> — SBOM / disclosure</sub></p>
    </td>
  </tr>
  <tr>
    <td colspan="2">
      <a href="docs/media/feature-palette.png"><img src="docs/media/feature-palette.png" alt="Command palette" width="920" /></a>
      <p align="center"><sub><b>Command palette</b> — <code>/</code> · Open Case · Go Evidence · …</sub></p>
    </td>
  </tr>
</table>

Recapture locally (macOS): `./scripts/capture-readme-screens.sh`

---

## Capabilities

| Area | What it does |
|---|---|
| **Case** | Local `case.sqlite` + `index.sqlite` per folder; reopen preserves ledger |
| **Evidence** | `.raw` / `.dd` / `.img` / `.bin` / `.e01` ingest with designation + integrity chips |
| **Inspector** | File detail + hex for the current selection (survives nav changes) |
| **Search** | Path / name / hash against the case index; **coverage** disclosed (`complete` / `partial`) |
| **Timeline** | Empty until real labels exist; optional CSV import (Plaso / Hayabusa sidecar) |
| **Bookmarks** | Persisted findings → report claim material |
| **Report / Transfer** | Export digests + Ed25519-signed offline pack; finalize blockers + SoD hint |
| **Workbench** | Five-region chrome · command palette · Dark/Light · **EN / ID** |

---

## Quick start

### Binary (storefront)

1. Download from the operator storefront — see [`docs/SELLING-PAGE.md`](docs/SELLING-PAGE.md).
2. Verify SHA-256 against published `SHA256SUMS`.
3. Expect Gatekeeper / SmartScreen — [`docs/SELLING-UNSIGNED.md`](docs/SELLING-UNSIGNED.md).

### From source

```bash
git clone https://github.com/Trareon-com/Trareon-Lab.git
cd Trareon-Lab
cargo run -p lab-slint --bin trareon-lab --features gui
```

Headless tests (no display):

```bash
cargo test --workspace --exclude lab-slint
cargo test -p lab-slint --features gui
```

Automation (skip folder/file pickers):

```bash
export TRAREON_CASE_DIR=/path/to/case
export TRAREON_IMPORT_PATH=/path/to/image.dd
export TRAREON_AUTO_OPEN=1          # open case on launch
export TRAREON_IMPORT_AUTO=1        # import after open
export TRAREON_START_SCREEN=Evidence
```

---

## Tutorial (examiner flow)

Full step-by-step guide: **[`docs/user/TUTORIAL.md`](docs/user/TUTORIAL.md)**

```text
Open / create case folder
  → Import disk image
  → Evidence · Inspector/hex · Search (read coverage)
  → Bookmark findings (b)
  → Report blockers · Export report / transfer pack
```

| Key | Action |
|-----|--------|
| `/` | Command palette / search |
| `1`–`6` | Case · Evidence · Search · Timeline · Bookmarks · Report |
| `b` | Bookmark selection |
| `Esc` | Dismiss palette / clear |
| `?` | Keyboard cheat sheet |

Lab QA script: [`docs/validation/EXAMINER-5TASK-SCRIPT.md`](docs/validation/EXAMINER-5TASK-SCRIPT.md)

---

## Supported OS (claimed matrix)

| OS | Versions | Arch |
|---|---|---|
| Windows | 10 22H2, 11 23H2/24H2 | x64 |
| macOS | 14, 15 | arm64 |
| Linux | Ubuntu 22.04/24.04, Debian 12, Kali (lab) | x86_64 |

Outside this table = not claimed.

---

## Docs

| Doc | Purpose |
|---|---|
| [`docs/user/TUTORIAL.md`](docs/user/TUTORIAL.md) | Complete examiner tutorial |
| [`PRD-Digital-Forensic-Analysis-Lab.md`](PRD-Digital-Forensic-Analysis-Lab.md) | Product requirements |
| [`RFC-Digital-Forensic-Analysis-Lab.md`](RFC-Digital-Forensic-Analysis-Lab.md) | Architecture RFC |
| [`docs/RELEASE-01-CAPABILITY-MATRIX.md`](docs/RELEASE-01-CAPABILITY-MATRIX.md) | R1 capability matrix |
| [`docs/KNOWN-ISSUES.md`](docs/KNOWN-ISSUES.md) | Known limits |
| [`docs/superpowers/specs/2026-07-18-dfir-workbench-research-ux.md`](docs/superpowers/specs/2026-07-18-dfir-workbench-research-ux.md) | Workbench UX research |

Smoke packaging: `./packaging/smoke.sh`

---

## License

**GNU General Public License v3.0 only** (`GPL-3.0-only`). See [`LICENSE`](LICENSE).
