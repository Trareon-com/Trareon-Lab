# Design: Proper desktop packaging (Windows / macOS / Linux)

**Date:** 2026-07-17  
**Status:** DRAFT — awaiting user review of this file  
**Decision:** Approach A (proper v1 storefront packages)  
**Depends on:** `docs/superpowers/specs/2026-07-17-storefront-binary-distribution-design.md` (DIST-001)  
**Related:** `docs/DISTRIBUTION-STOREFRONT.md`, `docs/SELLING-UNSIGNED.md`, `packaging/`

## 1. Problem

Current Windows guidance is essentially “copy a release EXE into a zip.” That is not a sellable desktop product. Buyers expect installable apps on Windows, macOS, and Linux: product name, icon, version metadata, Start Menu / Applications / `.desktop` integration, and uninstall where the platform supports it.

## 2. Goals

- Ship **proper desktop application packages** for the three supported storefront platforms.
- Keep DIST-001: packages are built offline, sold on Lynk.id/Gumroad, **never** attached to GitHub Releases.
- Full app, no in-app license/DRM.
- Signing/notarization/Authenticode remain **optional** hardening, not sell blockers.
- One shared product identity (name, icon, version) across OS packages.

## 3. Non-goals (v1)

- Windows `.msi` (WiX) and `.msix` — deferred to a later phase unless enterprise demand appears.
- Apple notarization / Developer ID as a sell requirement.
- Auto-update server / Sparkle / WinGet / Flathub publication.
- Changing the Slint UI toolkit or rewriting the app shell.
- Publishing installers from CI to public download URLs.

## 4. Product identity

| Field | Value |
|---|---|
| Display name | Trareon Lab |
| Executable (Windows/Linux) | `trareon-lab` / `trareon-lab.exe` |
| macOS bundle name | `Trareon Lab.app` |
| Publisher | Trareon |
| Version | workspace Cargo version at freeze (today `0.1.0` until bumped for storefront `1.0.0`) |
| Icon source | new shared assets under `packaging/branding/` (derive `.ico` / `.icns` / PNG from one master) |
| Windows AppUserModelID | `Trareon.Lab` |
| macOS bundle id | `com.trareon.lab` |
| Linux desktop id | `com.trareon.Lab` |

## 5. Storefront artifacts (v1)

Version placeholder `{VER}` = freeze version string (e.g. `1.0.0`).

| Platform | Primary (buyer default) | Secondary |
|---|---|---|
| Windows x64 | `TrareonLab-{VER}-windows-x64-setup.exe` (NSIS) | `TrareonLab-{VER}-windows-x64-portable.zip` |
| macOS arm64 | `TrareonLab-{VER}-macos-arm64.dmg` containing `Trareon Lab.app` | optional `.zip` of the `.app` |
| Linux x64 | `TrareonLab-{VER}-linux-x64.AppImage` | optional `.deb` |

All land under local `dist/{VER}/` with a root `SHA256SUMS`. Gitignored; upload manually to storefront.

## 6. Platform packaging behavior

### Windows (NSIS setup + portable)

- Install to `%ProgramFiles%\Trareon\Lab\` (or `Trareon Lab`).
- Start Menu shortcut; optional Desktop shortcut (default off or on — implementer picks default **on** for v1 indie sell).
- Add/Remove Programs uninstall entry (display name, version, publisher, icon).
- PE version resource + embedded icon on `trareon-lab.exe` (via `winres` / equivalent at build time).
- Portable zip: folder with exe + same icon metadata; no installer; documented as secondary.

### macOS (`.app` + `.dmg`)

- Bundle layout: `Trareon Lab.app/Contents/{MacOS,Resources,Info.plist}`.
- `Info.plist`: `CFBundleDisplayName`, `CFBundleIdentifier` `com.trareon.lab`, version strings, icon.
- DMG: standard drag-to-Applications layout (app + Applications symlink).
- Gatekeeper unsigned path remains disclosed in `docs/SELLING-UNSIGNED.md`.

### Linux (AppImage + optional `.deb`)

- AppImage: single executable file, embeds binary + `.desktop` + icon.
- Optional `.deb`: installs binary under `/usr/bin/trareon-lab`, desktop file under `/usr/share/applications/`, icon under `/usr/share/icons/`.
- `.tar.gz` raw binary is **not** the primary sell SKU anymore (may remain as operator debug artifact only).

## 7. Tooling approach (implementation intent)

Prefer scripts under `packaging/` that operators run on each OS host:

| OS | Build release binary | Package |
|---|---|---|
| Windows | `cargo build -p lab-slint --release --features gui` + winres | NSIS script `packaging/windows/trareon-lab.nsi` + `packaging/build-windows-desktop.ps1` |
| macOS | same cargo build | `packaging/build-macos-desktop.sh` → `.app` + `.dmg` (hdiutil) |
| Linux | same cargo build | `packaging/build-linux-desktop.sh` → AppImage (and optional deb) |

Exact AppImage helper (e.g. `linuxdeploy` / `appimagetool`) and NSIS install path are locked in the implementation plan; this spec only requires the **buyer-facing outputs** above.

Replace “copy exe PowerShell only” docs (`packaging/WINDOWS-UNSIGNED.md`) with pointers to the desktop builders; keep a thin portable path as secondary.

## 8. Docs / storefront copy updates

- `docs/DISTRIBUTION-STOREFRONT.md` — list the six artifact names and “primary vs secondary.”
- `docs/SELLING-PAGE.md` / `SELLING-UNSIGNED.md` — describe installer/DMG/AppImage open paths, not “run naked exe.”
- `docs/WINDOWS-LAB-QUEUE.md` W2/W5 — W2 builds desktop setup+portable; W5 MSI stays deferred.
- Decision register: new row **PKG-001** for proper desktop packaging Approach A.

## 9. Risks

| Risk | Mitigation |
|---|---|
| SmartScreen / Gatekeeper friction | Already disclosed; optional later signing |
| Icon/branding missing | Commit SVG/PNG master + generated ico/icns in `packaging/branding/` |
| Version drift across OS builds | Single `{VER}` from Cargo workspace / env `TRAREON_RELEASE_VERSION` |
| Accidental GitHub upload | Existing `scripts/check-no-github-binaries.sh`; extend pattern list for `.dmg`/`.AppImage`/setup |
| MSI/MSIX requested later | Explicit non-goal; reopen PKG-001 |

## 10. Success criteria

- Operator on each OS can produce the primary artifact with one documented script.
- Buyer can install/open Trareon Lab like a normal desktop app on Win/macOS/Linux without manually hunting a raw exe.
- Storefront checklist lists the primary packages; GitHub still has no installer assets.
- Portable Windows zip remains available but is not the only Windows SKU.

## 11. Out of scope follow-ups

- WiX MSI / MSIX
- Notarized macOS / Authenticode as default
- Auto-update
- Linux Flathub / Windows Store listing
