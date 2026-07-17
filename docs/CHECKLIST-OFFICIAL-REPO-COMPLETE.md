# Official R1.0.0 — repository completion vs operator blockers

**Synced:** 2026-07-17  
**Sellable Engineering Alpha:** complete (`v0.9.0-sellable`)  
**Official candidate RC:** `v1.0.0-rc1-unsigned`  
**Repo artifact track:** COMPLETE  
**Plan checkboxes:** 219 done / 8 open (human evidence only)  
**End-state map:** `docs/OFFICIAL-END-STATE.md`

## Created in-repo

- Path C/D operator packets + escalation
- Signed packaging scripts + verify + rebuild
- O10 smoke templates; O1–O3/O11 evidence **examples** under `release-evidence/OFFICIAL-1.0.0/templates/`
- Go-live scripts (SBOM / cut / publish / close)
- Final 8 gates + obtain sign-offs + crypto receipt
- Transfer security review (gap fill)
- O4 SBOM vuln/license review template
- `scripts/smoke-week2.sh` → e2e alias
- Mac/Linux signing queue + Windows lab queue (Windows last)
- `gather.sh` exits 0 only when non-stub evidence exists

## Still open (cannot invent)

1. External crypto review **received**
2. Indonesia wet/digital sign-off (O8)
3. Crypto review accepted (O9)
4. Final gather PASS
5. Annotated Official `v1.0.0`
6. GitHub Release with **signed** installers
7. Push Official tag + release
8. Close program / PRD released

## Honesty rule

Do **not** relabel RC/Alpha as Official Production. Windows Authenticode + ThinkPad smoke: `docs/WINDOWS-LAB-QUEUE.md` only.
