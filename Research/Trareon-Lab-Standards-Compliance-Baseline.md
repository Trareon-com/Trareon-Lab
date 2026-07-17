# Trareon Lab — Standards and Compliance Baseline

**Snapshot date:** 2026-07-17
**Purpose:** Product-control baseline for an offline forensic analysis application.
**Authority:** Official ISO catalogue/status pages, NIST, SWGDE, and CASE/UCO sources only.

## 1. Executive position

Trareon Lab should be described as **supporting laboratories and examiners in implementing controlled, traceable, repeatable digital-forensic workflows**. The software itself must not be described as ISO-certified, ISO-compliant, accredited, court-admissible, or scientifically infallible.

The applicable standards govern different subjects:

- ISO/IEC 27037, 27041, 27042, and 27043 guide digital-evidence handling and investigation methods.
- ISO/IEC 17025 governs the competence, impartiality, and consistent operation of a testing/calibration laboratory—not a software product.
- ISO/IEC 17043 governs proficiency-testing providers and schemes—not ordinary case-analysis software.
- ISO/IEC 27001 and 27701 govern organizational management systems. They are relevant to the maker/operator and handling of sensitive case data, but do not validate a forensic conclusion.

The full standards are copyrighted and mostly paywalled. This baseline uses their official catalogue abstracts and status data; a clause-level conformity mapping requires licensed copies reviewed by qualified quality/legal personnel.

## 2. Current editions and status

| Reference | Current catalogue edition | Status on 2026-07-17 | Product relevance |
|---|---|---|---|
| [ISO/IEC 27037:2012](https://www.iso.org/standard/44381.html) | Edition 1, 2012-10 | Published; catalogue stage 90.60 and labels it under review | Identification, collection, acquisition, preservation, and exchange of potential digital evidence |
| [ISO/IEC 27041:2015](https://www.iso.org/standard/44405.html) | Edition 1, 2015-06 | Published; systematic review, stage 90.20 | Demonstrating that investigative methods are fit for purpose, including validation and external test evidence |
| [ISO/IEC 27042:2015](https://www.iso.org/standard/44406.html) | Edition 1, 2015-06 | Published; systematic review, stage 90.20 | Continuity, validity, reproducibility, repeatability, method selection, interpretation, and independent scrutiny |
| [ISO/IEC 27043:2015](https://www.iso.org/standard/44407.html) | Edition 1, 2015-03 | Published; catalogue stage 90.60 after review opened in 2025 | Investigation process from preparation through closure |
| [ISO/IEC 17025:2017](https://www.iso.org/standard/66912.html) | Edition 3, 2017-11 | Published; confirmed in 2023, stage 90.93 | Laboratory competence, impartiality, consistent operation, and confidence in results |
| [ISO/IEC 17043:2023](https://www.iso.org/standard/80864.html) | Edition 2, 2023-05 | Published, stage 60.60 | Competence/impartiality of proficiency-testing providers and consistent PT schemes |
| [ISO/IEC 27001:2022](https://www.iso.org/standard/27001.html) + [Amd 1:2024](https://www.iso.org/standard/88435.html) | Edition 3 plus amendment | Published | Organizational ISMS and security-risk management |
| [ISO/IEC 27701:2025](https://www.iso.org/standard/27701.html) | Edition 2, 2025-10 | Published, stage 60.60; 2019 edition withdrawn | Organizational PIMS for PII controllers/processors |

**Change-control rule:** Store this table as a versioned standards register, assign an owner, and re-check all stage 90.20/90.60 items at least quarterly and before a major release.

## 3. Product controls derived from the standards

### 3.1 Evidence integrity and continuity

The official ISO/IEC 27037 scope covers identification, collection, acquisition, and preservation. SWGDE states that original evidence should be avoided for examination where possible, uncontained evidence should be write-protected, and static analysis should use a copy ([SWGDE Computer Forensic Examination](https://www.swgde.org/documents/published-complete-listing/18-f-001-swgde-best-practices-for-computer-forensic-examination/)).

Required controls:

- Default every imported source to immutable/read-only; never silently modify evidence.
- Verify `.fsnap` manifests and source hashes before analysis; preserve the received hash and calculate a local verification hash.
- Distinguish original, verified working copy, and every derivative artifact with immutable IDs.
- Record source container, byte offset/path, extraction method, parser version, rule-pack version, operator, timestamp, and parent artifact for every result.
- Use append-only, hash-chained audit events; corrections create superseding events and never rewrite history.
- Make write access to physical devices an explicit exceptional workflow with prominent warnings and recorded authorization.
- Preserve both raw timestamp values and normalized UTC values, including source time zone, clock offset, conversion rule, and ambiguity.

### 3.2 Fit-for-purpose methods and independent scrutiny

ISO/IEC 27041 calls for requirements, method descriptions, validation, and evidence that an implementation satisfies its requirements. ISO/IEC 27042 requires enough information for independent scrutiny and addresses validity, reproducibility, and repeatability.

Required controls:

- Give every parser/analyzer a stable method ID, owner, intended use, supported formats/versions, known limitations, and validation status.
- Pin the executable build, parser/rule versions, configuration, locale, time-zone database, and input hashes in each run manifest.
- Save machine-readable inputs, parameters, outputs, warnings, errors, and skipped/unsupported items.
- Expose a deterministic **Re-run exactly** action and a **Compare runs** view.
- Mark experimental, unvalidated, partially validated, and deprecated methods in the UI and reports.
- Require a written justification and approval when an examiner deviates from a validated method.
- Separate observation, automated classification, examiner interpretation, and conclusion; never present a heuristic hit as fact.
- Support second-method verification and technical/peer review with reviewer identity, disagreements, disposition, and sign-off.

### 3.3 Investigation process controls

ISO/IEC 27043 spans preparation through investigation closure. Product support should therefore be case-centered rather than a collection of disconnected viewers.

Required controls:

- Case request, legal authority, purpose, scope, permitted dates/artifacts/search terms, and handling restrictions.
- Pre-analysis evidence acceptance and integrity verification.
- Versioned examination plan, task assignments, contemporaneous notes, bookmarks, and findings.
- Explicit states: intake, verification, processing, examination, interpretation, review, reporting, amendment, closure, and retention/disposition.
- Scope-bound search and export controls; every export records requester, selection logic, destination, hashes, and disposition.
- Amendment workflow that references the released report, identifies changes, and preserves the original.

### 3.4 Laboratory-quality support

ISO says ISO/IEC 17025 is for laboratories and accreditation bodies use it as an accreditation criterion. It is not a product certification ([ISO/IEC 17025 official page](https://www.iso.org/standard/66912.html)).

Trareon Lab can support, but cannot establish, laboratory conformity through:

- Role-based authorization and separation of examiner, reviewer, report authorizer, and administrator duties.
- Competency records linking personnel to approved methods, training, authorizations, and expiry dates.
- Controlled SOP/method documents and acknowledgement of revisions.
- Equipment/workstation identity, software inventory, environment metadata, and maintenance/verification records.
- Nonconforming-work records, impact assessment, containment, correction, reprocessing, and notification.
- Internal audit evidence, corrective actions, risk records, and management-review exports.
- Controlled report templates and digital signatures with an independently verifiable package.

### 3.5 Proficiency-testing support

ISO/IEC 17043 applies to the **provider and operation of a proficiency-testing scheme**. A normal Trareon Lab deployment must not claim conformity merely because it includes test cases.

Useful supporting controls:

- Blind challenge-package import with participant isolation and embargoed expected results.
- Scheme/round ID, provider identity, instructions, deadlines, method constraints, and participant submission lock.
- Signed submissions, objective comparison to assigned values, reviewer records, scoring rationale, appeals, and amendment history.
- Separation of training datasets from formal PT rounds.
- Exportable evidence for the PT provider; the provider remains responsible for scheme design, competence, impartiality, and interpretation.

## 4. Validation and test-evidence baseline

[NIST CFTT](https://www.nist.gov/itl/csd/secure-systems-and-applications/computer-forensics-tool-testing-program-cftt) tests forensic tools by functionality using conformance/quality methodologies. [NIST Federated Testing](https://www.nist.gov/itl/csd/secure-systems-and-applications/computer-forensics-tool-testing-program-cftt/federated) supplies common test suites and report formats, including analysis-tool tests. [SWGDE Minimum Requirements for Testing Tools](https://www.swgde.org/documents/published-complete-listing/18-q-001-minimum-requirements-for-testing-tools-used-in-digital-and-multimedia-forensics/) requires testing before casework to determine expected performance and limitations.

Every released forensic method should have a signed validation dossier containing:

1. Method ID, version, intended use, scope, acceptance criteria, and risk classification.
2. Tool/build identity, source revision, compiler/build profile, bundled-data versions, and platform matrix.
3. Test design with positive, negative, boundary, malformed, adversarial, and unsupported-format cases.
4. Traceable datasets with hashes, construction notes, ground truth, and license/handling constraints.
5. Expected versus actual machine-readable output and preserved raw logs.
6. Repeatability tests on the same environment and reproducibility tests across supported OS/architectures.
7. Differential tests against an independent implementation where feasible; discrepancies are investigated, not majority-voted away.
8. Performance/resource observations where exhaustion could cause omission, truncation, or nondeterminism.
9. Known limitations, error conditions, false-positive/false-negative risks, and examiner mitigations.
10. Reviewer approval, deviations, unresolved risks, release decision, and revalidation triggers.

Revalidation triggers include parser/rule changes, dependency/compiler changes, OS changes, newly supported evidence versions, defect fixes affecting results, changed test ground truth, or unexplained cross-tool disagreement.

### NIST reference-data controls

[NSRL](https://www.nist.gov/itl/csd/secure-systems-and-applications/national-software-reference-library-nsrl/about-nsrl/nsrl) publishes a Reference Data Set used mainly to eliminate known OS/application files. The RDS is updated quarterly, and current releases use RDSv3 SQLite ([current RDS](https://www.nist.gov/itl/csd/secure-systems-and-applications/national-software-reference-library-nsrl/nsrl-download-0)).

- Bundle/import NSRL as a versioned, hashed offline dataset; record the exact RDS edition used per case.
- Treat an NSRL match as “known reference software,” not proof that a file is harmless.
- Do not hide matches irreversibly; preserve counts and allow an examiner to reveal them.
- Keep known-good, notable/known-bad, and organization-defined sets semantically separate.

## 5. Provenance and interoperable records

[CASE](https://caseontology.org/resources/case_design_document.html) exists to normalize, combine, validate, search, and exchange cyber-investigation information while tracing results to sources, actors, tools, and actions. Its provenance model uses `InvestigativeAction` and `ProvenanceRecord` ([CASE FAQ](https://caseontology.org/resources/faq.html)). CASE extends the [Unified Cyber Ontology](https://github.com/ucoProject/UCO).

Product direction:

- Use a native internal evidence graph that can export/import a version-pinned CASE/UCO profile.
- Assign globally unique IDs to cases, evidence, actions, tools, identities, locations, and derived artifacts.
- Represent each transformation as inputs → investigative action → outputs plus a new provenance record.
- Preserve uncertainty, confidence basis, data markings, and access restrictions; do not flatten them during export.
- Validate CASE/UCO exports against the pinned ontology/SHACL version and include the validation report.

## 6. Security and privacy management boundaries

ISO/IEC 27001:2022 is an organizational ISMS requirement; ISO/IEC 27701:2025 is an organizational PIMS requirement for PII controllers/processors. They are directly relevant because forensic case material commonly contains secrets, privileged material, and unrelated third-party PII, but they do not establish analytical validity.

Product-enabling controls should include offline-by-default operation, encryption at rest, least privilege, case isolation, configurable retention, secure deletion of derivatives/cache, export minimization, immutable access logs, protected secrets, signed updates/rule packs, and documented backup/restore verification. The deploying organization determines its ISMS/PIMS scope, risk treatment, lawful basis, roles, retention, and accreditation/certification decisions.

## 7. Claims boundary

### Acceptable wording

- “Designed to support workflows aligned with ISO/IEC 27037, 27041, 27042, and 27043.”
- “Provides records and controls that can support an organization’s ISO/IEC 17025 quality system.”
- “Supports proficiency-test administration; provider conformity to ISO/IEC 17043 is independently assessed.”
- “Produces traceable, versioned, independently verifiable analysis packages.”

### Prohibited without independent substantiation

- “ISO-certified forensic software,” “ISO/IEC 17025 compliant tool,” or “ISO/IEC 17043 accredited software.”
- “Court-approved,” “admissible evidence,” “error-free,” “scientifically proven,” or “guaranteed repeatable.”
- Claims that a hash match proves benignness/maliciousness, or that a parser output alone proves an event occurred.

## 8. Proposed PRD traceability matrix

Maintain one row per atomic control; never map a whole standard to one broad feature.

| Field | Required content |
|---|---|
| `TR-ID` | Stable identifier, e.g. `TR-LAB-VAL-001` |
| `Source` | Standard/document, edition/version, clause when licensed, official URL |
| `Source status` | Published/confirmed/under review and verification date |
| `Control objective` | Outcome required in plain language |
| `PRD requirement` | Testable “shall” statement |
| `Component/owner` | Implementing module and accountable owner |
| `Risk` | Integrity, omission, misinterpretation, privacy, security, or quality risk |
| `Verification method` | Inspection, automated test, dataset comparison, independent review, or audit |
| `Evidence artifact` | Test report, run manifest, audit export, signed package, screenshot, etc. |
| `Acceptance criteria` | Objective pass/fail threshold |
| `Release status` | Planned/implemented/validated/limited/deprecated |
| `Exceptions` | Approved deviation, rationale, approver, expiry, and residual risk |
| `Revalidation trigger` | Changes that invalidate prior evidence |

Example:

| TR-ID | Source | PRD requirement | Verification | Evidence |
|---|---|---|---|---|
| `TR-LAB-PROV-001` | ISO/IEC 27042:2015; CASE provenance | Every derived result shall retain a machine-verifiable path to input hash, action, tool/build, configuration, operator, and time. | Import fixed `.fsnap`, run twice on all supported OSs, compare provenance graph and outputs. | Signed run manifests, graph export, hashes, cross-platform comparison report. |

## 9. Release gates

1. No forensic parser/analyzer reaches “validated” without an approved dossier and preserved test data.
2. No case report can be finalized while source-integrity verification has failed or relevant critical processing errors remain undisclosed.
3. Every report lists evidence identifiers/hashes, method/tool versions, scope, results, limitations, deviations, and authorizer.
4. Every released build can reproduce a prior run in a preserved supported environment or explain why exact reproduction is impossible.
5. Standards status and controlled wording are reviewed before every major release.
