# Official corpus freeze prep

**Status:** PREP ONLY — no Official `Validated` promotion yet.

## Preserve logs here

Place raw validation run logs in this directory before updating `docs/RELEASE-01-CAPABILITY-MATRIX.md` validation columns to `CORPUS_VALIDATED` / Official `Validated`.

## Current in-tree evidence (not Official freeze)

| Domain | Evidence today | Official target |
|---|---|---|
| Storage raw/FS | `lab-storage` / `lab-fs` unit tests; `docs/validation/STORAGE-DOSSIER.md` | CORPUS_VALIDATED after multi-OS corpora |
| Artifacts | `lab-artifacts` unit tests; `docs/validation/ARTIFACTS-DOSSIER.md` | CORPUS_VALIDATED after golden corpora |
| Transfer | `lab-transfer` hostile tests | Keep with O11 |

Do not copy synthetic unit PASS into this folder and call it Official freeze.
