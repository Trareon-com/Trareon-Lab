# Evidence Object Contract

**Schema version:** evidence-object-1  
**Related schemas:** `schemas/evidence-object.schema.json`

## Required fields

| Field | Type | Notes |
|---|---|---|
| evidence_uuid | UUID v4 | Stable; independent of filename |
| case_uuid | UUID v4 | Owning case |
| source_designation | enum | `original`, `forensic_copy`, `verified_working_copy`, `logical_view`, `derivative` |
| evidence_class | string | e.g. `fsnap_package`, `disk_image`, `memory_image`, `pcap`, `file` |
| byte_length | integer ≥ 0 | |
| acquisition_hashes | array | algorithm + digest pairs from source |
| import_hashes | array | locally computed digests |
| original_locator | string | path/URI as received; not authoritative identity |
| authority_scope_link | UUID | link to authority/scope record |
| acquisition_tool | object | name, version, build |
| import_tool | object | name, version, build |
| time_source_metadata | object | source clock quality |
| write_protection_observation | enum | `observed_ro`, `not_observed`, `exception_write_authorized` |
| validation_state | enum | `VerifiedMatch`, `VerifiedMismatch`, `ComputedUnanchored`, `SignatureInvalid`, `IncompleteInput`, `Unsupported` |
| provenance_event_refs | UUID[] | ordered |

## Immutability

Source evidence bytes are never updated in place. Corrections create a new signed metadata revision linked to `previous_revision_uuid`.
