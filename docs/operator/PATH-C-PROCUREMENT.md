# Path C — certificate procurement packet

**Status:** PACKET READY — registration/purchase NOT_STARTED  
**Owner:** Release Engineering  
**Do not commit secrets** (Team ID may be recorded; private keys never)

## Apple Developer Program (O2 prerequisite)

| Field | Value |
|---|---|
| Action | Register / renew Apple Developer Program |
| Invoice reference | _(operator)_ |
| Team ID | _(operator — public org id OK)_ |
| Expiry | _(operator)_ |
| Recorded in runbook | NOT_STARTED |
| Next decision date if blocked | 2026-08-15 |

## Windows Authenticode (O1 prerequisite)

| Field | Value |
|---|---|
| Preferred type | EV code-signing |
| Vendor | _(operator)_ |
| Order / ticket | _(operator)_ |
| ETA | _(operator)_ |
| Escalate if ETA > 2026-09-15 | See `docs/operator/PATH-C-ESCALATION.md` |
| Recorded in runbook | NOT_STARTED |

## Linux signing key (O3 prerequisite)

| Field | Value |
|---|---|
| Plan | `docs/LINUX-PACKAGE-SIGNING-KEY-PLAN.md` |
| Public key path | `release-evidence/OFFICIAL-1.0.0/linux-signing-pubkey.asc` |
| Generated | NOT_STARTED |

## Status-call log

| Date | Path | Result | Next |
|---|---|---|---|
| _(open)_ | C | | |
| _(open)_ | D | | |
