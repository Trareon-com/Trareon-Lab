# Lab Core Perfect freeze

Lab Core Perfect is frozen only when this file exists, the static gate
`bash scripts/check-lab-core-perfect-gate.sh` is green, and the listed crate
test commands are green on the same source revision.

## Freeze checklist

- [x] Record the source revision and UTC build timestamp (branch
  `cursor/lab-presale-hardening`, engineering freeze 2026-07-18).
- [x] Confirm `local_ai` and `demo_seed` are off by default.
- [x] Run `cargo test -p lab-slint --features gui`.
- [x] Run `cargo test -p lab-core -p lab-storage`.
- [x] Run `bash scripts/check-lab-core-perfect-gate.sh`.
- [x] Run `graphify update .`.
- [x] Preserve the test logs and `packaging/SHA256SUMS` with release evidence.
- [x] Confirm APFS, E01, YARA/hash-set, search, and export dossiers state their
  exact validated subsets and explicit limitations.
- [x] Confirm unsigned, lab-use-only, not-court-ready, and not-ISO-certified
  disclosures remain visible.

This engineering freeze does not claim accreditation, legal admissibility,
full PDF/A conformance, universal E01/APFS compatibility, signing, or
notarization.
