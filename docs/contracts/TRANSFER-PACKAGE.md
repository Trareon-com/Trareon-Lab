# Transfer Package Contract

**Schema version:** transfer-package-1  
**ADR:** ADR-008 amendment (FR-ART-006 offline collaboration)  
**Related:** `schemas/transfer-package.schema.json`, `schemas/signature-envelope.schema.json`

## Purpose

Offline collaboration between analysts **without** a central server: export a minimized, signed package of selected bookmarks/findings citations, then import into another case with disclosure preview and integrity verification.

## Required fields

| Field | Type | Notes |
|---|---|---|
| schema_version | const `transfer-package-1` | |
| transfer_uuid | UUID | |
| source_case_uuid | UUID | Origin case |
| created_at_utc | date-time | |
| destination | string | Intended recipient/org label |
| purpose | string | Why shared |
| authority_note | string | Authority/approval note (org-defined) |
| selected_bookmark_digests | string[] | SHA-256 hex of canonical bookmark JSON |
| result | enum | `export_ready`, `imported`, `rejected` |
| signature | object | Ed25519 signature envelope fields (algorithm, key_id, payload_digest, signature, trust_state) |

## Disclosure preview (required before import applies)

Importer UI/API must show destination, purpose, authority_note, counts of bookmarks, and payload digest **before** writing into the destination case.

## Rules

- Fail closed on tampered signature → `INTEGRITY_FAILED` / result `rejected`.
- No silent inclusion of raw evidence bytes unless explicitly listed in a future package profile (R1: citations + digests only).
- Source `.fsnap` / disk images are **not** embedded in R1 transfer packages.

## Non-goals

- Real-time sync.
- Cross-case writable shared store.
