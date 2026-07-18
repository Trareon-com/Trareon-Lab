# E01 validation dossier (M8)

## Status

**CORPUS_VALIDATED** / **Validated** for the single-segment EWF v1 subset below.

## Validated subset

- EVF header plus `header`, `volume`, `sectors`, `table`, and `done` sections.
- Zlib-compressed and uncompressed chunks with per-chunk CRC-32.
- Linear random reads across multiple chunks.
- Case, examiner, evidence-number, description, and hash-algorithm metadata.
- Import into the evidence/provenance ledger and whole-image SHA-256.
- Fail-closed handling for truncated headers and out-of-bounds section lengths.

Evidence: `crates/lab-storage/tests/e01_roundtrip.rs`,
`crates/lab-storage/tests/e01_hostile.rs`, and
`cargo test -p lab-storage`.

## Explicit limitations

This validation does not cover multi-segment E01 sets, Ex01/EWF2, encrypted
containers, every third-party EWF producer variant, recovery of damaged
containers, or media-error reconstruction. Inputs outside the documented
single-segment subset fail closed; they must not be described as fully
validated E01 support.
