# Selling page — Trareon Lab 1.0

**Channel:** Lynk.id / Gumroad (paid download). Source: public GitHub (GPL-3.0-only).  
**Build:** full feature binary — **no license key, activation, or DRM in the app.**

## Features

Offline case examination, `.fsnap` import, disk raw ingest, FS browsing, Windows/macOS/Linux artifact subsets, bookmarks, signed offline share packs, timeline, draft/sealed reports.

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
