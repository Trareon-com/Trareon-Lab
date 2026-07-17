# `.fsnap` Read / Import Contract

**Schema version:** fsnap-read-1  
**ADR:** ADR-006 ACCEPTED  
**Compatibility:** Trareon Acquire `.fsnap` specification versions listed below

## Compatibility table

| Acquire `.fsnap` version | Lab support | Notes |
|---|---|---|
| 1.0 | verify + import | baseline |
| 1.1 | verify + import | additive manifest fields ignored if unknown and non-critical |
| unknown major | reject | open cross-product ADR; no silent rewrite |

## Verification requirements

Package version negotiation; manifest canonicalization; declared hash verification; signature trust states (`VALID_TRUSTED`, `VALID_UNTRUSTED`, `INVALID`, `EXPIRED_OR_REVOKED`, `NOT_SIGNED`, `NOT_CHECKED`); missing/extra member detection; path traversal rejection; decompression limits; duplicate names rejected; symlinks rejected unless explicitly allowlisted; sparse files preserved as sparse derived views; timestamp preservation recorded; encrypted members fail closed without key; unknown extensions quarantined as unsupported members; partial/corrupt packages fail closed; cancellation cooperative; resumability via checkpointed member digests; deterministic import result schema (`schemas/fsnap-import-result.schema.json`).

## Non-goals

Trareon Lab must not silently repair or rewrite a source `.fsnap` package.
