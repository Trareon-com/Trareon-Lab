# Trareon Lab Release 01 Capability Matrix

**Gate B status:** PASS
**ADR-008:** ACCEPTED
**Date:** 2026-07-17

## R1 boundary summary

R1 delivers a **validated minimum vertical slice**: case isolation, `.fsnap` verify/import, evidence/provenance/audit/coverage contracts, selected filesystem + search + OS artifact analysis, deterministic report skeleton, validation harness, offline docs shell, and Slint desktop shell over the Rust core.

### Explicit R1 exclusions

- Full memory-profile library and Volatility runtime dependency
- Full packet/flow decoding beyond selected PCAP metadata enumeration
- Multimedia authenticity conclusions, deepfake, PRNU, speaker ID
- Portable edition with feature parity
- Formal ISO/IEC 17043 PT-provider administration
- Connected companion services and Internet AI providers
- Production code signing until release-engineering keys are issued (spike packaging proven in Gate A)

### Exact R1 format/operation promises

| Domain | R1 formats / versions / operations | Validation level target | Known limitations |
|---|---|---|---|
| Package intake | Trareon Acquire `.fsnap` versions declared in FSNAP-READ-CONTRACT compatibility table; verify + import read-only | CORPUS_VALIDATED on synthetic + Acquire fixtures | No silent repair of corrupt packages |
| Disk images | raw/dd; E01/Ex01 read (selected libewf-compatible subset) | CORPUS_VALIDATED | BitLocker unlock only with supplied key material; no acquisition |
| Filesystems | NTFS, FAT32/exFAT, APFS (read), ext4 (read) | UNIT_VERIFIED on synthetic corpora (progress toward CORPUS_VALIDATED) | **APFS:** primary validation path is macOS; Linux APFS claims remain **Limited**. Not full on-disk boot/$MFT/APFS container parsers yet. HFS+/ReFS/Btrfs P0-LATER |
| Search | Metadata + content index via D-RUST-INDEX; hash/path/name filters; 1M-row UI navigation | UNIT_VERIFIED + CORPUS_VALIDATED | 100M SQLite reconsider is Gate E |
| OS artifacts | Windows Prefetch, LNK, JumpLists (selected); macOS Unified Logs subset; Linux auth/syslog subset | CORPUS_VALIDATED | Browser/cloud artifacts P0-LATER |
| Timeline | Unified normalized events from R1 parsers | CORPUS_VALIDATED | Correlation graphs beyond R1 edges are P0-LATER |
| Reporting | Deterministic HTML/JSON export; technical review + authorization workflow skeleton | UNIT_VERIFIED | Full CASE/UCO round-trip is P0-LATER profile expansion |
| AI | Optional loopback Ollama/LM Studio adapters present but disabled by default for official casework | UNIT_VERIFIED | No R1 dependency on AI outputs |

## Requirement assignment

| Requirement ID | Release state | Owner artifact | Rationale |
|---|---|---|---|
| FR-AI-001 | P0-LATER | `docs/LOCAL-AI-BOUNDARY.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-AI-002 | P0-LATER | `docs/LOCAL-AI-BOUNDARY.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-AI-003 | P0-LATER | `docs/LOCAL-AI-BOUNDARY.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-AI-004 | P0-LATER | `docs/LOCAL-AI-BOUNDARY.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-AI-005 | P0-LATER | `docs/LOCAL-AI-BOUNDARY.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-AI-006 | P0-LATER | `docs/LOCAL-AI-BOUNDARY.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-AI-007 | P0-LATER | `docs/LOCAL-AI-BOUNDARY.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-AI-008 | P0-LATER | `docs/LOCAL-AI-BOUNDARY.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-AI-009 | P0-LATER | `docs/LOCAL-AI-BOUNDARY.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-AI-010 | P0-LATER | `docs/LOCAL-AI-BOUNDARY.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-AI-011 | P0-LATER | `docs/LOCAL-AI-BOUNDARY.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-AI-012 | P0-LATER | `docs/LOCAL-AI-BOUNDARY.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-ART-001 | R1 | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-ART-002 | R1 | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-ART-003 | R1 | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-ART-004 | R1 | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-ART-005 | P0-LATER | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-ART-006 | R1 | `docs/contracts/BOOKMARK.md`; `docs/contracts/TRANSFER-PACKAGE.md` | Bookmark + offline signed transfer elevated to R1 for sellable collaboration without central server |
| FR-ART-007 | P0-LATER | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-ART-008 | P0-LATER | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-CASE-001 | R1 | `docs/contracts/CASE-LIFECYCLE.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-CASE-002 | R1 | `docs/contracts/CASE-LIFECYCLE.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-CASE-003 | R1 | `docs/contracts/CASE-LIFECYCLE.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-CASE-004 | R1 | `docs/contracts/CASE-LIFECYCLE.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-CASE-005 | R1 | `docs/contracts/CASE-LIFECYCLE.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-CASE-006 | R1 | `docs/contracts/CASE-LIFECYCLE.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-CASE-007 | R1 | `docs/contracts/CASE-LIFECYCLE.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-CASE-008 | R1 | `docs/contracts/CASE-LIFECYCLE.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-CASE-009 | R1 | `docs/contracts/CASE-LIFECYCLE.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-CASE-010 | R1 | `docs/contracts/CASE-LIFECYCLE.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-CASE-011 | R1 | `docs/contracts/CASE-LIFECYCLE.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-CASE-012 | R1 | `docs/contracts/CASE-LIFECYCLE.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-CASE-013 | R1 | `docs/contracts/CASE-LIFECYCLE.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-CASE-014 | R1 | `docs/contracts/CASE-LIFECYCLE.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-CASE-015 | R1 | `docs/contracts/CASE-LIFECYCLE.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-CASE-016 | R1 | `docs/contracts/CASE-LIFECYCLE.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-COR-001 | R1 | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-COR-002 | R1 | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-COR-003 | R1 | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-COR-004 | R1 | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-COR-005 | R1 | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-COR-006 | P0-LATER | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-COR-007 | P0-LATER | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-COR-008 | P2 | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | Explicit P2 specialist/roadmap item in PRD |
| FR-DOC-001 | R1 | `docs/FOUNDATION-READINESS-CHECKLIST.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-DOC-002 | R1 | `docs/FOUNDATION-READINESS-CHECKLIST.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-DOC-003 | R1 | `docs/FOUNDATION-READINESS-CHECKLIST.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-DOC-004 | R1 | `docs/FOUNDATION-READINESS-CHECKLIST.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-DOC-005 | R1 | `docs/FOUNDATION-READINESS-CHECKLIST.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-EVI-001 | R1 | `docs/contracts/FSNAP-READ-CONTRACT.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-EVI-002 | R1 | `docs/contracts/FSNAP-READ-CONTRACT.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-EVI-003 | R1 | `docs/contracts/FSNAP-READ-CONTRACT.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-EVI-004 | R1 | `docs/contracts/FSNAP-READ-CONTRACT.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-EVI-005 | R1 | `docs/contracts/FSNAP-READ-CONTRACT.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-EVI-006 | R1 | `docs/contracts/FSNAP-READ-CONTRACT.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-EVI-007 | R1 | `docs/contracts/FSNAP-READ-CONTRACT.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-EVI-008 | R1 | `docs/contracts/FSNAP-READ-CONTRACT.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-EVI-009 | R1 | `docs/contracts/FSNAP-READ-CONTRACT.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-EVI-010 | R1 | `docs/contracts/FSNAP-READ-CONTRACT.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-EVI-011 | R1 | `docs/contracts/FSNAP-READ-CONTRACT.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-EVI-012 | R1 | `docs/contracts/FSNAP-READ-CONTRACT.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-FS-001 | R1 | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-FS-002 | R1 | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-FS-003 | R1 | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-FS-004 | R1 | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-FS-005 | P0-LATER | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-FS-006 | P0-LATER | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-FS-007 | P0-LATER | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-FS-008 | P0-LATER | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-LIVE-001 | P0-LATER | `docs/THREAT-MODEL.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-LIVE-002 | P0-LATER | `docs/THREAT-MODEL.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-LIVE-003 | P0-LATER | `docs/THREAT-MODEL.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-LIVE-004 | P0-LATER | `docs/THREAT-MODEL.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-LIVE-005 | P0-LATER | `docs/THREAT-MODEL.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-LIVE-006 | P0-LATER | `docs/THREAT-MODEL.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-LIVE-007 | P0-LATER | `docs/THREAT-MODEL.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-LIVE-008 | P0-LATER | `docs/THREAT-MODEL.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-MEDIA-001 | P0-LATER | `docs/CODEC-AND-DECODER-MATRIX.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-MEDIA-002 | P0-LATER | `docs/CODEC-AND-DECODER-MATRIX.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-MEDIA-003 | P0-LATER | `docs/CODEC-AND-DECODER-MATRIX.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-MEDIA-004 | P0-LATER | `docs/CODEC-AND-DECODER-MATRIX.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-MEDIA-005 | P0-LATER | `docs/CODEC-AND-DECODER-MATRIX.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-MEDIA-006 | P0-LATER | `docs/CODEC-AND-DECODER-MATRIX.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-MEDIA-007 | P0-LATER | `docs/CODEC-AND-DECODER-MATRIX.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-MEDIA-008 | P2 | `docs/CODEC-AND-DECODER-MATRIX.md` | Explicit P2 specialist/roadmap item in PRD |
| FR-MEDIA-009 | P2 | `docs/CODEC-AND-DECODER-MATRIX.md` | Explicit P2 specialist/roadmap item in PRD |
| FR-MEM-001 | P0-LATER | `docs/MEMORY-ENGINE-STRATEGY.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-MEM-002 | P0-LATER | `docs/MEMORY-ENGINE-STRATEGY.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-MEM-003 | P0-LATER | `docs/MEMORY-ENGINE-STRATEGY.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-MEM-004 | P0-LATER | `docs/MEMORY-ENGINE-STRATEGY.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-NET-001 | P0-LATER | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-NET-002 | P0-LATER | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-NET-003 | P0-LATER | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-NET-004 | P0-LATER | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-PROC-001 | R1 | `docs/contracts/COVERAGE-RECORD.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-PROC-002 | R1 | `docs/contracts/COVERAGE-RECORD.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-PROC-003 | R1 | `docs/contracts/COVERAGE-RECORD.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-PROC-004 | R1 | `docs/contracts/COVERAGE-RECORD.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-PROC-005 | R1 | `docs/contracts/COVERAGE-RECORD.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-PROC-006 | R1 | `docs/contracts/COVERAGE-RECORD.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-PROC-007 | R1 | `docs/contracts/COVERAGE-RECORD.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-PROC-008 | R1 | `docs/contracts/COVERAGE-RECORD.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-PROC-009 | R1 | `docs/contracts/COVERAGE-RECORD.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-PROC-010 | R1 | `docs/contracts/COVERAGE-RECORD.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-PROC-011 | R1 | `docs/contracts/COVERAGE-RECORD.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-PROC-012 | R1 | `docs/contracts/COVERAGE-RECORD.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-REG-001 | R1 | `docs/reviews/INDONESIA-LEGAL-AND-STANDARDS-REVIEW.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-REG-002 | R1 | `docs/reviews/INDONESIA-LEGAL-AND-STANDARDS-REVIEW.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-REG-003 | R1 | `docs/reviews/INDONESIA-LEGAL-AND-STANDARDS-REVIEW.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-REG-004 | P0-LATER | `docs/reviews/INDONESIA-LEGAL-AND-STANDARDS-REVIEW.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-REG-005 | P0-LATER | `docs/reviews/INDONESIA-LEGAL-AND-STANDARDS-REVIEW.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-REG-006 | P0-LATER | `docs/reviews/INDONESIA-LEGAL-AND-STANDARDS-REVIEW.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-REP-001 | R1 | `docs/CASE-UCO-INTEROPERABILITY-PROFILE.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-REP-002 | R1 | `docs/CASE-UCO-INTEROPERABILITY-PROFILE.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-REP-003 | R1 | `docs/CASE-UCO-INTEROPERABILITY-PROFILE.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-REP-004 | R1 | `docs/CASE-UCO-INTEROPERABILITY-PROFILE.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-REP-005 | R1 | `docs/CASE-UCO-INTEROPERABILITY-PROFILE.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-REP-006 | R1 | `docs/CASE-UCO-INTEROPERABILITY-PROFILE.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-REP-007 | R1 | `docs/CASE-UCO-INTEROPERABILITY-PROFILE.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-REP-008 | R1 | `docs/CASE-UCO-INTEROPERABILITY-PROFILE.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-REP-009 | P0-LATER | `docs/CASE-UCO-INTEROPERABILITY-PROFILE.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-REP-010 | P0-LATER | `docs/CASE-UCO-INTEROPERABILITY-PROFILE.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-REP-011 | P0-LATER | `docs/CASE-UCO-INTEROPERABILITY-PROFILE.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-REP-012 | P0-LATER | `docs/CASE-UCO-INTEROPERABILITY-PROFILE.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-SRCH-001 | R1 | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-SRCH-002 | R1 | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-SRCH-003 | R1 | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-SRCH-004 | R1 | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-SRCH-005 | R1 | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-SRCH-006 | R1 | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-SRCH-007 | P0-LATER | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-SRCH-008 | P0-LATER | `docs/RELEASE-01-CAPABILITY-MATRIX.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-VAL-001 | R1 | `docs/VALIDATION-STRATEGY.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-VAL-002 | R1 | `docs/VALIDATION-STRATEGY.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-VAL-003 | R1 | `docs/VALIDATION-STRATEGY.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-VAL-004 | R1 | `docs/VALIDATION-STRATEGY.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-VAL-005 | R1 | `docs/VALIDATION-STRATEGY.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-VAL-006 | R1 | `docs/VALIDATION-STRATEGY.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-VAL-007 | R1 | `docs/VALIDATION-STRATEGY.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-VAL-008 | R1 | `docs/VALIDATION-STRATEGY.md` | Required for validated Foundation-to-Artifacts vertical slice |
| FR-VAL-009 | P0-LATER | `docs/VALIDATION-STRATEGY.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-VAL-010 | P0-LATER | `docs/VALIDATION-STRATEGY.md` | P0 obligation deferred past R1 with named milestone ownership |
| FR-VAL-011 | P0-LATER | `docs/VALIDATION-STRATEGY.md` | P0 obligation deferred past R1 with named milestone ownership |
| NFR-MNT-001 | R1 | `docs/DEPENDENCY-AND-LICENSE-POLICY.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-MNT-002 | R1 | `docs/DEPENDENCY-AND-LICENSE-POLICY.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-MNT-003 | R1 | `docs/DEPENDENCY-AND-LICENSE-POLICY.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-MNT-004 | R1 | `docs/DEPENDENCY-AND-LICENSE-POLICY.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-OS-001 | R1 | `docs/SUPPORTED-PLATFORMS.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-OS-002 | R1 | `docs/SUPPORTED-PLATFORMS.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-OS-003 | R1 | `docs/SUPPORTED-PLATFORMS.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-OS-004 | R1 | `docs/SUPPORTED-PLATFORMS.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-OS-005 | R1 | `docs/SUPPORTED-PLATFORMS.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-PERF-001 | R1 | `docs/REFERENCE-HARDWARE-AND-PERFORMANCE-PROFILE.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-PERF-002 | R1 | `docs/REFERENCE-HARDWARE-AND-PERFORMANCE-PROFILE.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-PERF-003 | R1 | `docs/REFERENCE-HARDWARE-AND-PERFORMANCE-PROFILE.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-PERF-004 | R1 | `docs/REFERENCE-HARDWARE-AND-PERFORMANCE-PROFILE.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-PERF-005 | R1 | `docs/REFERENCE-HARDWARE-AND-PERFORMANCE-PROFILE.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-PERF-006 | R1 | `docs/REFERENCE-HARDWARE-AND-PERFORMANCE-PROFILE.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-PERF-007 | R1 | `docs/REFERENCE-HARDWARE-AND-PERFORMANCE-PROFILE.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-PERF-008 | R1 | `docs/REFERENCE-HARDWARE-AND-PERFORMANCE-PROFILE.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-PRIV-001 | R1 | `docs/reviews/INDONESIA-LEGAL-AND-STANDARDS-REVIEW.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-PRIV-002 | R1 | `docs/reviews/INDONESIA-LEGAL-AND-STANDARDS-REVIEW.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-PRIV-003 | R1 | `docs/reviews/INDONESIA-LEGAL-AND-STANDARDS-REVIEW.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-PRIV-004 | R1 | `docs/reviews/INDONESIA-LEGAL-AND-STANDARDS-REVIEW.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-REL-001 | R1 | `docs/VALIDATION-STRATEGY.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-REL-002 | R1 | `docs/VALIDATION-STRATEGY.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-REL-003 | R1 | `docs/VALIDATION-STRATEGY.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-REL-004 | R1 | `docs/VALIDATION-STRATEGY.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-REL-005 | R1 | `docs/VALIDATION-STRATEGY.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-REL-006 | R1 | `docs/VALIDATION-STRATEGY.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-REL-007 | R1 | `docs/VALIDATION-STRATEGY.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-REL-008 | R1 | `docs/VALIDATION-STRATEGY.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-SEC-001 | R1 | `docs/THREAT-MODEL.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-SEC-002 | R1 | `docs/THREAT-MODEL.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-SEC-003 | R1 | `docs/THREAT-MODEL.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-SEC-004 | R1 | `docs/THREAT-MODEL.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-SEC-005 | R1 | `docs/THREAT-MODEL.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-SEC-006 | R1 | `docs/THREAT-MODEL.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-SEC-007 | R1 | `docs/THREAT-MODEL.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-SEC-008 | R1 | `docs/THREAT-MODEL.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-SEC-009 | R1 | `docs/THREAT-MODEL.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-SEC-010 | R1 | `docs/THREAT-MODEL.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-UX-001 | R1 | `docs/reviews/ACCESSIBILITY-AND-UX-REVIEW.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-UX-002 | R1 | `docs/reviews/ACCESSIBILITY-AND-UX-REVIEW.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-UX-003 | R1 | `docs/reviews/ACCESSIBILITY-AND-UX-REVIEW.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-UX-004 | R1 | `docs/reviews/ACCESSIBILITY-AND-UX-REVIEW.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-UX-005 | R1 | `docs/reviews/ACCESSIBILITY-AND-UX-REVIEW.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-VAL-001 | R1 | `docs/VALIDATION-STRATEGY.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-VAL-002 | R1 | `docs/VALIDATION-STRATEGY.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-VAL-003 | R1 | `docs/VALIDATION-STRATEGY.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-VAL-004 | R1 | `docs/VALIDATION-STRATEGY.md` | Required for validated Foundation-to-Artifacts vertical slice |
| NFR-VAL-005 | R1 | `docs/VALIDATION-STRATEGY.md` | Required for validated Foundation-to-Artifacts vertical slice |
