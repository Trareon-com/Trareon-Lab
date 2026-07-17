# FR-DOC-002 enforcement

**Rule:** No capability may be labeled Official `Validated` (or over-claim `CORPUS_VALIDATED` without frozen corpora) without matching offline user guide + validation dossier.

## Gate command

```bash
bash scripts/check-fr-doc-002.sh
```

## Current posture (2026-07-17)

- Engineering Alpha methods: `UNIT_VERIFIED` / `Limited` where dossiers say so.
- Official `Validated` promotions: **not performed** — wait for corpora under `release-evidence/OFFICIAL-1.0.0/corpora/` plus guide inventory in the Official manifest.
- Package intake may remain `CORPUS_VALIDATED` only for `.fsnap` synthetic + Acquire fixtures already covered by Foundation/R1 intake tests.
