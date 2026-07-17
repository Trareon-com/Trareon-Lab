# Session — Official go-live scripts (final repo creatables)

**Date:** 2026-07-17  
**Scope:** Finish remaining in-repo Official artifacts; leave 8 human gates open; Windows lab queued at end.

## Done

- Added fail-closed go-live scripts:
  - `scripts/generate-official-sbom.sh` (wrote fallback `sbom.cdx.json`)
  - `scripts/cut-official-v1.sh`
  - `scripts/publish-official-release.sh`
  - `scripts/close-official-program.sh`
- Operator docs: `FINAL-8-GATES.md`, `OBTAIN-SIGNOFFS.md`, crypto receipt stub
- Wired Windows queue → go-live sequence
- Plan notes updated for the 8 open rows (still unchecked)
- Verified: cut/close exit 1 without O1; signing dry-run PASS

## Not done (blocked)

Human O8/O9 signatures, Path C certs, Windows W1–W6 lab, Official `v1.0.0` tag/release.

## Next human step

1. Path C/D + O8/O9 per `OBTAIN-SIGNOFFS.md`
2. Windows lab `docs/WINDOWS-LAB-QUEUE.md`
3. Then `FINAL-8-GATES.md` execution order
