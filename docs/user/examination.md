# Examination shell (offline)

Trareon Lab examination UI (Slint) is offline-first: one case per process, no central collaboration server.

## Screens

- **Case** — title plus counts loaded from the case database (`evidence_object`, `coverage_record`)
- **Evidence** — registry of imported objects (Day 11+ deepens disk readers)
- **Search** — backed by `lab-index` rows
- **Timeline** — artifact/FS events (populated as parsers land)
- **Bookmarks** — first-class citations (`docs/contracts/BOOKMARK.md`)
- **Report** — deterministic export skeleton

## Counts on open

Opening a case must show **real** evidence and coverage counts from SQLite, not hard-coded demo numbers. See `apps/lab-slint/tests/case_open_counts.rs`.

## Unsigned builds

Installers are unsigned in the zero-cost sell program. Operators follow `docs/SELLING-UNSIGNED.md`.
