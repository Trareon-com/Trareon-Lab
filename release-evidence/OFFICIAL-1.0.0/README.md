# Official R1.0.0 release evidence

Skeleton for Official Production gates O1–O12.  
**No gate is PASS until the named artifact exists and the accountable owner records acceptance in `docs/OFFICIAL-RELEASE-RUNBOOK.md`.**

## Layout

```
release-evidence/OFFICIAL-1.0.0/
  README.md                 # this file
  GATES.md                  # O1–O12 checklist (unchecked until evidence lands)
  MANIFEST.txt              # filled at freeze; lists SHA + artifact paths
  gather.sh                 # fails closed if any required gate file is missing
  corpora/                  # raw validation logs (operator-filled)
  perf/                     # frozen performance baselines
  windows-sig.txt           # O1
  macos-notarization.json   # O2
  linux-sig.txt             # O3
  linux-signing-pubkey.asc  # public key only (see docs/LINUX-PACKAGE-SIGNING-KEY-PLAN.md)
  sbom.cdx.json             # O4 CycloneDX
```

## How to gather

```bash
./release-evidence/OFFICIAL-1.0.0/gather.sh
```

The script exits non-zero until every O1–O12 evidence path listed in `GATES.md` is present. Do not weaken the script to greenwash an unsigned Engineering Alpha.
