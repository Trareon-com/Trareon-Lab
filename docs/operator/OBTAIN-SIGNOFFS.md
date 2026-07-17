# Obtain O8 / O9 human sign-offs

**Windows lab not required for these steps.**

## O8 — Indonesia legal/quality

1. Send pack: `bash scripts/build-reviewer-draft-pack.sh` + `docs/operator/PATH-D-REVIEWER-SEND.md`
2. Hold meeting: `docs/reviews/LEGAL-QUALITY-MEETING-AGENDA.md`
3. File comments: `docs/reviews/INDONESIA-LEGAL-COMMENTS-2026-09-25.md`
4. Collect wet/digital signature on `docs/reviews/INDONESIA-OFFICIAL-SIGNOFF.md`
5. Record acceptance in `docs/operator/HUMAN-SIGNOFFS-METADATA.md`

## O9 — External cryptography review

1. Send crypto invitation from `docs/reviews/REVIEWER-BOOKING.md`
2. When the written review arrives, paste/link it into `docs/reviews/CRYPTO-EXTERNAL-REVIEW.md` and flip status from `NOT_RECEIVED`
3. Fill receipt: `docs/reviews/CRYPTO-EXTERNAL-REVIEW-RECEIPT.md`
4. Release Manager marks Accepted in `docs/operator/HUMAN-SIGNOFFS-METADATA.md`

Do not run `scripts/cut-official-v1.sh` until both O8 and O9 are Accepted and O1–O3 evidence exists (Windows O1 via `docs/WINDOWS-LAB-QUEUE.md`).
