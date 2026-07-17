# CASE/UCO Interoperability Profile

**Profile ID:** trareon-case-uco-2026.07  
**Ontology target:** CASE 1.3.0 / UCO 1.3.0 subset (version-pinned)

## Mapping classes

| Trareon concept | CASE/UCO mapping | Mapping class |
|---|---|---|
| Case | `case:Case` | lossless within profile |
| Evidence object | `observable:File` / `observable:Disk` as applicable | lossy for Lab-specific validation_state (extension) |
| Provenance event | `core:Action` | lossless for profile actions |
| Actor | `identity:Identity` | lossless |
| Location | `location:Location` | lossless when present |
| Relationship | `core:Relationship` | lossless for listed predicates |
| Finding | Trareon extension `trareon:Finding` | Trareon extension |
| Timeline event | `core:Action` + time | lossy aggregation |
| Hash | `types:Hash` | lossless for SHA-256 |

## Rules

- Stable identifiers: Trareon UUIDs exported as `@id` IRIs under `urn:trareon:lab:`.
- Unknown vocabulary terms on import: preserve as extension bag; never drop silently.
- Round-trip expectation: profile-covered fields round-trip losslessly; extensions may be Lab-only.
- Unsupported concepts remain in native Trareon records as authoritative.
