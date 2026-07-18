# FR-DOC-002 enforcement

**Rule:** No capability may be labeled Official `Validated` (or over-claim `CORPUS_VALIDATED` without frozen corpora) without matching offline user guide + validation dossier.

## Gate command

```bash
bash scripts/check-fr-doc-002.sh
```

## Current posture (2026-07-17)

- Engineering Alpha methods use dossier-scoped levels, including
  `CORPUS_VALIDATED` / `Validated` only for the exact frozen subsets named in
  APFS, E01, YARA/hash-set, search, and export dossiers.
- These subset promotions are not an Official release claim. Official status
  still requires the frozen release-evidence corpora and guide inventory in
  the Official manifest.
- Package intake may remain `CORPUS_VALIDATED` only for `.fsnap` synthetic + Acquire fixtures already covered by Foundation/R1 intake tests.
