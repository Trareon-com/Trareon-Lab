# Trareon Lab RFC and Foundation Preparation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `executing-plans` to implement this plan task by task. Do not start production application code until every mandatory gate in this plan is marked `PASS`.

**Goal:** Convert the approved Trareon Lab PRD into an implementation-ready, auditable architecture package and a separately executable Foundation implementation plan.

**Architecture:** This plan produces decision records and executable contracts before selecting the desktop shell. The product architecture is constrained to an authoritative Rust forensic core, a replaceable desktop presentation shell, isolated case processes, deterministic evidence handling, sandboxed analysis packs, and optional loopback-only local AI adapters. The exact desktop stack is selected by a scored proof-of-capability gate, not by preference alone.

**Tech Stack:** Markdown decision artifacts; shell-based document checks; Rust core as a fixed architectural constraint; desktop candidates Tauri 2 + Svelte 5, Slint + Rust, and Avalonia + Rust FFI; SQLite candidate for case metadata/indexing; JSON/JSONL and canonical JSON for portable records; SHA-256 minimum evidence digest; Ed25519 candidate for signatures.

## Global Constraints

- Source of truth: `PRD-Digital-Forensic-Analysis-Lab.md` v0.9 or its approved successor.
- Indonesia-first, international-ready. Legal and standards language must distinguish “aligned with”, “validated against”, “certified”, and “accredited”.
- No claim that Trareon Lab itself is ISO/IEC 17025 accredited or ISO/IEC 17043 compliant without an authorized conformity-assessment outcome.
- Full forensic analysis works offline. Ollama and LM Studio are optional localhost services and may never become evidence-processing dependencies.
- One open case runs in one isolated application process. Evidence from different cases may not share writable state, caches, temporary extraction directories, indexes, AI context, or logs.
- Evidence bytes are immutable. Derived artifacts are content-addressed and linked to their source, tool version, parameters, timestamps, and validation status.
- `.fsnap` compatibility is read/import compatibility with Trareon Acquire. Trareon Lab may not silently repair or rewrite a source package.
- No production code, repository remote, third-party binary vendoring, or license commitment is authorized by this preparation plan.
- Every mandatory decision records alternatives, evidence, owner, date, rationale, consequences, and reopening criteria.
- No `TBD`, `TODO`, “and others”, or unbounded “supports all” wording may remain in a gate artifact.
- Commit steps below assume the user has approved creating a dedicated Git repository at `Trareon Lab`. If not approved, stop before Task 1 Step 3 and request authorization.

## Gate Summary

| Gate | Required outcome | Blocks |
|---|---|---|
| A — Architecture | One desktop stack and component boundary selected through mandatory tests | All production code |
| B — Release boundary | Every P0 capability assigned to R1, later P0 milestone, or P2 with rationale | Backlog and UI implementation |
| C — Evidence trust | `.fsnap`, provenance, crypto, audit, and case-isolation contracts approved | Evidence ingestion |
| D — Safety and validation | Threat model, pack sandbox, validation corpus, and false-positive policy approved | Parser/pack execution |
| E — Delivery | Performance profile, platform matrix, documentation set, and Foundation plan approved | Foundation implementation |

---

## Task 1: Establish the Controlled Planning Repository

**Files:**

- Create: `.gitignore`
- Create: `docs/DECISION-REGISTER.md`
- Create: `docs/SOURCE-REGISTER.md`
- Create: `docs/TRACEABILITY-INDEX.md`
- Modify: `PRD-Digital-Forensic-Analysis-Lab.md`

**Step 1: Confirm the baseline exists and is not already versioned**

Run:

```bash
cd "/Users/user/Projects/Trareon/Trareon Lab"
test -f PRD-Digital-Forensic-Analysis-Lab.md
test ! -d .git
```

Expected: both commands exit `0`. If `.git` exists, inspect it and preserve its history instead of reinitializing.

**Step 2: Record explicit repository authorization**

Before mutation, record the user’s approval in the task log. If approval has not been given, stop here. Do not infer permission to create a remote or publish files.

**Step 3: Initialize the local repository**

Run:

```bash
git init
git branch -M main
```

Expected: a local `main` branch exists; no remote is configured.

**Step 4: Create repository exclusions**

Create `.gitignore` with exactly these initial safety exclusions:

```gitignore
.DS_Store
.idea/
.vscode/
target/
node_modules/
dist/
coverage/
*.log
*.tmp
*.dmp
*.raw
*.mem
*.E01
*.Ex01
*.L01
*.fsnap
test-results/private/
fixtures/restricted/
```

**Step 5: Create the decision register**

Create `docs/DECISION-REGISTER.md` with columns for `ID`, `Decision`, `Status`, `Owner`, `Evidence`, `Date`, `Reopen condition`, and `Affected requirements`. Pre-register these decisions:

- ADR-001 desktop stack;
- ADR-002 case database and search index;
- ADR-003 cryptographic profile;
- ADR-004 process and case isolation;
- ADR-005 analysis-pack sandbox;
- ADR-006 `.fsnap` import contract;
- ADR-007 optional local-AI boundary;
- ADR-008 Release 1 capability boundary;
- ADR-009 supported OS and reference hardware;
- ADR-010 decoder and codec licensing;
- ADR-011 memory-analysis engine strategy;
- ADR-012 CASE/UCO interoperability profile.

All entries start as `PROPOSED`; only a passed gate may change one to `ACCEPTED`.

**Step 6: Create the traceability index**

Create `docs/TRACEABILITY-INDEX.md` mapping every PRD `FR-*` and `NFR-*` identifier to one owner artifact and one release state. Enforce these release states only: `R1`, `P0-LATER`, `P2`, `REJECTED`.

**Step 7: Create the controlled source register**

Create `docs/SOURCE-REGISTER.md` for standards, regulations, technical specifications, and benchmark methods. Each entry records title, issuing authority, exact edition/version, publication/effective date, Indonesian adoption status where relevant, authoritative locator, controlled-copy owner, access date, applicability, license/copyright handling, and next review date. Use official publishers and government sources for normative claims; do not copy restricted standards text into the repository.

At minimum register ISO/IEC 27037, 27041, 27042, 27043, 17025, 17043, the selected NIST/SWGDE/INTERPOL/CASE/UCO/C2PA materials, applicable Indonesian law and SNI adoptions, and the Trareon Acquire `.fsnap` contract.

**Step 8: Mark PRD review status without changing scope**

Change the PRD status from `Draft for Review` to `Product Baseline Approved; Architecture Pending` and append a change-control note that scope changes require an entry in `docs/DECISION-REGISTER.md`.

**Step 9: Verify the controlled baseline**

Run:

```bash
rg -n "Product Baseline Approved; Architecture Pending|change control" PRD-Digital-Forensic-Analysis-Lab.md
rg -n "ADR-00[1-9]|ADR-01[0-2]" docs/DECISION-REGISTER.md
rg -n "edition|effective date|authoritative|next review|ISO/IEC 27042|ISO/IEC 17025|ISO/IEC 17043" docs/SOURCE-REGISTER.md
rg -o "(FR|NFR)-[A-Z0-9-]+" PRD-Digital-Forensic-Analysis-Lab.md | sort -u > /tmp/trareon-prd-ids.txt
rg -o "(FR|NFR)-[A-Z0-9-]+" docs/TRACEABILITY-INDEX.md | sort -u > /tmp/trareon-trace-ids.txt
diff -u /tmp/trareon-prd-ids.txt /tmp/trareon-trace-ids.txt
git diff --check
```

Expected: all searches succeed, `diff` is empty, and `git diff --check` reports nothing.

**Step 10: Commit**

```bash
git add .gitignore PRD-Digital-Forensic-Analysis-Lab.md docs/DECISION-REGISTER.md docs/SOURCE-REGISTER.md docs/TRACEABILITY-INDEX.md docs/superpowers/plans/2026-07-17-trareon-lab-rfc-foundation-preparation.md
git commit -m "docs: establish Trareon Lab product baseline"
```

---

## Task 2: Select the Desktop Architecture Through a Proof-of-Capability Gate

**Files:**

- Create: `docs/ARCHITECTURE-DECISION-MATRIX.md`
- Create: `spikes/README.md`
- Create: `spikes/tauri/`
- Create: `spikes/slint/`
- Create: `spikes/avalonia/`
- Create: `RFC-Digital-Forensic-Analysis-Lab.md`
- Modify: `docs/DECISION-REGISTER.md`

**Step 1: Define mandatory rejection gates**

In `docs/ARCHITECTURE-DECISION-MATRIX.md`, reject any candidate that cannot demonstrate all of the following on Windows, macOS, and Linux:

1. installation without a separately installed language runtime;
2. a virtualized table rendering 1,000,000 synthetic rows without loading them all into UI memory;
3. cancellable background work with bounded queues and responsive UI;
4. one-case-per-process launch and crash containment;
5. secure IPC with schema validation and request correlation IDs;
6. keyboard-first workspace, dockable or equivalent multi-pane layout, accessibility labels, and scalable text;
7. signed installer/update artifact support while preserving fully offline operation;
8. deterministic build inputs with generated SBOM and third-party license inventory;
9. no evidence bytes or secrets in webview/devtools-accessible storage;
10. stable Rust-core integration without duplicating forensic business logic in the UI layer.

The database/index sub-spike must also compare bundled SQLite-only indexing, SQLite plus an embedded full-text/index engine, and a purpose-built Rust index against schema migration safety, crash recovery, deterministic query behavior, disk amplification, 100-million-record search latency, licensing, and case portability.

**Step 2: Define weighted scoring after mandatory gates**

Score passing candidates out of 100:

- security boundary and sandbox compatibility: 25;
- large-dataset UX and rendering performance: 20;
- cross-platform packaging and signing: 15;
- accessibility and professional desktop interaction: 15;
- maintainability and testability: 10;
- reuse with Trareon Acquire: 10;
- binary size and idle resource use: 5.

Tie-break order: stronger security boundary, then lower peak memory, then reuse with Trareon Acquire. Record raw measurements, not subjective labels alone.

**Step 3: Build equal vertical spikes**

Each spike must implement the same synthetic workflow: open a fake case, stream a million metadata rows, filter by hash prefix, open a detail pane, start/cancel a background hash job, simulate worker crash, export a deterministic JSON result, and reopen the case in a second process only after the first releases its lock.

Use synthetic data only. Do not include production evidence parsers.

**Step 4: Run the measurement protocol**

Measure cold start, idle RSS, peak RSS, initial table display, filter latency p50/p95, cancellation latency, crash recovery, installer size, and accessibility smoke results on every target OS. Use the reference hardware tiers defined in Task 8; do not compare candidates across different tiers.

**Step 5: Write the RFC from the selected result**

Create `RFC-Digital-Forensic-Analysis-Lab.md` using `../Template/Desktop/RFC-Desktop.md` as the structural source. It must specify:

- selected stack and rejected alternatives;
- authoritative Rust-core boundary;
- UI/IPC trust boundary;
- component diagram and repository layout;
- case-process lifecycle and locking;
- worker pools, cancellation, backpressure, and crash recovery;
- case database, index, blob store, cache, and export boundaries;
- pack runtime and capability broker;
- `.fsnap` import pipeline;
- AI adapter boundary;
- packaging/signing/SBOM strategy;
- offline dependency and reproducible-build policy;
- test pyramid and platform CI matrix;
- migration/versioning policy;
- failure modes and recovery UX.

**Step 6: Accept ADR-001 and ADR-002 only if all mandatory gates pass**

Update ADR-001 with the matrix link, selected candidate, consequences, and exact reopening thresholds. Record the selected case database/index design in ADR-002 with measured migration, crash-recovery, query, and storage results. If no desktop candidate passes, keep ADR-001 `PROPOSED`, record the failures, and stop the plan. Do not choose the highest-scoring failed candidate.

**Step 7: Verify and commit**

Run:

```bash
rg -n "Mandatory gates|Weighted score|Raw measurements|Tie-break" docs/ARCHITECTURE-DECISION-MATRIX.md
rg -n "authoritative Rust|case process|IPC|backpressure|SBOM|migration|failure mode" RFC-Digital-Forensic-Analysis-Lab.md
rg -n "ADR-001.*ACCEPTED|ADR-002.*ACCEPTED" docs/DECISION-REGISTER.md
rg -n "TBD|TODO|supports all|and others" docs/ARCHITECTURE-DECISION-MATRIX.md RFC-Digital-Forensic-Analysis-Lab.md && exit 1 || true
git diff --check
git add docs/ARCHITECTURE-DECISION-MATRIX.md docs/DECISION-REGISTER.md RFC-Digital-Forensic-Analysis-Lab.md spikes
git commit -m "docs: select Trareon Lab desktop architecture"
```

Expected: all required sections exist, forbidden ambiguity is absent, and ADR-001 is accepted.

---

## Task 3: Freeze the Release 1 Capability Boundary

**Files:**

- Create: `docs/RELEASE-01-CAPABILITY-MATRIX.md`
- Create: `docs/IMPLEMENTATION-ROADMAP.md`
- Modify: `docs/TRACEABILITY-INDEX.md`
- Modify: `docs/DECISION-REGISTER.md`

**Step 1: Define the Release 1 vertical slice**

Release 1 must be useful without claiming full P0 breadth. Its minimum vertical slice is:

- create/open/close/recover a case;
- define authority and scope;
- import and verify a Trareon Acquire `.fsnap` package;
- register immutable evidence and provenance;
- perform read-only file browsing, metadata inspection, hashing, text/hex preview, and indexed search on supported evidence;
- create findings, bookmarks, notes, and timeline entries;
- expose coverage, unsupported items, errors, and validation status;
- export a deterministic Indonesian report and machine-readable audit package;
- run optional Ollama/LM Studio assistance on derived text only, with citations and explicit human acceptance;
- provide bundled Indonesian user, examiner, administrator, validation, and troubleshooting documentation.

**Step 2: Classify every requirement**

In `docs/RELEASE-01-CAPABILITY-MATRIX.md`, give every PRD requirement exactly one state: `R1`, `P0-LATER`, `P2`, or `REJECTED`. Include rationale, dependencies, validation method, and owning milestone.

No category label such as “RAM analysis” is sufficient: enumerate the actual formats, operating-system versions, artifact families, and operations promised by R1.

**Step 3: Define milestone sequence and exit gates**

Use these non-overlapping milestones:

1. Foundation — case, authority, `.fsnap`, provenance, audit, coverage, validation harness, docs skeleton;
2. Storage — image/container support, filesystems, indexing, search, deleted-file/recovery support explicitly selected for R1;
3. Artifacts — selected Windows/macOS/Linux artifacts, normalized timeline, entities, findings, reporting;
4. Volatile — selected memory profiles and packet/flow formats;
5. Intelligence — deterministic correlation and optional local AI;
6. Specialist Packs — multimedia, mobile, cloud, IoT, blockchain, deepfake, and other approved P2 packs.

Each milestone must list entry criteria, exit criteria, artifacts, test corpus, performance budget, documentation, and rollback boundary.

**Step 4: Accept ADR-008**

Record the exact R1 boundary, including explicit exclusions. A capability moves between states only through change control with schedule, validation, licensing, and security impact.

**Step 5: Verify and commit**

Run:

```bash
rg -o "(FR|NFR)-[A-Z0-9-]+" PRD-Digital-Forensic-Analysis-Lab.md | sort -u > /tmp/trareon-prd-ids.txt
rg -o "(FR|NFR)-[A-Z0-9-]+" docs/RELEASE-01-CAPABILITY-MATRIX.md | sort -u > /tmp/trareon-release-ids.txt
diff -u /tmp/trareon-prd-ids.txt /tmp/trareon-release-ids.txt
rg -n "R1|P0-LATER|P2|REJECTED" docs/RELEASE-01-CAPABILITY-MATRIX.md
rg -n "ADR-008.*ACCEPTED" docs/DECISION-REGISTER.md
git diff --check
git add docs/RELEASE-01-CAPABILITY-MATRIX.md docs/IMPLEMENTATION-ROADMAP.md docs/TRACEABILITY-INDEX.md docs/DECISION-REGISTER.md
git commit -m "docs: freeze Trareon Lab Release 1 boundary"
```

---

## Task 4: Specify Evidence, Case, and `.fsnap` Contracts

**Files:**

- Create: `docs/contracts/CASE-LIFECYCLE.md`
- Create: `docs/contracts/EVIDENCE-OBJECT.md`
- Create: `docs/contracts/PROVENANCE-EVENT.md`
- Create: `docs/contracts/AUDIT-EVENT.md`
- Create: `docs/contracts/COVERAGE-RECORD.md`
- Create: `docs/contracts/FSNAP-READ-CONTRACT.md`
- Create: `docs/CASE-UCO-INTEROPERABILITY-PROFILE.md`
- Create: `schemas/case.schema.json`
- Create: `schemas/evidence-object.schema.json`
- Create: `schemas/provenance-event.schema.json`
- Create: `schemas/audit-event.schema.json`
- Create: `schemas/coverage-record.schema.json`
- Create: `schemas/fsnap-import-result.schema.json`
- Modify: `docs/DECISION-REGISTER.md`

**Step 1: Define the case state machine**

Specify only these case states: `CREATED`, `OPEN`, `READ_ONLY`, `RECOVERY_REQUIRED`, `CLOSED`, `ARCHIVED`. Define legal transitions, lock ownership, unclean-shutdown behavior, clock handling, and export eligibility.

**Step 2: Define immutable evidence identity**

The evidence object must contain a stable UUID, source designation, evidence class, byte length, acquisition/import hashes, original locator, authority/scope link, acquisition tool/version, import tool/version, time-source metadata, write-protection observation, validation state, and ordered provenance-event references.

Source evidence is never updated in place. Corrections create a new signed metadata revision linked to the prior revision.

**Step 3: Define provenance, audit, and coverage events**

- Provenance records evidence-transforming operations and derived-object lineage.
- Audit records security- and workflow-relevant user/system actions.
- Coverage records attempted, succeeded, partial, unsupported, skipped, failed, and not-applicable analysis units.

Every event includes schema version, event UUID, case UUID, actor identity, process identity, UTC timestamp with original offset, monotonic sequence, previous-event digest, payload digest, and software build identity.

**Step 4: Define `.fsnap` verification and import**

The contract must cover package version negotiation, manifest canonicalization, declared hash verification, signature trust states, missing/extra member detection, path traversal rejection, decompression limits, duplicate names, symlinks, sparse files, timestamp preservation, encrypted members, unknown extensions, partial/corrupt packages, cancellation, resumability, and deterministic import results.

Use a compatibility table tied to the current Trareon Acquire `.fsnap` specification. If the Acquire contract is incomplete, open a cross-product ADR instead of inventing incompatible behavior.

**Step 5: Write JSON Schemas and conformance fixtures**

Use JSON Schema 2020-12. For each schema create at least one valid and three invalid synthetic fixtures: missing required field, wrong type/enum, and integrity-link violation represented for semantic validation.

**Step 6: Define the CASE/UCO interoperability profile**

Map Trareon case, evidence, provenance, actor, action, location, relationship, finding, timeline, and hash concepts to an exact versioned CASE/UCO profile. Distinguish lossless mapping, lossy mapping, Trareon extension, and unsupported concept. Define stable identifiers, namespace rules, import/export validation, round-trip expectations, and behavior for unknown vocabulary terms.

**Step 7: Accept ADR-004, ADR-006, and ADR-012**

The ADRs must link the state machine, process isolation, schema versions, `.fsnap` compatibility table, and CASE/UCO mapping profile.

**Step 8: Verify and commit**

Run the repository-selected JSON Schema validator over all valid and invalid fixtures, then:

```bash
rg -n "CREATED|RECOVERY_REQUIRED|ARCHIVED" docs/contracts/CASE-LIFECYCLE.md
rg -n "path traversal|decompression|duplicate|partial|resum" docs/contracts/FSNAP-READ-CONTRACT.md
rg -n "lossless|lossy|Trareon extension|round-trip|unknown vocabulary" docs/CASE-UCO-INTEROPERABILITY-PROFILE.md
rg -n '"\$schema".*2020-12' schemas/*.json
rg -n "ADR-004.*ACCEPTED|ADR-006.*ACCEPTED|ADR-012.*ACCEPTED" docs/DECISION-REGISTER.md
git diff --check
git add docs/contracts docs/CASE-UCO-INTEROPERABILITY-PROFILE.md schemas fixtures/contracts docs/DECISION-REGISTER.md
git commit -m "docs: define case and evidence trust contracts"
```

---

## Task 5: Lock the Cryptographic and Identity Profile

**Files:**

- Create: `docs/CRYPTOGRAPHIC-PROFILE.md`
- Create: `docs/IDENTITY-AND-ROLE-MODEL.md`
- Create: `schemas/signature-envelope.schema.json`
- Modify: `docs/DECISION-REGISTER.md`

**Step 1: Define digest usage**

Require SHA-256 for baseline interoperability and allow a second configured modern digest only when it is recorded with an algorithm identifier. MD5/SHA-1 may be computed solely for legacy correlation and must be visibly labeled non-authoritative for integrity.

**Step 2: Define signature envelopes and trust states**

Specify canonicalization, algorithm identifiers, key identifiers, certificate/public-key material, signing time, time-source quality, payload digest, signature bytes, and verification result. Trust states are exactly: `VALID_TRUSTED`, `VALID_UNTRUSTED`, `INVALID`, `EXPIRED_OR_REVOKED`, `NOT_SIGNED`, `NOT_CHECKED`.

Evaluate Ed25519 and an Indonesia deployment profile based on organization PKI requirements. Do not couple evidence verification to Internet access; revocation data may be imported as a signed offline trust bundle.

**Step 3: Define actor identity and roles**

Define local examiner, reviewer, administrator, validation operator, and observer roles. Every privileged operation records actor, role, authentication method, and approval when dual control is required. OS identity is an input, not the sole stable actor identifier.

**Step 4: Define key lifecycle**

Cover generation, import, hardware-backed availability, encrypted storage, backup/recovery, rotation, revocation, offline trust bundles, loss handling, and export. Never invent a proprietary cryptographic primitive.

**Step 5: Accept ADR-003**

Require a named external cryptography review before calling the profile stable.

**Step 6: Verify and commit**

```bash
rg -n "SHA-256|legacy correlation|canonical|revocation|offline trust bundle|key loss" docs/CRYPTOGRAPHIC-PROFILE.md
rg -n "examiner|reviewer|administrator|validation operator|observer" docs/IDENTITY-AND-ROLE-MODEL.md
rg -n "ADR-003.*ACCEPTED" docs/DECISION-REGISTER.md
git diff --check
git add docs/CRYPTOGRAPHIC-PROFILE.md docs/IDENTITY-AND-ROLE-MODEL.md schemas/signature-envelope.schema.json docs/DECISION-REGISTER.md
git commit -m "docs: define cryptographic and identity profiles"
```

---

## Task 6: Threat-Model the Product and Define the Analysis-Pack Sandbox

**Files:**

- Create: `docs/THREAT-MODEL.md`
- Create: `docs/ANALYSIS-PACK-CONTRACT.md`
- Create: `schemas/analysis-pack-manifest.schema.json`
- Create: `docs/LOCAL-AI-BOUNDARY.md`
- Modify: `docs/DECISION-REGISTER.md`

**Step 1: Model assets, actors, and trust boundaries**

Include hostile evidence, malformed containers, decompression bombs, parser exploits, malicious packs, local unprivileged users, privileged insiders, compromised AI services, poisoned models, report tampering, case crossover, temporary-file disclosure, DLL/library search hijacking, installer tampering, and supply-chain compromise.

Map threats using STRIDE and record prevention, detection, containment, recovery, residual risk, and validation test for each threat.

**Step 2: Define the capability-based pack contract**

Each pack manifest declares identity, publisher, version, compatible core API, evidence/media types, parsers, requested capabilities, resource limits, output schemas, deterministic/non-deterministic operations, network need, bundled licenses, signatures, and validation corpus identity.

Default capabilities are no network, no host filesystem, no process launch, no environment secrets, no clipboard, no arbitrary native library loading, read-only brokered evidence ranges, and write-only content-addressed derived output.

Packs execute out of process with CPU, memory, output-size, recursion, file-count, and wall-time limits. A crash or timeout produces a coverage event and may not crash the case process.

**Step 3: Define pack trust and release states**

States are `BUNDLED_VALIDATED`, `SIGNED_TRUSTED`, `SIGNED_UNTRUSTED`, `UNSIGNED_BLOCKED`, `REVOKED`, and `INCOMPATIBLE`. Developer mode must be explicit, visibly persistent, case-audited, and excluded from validated reports unless the reviewer accepts the deviation.

**Step 4: Define the local-AI boundary**

Ollama and LM Studio adapters connect only to explicitly configured loopback endpoints. Send derived/minimized content, never raw evidence by default. Every AI output records model identity, endpoint type, prompt-template version, source citations, parameters, timestamp, acceptance state, and disclaimer. AI may suggest; it may not establish evidence integrity, alter provenance, automatically close findings, or issue an examiner conclusion.

**Step 5: Accept ADR-005 and ADR-007**

Run a security design review. Any uncontained native plugin mechanism fails the gate.

**Step 6: Verify and commit**

```bash
rg -n "hostile evidence|parser exploit|case crossover|supply-chain" docs/THREAT-MODEL.md
rg -n "no network|out of process|resource limit|UNSIGNED_BLOCKED|REVOKED" docs/ANALYSIS-PACK-CONTRACT.md
rg -n "loopback|derived|minimized|source citations|may not" docs/LOCAL-AI-BOUNDARY.md
rg -n "ADR-005.*ACCEPTED|ADR-007.*ACCEPTED" docs/DECISION-REGISTER.md
git diff --check
git add docs/THREAT-MODEL.md docs/ANALYSIS-PACK-CONTRACT.md docs/LOCAL-AI-BOUNDARY.md schemas/analysis-pack-manifest.schema.json docs/DECISION-REGISTER.md
git commit -m "docs: define threat model and extension sandbox"
```

---

## Task 7: Define the Validation Program and Corpus Governance

**Files:**

- Create: `docs/VALIDATION-STRATEGY.md`
- Create: `docs/CORPUS-GOVERNANCE.md`
- Create: `docs/METHOD-VALIDATION-RECORD.md`
- Create: `docs/PROFICIENCY-TESTING-PROFILE.md`
- Create: `validation/manifest.schema.json`
- Create: `validation/public/README.md`
- Create: `validation/synthetic/README.md`
- Create: `validation/restricted/README.md`

**Step 1: Define validation levels**

Use four labels only:

- `UNIT_VERIFIED` — implementation logic tests;
- `CORPUS_VALIDATED` — expected results on identified corpus;
- `CROSS_TOOL_CORROBORATED` — compared with named independent tools/methods;
- `LAB_METHOD_APPROVED` — accepted under the deploying laboratory’s controlled method.

The UI and reports must never collapse these into a generic “verified” badge.

**Step 2: Define method-validation records**

Each method records intended use, scope/limitations, versions, fixtures, ground truth, accuracy, precision/recall where applicable, false-positive and false-negative results, repeatability, reproducibility, environmental conditions, uncertainty where relevant, acceptance criteria, deviations, reviewer, approval, and revalidation trigger.

**Step 3: Define corpus governance**

Separate public, synthetic, and restricted corpora. Every fixture needs provenance, license/legal basis, sensitivity, expected result, hashes, generation recipe when synthetic, retention, access control, and redistribution rule. Real case evidence may never be copied into the public corpus.

**Step 4: Define ISO/IEC 17043-aligned proficiency support**

Specify blind-test package import, assigned values, participant result sealing, scoring, homogeneity/stability records where applicable, disclosure control, appeals/corrections, and export. State clearly that software support does not make Trareon Lab a proficiency-testing provider.

**Step 5: Define regression gates**

No parser or analysis-pack release passes if it increases critical false positives, loses previously supported ground-truth artifacts, weakens determinism, or changes a report conclusion without a reviewed method-version change.

**Step 6: Verify and commit**

```bash
rg -n "UNIT_VERIFIED|CORPUS_VALIDATED|CROSS_TOOL_CORROBORATED|LAB_METHOD_APPROVED" docs/VALIDATION-STRATEGY.md
rg -n "false-positive|false-negative|repeatability|reproducibility|revalidation" docs/METHOD-VALIDATION-RECORD.md
rg -n "public|synthetic|restricted|redistribution" docs/CORPUS-GOVERNANCE.md
rg -n "does not make Trareon Lab a proficiency-testing provider" docs/PROFICIENCY-TESTING-PROFILE.md
git diff --check
git add docs/VALIDATION-STRATEGY.md docs/CORPUS-GOVERNANCE.md docs/METHOD-VALIDATION-RECORD.md docs/PROFICIENCY-TESTING-PROFILE.md validation
git commit -m "docs: define forensic validation program"
```

---

## Task 8: Set Platform, Hardware, and Performance Budgets

**Files:**

- Create: `docs/SUPPORTED-PLATFORMS.md`
- Create: `docs/REFERENCE-HARDWARE-AND-PERFORMANCE-PROFILE.md`
- Create: `docs/PERFORMANCE-BENCHMARK-PROTOCOL.md`
- Modify: `docs/DECISION-REGISTER.md`

**Step 1: Freeze the initial OS matrix**

List exact Windows editions/build ranges, macOS versions and architectures, and Linux distributions/display systems. For each combination define installer format, signing/notarization expectation, filesystem access limits, privilege requirements, live-host limitations, and CI/physical validation coverage.

No “Windows/macOS/Linux” umbrella claim is allowed without the exact matrix.

**Step 2: Define three reference hardware tiers**

- Minimum field/lab workstation;
- Recommended examiner workstation;
- High-volume workstation.

Record CPU cores/architecture, RAM, system storage, evidence storage, sustained read/write throughput, GPU status, and screen resolution. Separate required capacity from benchmark machine identity.

**Step 3: Define measurable budgets**

At minimum set budgets for cold start, case open, `.fsnap` manifest verification, hashing throughput, indexing throughput, search p50/p95, timeline first render, one-million-row navigation, memory overhead per indexed item, cancellation latency, crash recovery, report generation, and temporary-storage amplification.

Each metric includes dataset identity, warm/cold state, concurrency, percentile, hardware tier, pass threshold, and measurement tool.

**Step 4: Define degraded-operation behavior**

Specify low-disk, low-memory, slow removable media, network share, encrypted source, corrupted source, and thermal-throttling behavior. The application must surface degraded confidence/coverage and may not silently skip work to meet a performance target.

**Step 5: Accept ADR-009**

Require physical-machine validation for installer, filesystem privileges, and performance claims. Virtual machines may supplement but not replace it.

**Step 6: Verify and commit**

```bash
rg -n "Windows|macOS|Linux|installer|privilege" docs/SUPPORTED-PLATFORMS.md
rg -n "Minimum|Recommended|High-volume|CPU|RAM|throughput" docs/REFERENCE-HARDWARE-AND-PERFORMANCE-PROFILE.md
rg -n "p50|p95|cancellation|temporary-storage|degraded" docs/PERFORMANCE-BENCHMARK-PROTOCOL.md
rg -n "ADR-009.*ACCEPTED" docs/DECISION-REGISTER.md
git diff --check
git add docs/SUPPORTED-PLATFORMS.md docs/REFERENCE-HARDWARE-AND-PERFORMANCE-PROFILE.md docs/PERFORMANCE-BENCHMARK-PROTOCOL.md docs/DECISION-REGISTER.md
git commit -m "docs: set platform and performance budgets"
```

---

## Task 9: Resolve Licensing, Supply Chain, and Specialist-Engine Strategy

**Files:**

- Create: `docs/DEPENDENCY-AND-LICENSE-POLICY.md`
- Create: `docs/SUPPLY-CHAIN-SECURITY.md`
- Create: `docs/MEMORY-ENGINE-STRATEGY.md`
- Create: `docs/CODEC-AND-DECODER-MATRIX.md`
- Modify: `docs/DECISION-REGISTER.md`

**Step 1: Define the offline dependency rule precisely**

“No external dependency” means the installed application requires no separately installed runtime or forensic application for core R1 workflows. Bundled libraries are permitted only when source/provenance, license, version pin, hashes, SBOM entry, update policy, vulnerability process, and validation coverage are controlled.

Do not rewrite mature parsers or codecs merely to avoid packaging a dependency. Reimplementation requires a documented security, licensing, determinism, or portability reason plus equivalent validation evidence.

**Step 2: Define supply-chain controls**

Require lockfiles, pinned toolchains, reproducible-build targets, isolated builders, artifact signing, provenance attestations, SBOM, license notices, dependency review, SAST, secret scanning, fuzzing for parsers, vulnerability intake, revocation, and offline update verification.

**Step 3: Decide the memory-engine strategy**

Compare a native Trareon implementation, a protocol-isolated bundled engine, and selected reusable libraries. Evaluate profile/symbol management, licensing, security boundary, deterministic output, validation corpus, supported OS/kernel matrix, update cadence, and maintainability. Accept ADR-011 only with explicit formats and profiles for R1 or `P0-LATER`.

**Step 4: Decide decoder/codec strategy**

For every R1 image/video/audio/document type, record parser/decoder, license, patent risk, sandbox boundary, metadata coverage, corruption handling, validation corpus, and fallback. Accept ADR-010 only after legal/license review.

Accept ADR-011 only after the memory-engine comparison is complete. If memory analysis is not in R1, the ADR must still lock the isolation/API strategy and mark the exact engine/profile decision as a named `P0-LATER` release gate rather than leaving an implicit dependency.

**Step 5: Verify and commit**

```bash
rg -n "no separately installed runtime|SBOM|reimplementation" docs/DEPENDENCY-AND-LICENSE-POLICY.md
rg -n "lockfile|reproducible|attestation|fuzz|revocation" docs/SUPPLY-CHAIN-SECURITY.md
rg -n "profile|symbol|license|sandbox|validation" docs/MEMORY-ENGINE-STRATEGY.md
rg -n "patent|corruption|fallback|legal" docs/CODEC-AND-DECODER-MATRIX.md
rg -n "ADR-010.*ACCEPTED|ADR-011.*ACCEPTED" docs/DECISION-REGISTER.md
git diff --check
git add docs/DEPENDENCY-AND-LICENSE-POLICY.md docs/SUPPLY-CHAIN-SECURITY.md docs/MEMORY-ENGINE-STRATEGY.md docs/CODEC-AND-DECODER-MATRIX.md docs/DECISION-REGISTER.md
git commit -m "docs: resolve dependency and specialist engine policy"
```

---

## Task 10: Create the Foundation Implementation Plan

**Files:**

- Create: `docs/superpowers/plans/2026-07-17-trareon-lab-foundation.md`
- Modify: `docs/TRACEABILITY-INDEX.md`
- Modify: `docs/DECISION-REGISTER.md`

**Step 1: Check Gates A–D**

Run a gate script or documented checklist that confirms ADR-001 through ADR-012 are `ACCEPTED`; every R1 requirement has an owner; and no mandatory artifact contains forbidden ambiguity.

Expected: all checks pass. If any fail, do not create a code implementation plan.

**Step 2: Write the separate Foundation plan**

Use the `writing-plans` skill. The plan must implement only the Foundation milestone from Task 3 and must use exact files from the selected RFC repository structure.

It must include TDD tasks for:

1. workspace/toolchain/CI bootstrap;
2. typed error and result model;
3. versioned case database and migration harness;
4. case state machine and exclusive lock;
5. content-addressed blob/derived store;
6. evidence, provenance, audit, and coverage schemas;
7. hash-chain and signature-envelope verification;
8. hostile `.fsnap` preflight and safe importer;
9. bounded worker/cancellation protocol;
10. minimal desktop case/evidence/coverage UI;
11. deterministic export/report skeleton;
12. documentation shell matching Trareon Acquire’s installed documentation experience;
13. cross-platform packaging smoke tests;
14. security/fuzz/property/rollback tests;
15. release-gate evidence bundle.

Every task must contain the failing test first, minimal implementation, targeted test, broader regression test, exact paths, and a small commit.

**Step 3: Link R1 requirements to Foundation tests**

Update `docs/TRACEABILITY-INDEX.md` so every Foundation-owned R1 requirement points to a planned test file and acceptance command. Do not mark requirements implemented.

**Step 4: Verify and commit**

```bash
test -f docs/superpowers/plans/2026-07-17-trareon-lab-foundation.md
rg -n "Write the failing test|Run test to verify it fails|Implement minimal|Run test to verify it passes|Commit" docs/superpowers/plans/2026-07-17-trareon-lab-foundation.md
rg -n "TBD|TODO|placeholder|later decide" docs/superpowers/plans/2026-07-17-trareon-lab-foundation.md && exit 1 || true
git diff --check
git add docs/superpowers/plans/2026-07-17-trareon-lab-foundation.md docs/TRACEABILITY-INDEX.md docs/DECISION-REGISTER.md
git commit -m "docs: add Trareon Lab Foundation implementation plan"
```

---

## Task 11: Run Independent Readiness Reviews and Close Gate E

**Files:**

- Create: `docs/reviews/PRODUCT-READINESS-REVIEW.md`
- Create: `docs/reviews/SECURITY-ARCHITECTURE-REVIEW.md`
- Create: `docs/reviews/FORENSIC-METHOD-REVIEW.md`
- Create: `docs/reviews/INDONESIA-LEGAL-AND-STANDARDS-REVIEW.md`
- Create: `docs/reviews/ACCESSIBILITY-AND-UX-REVIEW.md`
- Create: `docs/FOUNDATION-READINESS-CHECKLIST.md`
- Modify: `docs/DECISION-REGISTER.md`

**Step 1: Perform five distinct reviews**

- Product review: R1 usefulness, workflow completeness, scope realism, documentation.
- Security review: hostile evidence, sandbox, IPC, cryptography, supply chain, recovery.
- Forensic-method review: integrity, provenance, validation, uncertainty, coverage, reproducibility.
- Indonesian legal/standards review: authority/scope, privacy, retention, admissibility-supporting records, safe conformity claims.
- Accessibility/UX review: progressive disclosure, keyboard operation, screen scaling, error recovery, novice/expert workflows.

The same person may not self-approve the artifact they authored for security, forensic-method, or legal review.

**Step 2: Record findings with closure evidence**

Each review finding includes severity, affected artifact/requirement, evidence, owner, required action, state, closure evidence, and reviewer acceptance. Critical or high open findings block Gate E.

**Step 3: Execute the final readiness checklist**

The checklist must confirm:

- Gates A–D passed;
- no unassigned R1 requirement;
- no unresolved critical/high review finding;
- stack, OS matrix, crypto, pack sandbox, `.fsnap`, R1 scope, and performance budgets accepted;
- validation and documentation work are scheduled inside Foundation, not deferred after release;
- Foundation plan tasks are independently testable and small enough for review;
- PRD, RFC, decisions, contracts, and plan agree on terminology and boundaries.

**Step 4: Verify, mark Gate E, and commit**

```bash
required_gate_e_files=(
  docs/FOUNDATION-READINESS-CHECKLIST.md
  docs/reviews/PRODUCT-READINESS-REVIEW.md
  docs/reviews/SECURITY-ARCHITECTURE-REVIEW.md
  docs/reviews/FORENSIC-METHOD-REVIEW.md
  docs/reviews/INDONESIA-LEGAL-AND-STANDARDS-REVIEW.md
  docs/reviews/ACCESSIBILITY-AND-UX-REVIEW.md
)
for required_file in "${required_gate_e_files[@]}"; do
  test -f "$required_file" || {
    printf 'Missing mandatory Gate E artifact: %s\n' "$required_file" >&2
    exit 1
  }
done

if rg -n "Gate E.*PASS" docs/FOUNDATION-READINESS-CHECKLIST.md; then
  :
else
  gate_e_status=$?
  exit "$gate_e_status"
fi

if rg -n "OPEN.*(CRITICAL|HIGH)|(CRITICAL|HIGH).*OPEN" docs/reviews; then
  exit 1
else
  review_status=$?
  test "$review_status" -eq 1 || exit "$review_status"
fi

if rg -n "TBD|TODO|supports all|and others" . \
  --glob 'docs/**' \
  --glob 'RFC-Digital-Forensic-Analysis-Lab.md' \
  --glob '!docs/superpowers/plans/2026-07-17-trareon-lab-rfc-foundation-preparation.md'; then
  exit 1
else
  forbidden_status=$?
  test "$forbidden_status" -eq 1 || exit "$forbidden_status"
fi
git diff --check
git status --short
git add docs/reviews docs/FOUNDATION-READINESS-CHECKLIST.md docs/DECISION-REGISTER.md
git commit -m "docs: approve Trareon Lab Foundation readiness"
```

Expected: Gate E is `PASS`, no critical/high findings remain open, and the working tree is clean after commit.

---

## Definition of Done

This preparation plan is complete only when:

- all five gates are `PASS`;
- the selected architecture passes every mandatory proof-of-capability test on the exact supported OS matrix;
- all PRD requirements appear exactly once in the Release 1 capability matrix and traceability index;
- R1 promises name exact formats, versions, operations, validation level, and limitations;
- case, evidence, provenance, audit, coverage, `.fsnap`, cryptographic, pack, and AI contracts are versioned and reviewable;
- the threat model and validation program have independent review evidence;
- performance claims use named hardware, corpus, percentile, and pass thresholds;
- licensing and supply-chain obligations are known before dependency adoption;
- the separate Foundation plan is TDD-based, file-specific, and ready for execution;
- no production code has been started prematurely.

## Plans Intentionally Deferred

After the Foundation milestone passes its own release gate, create separate implementation plans for:

1. Storage and Filesystem Analysis;
2. OS Artifact and Unified Timeline Analysis;
3. Memory and Network Analysis;
4. Deterministic Correlation and Optional Local AI;
5. Multimedia and Specialist Analysis Packs;
6. Connected Companion Services, only after a separate online threat model and user approval.

These plans may share schemas and the Rust core API, but must remain independently testable, releasable, and reversible.
