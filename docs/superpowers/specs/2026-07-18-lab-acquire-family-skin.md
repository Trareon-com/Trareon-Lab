# Spec: Lab Acquire-family skin (2026-07-18)

## Status

**Superseded for Lab chrome (2026-07-19):** Lab now uses the **examination workbench** shell (disclosure · TRAREON|LAB header · 48↔180 rail · inspector Properties|Hex · status+progress · log overlay). See implementation / `apps/lab-slint/tests/lab_workbench_chrome.rs`.

**Acquire product shell** (TRAREON|ACQUIRE · Prepare→Acquire→Seal wizard) remains a **separate future** design — not this Lab remount.

## Goal (historical)

Align Trareon Lab chrome with Trareon Acquire *Evidence Control Room* visuals, then remount Lab examination features with optimized UX.

## V2 remount (2026-07-19) — superseded

- Shell was **AcquireChrome**: sidebar + header strip + body + footer (+ inspector drawer).
- Sidebar IA: **Case · Evidence · Search · Timeline · Bookmarks · Report** + text Tools (Quick Verify · Runs · Transfer).
- Header: **CASE · COVERAGE · INTEGRITY · UNSIGNED LAB** (compact chip); live coverage/evidence counts; tap CASE/COVERAGE → Case Home.
- Case Home: state-bound stepper Open→Import→Examine→Bookmark→Report; **one primary CTA**; no Guided/Standard/Expert pills; no host-disk / fsnap skeleton.
- Inspector: off by default; opens on evidence selection / `i`; Esc closes.
- Meta (Capabilities / About / Hex / …): palette + PlaceholderScreen.
- System healthy: sidebar only (not footer).
- Geometry: sidebar `248px`, header `76px`, footer `36px` (1280×800 target).
- Mockup canon: `docs/design/mockups/mockup-lab-case-home-optimized.png`.

## Tokens

- Font: bundled **Inter** (`apps/lab-slint/ui/assets/fonts/`, OFL)
- Icons: colorizable PNG masks via Slint `Image.colorize`
- Light default: canvas `#F5F6F8`, accent `#B95410` / `#C05600`, warn `#F8F3EF`
- Dark via Settings (footer), not default
- Reference: `docs/design/trareon-acquire-reference.png`

## Chrome mapping (Lab-honest)

| Acquire (visual) | Lab |
|------------------|-----|
| Brand sidebar | TRAREON LAB + EVIDENCE CONTROL ROOM |
| Write-blocker zone | **Coverage** status |
| Integrity zone | SHA-256 policy chip |
| UNSIGNED zone | Compact tan chip |
| Stepper | Open → Import → Examine → Bookmark → Report (Case Home only) |
| Footer | Time / User / Workstation · Lab Log · Settings · locale |

## Non-goals

- Hardware Tableau write-blocker UI
- Boot media / Identify hardware product claims
- Fake AFF4/ZFF acquisition as functional surface
- Streaming carver, API, plugin work

## Verification

- `cargo test -p lab-slint --features gui` (includes `acquire_chrome`)
- Recapture `docs/media/feature-home.png` / `lab-ui-case-home.png`
- Examiner path: Open → Import → Evidence → Search/Bookmark → Report
