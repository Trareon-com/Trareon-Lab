# Analysis Pack Contract

**ADR-005:** ACCEPTED

## Manifest

See `schemas/analysis-pack-manifest.schema.json`. Each pack declares identity, publisher, version, compatible core API, evidence/media types, parsers, requested capabilities, resource limits, output schemas, deterministic/non-deterministic ops, network need, licenses, signatures, validation corpus identity.

## Default capabilities (deny by default)

no network; no host filesystem; no process launch; no environment secrets; no clipboard; no arbitrary native library loading; read-only brokered evidence ranges; write-only content-addressed derived output.

## Execution

Packs execute **out of process** with CPU, memory, output-size, recursion, file-count, and wall-time limits. Crash or timeout produces a coverage event and must not crash the case process.

## Trust states

`BUNDLED_VALIDATED`, `SIGNED_TRUSTED`, `SIGNED_UNTRUSTED`, `UNSIGNED_BLOCKED`, `REVOKED`, `INCOMPATIBLE`.

Developer mode must be explicit, visibly persistent, case-audited, and excluded from validated reports unless the reviewer accepts the deviation.
