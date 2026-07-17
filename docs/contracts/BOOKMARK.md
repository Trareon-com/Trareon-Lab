# Bookmark Contract

**Schema version:** bookmark-1  
**ADR:** ADR-008 amendment (FR-ART-006 → R1)  
**Related:** `schemas/bookmark.schema.json`

## Purpose

Bookmark is a first-class examination object that cites evidence, artifact hits, timeline events, or byte ranges so analysts can return to supporting data and include citations in findings/reports.

## Required fields

| Field | Type | Notes |
|---|---|---|
| schema_version | const `bookmark-1` | |
| bookmark_uuid | UUID | Stable ID |
| case_uuid | UUID | Must match open case |
| target_kind | enum | `evidence_object`, `artifact_hit`, `timeline_event`, `fs_path`, `byte_range` |
| target_ref | string | Opaque ID or path within case |
| citation | string | Human-readable cite text (minLength 1) |
| author_role | string | Role label at creation (not legal status) |
| created_at_utc | date-time | UTC |
| review_state | enum | `open`, `accepted`, `rejected`, `superseded` |

## Optional fields

| Field | Type | Notes |
|---|---|---|
| byte_range | object | `{ start: integer>=0, end: integer>=start }` when `target_kind=byte_range` |
| tags | string[] | Free tags |
| note | string | Examiner note |
| evidence_uuid | UUID | Convenience link when known |

## Rules

- Bookmarks never mutate source evidence.
- Deleting a bookmark is soft-supersede preferred (`review_state=superseded`) for auditability.
- Export to transfer package includes only bookmarks explicitly selected.

## Non-goals

- Live multi-user editing of the same bookmark.
- Cross-case bookmark federation without transfer package.
