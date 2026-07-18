# YARA and hash-set validation dossier

## Status

**CORPUS_VALIDATED** for the frozen rule and hash-set subset below.

## Scope

| Capability | Implementation | Validation level |
|---|---|---|
| YARA compilation and scanning | `yara-x` 1.19.0, exact Cargo pin | Validated |
| Rule loading | Sorted `.yar` and `.yara` files plus built-in EICAR substring rule | Validated |
| Rule semantics | Frozen text, hex, metadata, and condition corpus | Validated |
| Hash-set pin | `trareon-hashset-db-1:sha256:<digest>` over database bytes | Validated |

`YARA_ENGINE_ID` is `yara-x/1.19.0`. The dependency is BSD-3-Clause and is
built with `constant-folding`, `exact-atoms`, and `fast-regexp`; optional YARA
modules are not enabled in this validation slice.

## Golden corpus

`crates/lab-yara/tests/fixtures/production.yar` contains:

- a case-insensitive text plus hex rule requiring both strings;
- a two-text rule allowing either string;
- severity and family metadata checked at the hit boundary.

`crates/lab-yara/tests/golden.rs` verifies false-positive resistance for the
AND condition and exact rule, metadata, offset, and matched-byte output. Each
matching rule produces one `YaraHit`, using its earliest observed pattern
match. The caller remains responsible for representing a hit as an
`Indication`; a hit is not itself a finding.

The hash-set test verifies the format version and the SHA-256 digest against a
known vector. Hashing is streamed in 64 KiB blocks.

## Reproduction

```text
cargo test -p lab-yara
cargo test -p lab-hashset
```

## Limits and promotion path

`Validated` applies only to the frozen corpus described above; it is not a
claim that every YARA language feature, module, or hostile rule is covered.
Broader promotion requires malformed-rule cases, large-file performance
results, and cross-platform release runs. Revalidate whenever the YARA-X
version, enabled features, built-in rules, fixture corpus, or hash-set format
version changes.
