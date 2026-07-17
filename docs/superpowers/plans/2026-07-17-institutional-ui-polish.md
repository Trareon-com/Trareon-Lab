# Institutional UI Polish Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the stub Slint shell with a premium Institutional Dark Lab workstation UI across all six examination screens, with runtime dark/light and ID/EN toggles, responsive layouts, and UX best practices from the accepted spec.

**Architecture:** Modular Slint design system (`Theme` + `Strings` globals, shared components, six screen modules) mounted in a chrome shell (header / sidebar / status). Rust `UiSnapshot` gains `dark_mode` and `locale`; display strings live only in Slint `Strings`. Internal `active-screen` keys stay English (`NavScreen::label()`) for stable wiring.

**Tech Stack:** Slint 1.9, Rust (`apps/lab-slint`), existing `UiSnapshot` / `NavScreen`

**Spec:** `docs/superpowers/specs/2026-07-17-institutional-ui-polish-design.md`

## Global Constraints

- Visual language: Institutional Dark Lab — charcoal/navy, teal accent `#3D9B8F`, borders not glow/shadows, mono for forensic strings.
- Default theme: dark (`dark_mode = true`). Default locale: `"en"`. Invalid locale coerces to `"en"`.
- All six screens: Case, Evidence, Search, Timeline, Bookmarks, Report — polished in this plan.
- No new parsers/backends; empty/honest stubs only; export must not fake success.
- Responsive per spec §11b: preferred window 1280×800, min ~1100×680; Evidence stacks when narrow; body scrolls via Flickable.
- UX: one primary CTA per context; ≥28px nav/toggle hit targets; no hard-coded chrome literals outside `Strings`.
- Keep `NavScreen::label()` English; do not break existing smoke tests.
- After code changes: `graphify update .`
- Commits: only when the user explicitly asks to commit (do not auto-commit during execution unless instructed).

---

## File map

| File | Responsibility |
|---|---|
| `apps/lab-slint/src/ui_model.rs` | Add `dark_mode`, `locale`, `set_dark_mode`, `set_locale` |
| `apps/lab-slint/tests/prefs_ui.rs` | Model tests for theme/locale defaults and coercion |
| `apps/lab-slint/ui/theme.slint` | `global Theme` token properties + `apply(dark: bool)` |
| `apps/lab-slint/ui/strings.slint` | `global Strings` all chrome copy for `en` / `id` |
| `apps/lab-slint/ui/components/nav_item.slint` | Sidebar row (≥28px, accent bar when active) |
| `apps/lab-slint/ui/components/status_badge.slint` | Case/report state chip |
| `apps/lab-slint/ui/components/empty_state.slint` | Title + body + optional CTA |
| `apps/lab-slint/ui/components/metric_tile.slint` | Count tile for Case overview |
| `apps/lab-slint/ui/components/section_card.slint` | Bordered surface panel |
| `apps/lab-slint/ui/components/lab_button.slint` | Primary/secondary themed button |
| `apps/lab-slint/ui/screens/case_screen.slint` | 3-column Case funnel |
| `apps/lab-slint/ui/screens/evidence_screen.slint` | Responsive list/detail |
| `apps/lab-slint/ui/screens/search_screen.slint` | Query + results |
| `apps/lab-slint/ui/screens/timeline_screen.slint` | Event list |
| `apps/lab-slint/ui/screens/bookmarks_screen.slint` | Findings/bookmarks list |
| `apps/lab-slint/ui/screens/report_screen.slint` | Report state + honest export stub |
| `apps/lab-slint/ui/app.slint` | Window chrome + screen switch + property bridge |
| `apps/lab-slint/src/main.rs` | Bind prefs, lists, theme/locale callbacks, demo rows |
| `apps/lab-slint/tests/ui_smoke.rs` | Unchanged expectations (verify still pass) |

---

### Task 1: Theme & locale on `UiSnapshot` (TDD)

**Files:**
- Create: `apps/lab-slint/tests/prefs_ui.rs`
- Modify: `apps/lab-slint/src/ui_model.rs`
- Test: `apps/lab-slint/tests/prefs_ui.rs`

**Interfaces:**
- Consumes: `UiSnapshot` default
- Produces:
  - `pub dark_mode: bool` (default `true`)
  - `pub locale: String` (default `"en"`)
  - `pub fn set_dark_mode(&mut self, dark: bool)`
  - `pub fn set_locale(&mut self, locale: impl Into<String>)` — if not `"en"` or `"id"`, set `"en"`

- [ ] **Step 1: Write the failing test**

```rust
//! Theme + locale prefs on UiSnapshot.

use lab_slint::UiSnapshot;

#[test]
fn defaults_are_dark_and_english() {
    let snap = UiSnapshot::default();
    assert!(snap.dark_mode);
    assert_eq!(snap.locale, "en");
}

#[test]
fn set_dark_mode_round_trips() {
    let mut snap = UiSnapshot::default();
    snap.set_dark_mode(false);
    assert!(!snap.dark_mode);
    snap.set_dark_mode(true);
    assert!(snap.dark_mode);
}

#[test]
fn set_locale_accepts_id_and_en() {
    let mut snap = UiSnapshot::default();
    snap.set_locale("id");
    assert_eq!(snap.locale, "id");
    snap.set_locale("en");
    assert_eq!(snap.locale, "en");
}

#[test]
fn set_locale_coerces_invalid_to_en() {
    let mut snap = UiSnapshot::default();
    snap.set_locale("fr");
    assert_eq!(snap.locale, "en");
    snap.set_locale("");
    assert_eq!(snap.locale, "en");
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p lab-slint --test prefs_ui -- --nocapture`  
Expected: FAIL (missing fields / methods)

- [ ] **Step 3: Implement minimal model changes**

In `UiSnapshot` add fields + default + methods:

```rust
pub dark_mode: bool,
pub locale: String,

// in Default:
dark_mode: true,
locale: "en".into(),

pub fn set_dark_mode(&mut self, dark: bool) {
    self.dark_mode = dark;
}

pub fn set_locale(&mut self, locale: impl Into<String>) {
    let loc = locale.into();
    self.locale = if loc == "en" || loc == "id" {
        loc
    } else {
        "en".into()
    };
}
```

- [ ] **Step 4: Run tests to verify pass**

Run: `cargo test -p lab-slint --test prefs_ui`  
Expected: PASS

Also run: `cargo test -p lab-slint --test ui_smoke`  
Expected: PASS (unchanged)

- [ ] **Step 5: Stop for review** (commit only if user asked)

---

### Task 2: `theme.slint` + `strings.slint` globals

**Files:**
- Create: `apps/lab-slint/ui/theme.slint`
- Create: `apps/lab-slint/ui/strings.slint`
- Modify: `apps/lab-slint/ui/app.slint` (import only; full chrome in Task 4)

**Interfaces:**
- Produces: `global Theme` with color properties matching spec §4.2; `public function apply(dark: bool)`
- Produces: `global Strings` with `in-out property <string> locale` and every chrome string as `property <string> ...` derived from locale
- Consumes: nothing from Rust yet

- [ ] **Step 1: Create `theme.slint`** with exact token names from the spec:

```slint
export global Theme {
    in-out property <bool> dark: true;
    in-out property <color> canvas: #0F1218;
    in-out property <color> surface: #171B24;
    in-out property <color> surface-raised: #1E2430;
    in-out property <color> border: #2A3140;
    in-out property <color> text-primary: #E8ECF2;
    in-out property <color> text-secondary: #9AA3B5;
    in-out property <color> accent: #3D9B8F;
    in-out property <color> accent-muted: #2A6B63;
    in-out property <color> danger: #C45C5C;
    in-out property <color> warning: #C9A227;
    in-out property <color> success: #4F9F6A;
    in-out property <color> info: #5B8FD9;
    in-out property <color> row-hover: #222836;
    in-out property <color> nav-active-bar: #3D9B8F;
    in-out property <color> mono-fg: #B8C0D0;
    in-out property <color> status-bar-bg: #12161E;

    public function apply(is-dark: bool) {
        root.dark = is-dark;
        if is-dark {
            canvas = #0F1218;
            surface = #171B24;
            surface-raised = #1E2430;
            border = #2A3140;
            text-primary = #E8ECF2;
            text-secondary = #9AA3B5;
            accent = #3D9B8F;
            accent-muted = #2A6B63;
            danger = #C45C5C;
            warning = #C9A227;
            success = #4F9F6A;
            info = #5B8FD9;
            row-hover = #222836;
            nav-active-bar = #3D9B8F;
            mono-fg = #B8C0D0;
            status-bar-bg = #12161E;
        } else {
            canvas = #F4F6F9;
            surface = #FFFFFF;
            surface-raised = #EEF1F6;
            border = #D8DEE8;
            text-primary = #12151C;
            text-secondary = #5A6478;
            accent = #2F7F75;
            accent-muted = #C5E4DF;
            danger = #B04040;
            warning = #A8841A;
            success = #2F7A4A;
            info = #3A6FB5;
            row-hover = #E4E9F1;
            nav-active-bar = #2F7F75;
            mono-fg = #3A4254;
            status-bar-bg = #E8ECF2;
        }
    }
}
```

(If Slint rejects `root.` inside global, assign properties without `root.` — use whatever compiles on Slint 1.9.)

- [ ] **Step 2: Create `strings.slint`** covering at least these keys (EN / ID):

| Property | EN | ID |
|---|---|---|
| `brand` | Trareon Lab | Trareon Lab |
| `nav-case` | Case | Kasus |
| `nav-evidence` | Evidence | Bukti |
| `nav-search` | Search | Cari |
| `nav-timeline` | Timeline | Linimasa |
| `nav-bookmarks` | Bookmarks | Markah |
| `nav-report` | Report | Laporan |
| `open-case` | Open Case | Buka Kasus |
| `import-evidence` | Import Evidence | Impor Bukti |
| `theme-dark` | Dark | Gelap |
| `theme-light` | Light | Terang |
| `lang-en` | EN | EN |
| `lang-id` | ID | ID |
| `case-overview` | Case Overview | Ringkasan Kasus |
| `evidence-overview` | Evidence Overview | Ringkasan Bukti |
| `places-to-start` | Places to Start | Mulai Dari |
| `status-evidence` | Evidence | Bukti |
| `status-coverage` | Coverage | Cakupan |
| `status-bookmarks` | Bookmarks | Markah |
| `empty-no-case` | No case open | Belum ada kasus |
| `empty-no-evidence` | No evidence imported | Belum ada bukti |
| `empty-no-selection` | Select a file | Pilih berkas |
| `empty-no-search` | No search results | Tidak ada hasil |
| `empty-no-timeline` | No timeline events | Tidak ada peristiwa |
| `empty-no-bookmarks` | No bookmarks yet | Belum ada markah |
| `empty-no-findings` | No findings yet | Belum ada temuan |
| `search-placeholder` | Search artifacts… | Cari artefak… |
| `search-action` | Search | Cari |
| `hits` | hits | hasil |
| `export-soon` | Export — coming soon | Ekspor — segera hadir |
| `report-state` | Report state | Status laporan |
| `col-name` | Name | Nama |
| `col-size` | Size | Ukuran |
| `col-path` | Path | Jalur |
| `col-kind` | Kind | Jenis |
| `col-summary` | Summary | Ringkasan |
| `col-provenance` | Provenance | Provenans |
| `deleted` | deleted | terhapus |
| `about-prefix` | About | Tentang |

Implement as:

```slint
export global Strings {
    in-out property <string> locale: "en";
    property <bool> is-id: locale == "id";

    property <string> brand: "Trareon Lab";
    property <string> nav-case: is-id ? "Kasus" : "Case";
    // ... all keys from the table above
}
```

- [ ] **Step 3: Smoke-compile by importing from `app.slint`**

Temporarily add at top of `app.slint`:

```slint
import { Theme } from "theme.slint";
import { Strings } from "strings.slint";
```

Run: `cargo build -p lab-slint --features gui`  
Expected: SUCCESS (or fix Slint syntax until it compiles)

- [ ] **Step 4: Stop for review**

---

### Task 3: Shared components

**Files:**
- Create: `apps/lab-slint/ui/components/nav_item.slint`
- Create: `apps/lab-slint/ui/components/status_badge.slint`
- Create: `apps/lab-slint/ui/components/empty_state.slint`
- Create: `apps/lab-slint/ui/components/metric_tile.slint`
- Create: `apps/lab-slint/ui/components/section_card.slint`
- Create: `apps/lab-slint/ui/components/lab_button.slint`

**Interfaces:**
- Consumes: `Theme`, `Strings`
- Produces: reusable components used by chrome + screens

- [ ] **Step 1: Implement `nav_item.slint`**

Requirements: min-height 28px; left accent bar when `active`; hover `Theme.row-hover`; `clicked` callback; label + optional glyph text.

```slint
import { Theme } from "../theme.slint";

export component NavItem inherits Rectangle {
    in-out property <string> label;
    in-out property <string> glyph: "•";
    in-out property <bool> active: false;
    callback clicked();

    height: 32px;
    background: active ? Theme.surface-raised : transparent;
    border-radius: 4px;

    HorizontalLayout {
        padding-left: 8px;
        padding-right: 8px;
        spacing: 10px;
        Rectangle {
            width: 3px;
            height: parent.height - 8px;
            y: 4px;
            background: active ? Theme.nav-active-bar : transparent;
            border-radius: 1px;
        }
        Text {
            text: glyph;
            color: active ? Theme.accent : Theme.text-secondary;
            vertical-alignment: center;
        }
        Text {
            text: label;
            color: active ? Theme.text-primary : Theme.text-secondary;
            font-size: 13px;
            vertical-alignment: center;
            horizontal-stretch: 1;
        }
    }

    ta := TouchArea {
        clicked => { root.clicked(); }
    }
    states [
        hover when ta.has-hover && !active: {
            background: Theme.row-hover;
        }
    ]
}
```

- [ ] **Step 2: Implement remaining components**

- `status_badge.slint`: pill-like rectangle (radius 4px, not full pill), text from `label`, background `accent-muted` or semantic color via `kind` enum/string (`ok`/`warn`/`danger`/`info`/`neutral`).
- `empty_state.slint`: centered title (14px semibold) + body (12px secondary) + optional slot for CTA via `show-cta` + `cta-label` + `cta-clicked`.
- `metric_tile.slint`: label (secondary) + value (20px primary) inside bordered surface.
- `section_card.slint`: `background: Theme.surface`, `border: 1px solid Theme.border`, radius 6px, padding 12–16px, title row + `@children`.
- `lab_button.slint`: `primary` bool — primary fills `Theme.accent` with light text; secondary is bordered surface; height ≥28px; `clicked` callback.

- [ ] **Step 3: Compile**

Run: `cargo build -p lab-slint --features gui`  
Expected: SUCCESS after `app.slint` imports at least one component (or export from a barrel import in Task 4).

- [ ] **Step 4: Stop for review**

---

### Task 4: App chrome shell (header, sidebar, status, toggles)

**Files:**
- Modify: `apps/lab-slint/ui/app.slint` (full rewrite of layout; keep property/callback names that Rust already uses, extend as needed)

**Interfaces:**
- Consumes: Theme, Strings, NavItem, LabButton, StatusBadge
- Produces Window properties (keep existing + add):

```slint
in-out property <string> case-title;
in-out property <int> evidence-count;
in-out property <int> coverage-count;
in-out property <int> bookmark-count;
in-out property <bool> open-case-focused;
in-out property <string> active-screen; // English keys: Case|Evidence|...
in-out property <string> about-line;
in-out property <bool> dark-mode: true;
in-out property <string> locale: "en";
// list props added in later tasks — declare stubs now if needed for compile:
in-out property <[string]> evidence-names: [];
in-out property <[string]> evidence-paths: [];
in-out property <[string]> evidence-sizes: [];
in-out property <[bool]> evidence-deleted: [];
in-out property <int> selected-file-index: -1;
in-out property <string> search-query: "";
in-out property <[string]> search-results: [];
in-out property <[string]> artifact-kinds: [];
in-out property <[string]> artifact-summaries: [];
in-out property <[string]> artifact-provenances: [];
in-out property <[string]> timeline-labels: [];
in-out property <[string]> finding-claims: [];
in-out property <[string]> finding-uuids: [];
in-out property <string> report-state: "draft";

callback open-case-clicked();
callback focus-open-case();
callback navigate(string);
callback import-evidence-clicked();
callback theme-toggled(bool);
callback locale-toggled(string);
callback search-submitted();
callback evidence-row-selected(int);
```

Layout structure:

```text
VerticalLayout
  Header (height ~48)
  HorizontalLayout (stretch)
    Sidebar (width: min(220px, max(180px, ...)) — use 200px fixed first; document shrink)
    Rectangle screen body (Flickable child per screen visibility)
  Status bar (height ~28)
```

- [ ] **Step 1: Rewrite `app.slint` chrome**

Key behaviors:
- `preferred-width: 1280px; preferred-height: 800px; min-width: 1100px; min-height: 680px;`
- `background: Theme.canvas;`
- On `dark-mode` change: `Theme.apply(dark-mode)` + set Palette color-scheme:

```slint
import { Palette, ColorScheme } from "std-widgets.slint";
// when toggling:
Palette.color-scheme = dark-mode ? ColorScheme.dark : ColorScheme.light;
Theme.apply(dark-mode);
```

- On `locale` change: `Strings.locale = locale;`
- Header: brand (`Strings.brand`) | case title mono | StatusBadge for open/created | ID/EN toggle | Dark/Light toggle
- Sidebar: six `NavItem`s using `Strings.nav-*`; `active: active-screen == "Case"` etc.; `clicked => navigate("Case")` (English keys)
- Status: localized prefixes + counts + truncated about-line
- Screen body: for now show a placeholder `Text { text: active-screen; }` until Task 5–10 replace with screen components
- `focus-open-case` still focuses the Open Case control (will live on Case screen — expose `open-btn` via forward or keep a hidden focus target in chrome until Case screen lands)

- [ ] **Step 2: Extend `apply` in `main.rs` for dark-mode/locale**

```rust
ui.set_dark_mode(snap.dark_mode);
ui.set_locale(snap.locale.clone().into());
ui.set_about_line(snap.about_disclosure.clone().into());
```

Wire:

```rust
ui.on_theme_toggled(move |dark| { /* snap.set_dark_mode(dark); apply */ });
ui.on_locale_toggled(move |loc| { /* snap.set_locale(loc); apply */ });
```

- [ ] **Step 3: Build and run smoke tests**

Run: `cargo test -p lab-slint`  
Run: `cargo build -p lab-slint --features gui`  
Expected: PASS / SUCCESS

Manual: `cargo run -p lab-slint --bin trareon-lab` — toggle theme/language; sidebar navigates; window opens at ~1280×800.

- [ ] **Step 4: Stop for review**

---

### Task 5: Case screen (AXIOM funnel + responsive columns)

**Files:**
- Create: `apps/lab-slint/ui/screens/case_screen.slint`
- Modify: `apps/lab-slint/ui/app.slint` (swap placeholder for CaseScreen when `active-screen == "Case"`)

**Interfaces:**
- Consumes: SectionCard, MetricTile, LabButton, EmptyState, StatusBadge, Theme, Strings
- Produces: CaseScreen with callbacks `open-case-clicked`, `import-evidence-clicked`, `navigate(string)` forwarded to root

- [ ] **Step 1: Implement three-column funnel**

```slint
export component CaseScreen inherits Rectangle {
    in-out property <string> case-title;
    in-out property <int> evidence-count;
    in-out property <int> coverage-count;
    in-out property <int> bookmark-count;
    in-out property <string> about-line;
    callback open-case-clicked();
    callback import-evidence-clicked();
    callback navigate(string);

    background: transparent;

    HorizontalLayout {
        padding: 12px;
        spacing: 12px;
        // Column 1 Case Overview
        // Column 2 Evidence Overview (metrics + Import secondary)
        // Column 3 Places to Start (Nav-like tiles calling navigate)
    }
}
```

Responsive: use `HorizontalLayout` with `horizontal-stretch: 1` on each column so they share width evenly and shrink together under 1200px without clipping Open Case. If Slint layout still overflows at min width, wrap the three columns in a `Flickable` with horizontal viewport only as last resort; prefer vertical stacking when `root.width < 700px` **of the screen body** via conditional:

```slint
if body-wide: HorizontalLayout { ... }
if !body-wide: VerticalLayout { ... }
property <bool> body-wide: self.width >= 900px;
```

(Measure screen component width, not full window.)

- [ ] **Step 2: Wire Open Case as primary** (`LabButton { primary: true; }`) and keep `focus-open-case` path working from `main.rs` (`ui.invoke_focus_open_case()`).

- [ ] **Step 3: Build + manual check Case at 1280 and ~1100 width**

Expected: no clipped primary CTA; Places to Start readable in ID and EN.

- [ ] **Step 4: Stop for review**

---

### Task 6: Evidence screen (list/detail + stack when narrow)

**Files:**
- Create: `apps/lab-slint/ui/screens/evidence_screen.slint`
- Modify: `apps/lab-slint/ui/app.slint`, `apps/lab-slint/src/main.rs`

**Interfaces:**
- Consumes: parallel arrays `evidence-names/paths/sizes/deleted`, `selected-file-index`
- Produces: `evidence-row-selected(int)` callback

- [ ] **Step 1: Implement EvidenceScreen**

- Wide (`width >= 640`): HorizontalLayout list 40% / detail 60%
- Narrow: VerticalLayout list then detail
- List rows: name + deleted badge; hover/selected use Theme tokens; height ≥28px
- Detail: path mono (`Theme.mono-fg`, font monospace), size, deleted
- Empty: `EmptyState` with Import CTA when `evidence-names.length == 0`

- [ ] **Step 2: Seed demo files in `main.rs` after open_case** so the list is not always empty in the GUI binary:

```rust
snap.set_evidence_files(vec![
    EvidenceFileRow {
        path: "/evidence/disk.E01".into(),
        name: "disk.E01".into(),
        size: 4_294_967_296,
        deleted: false,
    },
    EvidenceFileRow {
        path: "/evidence/unalloc.bin".into(),
        name: "unalloc.bin".into(),
        size: 1024,
        deleted: true,
    },
]);
// then sync arrays to Slint properties in apply()
```

Helper in `apply`:

```rust
fn sync_evidence(ui: &AppWindow, snap: &UiSnapshot) {
    // map Vec<EvidenceFileRow> into Slint ModelRc / Vec shared as properties
}
```

Use `slint::ModelRc` + `VecModel` if `in-out property <[string]>` binding from Rust requires it (Slint 1.9 pattern). If parallel string arrays are awkward, a single `StandardListView` model is acceptable for names with detail resolved by index from Rust on selection.

- [ ] **Step 3: Wire `on_evidence_row_selected` → `snap.select_file(i)` → re-apply**

- [ ] **Step 4: `cargo test -p lab-slint` + manual resize Evidence pane**

Expected: stacks when narrow; selection highlight works.

- [ ] **Step 5: Stop for review**

---

### Task 7: Search screen

**Files:**
- Create: `apps/lab-slint/ui/screens/search_screen.slint`
- Modify: `apps/lab-slint/ui/app.slint`, `apps/lab-slint/src/main.rs`

**Interfaces:**
- Consumes: `search-query`, `search-results`, `artifact-kinds/summaries/provenances`
- Produces: `search-submitted` → Rust may call `snap.set_search(...)` stub results for demo

- [ ] **Step 1: Implement SearchScreen** — LineEdit + LabButton primary; results table; footer `N Strings.hits`
- Prefer artifact_* rows when length > 0; else search-results strings
- Empty state when query submitted and zero hits (track with `in-out property <bool> search-attempted` set true on submit)

- [ ] **Step 2: Demo stub in main on submit:**

```rust
snap.set_search(query, vec!["hit: browser history".into(), "hit: prefetch".into()]);
// optional: also set_artifact_hits for richer table
```

- [ ] **Step 3: Build + toggle ID/EN on Search labels**

- [ ] **Step 4: Stop for review**

---

### Task 8: Timeline screen

**Files:**
- Create: `apps/lab-slint/ui/screens/timeline_screen.slint`
- Modify: `apps/lab-slint/ui/app.slint`, `apps/lab-slint/src/main.rs`

- [ ] **Step 1: Dense list of `timeline-labels`** (mono timestamp prefix if label contains `|`, else whole line mono/secondary)
- Thin filter strip UI: disabled LineEdit placeholder localized — visual only
- EmptyState when empty
- Seed demo timeline in main for GUI:

```rust
snap.set_timeline(vec![
    "2026-07-17 10:02:11 | FILE | disk.E01 mounted".into(),
    "2026-07-17 10:05:44 | ART  | Chrome History parsed".into(),
]);
```

Note: `set_timeline` navigates to Timeline — seed before first paint or set fields without navigate if adding a non-navigating setter; prefer assigning `snap.timeline_labels = ...` directly in main setup to avoid stealing Case home on launch.

- [ ] **Step 2: Build + manual**

- [ ] **Step 3: Stop for review**

---

### Task 9: Bookmarks screen

**Files:**
- Create: `apps/lab-slint/ui/screens/bookmarks_screen.slint`
- Modify: `apps/lab-slint/ui/app.slint`, `apps/lab-slint/src/main.rs`

- [ ] **Step 1: List `finding-claims` + `finding-uuids` (mono)**; show bookmark-count in header strip
- EmptyState when no findings and bookmark-count == 0
- Seed one finding in demo main via `set_findings` without forcing Report navigation

- [ ] **Step 2: Build + ID/EN**

- [ ] **Step 3: Stop for review**

---

### Task 10: Report screen

**Files:**
- Create: `apps/lab-slint/ui/screens/report_screen.slint`
- Modify: `apps/lab-slint/ui/app.slint`

- [ ] **Step 1: StatusBadge for `report-state`; findings list; LabButton secondary disabled OR enabled but only showing `Strings.export-soon` without file IO**

Honest behavior:

```slint
LabButton {
    primary: false;
    label: Strings.export-soon;
    clicked => { /* no-op or set a status text — must not write files */ }
}
```

- [ ] **Step 2: Build + verify no export side effects**

- [ ] **Step 3: Stop for review**

---

### Task 11: Wire-up completeness, UX pass, regression tests

**Files:**
- Modify: `apps/lab-slint/src/main.rs`, any screen with leftover hard-coded English
- Modify: `apps/lab-slint/ui/**/*.slint` (grep for literal user strings)

**Interfaces:**
- Consumes: all prior screens
- Produces: shippable GUI binary behavior

- [ ] **Step 1: Grep for hard-coded chrome English**

Run: `rg -n '"Case"|"Evidence"|"Open Case"|"Search"' apps/lab-slint/ui --glob '*.slint'`  
Expected: only English keys for `active-screen` comparisons and `navigate("...")` arguments — no user-visible literals outside `strings.slint`.

- [ ] **Step 2: Ensure `apply` syncs every list + prefs; navigation still uses English keys**

- [ ] **Step 3: Full test suite**

Run: `cargo test -p lab-slint`  
Expected: all PASS including `prefs_ui`, `ui_smoke`, existing tests

- [ ] **Step 4: Manual UX checklist (both themes × both locales)**

1. Case funnel readable; Open Case focus on launch  
2. Sidebar active state tracks screen  
3. Evidence select + narrow stack  
4. Search submit shows hits + count  
5. Timeline / Bookmarks / Report empty or demo data look intentional  
6. Theme toggle instant; Light mode contrast OK  
7. Locale ID updates all chrome  
8. Resize 1100→1400: no clipped CTAs  
9. Export does not create files  

- [ ] **Step 5: `graphify update .`**

- [ ] **Step 6: Stop for review** (commit when user requests)

---

## Spec coverage checklist (plan self-review)

| Spec requirement | Task |
|---|---|
| Institutional tokens + dark default | 2, 4 |
| Light theme toggle | 4 |
| ID/EN toggle + Strings global | 2, 4, 11 |
| Modular file layout | 2–10 |
| Case funnel 3-column | 5 |
| Evidence split + responsive stack | 6 |
| Search / Timeline / Bookmarks / Report | 7–10 |
| Honest export stub | 10 |
| UiSnapshot dark_mode/locale + coerce | 1 |
| Keep NavScreen::label English / smoke tests | 1, 4, 11 |
| Window 1280×800 / min 1100×680 | 4 |
| UX §11b (CTA, focus, density, hit targets) | 3–6, 11 |
| No backend feature creep | Global + 10 |
| graphify update | 11 |

**Placeholder scan:** none intentional.  
**Type consistency:** `active-screen` / `navigate` use English keys `Case|Evidence|Search|Timeline|Bookmarks|Report`; locale is `"en"`|`"id"`; `selected-file-index` uses `-1` for none in Slint (map from `Option<usize>` in Rust).
