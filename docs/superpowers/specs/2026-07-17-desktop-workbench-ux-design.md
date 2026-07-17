# Design: Desktop workbench UX (Slint native)

**Date:** 2026-07-17  
**Status:** ACCEPTED (plan execution)  
**Stack:** `apps/lab-slint` (Slint + Rust) — **no** React/web rewrite  
**Supersedes (chrome/density):** portions of `2026-07-17-institutional-ui-polish-design.md`  
**Keeps:** Institutional Dark Lab tokens, honesty rules, EN/ID strings, `NavScreen::label()` English keys

## 1. Problem

The institutional polish made Trareon Lab look like a product, but interaction still feels like a form shell: full-screen page swaps, detail embedded per screen, keyboard hints not wired, no command palette, no chrome-level inspector.

Examiners need a **workbench** (VS Code / Linear / Figma density), not a marketing dashboard.

## 2. Goals

- Five-region chrome: TopBar · Nav (collapsible) · Main · Inspector · Status/Log
- Keyboard-first: FocusScope global shortcuts + command palette
- Dense tables/lists (24–28px rows, 4/8 spacing)
- Selection survives navigation via inspector
- Dark mode primary; accent only for actions/active
- Keep existing tests green; extend keyboard/palette coverage

## 3. Non-goals

- React/Tauri/shadcn rewrite
- Real timeline engine, hex viewer, PDF export
- Prefs persistence to disk
- Fake data theater

## 4. Layout

```text
┌─ TopBar: brand · case · ⌘K command · import · prefs ───────────────┐
├─ Nav 48|200 ─┬─ Main (flex) ──────────────┬─ Inspector 280–360 ────┤
├──────────────┴────────────────────────────┴────────────────────────┤
│ Status metrics · last action                    [Log ▾] ring buffer│
└────────────────────────────────────────────────────────────────────┘
```

## 5. Shortcuts

| Key | Action |
|---|---|
| `/` or palette open | Command palette |
| `Ctrl/Cmd+O` (via palette / Open) | Open case |
| `Ctrl/Cmd+I` (via palette / Import) | Import |
| `1`–`6` | Jump screens |
| `b` | Bookmark selection |
| `Escape` | Close palette / clear selection |
| Toggle nav / log / inspector | Via palette commands + chrome buttons |

## 6. Density tokens

- Chrome pad 8px; row height 26px; section titles 12px
- Prefer pane borders over nested cards
- Focus: accent left bar / 1px border on focused control

## 7. Case Home

- No case: single onboarding plate (unchanged intent)
- Case open: dense strip — next action + metrics + activity lines (no metric-tile wall)

## 8. Honesty

Timeline empty = not in v1; export = coming soon; status/log show real errors.
