# Trareon Lab Implementation Roadmap

**Gate B companion to** `docs/RELEASE-01-CAPABILITY-MATRIX.md`  
**Date:** 2026-07-17  
**Selected shell:** Slint + Rust (`ADR-001`)  
**Selected index:** Purpose-built Rust index (`ADR-002`)

## Milestone sequence

| # | Milestone | Entry criteria | Exit criteria | Primary artifacts | Test corpus | Performance budget owner | Documentation | Rollback boundary |
|---|---|---|---|---|---|---|---|---|
| 1 | Foundation | Gate A–E PASS; ADR-001–012 ACCEPTED | Case lock, `.fsnap` import, provenance/audit/coverage, Slint shell smoke, signed offline update path stub | `crates/lab-core`, `crates/lab-case`, `apps/lab-slint` | synthetic `.fsnap`, hostile path fixtures | cold start, case open, cancel | installed docs shell | Revert to planning-only; no evidence mutation |
| 2 | Storage | Foundation release gate PASS | R1 disk images + NTFS/FAT/exFAT/APFS/ext4 read paths; deleted-file recovery selected for R1 | `crates/lab-storage`, `crates/lab-fs` | public/synthetic FS corpora | hash + index throughput | FS capability matrix chapter | Disable FS packs; keep case/evidence intact |
| 3 | Artifacts | Storage exit PASS | Selected Win/macOS/Linux artifacts + unified timeline + findings/report path | `crates/lab-artifacts`, `crates/lab-timeline` | OS artifact fixtures | timeline first render, 1M-row nav | artifact limitation matrix | Disable artifact packs selectively |
| 4 | Volatile | Artifacts exit PASS | Selected memory profiles + PCAP/flow metadata per ADR-011/R1 matrix | `crates/lab-memory`, `crates/lab-net` | restricted memory + PCAP corpora | memory overhead, cancel | memory/net limitation chapter | Quarantine volatile packs |
| 5 | Intelligence | Volatile exit PASS | Deterministic correlation graphs; optional local AI adapters | `crates/lab-correlate`, `crates/lab-ai-bridge` | correlation fixtures | search p95, AI timeout | AI boundary docs | Disable AI + advanced correlation |
| 6 | Specialist Packs | Intelligence exit PASS | Multimedia/mobile/cloud/IoT/blockchain packs as signed optional packs | `packs/*` | pack-specific corpora | pack resource limits | pack manifests + licenses | Uninstall packs; core remains |

## Change control

A capability moves between `R1`, `P0-LATER`, and `P2` only through ADR-008 change control with schedule, validation, licensing, and security impact recorded in `docs/DECISION-REGISTER.md`.
