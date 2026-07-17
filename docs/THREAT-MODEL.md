# Threat Model

**Method:** STRIDE  
**Date:** 2026-07-17  
**Scope:** Trareon Lab offline desktop laboratory

## Assets

Case data, evidence bytes, derived artifacts, keys/trust bundles, audit/provenance chains, reports, validation corpora, pack binaries, installer artifacts.

## Actors

Hostile evidence authors; malformed container authors; local unprivileged users; privileged insiders; compromised loopback AI services; poisoned models; supply-chain attackers; installer tampering adversaries.

## Representative threats

| Threat | STRIDE | Prevention | Detection | Containment | Recovery | Residual risk | Validation test |
|---|---|---|---|---|---|---|---|
| Hostile evidence / parser exploit | T | sandboxed packs; capability broker; no network | crash/timeout coverage events | kill pack process | reopen case; quarantine pack | novel exploits | adversarial corpus fuzz |
| Decompression bomb | D | size/recursion/time limits | resource limit events | abort member | resume other members | pathological formats | bomb fixtures |
| Case crossover | I | one-case-per-process; isolated dirs | lock/audit anomalies | deny second open | recovery state | misconfiguration | lock spike tests |
| Malicious pack | E | UNSIGNED_BLOCKED; signatures; trust states | trust state UI | refuse load | revoke pack | trusted-but-buggy pack | pack contract tests |
| Compromised AI service | S | loopback-only; minimized derived content; no write authority | endpoint allowlist audit | disable AI | clear AI context | local malware on host | AI boundary tests |
| Report tampering | T | hash-chained seals; signatures | seal verify fail | fail-closed final report | re-export from sealed case | stolen signing key | seal verify fixtures |
| DLL/library hijack | E | signed installers; safe load paths | integrity check | refuse start | reinstall | unsigned dev builds | packaging smoke |
| Supply-chain compromise | T | lockfiles, SBOM, review, attestations | vuln intake | revoke update | rollback channel | zero-day upstream | supply-chain checklist |
| Temporary-file disclosure | I | per-case temp; secure delete | path audits | case isolation | sanitize | OS forensic remnants | temp path tests |
| Installer tampering | T | code signing/notarization | signature invalid | refuse install | re-download | stolen cert | installer verify |

## Gate D note

Any uncontained native plugin mechanism that bypasses the pack sandbox fails Gate D.
