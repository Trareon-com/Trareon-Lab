# Official R1.0.0 release evidence

Skeleton for Official Production gates O1–O12.  
**No gate is PASS until the named artifact exists and the accountable owner records acceptance in `docs/OFFICIAL-RELEASE-RUNBOOK.md`.**

## Layout

```
release-evidence/OFFICIAL-1.0.0/
  README.md
  GATES.md
  MANIFEST.txt
  gather.sh                 # exits 0 only when real (non-stub) evidence exists
  templates/                # *.example formats — NOT gather PASS
  corpora/
  perf/
  o10/                      # physical smoke JSON (must leave NOT_RUN)
  o11-bookmark-transfer.log # O11 — create from templates/ at freeze
  windows-sig.txt           # O1
  macos-notarization.json   # O2
  linux-sig.txt             # O3
  linux-signing-pubkey.asc
  sbom.cdx.json             # O4
```

## How to gather

```bash
./release-evidence/OFFICIAL-1.0.0/gather.sh
```

Copy formats from `templates/*.example` only when filling real evidence paths above. Do not rename `.example` files into PASS slots without real signatures.
