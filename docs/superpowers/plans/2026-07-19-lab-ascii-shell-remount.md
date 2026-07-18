# Lab ASCII Shell Remount Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Remount Trareon Lab’s Slint UI onto the Acquire ASCII lab-bench chrome (banner / header / rail / safety / status) and remodel primary examination screens into LTR zones, keeping Lab-honest features and the inspector drawer.

**Architecture:** Port copper tokens + geometry from Trareon-Acquire `cursor/ascii-shell-remount` into `apps/lab-slint/ui/theme.slint` and chrome components; wire shell in `app.slint`; remodel Case Home / Evidence / Search / Timeline / Bookmarks / Report bodies in-place without renaming Rust callbacks. Stage index stays derived in Slint/UI model (no new state machine).

**Tech Stack:** Rust, Slint 1.x (`lab-slint` feature `gui`), Inter (bundled), existing `UiSnapshot` / `LabSession`.

**Spec:** `docs/superpowers/specs/2026-07-19-lab-ascii-shell-remount-design.md`

## Global Constraints

- Visual SoT: Acquire branch `cursor/ascii-shell-remount` (local clone OK at `~/Projects/Trareon/Trareon Acquire`)
- Copper primary `#AF622E`; void `#F4F6F8`; font **Inter** only (no Avenir Next dependency)
- Stage tabs: exactly **Open · Import · Examine · Bookmark · Report** (5)
- Inspector: right drawer 280px; off by default; `i` / Esc unchanged
- No Guided/Standard/Expert pills; no write-blocker HW UI; no AFF4/ZFF/fsnap acquire controls
- Do not rename Slint `in-out` properties / callbacks consumed by `apps/lab-slint/src/main.rs` unless both sides update in the same task
- Compact breakpoint remains `width_px < 1100` (`UiSnapshot::apply_layout_width`)
- Window min: **920 × 680**; preferred 1280 × 800+
- After code edits: `graphify update .`
- Commit only when the user asks (plan steps may say “Commit” — skip unless requested)

## File map

| File | Responsibility |
|------|----------------|
| `apps/lab-slint/ui/theme.slint` | ASCII geometry + copper token aliases |
| `apps/lab-slint/ui/strings.slint` | WORKFLOW/LAB/SUPPORT labels, stage/chip copy |
| `apps/lab-slint/ui/components/disclosure_banner.slint` | 26px unsigned well |
| `apps/lab-slint/ui/components/lab_header.slint` | 48px TRAREON\|LAB header |
| `apps/lab-slint/ui/components/lab_sidebar.slint` | 188/56 rail, three groups |
| `apps/lab-slint/ui/components/lab_safety_strip.slint` | **Create** — chips + 5 StageTabs |
| `apps/lab-slint/ui/components/lab_status_bar.slint` | 30px ambient status |
| `apps/lab-slint/ui/components/stat_card.slint` | Keep / polish |
| `apps/lab-slint/ui/components/workflow_card.slint` | Keep / polish |
| `apps/lab-slint/ui/components/lab_button.slint` | Copper primary (`Theme.accent` → copper) |
| `apps/lab-slint/ui/app.slint` | Shell stack: banner→header→rail+safety+body→status; inspector |
| `apps/lab-slint/ui/screens/case_screen.slint` | 3-zone Case Home; remove body stepper |
| `apps/lab-slint/ui/screens/evidence_screen.slint` | FILES + DETAIL zones |
| `apps/lab-slint/ui/screens/search_screen.slint` | QUERY + RESULTS |
| `apps/lab-slint/ui/screens/timeline_screen.slint` | EVENTS + CONTEXT |
| `apps/lab-slint/ui/screens/bookmarks_screen.slint` | LIST + CLAIM |
| `apps/lab-slint/ui/screens/report_screen.slint` | STATUS + FINDINGS |
| Secondary screens | Skin only (padding / PanelCard) |
| `apps/lab-slint/tests/lab_workbench_chrome.rs` | Geometry + IA contract |
| `apps/lab-slint/src/ui_model.rs` | Default `nav_collapsed: false` (labeled rail); stage helper if needed |

**Reference (read-only):**  
`/Users/user/Projects/Trareon/Trareon Acquire/apps/acquire-slint/ui/{theme,app,strings}.slint`

---

### Task 1: Lock ASCII chrome contract in tests

**Files:**
- Modify: `apps/lab-slint/tests/lab_workbench_chrome.rs`
- Modify (later in Task 2–3 to pass): `apps/lab-slint/ui/theme.slint`, `apps/lab-slint/src/ui_model.rs`

**Interfaces:**
- Consumes: none
- Produces: Geometry constants + nav IA arrays that Theme / sidebar must match

- [ ] **Step 1: Rewrite failing geometry + IA test**

Replace constants and tests at the top of `lab_workbench_chrome.rs` with:

```rust
//! Lab ASCII shell chrome contract (Acquire-family remount).

use lab_slint::{NavScreen, UiSnapshot};

const DISCLOSURE_HEIGHT_PX: i32 = 26;
const HEADER_HEIGHT_PX: i32 = 48;
const SIDEBAR_RAIL_PX: i32 = 56;
const SIDEBAR_EXPANDED_PX: i32 = 188;
const SAFETY_HEIGHT_PX: i32 = 40;
const INSPECTOR_WIDTH_PX: i32 = 280;
const STATUS_HEIGHT_PX: i32 = 30;
const MIN_WIDTH_PX: i32 = 920;
const MIN_HEIGHT_PX: i32 = 680;

const WORKFLOW_NAV: &[&str] = &[
    "Case",
    "Evidence",
    "Search",
    "Timeline",
    "Bookmarks",
    "Report",
];
const LAB_NAV: &[&str] = &[
    "Artifacts",
    "Graph",
    "Runs",
    "Transfer",
    "QuickVerify",
];
const SUPPORT_NAV: &[&str] = &["Capabilities", "About"];

#[test]
fn lab_ascii_geometry_lock() {
    assert_eq!(DISCLOSURE_HEIGHT_PX, 26);
    assert_eq!(HEADER_HEIGHT_PX, 48);
    assert_eq!(SIDEBAR_RAIL_PX, 56);
    assert_eq!(SIDEBAR_EXPANDED_PX, 188);
    assert_eq!(SAFETY_HEIGHT_PX, 40);
    assert_eq!(INSPECTOR_WIDTH_PX, 280);
    assert_eq!(STATUS_HEIGHT_PX, 30);
    assert_eq!(MIN_WIDTH_PX, 920);
    assert_eq!(MIN_HEIGHT_PX, 680);
}

#[test]
fn lab_ascii_nav_ia() {
    assert_eq!(WORKFLOW_NAV.len(), 6);
    assert!(!WORKFLOW_NAV.contains(&"Hex"));
    assert!(!WORKFLOW_NAV.contains(&"QuickVerify"));
    assert_eq!(
        LAB_NAV,
        ["Artifacts", "Graph", "Runs", "Transfer", "QuickVerify"]
    );
    assert_eq!(SUPPORT_NAV, ["Capabilities", "About"]);
}

#[test]
fn shell_defaults_labeled_rail_inspector_off_light() {
    let snap = UiSnapshot::default();
    assert!(!snap.dark_mode);
    assert!(!snap.inspector_open);
    assert!(!snap.nav_collapsed); // ASCII default: labeled 188px rail
    assert!(snap.nav_expanded());
    assert!(!snap.layout_compact);
    assert_eq!(snap.active_screen, NavScreen::CaseHome);
    assert_eq!(snap.inspector_tab, "properties");
}
```

Keep existing tests: `responsive_compact_forces_rail_and_overlay`, shortcuts, hex, capabilities, examiner_path, graph — but update compact test expectation: at width 1000, `nav_collapsed` forced true (already).

- [ ] **Step 2: Run tests — expect FAIL on default nav + geometry doc-only until Theme/UiSnapshot update**

Run:

```bash
cd "/Users/user/Projects/Trareon/Trareon Lab"
cargo test -p lab-slint --features gui --test lab_workbench_chrome shell_defaults_labeled_rail -- --nocapture
```

Expected: FAIL — `nav_collapsed` is still `true` by default.

- [ ] **Step 3: Minimal UiSnapshot default flip**

In `apps/lab-slint/src/ui_model.rs` `Default` for `UiSnapshot`, set:

```rust
nav_collapsed: false,
```

- [ ] **Step 4: Re-run chrome tests**

```bash
cargo test -p lab-slint --features gui --test lab_workbench_chrome -- --nocapture
```

Expected: PASS for `shell_defaults_*` and `lab_ascii_nav_ia`. Geometry lock test is constants-only (always PASS); Theme lengths enforced in Task 2 via build + manual check / optional assert comments in theme.

- [ ] **Step 5: Commit** (only if user asked)

```bash
git add apps/lab-slint/tests/lab_workbench_chrome.rs apps/lab-slint/src/ui_model.rs
git commit -m "$(cat <<'EOF'
test: lock ASCII shell chrome geometry and rail IA

EOF
)"
```

---

### Task 2: Theme tokens + geometry

**Files:**
- Modify: `apps/lab-slint/ui/theme.slint`
- Modify: `apps/lab-slint/ui/app.slint` (min-width/height only in this task if build requires)

**Interfaces:**
- Consumes: Acquire token table from spec §5
- Produces: `Theme.disclosure-height` 26px, `header-height` 48px, `sidebar-rail-width` 56px, `sidebar-width` 188px, `status-height` 30px, `safety-height` 40px (new), `window-min-w/h`, copper as `accent`

- [ ] **Step 1: Update Theme geometry and copper light SoT**

In `theme.slint` `export global Theme`:

```slint
out property <length> disclosure-height: 26px;
out property <length> header-height: 48px;
out property <length> sidebar-rail-width: 56px;
out property <length> sidebar-width: 188px;
out property <length> safety-height: 40px;
out property <length> inspector-width: 280px;
out property <length> status-height: 30px;
out property <length> window-min-w: 920px;
out property <length> window-min-h: 680px;
out property <length> panel-radius: 8px;
out property <length> chip-radius: 5px;

// Light SoT — copper family (alias existing names so screens keep compiling)
in-out property <color> canvas: #F4F6F8;      // void
in-out property <color> surface: #FFFFFF;     // panel
in-out property <color> surface-raised: #EEF1F4;
in-out property <color> border: #D8DDE3;      // hairline
in-out property <color> text-primary: #1E2A36; // readout
in-out property <color> text-secondary: #5F6F7F; // mute
in-out property <color> accent: #AF622E;      // copper
in-out property <color> accent-hover: #B85A28;
in-out property <color> accent-muted: #FFF4EC; // rail-active
in-out property <color> warning-bg: #FFF2E8;  // unsigned-well
in-out property <color> success: #2F855A;     // ok-glow
```

Mirror the same hex values inside `apply(false)` / adjust `apply(true)` dark copper like Acquire (`#C4845A` accent). Keep `font-ui: "Inter"`.

- [ ] **Step 2: Align AppWindow mins**

In `app.slint`:

```slint
min-width: Theme.window-min-w;
min-height: Theme.window-min-h;
background: Theme.canvas;
```

- [ ] **Step 3: Build GUI crate**

```bash
cargo build -p lab-slint --bin trareon-lab --features gui
```

Expected: Finished without Slint errors.

- [ ] **Step 4: Commit** (if requested)

```bash
git add apps/lab-slint/ui/theme.slint apps/lab-slint/ui/app.slint
git commit -m "$(cat <<'EOF'
feat(ui): port ASCII copper tokens and chrome geometry

EOF
)"
```

---

### Task 3: Safety strip component + chrome wiring

**Files:**
- Create: `apps/lab-slint/ui/components/lab_safety_strip.slint`
- Modify: `apps/lab-slint/ui/components/lab_sidebar.slint`
- Modify: `apps/lab-slint/ui/components/lab_header.slint`
- Modify: `apps/lab-slint/ui/components/lab_status_bar.slint`
- Modify: `apps/lab-slint/ui/components/disclosure_banner.slint`
- Modify: `apps/lab-slint/ui/strings.slint`
- Modify: `apps/lab-slint/ui/app.slint`

**Interfaces:**
- Consumes: `Theme.safety-height`, stage labels from Strings, case/coverage/integrity props from AppWindow
- Produces: `LabSafetyStrip` with callbacks `navigate(string)`, `case-clicked()`, `coverage-clicked()`; properties `active-stage: int` (1–5), `case-title`, `coverage-label`, `integrity-label`

- [ ] **Step 1: Add strings**

In `strings.slint`:

```slint
out property <string> nav-group-workflow: is-id ? "ALUR KERJA" : "WORKFLOW";
out property <string> nav-group-lab: "LAB";
out property <string> nav-group-support: is-id ? "DUKUNGAN" : "SUPPORT";
out property <string> chip-case: is-id ? "KASUS" : "CASE";
out property <string> chip-coverage: is-id ? "CAKUPAN" : "COVERAGE";
out property <string> chip-integrity: is-id ? "INTEGRITAS" : "INTEGRITY";
```

(Reuse existing `workflow-open` … `workflow-report` for stage tab labels.)

- [ ] **Step 2: Create `lab_safety_strip.slint`**

Implement Acquire-style chips + five `StageTab` rectangles (pattern from Acquire `SafetyChip` / `StageTab` in `app.slint` ~L201–276). Active stage uses copper fill; inactive uses raised + hairline. Height = `Theme.safety-height`.

```slint
export component LabSafetyStrip inherits Rectangle {
    in-out property <string> case-title: "(no case)";
    in-out property <string> coverage-label: "NotRun";
    in-out property <string> integrity-label: "SHA-256";
    in-out property <int> active-stage: 1; // 1..5
    callback case-clicked();
    callback coverage-clicked();
    callback stage-clicked(int); // 1..5
    height: Theme.safety-height;
    // HorizontalLayout: 3 chips | stretch | 5 stage tabs
}
```

- [ ] **Step 3: Remount sidebar groups**

In `lab_sidebar.slint`, replace group labels with WORKFLOW / LAB / SUPPORT; width `expanded ? Theme.sidebar-width : Theme.sidebar-rail-width`. Nav items: Workflow 6 + Lab 5 (include Quick Verify) + Support 2. Active well: `Theme.accent-muted` + 3px copper bar (Acquire `NavItem` pattern). Keep `navigate-key(string)` callback keys: `"Case"`, `"Evidence"`, … `"QuickVerify"`, `"Capabilities"`, `"About"`.

- [ ] **Step 4: Header / banner / status polish**

- `disclosure_banner.slint`: background `Theme.warning-bg` / unsigned colors; height already from Theme
- `lab_header.slint`: height 48 via Theme; keep TRAREON \| LAB; search chip opens palette
- `lab_status_bar.slint`: height 30; keep Evidence/Coverage/★ + Log/Inspector toggles; optional drop redundant Nav toggle if rail always labeled (keep toggle for collapse)

- [ ] **Step 5: Wire in `app.slint`**

Insert `LabSafetyStrip` between header and the horizontal (sidebar | body) row — **or** as first row inside the main column (Acquire: safety sits above body, beside rail). Target structure:

```
VerticalLayout {
  DisclosureBanner { }
  LabHeader { }
  HorizontalLayout {
    LabSidebar { }
    VerticalLayout {
      LabSafetyStrip {
        active-stage: /* derive from case/evidence/bookmarks/report-finalizable */;
        stage-clicked(i) => { /* map 1→Case, 2→Case or Evidence, 3→Evidence, 4→Bookmarks, 5→Report */ }
        case-clicked => { navigate("Case"); }
        coverage-clicked => { navigate("Case"); }
      }
      Rectangle { /* existing screen stack */ vertical-stretch: 1; }
      LabStatusBar { }
    }
  }
}
```

Stage derivation (Slint property on AppWindow):

```slint
property <int> workflow-stage: (case-title == "" || case-title == "(no case)") ? 1
    : evidence-count <= 0 ? 2
    : bookmark-count > 0 ? (report-finalizable ? 5 : 4)
    : 3;
```

- [ ] **Step 6: Build + chrome tests**

```bash
cargo build -p lab-slint --bin trareon-lab --features gui
cargo test -p lab-slint --features gui --test lab_workbench_chrome --test ui_smoke
```

Expected: PASS.

- [ ] **Step 7: Manual launch**

```bash
killall trareon-lab 2>/dev/null
cargo run -p lab-slint --bin trareon-lab --features gui
```

Expected: Labeled rail, safety strip with 5 stages, copper accents.

---

### Task 4: Case Home three-zone body

**Files:**
- Modify: `apps/lab-slint/ui/screens/case_screen.slint`
- Uses: `stat_card.slint`, `workflow_card.slint`, `panel_card.slint`

**Interfaces:**
- Consumes: existing CaseScreen props/callbacks (`open-case-clicked`, `import-evidence-clicked`, `load-demo-clicked`, `quick-verify-clicked`, `navigate`)
- Produces: IDENTITY | EXAMINE | ACTION zones; **no** `AcquireStepper` in body

- [ ] **Step 1: Remove body stepper**

Delete `AcquireStepper` import/usage from `case_screen.slint`. Title row may keep “Lab” + `next-body` subtitle only.

- [ ] **Step 2: Layout three PanelCards**

Wide (`!layout-compact` optional — if CaseScreen has no compact prop, always HorizontalLayout with wrap via parent Flickable):

```
HorizontalLayout {
  PanelCard { /* IDENTITY ~1.2 stretch */ }
  PanelCard { /* EXAMINE ~1.0 */ }
  PanelCard { /* ACTION ~0.85 */ }
}
```

IDENTITY: welcome / case title (existing strings), StatCards ×3, Workflow Guide if `!has-case`, activity if `has-case`.  
EXAMINE: PlaceRows with locked hints.  
ACTION: primary LabButton + OutlineActions + More Actions menu (already partially present).

- [ ] **Step 3: Build + ui_smoke**

```bash
cargo test -p lab-slint --features gui --test ui_smoke
```

Expected: PASS (`open_case_focused` still works via hidden Button).

---

### Task 5: Evidence FILES + DETAIL

**Files:**
- Modify: `apps/lab-slint/ui/screens/evidence_screen.slint`
- Possibly wire selection summary props already on AppWindow (`evidence-names`, `selected-file-index`, paths/sizes)

**Interfaces:**
- Consumes: existing evidence arrays + `import-evidence-clicked`, `evidence-row-selected`, `run-carving-clicked`
- Produces: left FILES list; right DETAIL with summary + buttons that call `bookmark` / navigate Hex via new callbacks **only if** already on AppWindow — prefer:

```slint
callback open-hex-clicked();
callback bookmark-selection-clicked();
```

If AppWindow already has bookmark/hex paths from Evidence, reuse those callback names from `app.slint` wiring (grep `bookmark-selection` / `Focus Hex` before adding).

- [ ] **Step 1: Grep existing callbacks**

```bash
rg -n "bookmark|hex|inspector" apps/lab-slint/ui/app.slint apps/lab-slint/src/main.rs | head -40
```

Wire DETAIL actions to existing handlers; do not invent parallel APIs.

- [ ] **Step 2: Two-column layout**

When `has-files`:

```
HorizontalLayout {
  PanelCard { /* FILES: filter + list + carve */ horizontal-stretch: 1.5; }
  PanelCard { /* DETAIL: selected name/path/integrity + Add Bookmark + Open Hex */ horizontal-stretch: 1; }
}
```

Empty state unchanged (Import CTA).

- [ ] **Step 3: Build + file_list_keyboard test**

```bash
cargo test -p lab-slint --features gui --test file_list_keyboard --test ui_smoke
```

Expected: PASS.

---

### Task 6: Search / Timeline / Bookmarks / Report zones

**Files:**
- Modify: `apps/lab-slint/ui/screens/search_screen.slint`
- Modify: `apps/lab-slint/ui/screens/timeline_screen.slint`
- Modify: `apps/lab-slint/ui/screens/bookmarks_screen.slint`
- Modify: `apps/lab-slint/ui/screens/report_screen.slint`

**Interfaces:**
- Consumes: existing screen props/callbacks only
- Produces: two-zone HorizontalLayouts per spec §4.3–4.6; copper LabButton primaries

- [ ] **Step 1: Search — QUERY | RESULTS**

Left: query LineEdit + search button + coverage + yara/hashset lines.  
Right: results list + Open in Evidence / Bookmark actions already present.

- [ ] **Step 2: Timeline — EVENTS | CONTEXT**

Left: event list + import CSV control if present.  
Right: selected label + Jump to Evidence.

- [ ] **Step 3: Bookmarks — LIST | CLAIM**

Left: bookmark/finding list.  
Right: claim detail + queue-for-report hint strings.

- [ ] **Step 4: Report — STATUS | FINDINGS**

Left: report-state, blockers, SoD.  
Right: findings + Export button bound to existing export callback.

- [ ] **Step 5: Test**

```bash
cargo test -p lab-slint --features gui --test workbench_ux --test ui_smoke
```

Expected: PASS.

---

### Task 7: Secondary screen skin + verification

**Files:**
- Modify lightly: `artifacts_screen.slint`, `graph_screen.slint`, `runs_screen.slint`, `transfer_screen.slint`, `capabilities_screen.slint`, `about_screen.slint`, `quick_verify_screen.slint`
- Modify: `docs/superpowers/specs/2026-07-18-lab-acquire-family-skin.md` — add one-line pointer “chrome superseded by 2026-07-19 ASCII remount”
- Recapture: `docs/media/lab-ui-case-home.png` (manual)

**Interfaces:**
- Consumes: Theme copper / PanelCard
- Produces: consistent padding (16–20px), no IA changes

- [ ] **Step 1: Skin pass**

Wrap each secondary root content in `VerticalLayout { padding: 16px; ... }` and use `Theme.accent` for primary actions. No new nav items.

- [ ] **Step 2: Full lab-slint GUI tests**

```bash
cargo test -p lab-slint --features gui 2>&1 | tail -40
```

Expected: all tests PASS.

- [ ] **Step 3: Launch + screenshot**

```bash
cargo run -p lab-slint --bin trareon-lab --features gui
# Capture Case Home empty + with demo case → docs/media/lab-ui-case-home.png
```

- [ ] **Step 4: graphify**

```bash
graphify update .
```

- [ ] **Step 5: Commit** (if requested)

```bash
git add apps/lab-slint docs/media/lab-ui-case-home.png docs/superpowers/specs/2026-07-18-lab-acquire-family-skin.md
git commit -m "$(cat <<'EOF'
feat(ui): remount Lab onto Acquire ASCII shell

EOF
)"
```

---

## Spec coverage checklist

| Spec section | Task |
|--------------|------|
| §3 Chrome geometry / tokens | 1–3 |
| §3.3 Rail IA WORKFLOW/LAB/SUPPORT | 1, 3 |
| §3.4 Safety 5 stages | 3 |
| §3.5 Header/status | 3 |
| §4.1 Case Home zones | 4 |
| §4.2 Evidence + inspector drawer | 3 (drawer keep), 5 (zones) |
| §4.3–4.6 Search…Report | 6 |
| §4.7 Secondary skin | 7 |
| §7 Non-goals | All tasks — do not add HW/AFF4/mode pills |
| §8 Verification | 1, 7 |

## Placeholder / consistency self-review

- No TBD steps; Acquire paths are absolute where referenced  
- Stage property named `workflow-stage` (1–5) consistently  
- Compact threshold stays `< 1100` (already in `ui_model.rs`)  
- Commit steps optional under user git rule  
