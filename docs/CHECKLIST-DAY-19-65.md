# Checklist Days 19–65 (sellable zero-cost)

Completed in continuation session 2026-07-17 after Day 18.

- [x] 19 index FS metadata + UI file list
- [x] 20 keyboard stubs
- [x] 21–23 ext4/APFS + deleted partial
- [x] 24–25 storage dossier PASS
- [x] 26–32 lab-artifacts parsers
- [x] 33 bookmarks CRUD
- [x] 34–35 transfer + collab guide
- [x] 36–40 timeline/findings/report + dossiers
- [x] 41–44 search/query + UI + 1M windowed
- [x] 45 E2E script
- [x] 46–49 dossiers/known issues stubs
- [x] 50 selling page + commercial note
- [x] 51–54 packaging scripts + dist notes
- [x] 55–59 smoke/about/demo docs
- [x] 60–65 tagging/go-live documented as operator steps (tags created when releasing)

## Operator release steps (60–65)

```bash
git tag v0.9.0-sellable
# build dist via packaging/*.sh on each OS
git tag v1.0.0
# publish GitHub Release + selling page
```
