# Transfer security review (Official)

**Date:** 2026-07-17  
**Scope:** FR-ART-006 bookmark + signed transfer package (R1)  
**Evidence:** `docs/contracts/TRANSFER-PACKAGE.md`, `schemas/transfer-package.schema.json`, `crates/lab-transfer/tests/transfer_sign.rs`, `crates/lab-core/tests/schema_fixtures.rs`, hostile transfer fixtures under `fixtures/contracts/`

## Verdict

**PASS** for Engineering Alpha / sellable-unsigned: schema validation, Ed25519 sign/verify path, and fail-closed invalid fixtures are covered by automated tests.

**NOT Official O11 PASS.** Official O11 still requires freeze-build evidence log (`release-evidence/OFFICIAL-1.0.0/o11-bookmark-transfer.log`) plus UI disclosure-preview confirmation on the release candidate.

## Threat model (R1)

| ID | Threat | Control | Residual |
|---|---|---|---|
| TR-1 | Tampered package accepted | Ed25519 envelope verify → `rejected` / `INTEGRITY_FAILED` | Key compromise outside product scope |
| TR-2 | Silent evidence exfil via transfer | R1 package = citations + digests only; no embedded disk/.fsnap | Future profiles must re-review |
| TR-3 | Import without disclosure | Contract requires destination/purpose/authority preview before apply | Operator must use Official UI path; CLI misuse is residual |
| TR-4 | Schema confusion / downgrade | `schema_version` const + fixture invalids | Keep schema aliases fail-closed |
| TR-5 | Untrusted-but-valid signature treated as trusted | Trust states include `VALID_UNTRUSTED` | Org trust-bundle import is operator process |

## Findings

| ID | Severity | Finding | Disposition |
|---|---|---|---|
| TS-1 | — | Valid/invalid transfer fixtures classified correctly | Accepted |
| TS-2 | — | Sign/import round-trip covered in `lab-transfer` | Accepted |
| TS-3 | Residual | O11 Official evidence log not yet filled | Open until freeze |
| TS-4 | Residual | External crypto review (O9) still NOT_RECEIVED for production keys | Open — Path D |

## Gate

Transfer Engineering Alpha exit: **PASS**.  
Official O11 exit: **OPEN** (see `docs/operator/FINAL-8-GATES.md`).
