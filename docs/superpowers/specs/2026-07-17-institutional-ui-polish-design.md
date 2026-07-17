# Design: Institutional UI polish (all examination screens)

**Date:** 2026-07-17  
**Status:** ACCEPTED (user approved written spec 2026-07-17; includes responsive + best-practice UX)  
**Decision:** Approach 2 — modular Slint design system + Institutional Dark Lab visual language  
**Stack:** `apps/lab-slint` (Slint + Rust `UiSnapshot`)  
**Related:** `Research/Digital-Forensic-Analysis-Best-Practices.md`, Magnet AXIOM Case Dashboard patterns, Infisical-style dense calm dark UI

## 1. Problem

The Foundation Slint shell (`apps/lab-slint/ui/app.slint`) is a stub: default buttons, plain text, no workstation layout. It does not read as a sellable digital-forensics product and cannot compete visually with Magnet AXIOM, FTK, or Oxygen Forensic Detective.

Buyers judge forensic tools in the first seconds of a demo. The UI must look like a paid lab workstation: dark-native, dense, trustworthy, and bilingual (ID/EN), with an explicit light theme for daytime lab use.

## 2. Goals

- Polish **all six** examination screens to a premium institutional-lab standard: Case, Evidence, Search, Timeline, Bookmarks, Report.
- Ship a coherent **design system** (tokens, chrome, shared components) rather than one-off styling.
- Runtime **dark / light** theme toggle in the header.
- Runtime **Indonesian / English** language toggle in the header; every user-visible chrome string goes through locale.
- Keep existing `UiSnapshot` / navigation / open-case / import-evidence contracts and smoke tests green; extend the model only where theme/locale/screen structure require it.
- Stay on Slint (no rewrite to Tauri/web for this work).

## 3. Non-goals

- New parsers, real search index, real timeline engine, hex viewer, graph visualizations, or report PDF export.
- Magic Patterns / Figma pipeline (MCP not required; implement directly in Slint).
- Persisting theme/locale to disk (in-memory for this pass; disk prefs are a follow-up).
- Changing GPL disclosure, selling model, or backend case schema.
- Purple/glow “hacker” aesthetics, marketing landing-page chrome, or card-heavy dashboard kitsch.

## 4. Visual language — Institutional Dark Lab

### 4.1 Principles

- Dark is the **default** native medium; light is a first-class alternate, not an afterthought.
- Depth via **surface layers + 1px borders**, not multi-layer shadows or glow.
- One muted teal accent for primary actions and active navigation.
- Semantic colors only for status (success / warning / danger / info).
- Sans for UI chrome; **monospace** for hashes, paths, offsets, timestamps, case IDs.
- Dense but calm: tight row rhythm in tables; quiet empty states (no emoji clutter).
- Corner radius 4–6px; no pill clusters; no floating badges over content.

### 4.2 Token set (`Theme` global in Slint)

| Token | Dark | Light |
|---|---|---|
| `canvas` | `#0F1218` | `#F4F6F9` |
| `surface` | `#171B24` | `#FFFFFF` |
| `surface-raised` | `#1E2430` | `#EEF1F6` |
| `border` | `#2A3140` | `#D8DEE8` |
| `text-primary` | `#E8ECF2` | `#12151C` |
| `text-secondary` | `#9AA3B5` | `#5A6478` |
| `accent` | `#3D9B8F` | `#2F7F75` |
| `accent-muted` | `#2A6B63` | `#C5E4DF` |
| `danger` | `#C45C5C` | `#B04040` |
| `warning` | `#C9A227` | `#A8841A` |
| `success` | `#4F9F6A` | `#2F7A4A` |
| `info` | `#5B8FD9` | `#3A6FB5` |
| `row-hover` | `#222836` | `#E4E9F1` |
| `nav-active-bar` | `#3D9B8F` | `#2F7F75` |
| `mono-fg` | `#B8C0D0` | `#3A4254` |
| `status-bar-bg` | `#12161E` | `#E8ECF2` |

Implementation: a Slint `global Theme` with properties rebound when `dark-mode` flips. Additionally set `Palette.color-scheme` so std-widgets (Button, LineEdit, etc.) follow the same scheme.

### 4.3 Typography

- UI: platform default Slint sans (or bundled Inter-equivalent if already available — do **not** add a new font dependency unless required for readability).
- Mono: Slint monospace for forensic strings.
- Sizes: title 18–20px, section 13–14px semibold, body 12–13px, mono meta 11–12px, status bar 11px.

## 5. Information architecture

### 5.1 Chrome (every screen)

```text
┌─ Header: brand | case title + state badge ──────── [ID|EN] [Dark|Light] ─┐
├─ Sidebar (220px) ─┬─ Screen body ────────────────────────────────────────┤
│  Case             │                                                      │
│  Evidence         │                                                      │
│  Search           │                                                      │
│  Timeline         │                                                      │
│  Bookmarks        │                                                      │
│  Report           │                                                      │
├───────────────────┴─ Status: evidence · coverage · bookmarks · about ────┤
```

- Window: preferred size **1280×800** (minimum ~1100×680) so the three-column Case funnel and Evidence split pane are usable; replace the current 880×520 stub size.
- Sidebar: icon glyph (simple unicode/shape) + localized label; active item shows accent bar on the left + `surface-raised` fill.
- Header toggles: segmented `ID | EN` and `Dark | Light` (localized accessible labels, not emoji-only).
- Status bar always visible; shows counts from `UiSnapshot` and a short about/disclosure line (localized prefix + existing disclosure text).

### 5.2 Screen layouts

Wire each screen to existing `UiSnapshot` fields. Where data is empty, show a deliberate empty state (title + one sentence + primary CTA if applicable)—never a blank gray void.

#### Case (CaseHome) — AXIOM-style funnel

Three columns, left → right:

1. **Case Overview** — title, state badge, Open Case (primary, focusable), about/disclosure snippet.
2. **Evidence Overview** — evidence count, coverage count, Import Evidence action.
3. **Places to Start** — shortcut tiles that call `navigate(...)` to Evidence / Search / Timeline / Bookmarks / Report (disabled styling when count is zero where that makes sense).

#### Evidence

Split pane:

- Left (~40%): file list from `evidence_files` (name, size, deleted flag).
- Right (~60%): selected file detail — path (mono), size, deleted state; empty “no selection” / “no evidence” states.
- Keyboard selection behavior already modeled in Rust remains authoritative; UI binds selection index.

#### Search

- Top: query field bound to `search_query` + Search affordance (Enter / button).
- Body: results table — prefer structured rows when `artifact_hits` present (kind / summary / provenance_ref mono); else fall back to `search_results` strings.
- Footer strip: hit count.

#### Timeline

- Dense vertical list of `timeline_labels` (treat each as one event row for this pass).
- Optional thin filter strip UI (visual only / stub) — no new backend filters.
- Empty state when no events.

#### Bookmarks

- List of `findings` (claim + bookmark_uuid mono) and/or bookmark count summary.
- Empty state when none.

#### Report

- Report state badge (`report_state`: draft / etc.).
- Findings list (same source as bookmarks/findings).
- Export/generate control may remain stubbed (callback or disabled with honest label “Export — coming soon” localized)—must not pretend to write a file.

## 6. Theme toggle

- Property: `dark-mode: bool` on root (default `true`).
- Header control flips it; updates `Theme` tokens and `Palette.color-scheme` (`ColorScheme.dark` / `.light`).
- All custom rectangles/text use `Theme.*` tokens, not hard-coded hex in screen files.
- No app restart required.

## 7. Language toggle (ID / EN)

- Property: `locale: string` — exactly `"en"` or `"id"` (default **`"en"`** for international demos; user can switch to ID immediately).
- Slint `global Strings` (or `Locale`) exposes every chrome label as properties recomputed from `locale` (functions or conditional bindings).
- Covered strings include: nav labels, header toggles, screen titles, column headers, button labels (Open Case, Import Evidence, Search, …), empty-state copy, status-bar prefixes, Places to Start titles.
- Rust `NavScreen::label()` keeps English statics for API/test stability. Display labels come only from Slint `Strings`, not from `NavScreen::label()`.

No gettext/`@tr` pipeline required for two languages in this pass; a property map is enough and stays offline-simple.

## 8. Module layout (Slint)

```text
apps/lab-slint/ui/
  app.slint              # Window shell: header, sidebar, status, screen switch
  theme.slint            # global Theme + apply_scheme()
  strings.slint          # global Strings / locale
  components/
    nav_item.slint
    status_badge.slint
    empty_state.slint
    metric_tile.slint
    section_card.slint        # bordered surface panel used by Case columns
  screens/
    case_screen.slint
    evidence_screen.slint
    search_screen.slint
    timeline_screen.slint
    bookmarks_screen.slint
    report_screen.slint
```

Rust `build.rs` continues to compile the root `ui/app.slint` (which imports the rest). Prefer keeping one Window export.

## 9. Rust / model changes

Minimal extensions to `UiSnapshot` (or parallel prefs struct bound into the window):

| Field / API | Purpose |
|---|---|
| `dark_mode: bool` | Theme preference (default true) |
| `locale: String` | `"en"` \| `"id"` (default `"en"`) |
| `set_dark_mode` / `set_locale` | Mutators for tests + wiring |
| Existing nav/open/import/search/timeline/findings APIs | Unchanged behavior |

Window property bindings mirror snapshot fields already used (`case-title`, counts, `active-screen`, etc.). Screen-specific lists (`evidence_files`, `artifact_hits`, …) must be exposed to Slint if not already — wire what the polished layouts need; stub empty vectors are fine.

Callbacks stay: `open-case-clicked`, `focus-open-case`, `navigate(string)`, `import-evidence-clicked`; add only if required: `theme-toggled`, `locale-toggled`, `search-submitted`, `evidence-row-selected(int)`.

## 10. Testing

- Keep existing smoke tests green (`ui_smoke.rs`, case open counts, artifacts/file-list tests).
- Add focused model tests:
  - `set_dark_mode` / `set_locale` round-trip and defaults.
  - Locale is only `"en"` or `"id"` (invalid input coerced or rejected — pick **coerce to `"en"`**).
- No screenshot golden tests required this pass.
- Manual check: toggle theme and language on each of the six screens; verify no hard-coded English left in chrome.

## 11. Implementation sequencing

1. `theme.slint` + `strings.slint` globals.
2. App chrome (header, sidebar, status) with toggles.
3. Case screen funnel.
4. Evidence → Search → Timeline → Bookmarks → Report.
5. Bind list data + empty states.
6. Tests + `graphify update .` after code changes.

## 11b. Responsive layout & UX best practices

The shell must remain usable from the minimum window size (~1100×680) up to large monitors without horizontal clipping of primary actions.

**Responsive rules (Slint layout, not a web breakpoint system):**

| Width / context | Behavior |
|---|---|
| ≥ ~1200px | Full chrome: sidebar 220px + multi-column Case funnel + Evidence split ~40/60 |
| ~1100–1200px | Sidebar may shrink to ~180px; Case columns stay three but with tighter padding (8–10px); Places to Start tiles stack text without overflow |
| Height tight | Status bar and header stay fixed; screen body scrolls (`Flickable`) — never clip primary CTAs above the fold on Case |
| Evidence narrow | If body width cannot sustain split, stack list above detail (vertical) rather than crushing columns below ~280px each |

**UX best practices (mandatory for this polish):**

1. **One primary action per context** — Case: Open Case; Evidence empty: Import; Search: submit query. Secondary actions visually quieter.
2. **Focus & keyboard** — Open Case remains focusable as today; nav items and list rows show clear focus/hover using `row-hover` / accent bar; Tab order: header toggles → sidebar → screen body → status (no focus trap).
3. **Feedback** — Active screen reflected in sidebar; selected evidence row highlighted; search hit count always visible after a search; theme/locale change is instant (no modal).
4. **Honesty** — Empty states explain what to do next; disabled/stub export labeled as unavailable, never fake success.
5. **Readability** — Body text ≥12px; mono meta ≥11px; contrast against canvas/surface meets dark-lab and light-lab legibility (manual check both themes).
6. **Density with rest** — Tables/lists dense (row padding ~6–8px); panels keep 12–16px padding so the UI does not feel cramped or sparse.
7. **Locale completeness** — No user-facing English literals left in chrome when `locale == "id"` (and vice versa for Indonesian-only strings).
8. **Touch-adjacent hit targets** — Sidebar rows and header toggles ≥28px tall for reliable clicking under lab mouse use.

## 12. Success criteria

- First viewport of Case screen reads as a commercial forensic workstation (sidebar + funnel + status), not a widget demo.
- All six screens share the same chrome and tokens; theme and language switches apply everywhere without restart.
- Dark and light both legible (contrast on borders/text checked manually).
- Layout remains usable from minimum window size through large displays per §11b (no clipped primary CTAs; Evidence stacks if needed).
- Existing navigation and open-case focus behavior preserved.
- No new backend features claimed in UI that the binary does not perform (export stub must be honest).

## 13. Risks

| Risk | Mitigation |
|---|---|
| Slint std-widgets look “default” next to custom chrome | Prefer custom-styled buttons/rows for primary chrome; use std-widgets only where they match tokens via Palette |
| Large `.slint` files become unmaintainable | Enforce module split in §8 |
| i18n string drift | Single `Strings` global; no raw user-facing literals in screen files |
| Scope creep into real analysis features | Non-goals §3; empty/honest stubs only |
| Smoke tests couple to English `NavScreen::label()` | UI display strings stay in Slint; Rust labels unchanged unless tests updated deliberately |

## 14. Open follow-ups (out of this implementation unless requested)

- Persist theme/locale to a small prefs file under the case or user config dir.
- Full Slint `@tr` / gettext extraction for more locales.
- Real hex/preview pane on Evidence; chart density on Timeline.
- Commit of this spec + implementation plan after user review of this file.
