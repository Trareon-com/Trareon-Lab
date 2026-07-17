# Path D — send reviewer drafts + draft build pack

**Status:** SEND PACKET READY — invitations NOT_SENT  
**Drafts:** `docs/reviews/REVIEWER-BOOKING.md`

## Recipients (operator fills)

| Review | Recipient | Sent date | Ack date |
|---|---|---|---|
| Indonesia legal/quality | | | |
| External crypto | | | |

## Draft build pack (assemble locally)

```bash
bash scripts/build-reviewer-draft-pack.sh
# Output: dist/reviewer-draft-pack/ (gitignored under dist/)
```

Pack includes: capability matrix, selling-unsigned, runbook, dossiers, known issues, release notes draft, ADR register excerpt.

## Deadlines

- Indonesia draft comments: **2026-09-25** → `docs/reviews/INDONESIA-LEGAL-COMMENTS-2026-09-25.md`
- Indonesia sign-off: **2026-10-09** → `docs/reviews/INDONESIA-OFFICIAL-SIGNOFF.md`
- Crypto review: **2026-09-30** → `docs/reviews/CRYPTO-EXTERNAL-REVIEW.md`
