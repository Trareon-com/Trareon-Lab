# PDF/A-oriented and CASE/UCO export validation dossier

## Status

**CORPUS_VALIDATED** for the deterministic documented subsets below.

## Scope

| Export | Profile | Artifact validation level |
|---|---|---|
| Textual PDF | Deterministic PDF 1.4 structure with PDF/A-1b identification metadata | Validated |
| CASE/UCO JSON-LD | `trareon-case-uco-2026.07` subset | Validated |

Both exporters are deterministic for identical inputs, preserve draft
`labeled_non_final` semantics, and attach a SHA-256 digest to the returned
artifact.

## PDF evidence

`export_case_pdfa` emits:

- a `%PDF-1.4` header, catalog, pages tree, page, content stream, font,
  information dictionary, cross-reference table, trailer, and `startxref`;
- XMP metadata with PDF/A part `1` and conformance `B` identification;
- escaped title, report text, case identifier, status, and draft label.

The test checks required metadata and content markers and confirms that
`startxref` resolves to the emitted cross-reference table.

The corpus validates the emitted object graph, escaping, XMP marker, draft
label, digest, and cross-reference offsets. `Validated` applies only to that
documented writer subset. The minimal writer uses a standard Type 1 font
without embedding and has no ICC output intent, so this is not a claim of full
PDF/A conformance. Full-conformance promotion still requires an external
validator, embedded fonts, an output intent, Unicode corpus coverage, and
preserved validator logs.

## CASE/UCO evidence

`export_case_uco` emits parseable JSON-LD with:

- stable `urn:trareon:lab:<case_uuid>` identifiers;
- `case:Case`, `core:name`, report text, status, and draft label;
- the exact `CASE_UCO_PROFILE_VERSION` exported from `run_manifest.rs`;
- pinned CASE, UCO core, and Trareon extension contexts.

The corpus validates the documented CASE/UCO subset and exact pinned profile
version. It does not validate arbitrary CASE/UCO vocabulary. Broader promotion
requires JSON-LD expansion against frozen contexts, ontology validation,
round-trip tests for all added fields, and preserved interoperability results.

## Reproduction

```text
cargo test -p lab-core
```

Revalidate whenever the PDF object structure, CASE/UCO context, profile
version, identifier policy, or artifact validation-level policy changes.
