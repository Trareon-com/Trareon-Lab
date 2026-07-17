# Official R1.0.0 — repository completion vs operator blockers

**Synced:** 2026-07-17  
**Sellable Engineering Alpha:** complete (`v0.9.0-sellable`)  
**Official Production:** program active; O1–O12 open  
**Repo track:** COMPLETE (agent-doable work finished; commit/push closes CI checkbox)

## Repository-complete (agent-doable)

- Week 1 contracts, runbook, DevSecOps map, design tokens, CI audit/secrets/SBOM/geiger, Official evidence skeleton
- Weeks 2–9 engineering crates/UI/tests already present from sellable path; Official plan checkboxes synced
- Storage/Artifacts Engineering Alpha method reviews PASS
- FR-VAL-009/010 persistence + UI hooks
- Signed packaging **contracts** + signing dry-run script (no secrets)
- Release notes draft + Indonesia sign-off template (unsigned) + crypto review stub (NOT_RECEIVED)
- FR-DOC-002 gate (`scripts/check-fr-doc-002.sh`) + honest UNIT_VERIFIED matrix corrections
- P0-LATER ADR backlog opened
- Official review deltas, discrepancy register, perf draft, corpus prep README

## Operator-blocked (cannot finish in-repo without real-world evidence)

| Blocker | Why |
|---|---|
| Apple Developer Program + Team ID | Paid account / invoice |
| Windows Authenticode purchase | Paid cert / vendor ETA |
| Linux key generation | Offline private key ceremony |
| Send reviewer invitations | Human email + acceptance |
| Path C/D status calls with real ETAs | Human |
| Promote methods to Official `Validated` | Corpora + freeze SHA + FR-DOC-002 |
| Physical 3-OS smoke logs | Hardware |
| Wet/digital legal + crypto sign-offs | Named reviewers |
| Signed installers + GitHub Official Release | Certs + O1–O12 gather PASS |
| Tag `v1.0.0` as Official | Only after gather PASS |

## Honesty rule

Do **not** relabel Engineering Alpha / `v0.9.0-sellable` as Official Production. Official requires every O1–O12 gate `PASS` in `docs/OFFICIAL-RELEASE-RUNBOOK.md`.
