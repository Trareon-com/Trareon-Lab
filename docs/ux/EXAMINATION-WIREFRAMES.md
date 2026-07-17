# Examination UI Wireframes (text)

**Scope:** R1 sellable-unsigned examination shell (Slint).  
**Day:** 4 of zero-cost sellable plan.

Legend: `[ ]` control · `===` region · `>` navigation · `*` primary action

---

## 1. Case Home

```
=== Trareon Lab — Case Home ===============================
 Case: [demo-case-001                    ]  State: OPEN
 Evidence: 3   Coverage: 12%   Bookmarks: 2

 [ Open Case… ]  [ New Case… ]  [ Import Evidence… ]

 Recent activity
  - 2026-07-17  imported evidence.raw
  - 2026-07-17  indexed filesystem (Limited)

 > Evidence  > Search  > Timeline  > Bookmarks  > Report  > About
============================================================
```

## 2. Evidence Browser

```
=== Evidence ==============================================
 Filter: [ name / path / hash____________ ] [ Apply ]

 UUID                                   Class      Bytes   State
 2222…2222  evidence.raw                disk_img   1.2G    VerifiedMatch
 5555…5555  logical-export.zip          logical    40M     ComputedUnanchored

 Detail pane
  source_designation: forensic_copy
  validation_state: VerifiedMatch
  [ Hash… ]  [ Browse FS ]  [ Add Bookmark* ]
============================================================
```

## 3. Search

```
=== Search ================================================
 Query: [ prefetch chrome________________ ]  Scope: [ All artifacts ▾ ]

 Results (42)
  ART-001  Prefetch  CHROME.EXE-…  score 0.91
  ART-014  Prefetch  SETUP.EXE-…   score 0.74

 [ Open hit ]  [ Add to timeline ]  [ Bookmark* ]
============================================================
```

## 4. Timeline

```
=== Timeline ==============================================
 Range: [2026-01-01] → [2026-07-17]   TZ: UTC
 Layers: [x] FS  [x] Artifacts  [ ] Network (Limited)

 |----o----o---------o--------------o----|
      ^ Prefetch      ^ File create

 Selected: Prefetch CHROME.EXE @ 2026-03-02T11:22:01Z
 [ Jump to evidence ]  [ Bookmark* ]
============================================================
```

## 5. Bookmarks

```
=== Bookmarks =============================================
 Filter review_state: [ open ▾ ]

 Citation                         Target              State
 Prefetch hit cited…              artifact_hit        open
 Evidence object cited…           evidence_object     accepted

 [ New Bookmark* ]  [ Export Transfer… ]  [ Supersede ]
============================================================
```

## 6. Transfer / Share (offline)

```
=== Transfer Package (offline) ============================
 Destination: [ Partner Lab A_______________ ]
 Purpose:     [ Technical review____________ ]
 Authority:   [ Approved by case owner______ ]

 Selected bookmarks: 2
 Payload digest: bbbb…bbbb

 Disclosure preview (required before import apply)
  [ Cancel ]  [ Sign & Export* ] / [ Verify & Import* ]
============================================================
```

## 7. Report / About

```
=== Report / About ========================================
 Report skeleton
  Findings linked to bookmarks: 2
  [ Export PDF stub ]  [ Export JSON ]

 About / disclosure
  Product: Trareon Lab (Engineering Alpha / Lab use)
  Signing: UNSIGNED — see docs/SELLING-UNSIGNED.md
  Claims: NOT court-ready / NOT ISO-certified
============================================================
```

## Keyboard stubs (Day 20 will harden)

| Key | Action |
|---|---|
| `/` | Focus search |
| `b` | Bookmark selection |
| `g` then `e/s/t/b/r` | Go Evidence/Search/Timeline/Bookmarks/Report |
| `Esc` | Close dialog / clear selection |
