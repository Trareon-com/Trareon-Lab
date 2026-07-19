# Selling page — Trareon Lab 1.0

**Channel:** Lynk.id / Gumroad (paid download). Source: public GitHub (GPL-3.0-only).  
**Build:** full feature binary — **no license key, activation, or DRM in the app.**

## Features

Offline examination workbench (ASCII copper shell):

- **Case home** — open/create case, workflow guide, recent cases, Quick Verify (ephemeral)
- **Evidence** — file list with pagination, integrity badges, hex inspector, carving
- **Search** — artifact hits, YARA / hash-set panels, bookmark from selection
- **Timeline** — Plaso/Hayabusa CSV import, filter + timezone readout
- **Bookmarks → Report** — claim queue, finalize blockers / SoD, multi-format export
- **LAB tools** — Artifacts, Graph (Find Related edges), Runs compare, Transfer (Ed25519 packages), Capabilities matrix
- Offline-first: `.fsnap` / raw disk ingest, FS browsing, Windows/macOS/Linux artifact subsets

## Screenshots

See `docs/media/feature-*.png` (home, evidence, search, timeline, bookmarks, report, artifacts, graph, runs, transfer, capabilities, about, hex, palette).

## Limits

- Installers typically **unsigned** — SmartScreen / Gatekeeper warnings; see `docs/SELLING-UNSIGNED.md`
- Not ISO accredited / not court-ready
- OS matrix: Win10/11 x64, macOS 14/15 arm64, Ubuntu/Debian/Kali x64
- Binaries are **not** published on GitHub Releases

## How buyers get the app

1. Purchase on Lynk.id or Gumroad.
2. Download the OS-specific archive + verify SHA-256 against the listed `SHA256SUMS` / freeze SHA.
3. Source corresponding to that freeze SHA is on the public GitHub repo (`LICENSE` = GPL-3.0-only).

## Operator build path

`docs/DISTRIBUTION-STOREFRONT.md`
