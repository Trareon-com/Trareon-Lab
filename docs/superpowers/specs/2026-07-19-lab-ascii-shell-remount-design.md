# Design: Lab ASCII shell remount (Acquire-family)

**Date:** 2026-07-19  
**Status:** Approved (user OK 2026-07-19) — implementation plan next  
 
**Supersedes (chrome):** examination-workbench geometry in `lab_workbench_chrome.rs` (48px header / 180 rail / no safety strip)  
**Does not supersede:** Lab examination capabilities, `UiSnapshot` / session APIs, inspector Properties|Hex behavior  

**Visual SoT:** [Trareon-Acquire `cursor/ascii-shell-remount`](https://github.com/Trareon-com/Trareon-Acquire/tree/cursor/ascii-shell-remount) — especially `apps/acquire-slint/ui/{app,theme,strings}.slint` and `.cursor/skills/frontend-design/DESIGN-TRAREON-ACQUIRE.md`.

---

## 1. Goal

Remount Trareon Lab’s Slint frontend onto the Acquire **ASCII lab-bench shell** so Lab and Acquire read as one product family, while keeping Lab-honest examination features (no fake acquisition / write-blocker hardware UI).

**Approach (locked):** Port ASCII chrome + remodel screen bodies in-place; keep existing Rust property/callback names unless a rename is unavoidable and tested.

---

## 2. Decisions (locked)

| Topic | Choice |
|-------|--------|
| Inspiration | Acquire `cursor/ascii-shell-remount` |
| Stage model | **5 tabs** — Open · Import · Examine · Bookmark · Report |
| Remount depth | **Full product** — chrome + Case Home + Evidence + Search + Timeline + Bookmarks + Report |
| Inspector | **Right drawer** (280px), off by default; `i` / selection / Esc |
| Font | Keep bundled **Inter** (Acquire uses Avenir Next; not guaranteed on all hosts) |
| Primary accent | Copper `#AF622E` (Acquire token), not neon / cream-serif defaults |
| Mode pills | **No** Guided / Standard / Expert on Lab |

---

## 3. Chrome architecture

```
┌─ disclosure banner 26px ──────────────────────────────────┐
│ UNSIGNED — Lab use only · NOT court-ready · NOT ISO…      │
├─ header 48px ─────────────────────────────────────────────┤
│ TRAREON | LAB │ Case chip │ Search (/) │ EN|ID │ theme    │
├── rail ──┬─ safety strip 40px ────────────────────────────┤
│ 188/56px │ CASE · COVERAGE · INTEGRITY │ stage tabs 1–5   │
│          ├─ body ──────────────────────────┬─ inspector ──┤
│ WORKFLOW │ screen zones (LTR / stack)      │ 280 drawer   │
│ LAB      │                                 │ (on demand)  │
│ SUPPORT  ├─ status 30px ───────────────────┴──────────────┤
│          │ Evidence · Coverage · ★ · Log · Settings · EN  │
└──────────┴────────────────────────────────────────────────┘
```

### 3.1 Geometry (wide defaults)

| Region | Size | Notes |
|--------|------|--------|
| Disclosure | 26px | `Theme.unsigned-well` |
| Header | 48px | Brand · case · search · locale · theme |
| Rail | 188px / **56px compact** | Grouped nav |
| Safety | 40px | Chips + 5 stage tabs |
| Status | 30px | Counts + Log + Settings |
| Inspector | 280px | Side when open & not compact; overlay when compact |
| Window min | 920 × 680 | Align Acquire `window-min-*` |

**Breakpoints**

| Mode | Width | Behavior |
|------|-------|----------|
| Wide | ≥ 1280 | Labeled rail; multi-column zones |
| Medium | 1100–1279 | Labeled rail; zones may tighten |
| Compact | &lt; 1100 | Icon rail 56px; body stacks in ScrollView; inspector overlay |

### 3.2 Honest Acquire → Lab mapping

| Acquire | Lab |
|---------|-----|
| TRAREON \| ACQUIRE | TRAREON \| LAB |
| Write-blocker chip | **Coverage** (count / % / NotRun) |
| Integrity chip | SHA-256 policy / OK state from model |
| PREPARE / ACQUIRE / SEAL | **Open → Import → Examine → Bookmark → Report** |
| SOURCE / PREPARE / RUN zones | Per-screen Lab zones (below) |
| START ACQUIRE | State-bound primary CTA (Open Case / Import / Browse / Report) |
| Hardware status icons | Evidence · Coverage · bookmark count |
| Rail Cases / Identify / Acquire… | See §3.3 |

### 3.3 Rail IA

**WORKFLOW:** Case · Evidence · Search · Timeline · Bookmarks · Report  

**LAB (tools):** Artifacts · Graph · Runs · Transfer · Quick Verify  

**SUPPORT:** Capabilities · About  

(Help/cheatsheet stays via `?` / palette if already wired — no fake QMS/Boot/Identify product claims.)

### 3.4 Safety strip

- Chips: **CASE** (title or “(no case)”), **COVERAGE**, **INTEGRITY**
- Stage tabs **1–5** bound to existing stage derivation (no case → Open; no evidence → Import; else Examine; bookmarks → Bookmark; finalizable → Report)
- Tap CASE / COVERAGE → Case Home
- Stage tabs may navigate to the natural screen for that stage (Open→Case, Import→Case/Evidence, Examine→Evidence, Bookmark→Bookmarks, Report→Report) without inventing new Rust machines

### 3.5 Header & status

- Header search focuses palette / Search screen (existing `/` behavior)
- EN | ID + theme toggle stay in header (Acquire pattern)
- Status: Evidence · Coverage · ★ bookmarks · Lab Log · Settings · (locale if not duplicated)

---

## 4. Screen bodies

### 4.1 Case Home — three zones

| Zone | ~Width | Content |
|------|--------|---------|
| IDENTITY | ~42% | Welcome / case title + state badge; 3 StatCards (Evidence Items, Data Coverage, Bookmarked Artifacts); activity + exceptions when case open |
| EXAMINE | ~33% | Shortcut rows (Evidence, Search, Timeline, Bookmarks, Report); locked rows stay readable with “Enabled after opening a case.” |
| ACTION | ~25% | One primary CTA; secondary outlined Load demo + Quick Verify; More Actions → Import Evidence |

Empty: welcome copy + Workflow Guide cards (Create → Import → Analyze) under metrics.  
**No** duplicate stepper in body (safety strip owns stages).  
**No** host-disk table / format chips / START ACQUIRE.

### 4.2 Evidence — two zones + drawer

| Zone | Content |
|------|---------|
| FILES ~60% | Filter, Import, carve/refresh if present; file table |
| DETAIL ~40% | Selection summary + Add Bookmark + Open Hex (opens inspector Hex tab) |
| Inspector | Properties \| Hex — unchanged contract |

### 4.3 Search

| Zone | Content |
|------|---------|
| QUERY ~35% | Query field, submit, coverage label, YARA/hash-set lines |
| RESULTS ~65% | Hits list; Open in Evidence; Bookmark |

### 4.4 Timeline

| Zone | Content |
|------|---------|
| EVENTS ~70% | Event list; TZ/filter if in model; Import CSV in toolbar |
| CONTEXT ~30% | Selected event + Jump to Evidence |

### 4.5 Bookmarks

| Zone | Content |
|------|---------|
| LIST ~60% | Bookmark rows |
| CLAIM ~40% | Detail + queue-for-report / work-product hint |

### 4.6 Report

| Zone | Content |
|------|---------|
| STATUS ~40% | Report state, finalize blockers, SoD hint |
| FINDINGS ~60% | Bound findings; Export using existing session formats |
| CTA | Single primary Export / Finalize from `report-finalizable` |

### 4.7 Secondary screens (skin only)

Artifacts · Graph · Runs · Transfer · Capabilities · About: wrap in ASCII chrome padding + `PanelCard` / copper accents; **no** heavy IA redesign in this pass.

---

## 5. Tokens & components

Port Acquire light SoT into `apps/lab-slint/ui/theme.slint` (names can alias existing Lab props for gradual migration):

| Token | Hex | Role |
|-------|-----|------|
| void | `#F4F6F8` | Window wash |
| panel / ink | `#FFFFFF` | Cards / rail |
| raised | `#EEF1F4` | Inputs / wells |
| hairline | `#D8DDE3` | 1px structure |
| copper | `#AF622E` | Active nav, stages, primary CTA |
| copper-dim / hover | `#8F4A24` / `#B85A28` | Accents |
| readout / mute / faint | `#1E2A36` / `#5F6F7F` / `#8A94A0` | Text |
| ok-glow | `#2F855A` | OK states |
| unsigned-well | `#FFF2E8` | Disclosure |

**Reusable Slint units (extend or add):**

- `NavItem` / grouped rail (Acquire-style active well)
- `SafetyChip`, `StageTab`
- `StatCard`, `WorkflowCard` (already started)
- `PanelCard`, `LabButton` / `ActionBtn` copper primary
- `PlaceRow` with locked hint

Prefer extracting chrome into `lab_sidebar` / `lab_header` / new `lab_safety_strip` / `lab_status_bar` rather than one mega-`app.slint`.

---

## 6. Rust / model constraints

- Preserve `UiSnapshot` public behavior and screen navigation keys used by tests
- Do **not** rename Slint callbacks consumed by `main.rs` without updating both sides in the same change
- Stage index continues to derive from case/evidence/bookmark/report-finalizable (no new state machine crate)
- Dark theme: header toggle continues to call `Theme.apply(bool)`

---

## 7. Non-goals

- Hardware write-blocker UI / Tableau claims  
- Boot media, Identify, QMS product surfaces  
- Fake AFF4 / ZFF / fsnap acquisition controls  
- Guided / Standard / Expert density pills  
- Streaming carver, plugins, cloud import  
- Replacing Inter with Avenir Next system dependency  

---

## 8. Verification

1. Update `apps/lab-slint/tests/lab_workbench_chrome.rs` geometry + rail IA to ASCII metrics and WORKFLOW/LAB/SUPPORT groups  
2. `cargo test -p lab-slint --features gui`  
3. Manual: empty Case Home → Open/demo → Import → Evidence → Search/Bookmark → Report; inspector `i` / Esc  
4. Recapture `docs/media/lab-ui-case-home.png` (and feature screenshots if packaging docs reference them)  
5. `graphify update .` after Slint/Rust edits  

---

## 9. Implementation order (for plan skill)

1. Theme tokens + geometry constants + chrome components (banner, header, rail, safety, status)  
2. Wire `app.slint` shell; keep inspector drawer  
3. Case Home three-zone body  
4. Evidence two-zone body  
5. Search / Timeline / Bookmarks / Report zone skins  
6. Secondary screen skin pass  
7. Tests + screenshot recapture  

---

## 10. Relationship to prior specs

- `2026-07-18-lab-acquire-family-skin.md` — historical ECR remount; **chrome geometry superseded** by this ASCII shell  
- DFIR workbench UX specs — examination **behavior** still applies; only chrome geometry/IA is replaced here
