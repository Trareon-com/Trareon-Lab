# Official R1.0.0 — repository completion vs operator blockers

**Synced:** 2026-07-17  
**Sellable Engineering Alpha:** complete (`v0.9.0-sellable`)  
**Official candidate RC:** `v1.0.0-rc1-unsigned`  
**Repo artifact track:** COMPLETE (go-live scripts included)  
**Plan checkboxes:** 219 done / 8 open (human evidence only)

## Created in-repo (finish pass)

- Path C/D operator packets + escalation
- Reviewer draft pack script + legal meeting agenda + comments file
- Signed packaging scripts: macOS / Linux / Windows (+ verify + rebuild orchestrator)
- O10 smoke templates (macOS/Linux/Windows)
- MANIFEST fill script + human signoff metadata + post-release / live-note / RC1 docs
- **Windows lab queue:** `docs/WINDOWS-LAB-QUEUE.md` (W1–W6 deferred to end)
- **Go-live scripts:** `generate-official-sbom.sh`, `cut-official-v1.sh`, `publish-official-release.sh`, `close-official-program.sh`
- **Final 8 gates map:** `docs/operator/FINAL-8-GATES.md`
- **Sign-off obtain guide:** `docs/operator/OBTAIN-SIGNOFFS.md`
- **Crypto receipt stub:** `docs/reviews/CRYPTO-EXTERNAL-REVIEW-RECEIPT.md`
- Fallback Official SBOM: `release-evidence/OFFICIAL-1.0.0/sbom.cdx.json` (regenerate with cyclonedx-cargo later)

## Still open (cannot invent)

1. External crypto review **received** (named human document)
2. Indonesia wet/digital sign-off obtained (O8)
3. Crypto review accepted (O9)
4. Final gather PASS (needs O1–O12 files)
5. Annotated Official `v1.0.0`
6. GitHub Release with **signed** installers
7. Push Official tag + release
8. Close program / PRD `Official Production 1.0.0 released`

## Execution after human + Windows lab

See `docs/operator/FINAL-8-GATES.md` and `docs/WINDOWS-LAB-QUEUE.md`.

## Honesty rule

Do **not** relabel RC/Alpha as Official Production. Windows Authenticode + ThinkPad smoke stay in `docs/WINDOWS-LAB-QUEUE.md` until a Windows lab is available.
