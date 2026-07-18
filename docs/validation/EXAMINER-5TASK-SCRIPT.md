# 5-task examiner script (workbench UX)

Run after `cargo run -p lab-slint --features gui`.

| # | Task | Pass criteria |
|---|------|----------------|
| 1 | Open Case (folder picker or `TRAREON_CASE_DIR`) | Case title updates; disclosure strip visible |
| 2 | Import Evidence (`.dd`/`.bin`/`.E01`) | Evidence count increments; designation/integrity chips; hex in Inspector |
| 3 | Search (`/` palette or Search screen) | Coverage banner shows complete/partial; no fake hits |
| 4 | Bookmark selection (`b` or Inspector) | Bookmarks count + Tagged chip; navigate Bookmarks |
| 5 | Report finalize attempt | Blockers / SoD hint visible; export digests or honest failure |

Also: toggle Nav / Inspector / Log; open Runs; Import timeline CSV (Plaso/Hayabusa) if available.
