# Case Lifecycle Contract

**Schema version:** case-lifecycle-1  
**ADR:** ADR-004 ACCEPTED

## States

Exact states only: `CREATED`, `OPEN`, `READ_ONLY`, `RECOVERY_REQUIRED`, `CLOSED`, `ARCHIVED`.

| State | Meaning | Writable derived store | Export eligibility |
|---|---|---|---|
| CREATED | Case metadata recorded; no evidence yet | yes | no official report |
| OPEN | Active examination in one exclusive process | yes | draft exports only |
| READ_ONLY | Review/authorization without mutation of derived analysis | no | yes with review flags |
| RECOVERY_REQUIRED | Unclean shutdown or lock anomaly detected | blocked until recovery | no |
| CLOSED | Sealed; hash-chained seal recorded | no | yes sealed |
| ARCHIVED | Offline retention layout; reopen creates new OPEN session with audit | no | archive export only |

## Legal transitions

`CREATED → OPEN`  
`OPEN → READ_ONLY | RECOVERY_REQUIRED | CLOSED`  
`READ_ONLY → OPEN | CLOSED`  
`RECOVERY_REQUIRED → OPEN | CLOSED` (after recovery audit)  
`CLOSED → ARCHIVED | READ_ONLY` (authorized reopen for review)  
`ARCHIVED → READ_ONLY` (restore for review only)

Forbidden: `CLOSED → OPEN` without authorization audit; any transition that shares writable caches across cases.

## Lock ownership

- One exclusive process lock file per case directory (`case.lock`) containing case UUID, holder PID, host, opened_at_utc.
- Second process must fail closed while lock is held by a live PID.
- Stale lock from dead PID may be recovered into `RECOVERY_REQUIRED` with audit event.

## Unclean shutdown

On next open: verify seal/lock digests; if mismatch or interrupted run manifest exists → `RECOVERY_REQUIRED`. Recovery replays idempotent checkpoints and emits coverage + audit events.

## Clock handling

All events store UTC instant plus original UTC offset. Monotonic per-case sequence numbers are authoritative for ordering when wall clocks skew.

## Export eligibility

Official/authorized reports require `CLOSED` or authorized `READ_ONLY` with dual-control when policy requires. Draft exports allowed from `OPEN` and must be labeled non-final.
